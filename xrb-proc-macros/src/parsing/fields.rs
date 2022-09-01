// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{bracketed, Ident, LitInt, Result, Token, Type, Attribute};

use proc_macro2::{Punct, Spacing, TokenStream as TokenStream2};

use quote::{ToTokens, TokenStreamExt, quote};

/// A field that can appear in `request!` and `reply!` macros.
///
/// This field can either be unused data, meaning it will be skipped over and
/// filled with empty data, or it can be an actual field with a name and a type.
///
/// Unused fields can specify any byte length - that number of bytes will be
/// skipped. Normal fields may only specify a byte length of `1`, `2`, or `4`
/// however; they must be compatible with [xrb::rw::WriteValue].
///
/// # Examples
/// ## Unused fields
/// ```rust
/// ?     // 1 unused byte
/// ?[1]  // 1 unused byte
/// ?[4]  // 4 unused bytes
/// ?[27] // 27 unused bytes
/// ?[3]  // 3 unused bytes
/// ```
/// ## Normal fields
/// ```rust
/// mode: Mode    // ok: length is 1 byte
/// mode: Mode[1] // ok: length is 1 byte
/// mode: Mode[2] // ok: length is 2 bytes
/// mode: Mode[3] // error: length is 3 bytes
/// mode: Mode[4] // ok: length is 4 bytes
/// ```
#[derive(Clone)]
pub enum Field {
	Unused(UnusedField),
	Normal(NormalField),
}

impl Field {
	/// Gets the length of this field.
	///
	/// Note that if this is a normal field, the length must be `1`, `2`, or `4`
	/// bytes, but if it is an unused field it can be any number of bytes in
	/// length.
	pub fn length(&self) -> u8 {
		match self {
			Self::Unused(field) => field.length,
			Self::Normal(field) => field.length,
		}
	}

	#[allow(dead_code)]
	/// An [`Option`] that wraps the [`UnusedField`] if `self` is `Unused`.
	pub fn unused(&self) -> Option<UnusedField> {
		match self {
			Self::Unused(field) => Some(*field),
			_ => None,
		}
	}

	#[allow(dead_code)]
	/// An [`Option`] that wraps the [`NormalField`] if `self` is `Normal`.
	pub fn normal(&self) -> Option<NormalField> {
		match self {
			Self::Normal(field) => Some(field.clone()),
			_ => None,
		}
	}
}

impl From<UnusedField> for Field {
	fn from(field: UnusedField) -> Self {
		Self::Unused(field)
	}
}

impl From<NormalField> for Field {
	fn from(field: NormalField) -> Self {
		Self::Normal(field)
	}
}

impl ToTokens for Field {
	/// Writes the field as tokens _if_ it is a [`NormalField`]. [`UnusedField`]s aren't written.
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Normal(field) => field.to_tokens(tokens),
			Self::Unused(_) => (),
		}
	}
}

impl Field {
	/// Returns a [`TokenStream`] with the appropriate serialization for this
	/// particular field.
	pub fn serialize_tokens(&self) -> TokenStream2 {
		match self {
			Self::Unused(unused) => {
				let length = unused.length;

				// Alias the turbofish so the code is a little cleaner...
				let turbofish = quote!(<u8 as crate::rw::WriteValue>);

				match length {
					// If the length is 2 bytes, use `write_2b_to`.
					2 => quote! {
						#turbofish::write_2b_to(0, &mut bytes)?;
					},
					// If the length is 4 bytes, use `write_4b_to`.
					4 => quote! {
						#turbofish::write_4b_to(0, &mut bytes)?;
					},
					// Otherwise, if the length is 1, 3, or 5+ bytes, use
					// `write_1b_to` `length` many times.
					_ => {
						let bytes = (0..length).map(|_| {
							quote! {
								#turbofish::write_1b_to(0, &mut bytes)?;
							}
						});

						quote!(#(#bytes)*)
					}
				}
			}
			Self::Normal(field) => {
				// Bind `name` and `ty` to `field.name` and `field.ty`.
				let (name, ty) = (&field.name, &field.ty);
				// Alias the turbofish so the code is a little cleaner...
				let turbofish = quote!(<#ty as crate::rw::WriteValue>);

				// Choose the appropriate `WriteValue` method based on the
				// length.
				let function = match field.length {
					// 1 byte
					1 => quote!(write_1b_to),
					// 2 bytes
					2 => quote!(write_2b_to),
					// 4 bytes
					4 => quote!(write_4b_to),
					// Panic if the field was another length. It shouldn't be.
					_ => panic!("expected a normal field byte of 1, 2, or 4"),
				};

				quote!(#turbofish::#function(self.#name, &mut bytes)?;)
			}
		}
	}
}

/// An unused field representing empty bytes.
///
/// The bytes of unused fields are skipped over and filled with empty data.
/// They are not guaranteed to be zero, however.
///
/// Unused fields can specify any byte length - since they are not written with
/// [xrb::rw::WriteValue], they can represent any number of unused bytes. This
/// is particularly helpful at the end of replies, where a large number of bytes
/// are often not used. If an unused field's length is omitted, it defaults to
/// `1` byte in length.
///
/// # Examples
/// ```rust
/// ?     // 1 unused byte
/// ?[1]  // 1 unused byte
/// ?[4]  // 4 unused bytes
/// ?[27] // 27 unused bytes
/// ?[3]  // 3 unused bytes
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct UnusedField {
	pub length: u8,
}

impl UnusedField {
	#[allow(dead_code)]
	/// Construct a new [`UnusedField`] with the default length of `1`.
	pub fn new() -> Self {
		Self { length: 1 }
	}

	#[allow(dead_code)]
	/// Construct a new [`UnusedField`] with the given length.
	pub fn with_length(length: u8) -> Self {
		Self { length }
	}
}

/// A normal field with a name and type. Its byte length must be `1`, `2`, or `4`.
///
/// Since fields are written with [xrb::rw::WriteValue], they must be exactly
/// `1`, `2`, or `4` bytes in length. If the length is omitted, they default to
/// `1` byte in length.
///
/// # Examples
/// ```rust
/// mode: Mode,    // ok: length is 1 byte
/// mode: Mode[1], // ok: length is 1 byte
/// mode: Mode[2], // ok: length is 2 bytes
/// mode: Mode[3], // error: length is 3 bytes
/// mode: Mode[4], // ok: length is 4 bytes
/// ```
#[derive(Clone)]
pub struct NormalField {
	pub attributes: Vec<Attribute>,
	pub name: Ident,
	pub ty: Type,
	pub length: u8,
}

impl NormalField {
	#[allow(dead_code)]
	/// Construct a new [`NormalField`] with the given attributes, name,
	/// type, and length.
	pub fn new(attributes: Vec<Attribute>, name: Ident, ty: Type, length: u8) -> Self {
		Self { attributes, name, ty, length }
	}
}

impl ToTokens for NormalField {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Write the attributes, including doc comments.
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// Write the name.
		self.name.to_tokens(tokens);
		// Write a single semicolon.
		tokens.append(Punct::new(':', Spacing::Alone));
		// Write the type.
		self.ty.to_tokens(tokens);

		// Together, these are in the format `name: Type`, just like, well,
		// normal fields in Rust. The length is not written here - that is to
		// be intentionally written elsewhere.
	}
}

/// The length of a field in bytes.
///
/// # Examples
/// ```rust
/// [1] // length is 1 byte
/// [2] // length is 2 bytes
/// [3] // length is 3 bytes
/// [4] // length is 4 bytes
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FieldLength {
	pub length: u8,
}

impl FieldLength {
	#[allow(dead_code)]
	/// Construct a new [`FieldLength`] node with the default length of `1`.
	fn new() -> Self {
		Self { length: 1 }
	}

	#[allow(dead_code)]
	/// Construct a new [`FieldLength`] node with the given length.
	fn with_length(length: u8) -> Self {
		Self { length }
	}
}

impl ToTokens for FieldLength {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.length.to_tokens(tokens);
	}
}

// Parsing {{{

impl Parse for Field {
	fn parse(input: ParseStream) -> Result<Self> {
		// If the next token is `?`, parse as an unused field, otherwise parse
		// as a normal field.
		if input.lookahead1().peek(Token![?]) {
			input.parse().map(Self::Unused)
		} else {
			input.parse().map(Self::Normal)
		}
	}
}

impl Parse for UnusedField {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse a `?` token, but don't save it. Returns an error if it isn't
		// there.
		input.parse::<Token![?]>()?;

		// Attempt to parse a length; default to `1` if it was missing.
		let len: Result<FieldLength> = input.parse();
		let value: u8 = len.map_or(1, |len| len.length);

		Ok(Self { length: value })
	}
}

impl Parse for NormalField {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse attributes, including doc comments.
		let attributes = input.call(Attribute::parse_outer)?;
		// Parse the field name as an identifier.
		let name: Ident = input.parse()?;
		// Parse a `:` token, but don't save it.
		input.parse::<Token![:]>()?;
		// Parse the field type.
		let ty: Type = input.parse()?;

		// Attempt to parse a length; default to `1` if it was missing.
		let len: Result<FieldLength> = input.parse();
		let value: u8 = len.map_or(1, |len| len.length);

		// If the length is not 1, 2, or 4 bytes, panic. [xrb::rw::WriteValue]
		// requires that values be written to 1, 2, or 4 bytes only.
		match value {
			1 => (),
			2 => (),
			4 => (),
			_ => panic!("expected a field length of 1, 2, or 4 bytes"),
		}

		Ok(Self {
			attributes,
			name,
			ty,
			length: value,
		})
	}
}

impl Parse for FieldLength {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse square brackets, but don't save the brackets themselves.
		let content;
		bracketed!(content in input);

		// Parse an integer literal value as `length`.
		let value: LitInt = content.parse()?;
		let length: u8 = value.base10_parse()?;

		Ok(Self { length })
	}
}

// }}}
