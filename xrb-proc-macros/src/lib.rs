// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod parsing;

use quote::{quote, ToTokens};
use syn::parse_macro_input;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use parsing::request::Request;

#[proc_macro]
/// Generate requests. I'm too tired to write proper documentation right now.
/// I will rewrite this documentation soon.
pub fn request(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as Request);

	let struct_definition = input.to_token_stream();

	let name = input.name;
	let reply = input.reply_ty;
	let length = input.length;

	let metabyte = input.meta_byte;

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
	let fields = input
		.definition
		.full()
		.map(|def| {
			def.fields
				.iter()
				.map(|field| {
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
								quote!(<#ty as xrb::rw::WriteValue>::write_1b_to(self.#name, &mut bytes)?;)
							}
							2 => {
								// self.#name.write_2b_to(&mut bytes)?;
								quote!(<#ty as xrb::rw::WriteValue>::write_2b_to(self.#name, &mut bytes)?;)
							}
							4 => {
								// self.#name.write_4b_to(&mut bytes)?;
								quote!(<#ty as xrb::rw::WriteValue>::write_4b_to(self.#name, &mut bytes)?;)
							}
							_ => panic!("expected a byte length of 1, 2, or 4"),
						}
					});

					// This is guaranteed to be `Some` because `field` can only
					// be either an unused field or a norm field. _One_ of those
					// two variables _must_ be `Some`.
					unused.or(norm).unwrap()
				})
				// Collect the iterator into a `Vec` of token streams that can
				// be written.
				.collect::<Vec<TokenStream2>>()
		})
		// TODO: Instead of defaulting to `quote!()`, the `Shorthand` definition
		//       should be mapped in the same way as above but with a single
		//       field. That means the unused/normal field handling should be
		//       extracted to reusable functions.
		.map_or(quote!(), |tokens| quote!(#(#tokens)*));

	let major = input.major_opcode;
	let minor = metabyte
		.minor_opcode()
		.map_or(quote!(None), |opcode| quote!(Some(#opcode)));

	let expanded = quote! {
		#struct_definition

		impl xrb::Request<#reply> for #name {
			fn opcode() -> u8 {
				#major
			}

			fn minor_opcode() -> u8 {
				#minor
			}

			fn length(&self) -> u16 {
				#length
			}
		}

		impl xrb::rw::Serialize for #name {
			fn serialize(self) -> xrb::rw::WriteResult<Vec<u8>> {
				let mut bytes = vec![];

				// Header {{{

				// Write the major opcode as one byte.
				<u8 as xrb::rw::WriteValue>::write_1b_to(#major, &mut bytes)?;
				// Write the metabyte, whether that be a data field, minor opcode,
				// or blank, as one byte.
				<u8 as xrb::rw::WriteValue>::write_1b_to(#metabyte, &mut bytes)?;
				// Write the request length as two bytes.
				<u16 as xrb::rw::WriteValue>::write_2b_to(#length, &mut bytes)?;

				// }}}

				#fields

				Ok(bytes)
			}
		}
	};

	expanded.into()
}
