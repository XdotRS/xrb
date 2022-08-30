// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{bracketed, Ident, LitInt, Result, Token, Type};

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
	/// Whether this is an unused field.
	pub fn unused(&self) -> bool {
		match self {
			Self::Unused(_) => true,
			_ => false,
		}
	}

	#[allow(dead_code)]
	/// Whether this is a normal field with a name and type.
	pub fn normal(&self) -> bool {
		match self {
			Self::Normal(_) => true,
			_ => false,
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
	fn new() -> Self {
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
	pub name: Ident,
	pub ty: Type,
	pub length: u8,
}

impl NormalField {
	#[allow(dead_code)]
	/// Construct a new [`NormalField`] with the given name, type, and length.
	pub fn new(name: Ident, ty: Type, length: u8) -> Self {
		Self { name, ty, length }
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
