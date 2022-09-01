// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod parsing;

use quote::{quote, ToTokens};
use syn::parse_macro_input;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use parsing::request::{Metabyte, Requests};

/// Generates X11 protocol request definitions based on a custom syntax.
///
/// # Overview
/// Let's use the example of a `GetProperty` request:
/// ```rust
/// requests! {
///     #20: pub struct GetProperty<6>(delete: bool) -> GetPropertyReply {
///         window: Window[4],
///         property: Atom[4],
///         property_type: Specificity<Atom>[4],
///         long_offset: u32[4],
///         long_length: u32[4],
///     }
/// }
/// ```
/// There's a lot of information there, so let's break it down.
///
/// To start with, the `GetProperty` request has major opcode `20` and it
/// generates a reply of type `GetPropertyReply`. You can see that in the
/// `requests!` syntax a major opcode is given at the start of each request
/// definition, in this case being `#20:`. Every request definition must include
/// the major opcode in such a way. For requests that define a minor opcode, the
/// minor opcode can also be specified before the colon: `#20 #13:` means major
/// opcode `20`, minor opcode `13`. You can also see that the reply return type
/// is given by `-> ReplyType`, in this case being `-> GetPropertyReply`.
///
/// Some of this definition should also be quite familiar: the visibility of the
/// request's generated struct is given with `pub` and is followed by `struct`,
/// just like any other struct definition in Rust. There is a notable difference
/// following the name of the request here, though: both the request length and
/// a field to be contained in the data byte of the request header have been
/// given. The length of the request is given in units of 4 bytes, included in
/// arrow brackets (`<` and `>`) immediately following the name. This length
/// declaration is optional: if omitted, it will default to `1`, which is the
/// length of the request if _no_ fields are given (other than the field in the
/// request header data byte, which doesn't contribute towards the length of the
/// request). After that, this request does indeed define a field that occupies
/// the request header data byte: this field is unique in that:
/// 1. It doesn't contribute towards the total length of the request.
/// 2. It must be exactly one byte in length.
///
/// This request header data byte field will be defined in the generated struct
/// definition of this request, just like fields in `struct`s normally are. Its
/// declaration is also optional: if omitted, it will default to being an unused
/// byte; that means that no field will be associated with the request header
/// data byte and it will simply be written as `0` when serialized. Note that
/// defining a field for the request header data byte is _not_ an option when a
/// minor opcode is also defined: the minor opcode is written to that data byte
/// as well.
///
/// Following all this information, the body of the request is actually defined
/// within `{` and `}`. This is very similar to the definition of fields in
/// structs that you may be used to in Rust, but with one key difference: the
/// length in bytes of each field is specified within square brackets (`[` and
/// `]`). This is so that X Rust Bindings knows how to write this field as
/// bytes; whether it should write the field as `1`, `2`, or `4` bytes. These
/// fields will be converted to ordinary struct fields in Rust when the struct
/// definition for this request is generated.
///
/// We won't go into detail on the serialization of these requests, but it may
/// be helpful to see the struct and `Request` trait implementation generated
/// by this example:
/// ```rust
/// pub struct GetProperty {
///     pub delete: bool,
///     pub window: Window,
///     pub property: Atom,
///     pub property_type: Specificity<Atom>,
///     pub long_offset: u32,
///     pub long_length: u32,
/// }
///
/// impl Request<GetPropertyReply> for GetProperty {
///     fn opcode() -> u8 {
///         20
///     }
///
///     fn minor_opcode() -> Option<u8> {
///         None
///     }
///
///     fn length(&self) -> u16 {
///         6
///     }
/// }
/// ```
///
/// # Shorthand
/// Now that we've had a look at a particularly complex request definition, let's
/// take a look at a simpler one and see how we can write it in a simpler way.
/// ```rust
/// requests! {
///     #8: pub struct MapWindow<2> { window: Window[4] }
/// }
/// ```
/// The `MapWindow` request is pretty simple compared to a `GetProperty` request.
/// It does not define any data field, so there is nothing after the request
/// length, nor does it have any reply, so the reply type can be omitted too.
/// The `MapWindow` request also only has a single field: a 4-byte field named
/// `window` with the type `Window`. This is actually a pretty common layout of
/// request in the X11 protocol, so this can actually be written slightly
/// shorter still:
/// ```rust
/// requests! {
///     #8: pub struct MapWindow<2> window: Window[4];
/// }
/// ```
/// In this shorthand definition, we have omitted the curly brackets (`{` and
/// `}`) in exchange for only being able to define a maximum of one field (aside
/// from the request header data byte field, as that is an exception). It is
/// also possible to define a request with no additional fields whatsoever, in
/// which case even the request length can be omitted. An example of this is the
/// `GrabServer` request, which can simply be defined like so:
/// ```rust
/// requests! {
///     #36: pub struct GrabServer;
/// }
/// ```
///
/// # Unused bytes
/// There is actually one final bit of the `requests!` syntax we haven't talked
/// about yet. Remember that the request header data byte field can be omitted,
/// leaving it to default to a single unused byte? Well, we can explicitly define
/// this using a special syntax for unused bytes: `?[1]` (or even `?` if you
/// omit the length, which will default to a single unused byte). That means
/// that the full definition of the `GrabServer` request is actually:
/// ```rust
/// requests! {
///     #36: pub struct GrabServer<1>(?[1]) -> () {}
/// }
/// ```
/// But we allow these omissions for convenience and readability.
#[proc_macro]
pub fn requests(input: TokenStream) -> TokenStream {
	let requests = parse_macro_input!(input as Requests).requests;

	// Expand each request as tokens.
	let expanded_requests = requests.iter().map(|request| {
		let struct_definition = request.to_token_stream();

		let (name, reply, length) = (&request.name, &request.reply_ty, request.length);

		let metabyte = &request.meta_byte;
		// The appropriate serialization code for the metabyte: either writes
		// the minor opcode or the databyte field, depending on the metabyte.
		let serialize_metabyte = match metabyte {
			Metabyte::Normal(databyte) => databyte.field.serialize_tokens(),
			Metabyte::Minor { minor_opcode } => quote! {
				<u8 as crate::rw::WriteValue>::write_1b_to(
					#minor_opcode,
					&mut bytes
				)?;
			},
		};

		let major = request.major_opcode;
		let minor = metabyte
			.minor_opcode()
			.map_or(quote!(None), |opcode| quote!(Some(#opcode)));

		// Map the request's fields to their serialization code.
		//
		// For example:
		// `?[1]` -> `0u8.write_1b_to(&mut bytes)?;`
		// `window: Window[4]` -> `self.window.write_4b_to(&mut bytes)?;`
		let serialize_field = request
			.definition
			.fields()
			.iter()
			.map(|field| field.serialize_tokens())
			.collect::<Vec<TokenStream2>>();

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
					// Serialize the metabyte.
					#serialize_metabyte
					// Write the request length as two bytes.
					<u16 as crate::rw::WriteValue>::write_2b_to(#length, &mut bytes)?;

					// }}}

					#(#serialize_field)*

					Ok(bytes)
				}
			}
		}
	});

	// Combine all of the requests into one [`TokenStream`] output.
	quote!(#(#expanded_requests)*).into()
}
