// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Type, Visibility, Token};
use syn::punctuated::Punctuated;

use proc_macro2::{Span, TokenStream as TokenStream2, Group, Delimiter};
use quote::{ToTokens, TokenStreamExt, quote};

use super::databyte::Databyte;
use super::definition::Definition;
use super::opcodes::Opcode;
use super::request_title::RequestTitle;
use super::fields::{Field, NormalField};

/// A fully parsed request, for use in a `requests!` macro.
///
/// # Examples
/// _All_ of the following are equivalent:
/// ```rust
/// #4: pub struct DeleteWindow<2> window: Window[4];
/// #4: pub struct DeleteWindow<2>(?) window: Window[4];
/// #4: pub struct DeleteWindow<2>(?[1]) window: Window[4];
/// #4: pub struct DeleteWindow<2> window: Window[4] -> ();
/// #4: pub struct DeleteWindow<2>(?[1]) window: Window[4] -> ();
///
/// #4: pub struct DeleteWindow<2> {
///	    window: Window[4],
/// }
///
/// #4: pub struct DeleteWindow<2>(?) {
///     window: Window[4],
/// }
///
/// #4: pub struct DeleteWindow<2>(?[1]) {
///     window: Window[4],
/// }
///
/// #4: pub struct DeleteWindow<2> -> () {
///     window: Window[4],
/// }
///
/// #4: pub struct DeleteWindow<2>(?[1]) -> () {
///     window: Window[4],
/// }
/// ```
/// In the above example, this struct looks like so:
/// ```
/// major_opcode: 4,
/// meta_byte: Databyte { field: ?[1] }, // ? = unused/empty
/// vis: Some(Token![pub]),
/// name: Ident("DeleteWindow",
/// length: 2,
/// // 1 field `window` with a type of `Window` and a byte-length of `4`
/// ```
#[derive(Clone)]
pub struct Request {
	/// The request's major opcode.
	pub major_opcode: u8,
	/// The second byte of the request: either the minor opcode or a [`Databyte`].
	pub meta_byte: Metabyte,
	/// The request's visibility (e.g. `pub`).
	pub vis: Option<Visibility>,
	/// The name of the request.
	pub name: Ident,
	/// The length of the request in units of 4-bytes; at least one unit, for
	/// the header.
	pub length: u16,
	pub reply_ty: Option<Type>,
	/// The definition of the request with zero or more fields.
	pub definition: Definition,
}

impl ToTokens for Request {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Visibility, if any.
		self.vis.to_tokens(tokens);
		// `struct` keyword ('just' a special identifier).
		tokens.append(Ident::new("struct", Span::call_site()));
		// Request name.
		self.name.to_tokens(tokens);

		// Create an empty 'punctuated list' of fields, where each field is
		// separated by a comma.
		let mut fields: Punctuated<NormalField, Token![,]> = Punctuated::new();

		// Get an [`Option`] wrapping the existance of a _data byte_ in the
		// metabyte - that is, whether a field was given that uses the second
		// byte in the request's header.
		let databyte = self.meta_byte.databyte();
		// Map the [`Option`] of the existance of a data byte to the existance
		// of a _normal field_ (i.e., a field that we can declare in a struct
		// definition, with a name and a type) within the data byte.
		//
		// We have to flatten this, as we simply want an `Option<NormalField>`,
		// but we would otherwise have an `Option<Option<NormalField>>`.
		let databyte_field = databyte.map(|d| d.field.normal()).flatten();

		// If there is indeed a normal field in the metabyte, we can push that
		// to the list of fields - that means that we will declare it as the
		// first field in the struct.
		databyte_field.map(|field| fields.push(field));

		// Loop over any further fields in the definition. These ones aren't
		// special; no unique serialization needs to be taken into account for
		// them, so this is easier.
		for field in self.definition.fields() {
			// If the field is a normal field, push it to the list of fields.
			field.normal().map(|field| fields.push(field));
		}

		// Group all the field definitions together and surround them with
		// 'braces', a.k.a. curly brackets. This pretty much just adds `{` at
		// the beginning and `}` at the end.
		Group::new(Delimiter::Brace, fields.to_token_stream()).to_tokens(tokens);

		// This will simply generate a struct for the request. Serialization
		// work is done in the macro.
		//
		// # Example
		// ```rust
		// pub struct DeleteWindow {
		//     window: Window,
		// }
		// ```
	}
}

/// Represents the second byte of the request header.
///
/// Either a minor opcode, a 1-byte field, or an unused byte.
#[derive(Clone)]
pub enum Metabyte {
	Normal(Databyte),
	Minor { minor_opcode: u8 },
}

impl Metabyte {
	/// Constructs a [`Metabyte`] with the given minor opcode.
	pub fn with_minor_opcode(minor_opcode: u8) -> Self {
		Self::Minor { minor_opcode }
	}

	#[allow(dead_code)]
	/// Gets the wrapped [`Databyte`] if this is [`Metabyte::Normal`], else [`None`].
	pub fn databyte(&self) -> Option<Databyte> {
		match self {
			Self::Normal(databyte) => Some(databyte.clone()),
			_ => None,
		}
	}

	#[allow(dead_code)]
	/// Gets the wrapped minor opcode if this is [`Metabyte::Minor`], else [`None`].
	pub fn minor_opcode(&self) -> Option<u8> {
		match self {
			Self::Minor { minor_opcode } => Some(*minor_opcode),
			_ => None,
		}
	}
}

impl From<Databyte> for Metabyte {
	fn from(databyte: Databyte) -> Self {
		Self::Normal(databyte)
	}
}

impl ToTokens for Metabyte {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// This will write the metabyte as tokens for it to be used when
		// implementing a serialize method. That means:
		// - If this is a minor opcode, write that minor opcode.
		// - If this is an unused field, write an empty byte.
		// - If this is a normal field, write `self.#name` (where `#name` is the
		//   name of the field). This assumes that `self` is a request `struct`
		//   with the given field.

		match self {
			// write `minor_opcode` as tokens
			Self::Minor { minor_opcode } => minor_opcode.to_tokens(tokens),
			// if it is a field...
			Self::Normal(data) => match &data.field {
				// unused -> write blank data as tokens
				Field::Unused(_) => 0u8.to_tokens(tokens),
				// if normal field (name, type, length)...
				Field::Normal(field) => {
					// bind the field's name to `name`
					let name = &field.name;

					// write `self.#name` as tokens
					quote!(self.#name).to_tokens(tokens);
				}
			}
		}
	}
}

// Parsing {{{

impl Parse for Request {
	fn parse(input: ParseStream) -> Result<Self> {
		let major_opcode: u8 = input.parse::<Opcode>()?.opcode;

		match input.lookahead1().peek(LitInt) {
			false => parse_normal(major_opcode, input),
			true => parse_minor(major_opcode, input),
		}
	}
}

/// Parses a request that does not have a minor opcode.
fn parse_normal(major_opcode: u8, input: ParseStream) -> Result<Request> {
	// Since we now know that there will be no minor opcode, we can ensure that
	// there is a `:` following the opcodes, which we know to simply be the one
	// major opcode.
	input.parse::<Token![:]>()?;

	// Parse the title (visibility, name, length).
	let title: RequestTitle = input.parse()?;
	// Attempt to parse a databyte definition.
	let databyte: Result<Databyte> = input.parse();
	// Parse the definition of zero or more fields.
	let definition: Definition = input.parse()?;

	// Convert either a read databyte or the default of a 1-byte unused field
	// to a [`Metabyte`].
	let meta_byte: Metabyte = databyte.unwrap_or_default().into();

	Ok(Request {
		major_opcode,
		meta_byte,
		vis: title.vis,
		name: title.name,
		length: title.length,
		reply_ty: definition.reply_type(),
		definition,
	})
}

/// Parses a request that has a minor opcode instead of a request header data
/// byte field.
fn parse_minor(major_opcode: u8, input: ParseStream) -> Result<Request> {
	// Parse the minor opcode as an 8-bit integer.
	let minor_opcode: u8 = input.parse::<Opcode>()?.opcode;
	// Ensure there is a `:` following all the opcodes.
	input.parse::<Token![:]>()?;

	// Parse the title (visibility, name, length).
	let title: RequestTitle = input.parse()?;
	// Parse the definition or zero or more fields.
	let definition: Definition = input.parse()?;

	// Convert the minor opcode to a [`Metabyte`].
	let meta_byte = Metabyte::with_minor_opcode(minor_opcode);

	Ok(Request {
		major_opcode,
		meta_byte,
		vis: title.vis,
		name: title.name,
		length: title.length,
		reply_ty: definition.reply_type(),
		definition,
	})
}

// }}}
