// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::{Result, parenthesized};
use syn::parse::{Parse, ParseStream};

use crate::parsing::fields::{Field, UnusedField};

/// The data byte that is contained within the header of requests that do not
/// have a minor opcode.
///
/// This is represented by a single one-byte field.
///
/// # Examples
/// ```rust
/// (?)                 // ok: 1 byte
/// (?[1])              // ok: 1 byte
/// (mode: Mode)        // ok: 1 byte
/// (toggle: bool)      // ok: 1 byte
/// (window: Window[4]) // error: must be one byte
/// ```
#[derive(Clone)]
pub struct Databyte {
	pub field: Field,
}

impl Databyte {
	#[allow(dead_code)]
	/// Construct a new [`Databyte`] from the given field.
	///
	/// Warning: this does not check whether the given field is exactly one byte
	/// in length. That is your responsibility when calling this constructor.
	pub fn new(field: Field) -> Self {
		Self { field }
	}
}

// Default [`Databyte`] is an unused field with a length of one.
impl Default for Databyte {
	fn default() -> Self {
		Self {
			field: UnusedField::default().into(),
		}
	}
}

// Parsing {{{

impl Parse for Databyte {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse parentheses, but don't save them directly.
		let content;
		parenthesized!(content in input);

		// Parse a single field.
		let field: Field = content.parse()?;

		// Panic if the length is not `1`, because the data byte in the
		// request header is only a single byte.
		if field.length() != 1 {
			panic!("expected a length of 1 byte for the request header data byte");
		}

		Ok(Self { field })
	}
}

// }}}
