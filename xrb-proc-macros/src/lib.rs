// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod parsing;

use quote::{quote, ToTokens};
use syn::{parse_macro_input, Result};
use syn::parse::{Parse, ParseStream};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use parsing::request::Request;
use parsing::fields::Field;

// This thing might work. It needs to be rewritten to be less terrible. I was
// close to finishing this and really wanted to see it work. I'm super tired.
// Please forgive me for what you are about to read. Or better yet, turn back
// now. You have been warned.

struct Requests {
	requests: Vec<Request>,
}

impl Parse for Requests {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut requests: Vec<Request> = vec![];

		while !input.is_empty() {
			requests.push(input.parse::<Request>()?);
		}

		Ok(Self { requests })
	}
}

#[proc_macro]
/// Generate requests. I'm too tired to write proper documentation right now.
/// I will rewrite this documentation soon.
pub fn request(input: TokenStream) -> TokenStream {
	// Parse the input as a vector of requests.
	let requests = parse_macro_input!(input as Requests).requests;

	// Map each request to itself expanded to a tokenstream.
	let expanded = requests.iter().map(|req| {
		// Clone the request so we don't have to deal with borrow checker...
		// please forgive me, I'm really tired and just want to see this work.
		let req = req.clone();
		let struct_definition = req.to_token_stream();

		let name = req.name;
		let reply = req.reply_ty;
		let length = req.length;

		let metabyte = req.meta_byte;

		// NOTE: This is kinda a mess. I don't really have time to do something
		//       about that. It works. If you are reading this, please do feel free
		//       to extract this into proper logic: all this is trying to is
		//       implement the following logic:
		//
		//       If the definition type is shorthand, write the field by checking
		//       whether it is an unused field or a normal field. If it is unused,
		//       simply write its byte length in empty bytes. Otherwise, write the
		//       field with the corresponding [`WriteValue`] function to its byte
		//       length.
		//
		//       If the definition is in full, iterate over the fields, and do the
		//       same writing as for the shorthand version, just for each and every
		//       field.
		//
		//       [xrb::rw::WriteValue] has methods to write as 1, 2, or 4 byte
		//       lengths. That means normal fields must be exactly 1, 2, or 4 bytes.
		//       Unused fields, however, don't have to use [xrb::rw::WriteValue]
		//       because their data doesn't matter - whatever their length is, that
		//       many empty bytes (`0u8` for simplicity, but they can be any value)
		//       shall be written.
		//
		//       Feel free to scrap any of this macro, as long as the same
		//       functionality is achieved. Submit this as a PR if you wish.
		let fields = req
			.definition
			.full()
			.map(|def| {
				def.fields
					.iter()
					.map(|field| expand(field))
					// Collect the iterator into a `Vec` of token streams that can
					// be written.
					.collect::<Vec<TokenStream2>>()
			})
			.map_or_else(|| {
				let short = req.definition
					.short()
					.map(|def| def.field.map(|field| expand(&field)))
					.flatten();

				quote!(#short)
			}, |tokens| quote!(#(#tokens)*));

		let major = req.major_opcode;
		let minor = metabyte
			.minor_opcode()
			.map_or(quote!(None), |opcode| quote!(Some(#opcode)));

		quote! {
			#struct_definition

			impl crate::Request<#reply> for #name {
				fn opcode() -> u8 {
					#major
				}

				fn minor_opcode() -> Option<u8> {
					#minor
				}

				fn length(&self) -> u16 {
					#length
				}
			}

			impl crate::rw::Serialize for #name {
				fn serialize(self) -> crate::rw::WriteResult<Vec<u8>> {
					let mut bytes = vec![];

					// Header {{{

					// Write the major opcode as one byte.
					<u8 as crate::rw::WriteValue>::write_1b_to(#major, &mut bytes)?;
					// Write the metabyte, whether that be a data field, minor opcode,
					// or blank, as one byte.
					<u8 as crate::rw::WriteValue>::write_1b_to(#metabyte, &mut bytes)?;
					// Write the request length as two bytes.
					<u16 as crate::rw::WriteValue>::write_2b_to(#length, &mut bytes)?;

					// }}}

					#fields

					Ok(bytes)
				}
			}
		}
	});

	// Merge all of the generated request definitions and return one final token stream.
	quote!(#(#expanded)*).into()
}

fn expand(field: &Field) -> TokenStream2 {
	let unused = field.unused().map(|u| {
		let len = u.length;

		// bytes.put_bytes(0u8, #len);
		quote!(<bytes::Bytes as bytes::BufMut>::put_bytes(&mut bytes, 0u8, #len))
	});

	let norm = field.normal().map(|n| {
		let name = n.name;
		let ty = n.ty;
		let len = n.length;

		match len {
			1 => {
				// self.#name.write_1b_to(&mut bytes)?;
				quote!(<#ty as crate::rw::WriteValue>::write_1b_to(self.#name, &mut bytes)?;)
			}
			2 => {
				// self.#name.write_2b_to(&mut bytes)?;
				quote!(<#ty as crate::rw::WriteValue>::write_2b_to(self.#name, &mut bytes)?;)
			}
			4 => {
				// self.#name.write_4b_to(&mut bytes)?;
				quote!(<#ty as crate::rw::WriteValue>::write_4b_to(self.#name, &mut bytes)?;)
			}
			_ => panic!("expected a byte length of 1, 2, or 4"),
		}
	});

	// This is guaranteed to be `Some` because `field` can only
	// be either an unused field or a norm field. _One_ of those
	// two variables _must_ be `Some`.
	unused.or(norm).unwrap()
}
