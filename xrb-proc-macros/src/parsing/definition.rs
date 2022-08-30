// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, Token, Result};

use crate::parsing::fields::Field;

/// A full 'body' definition of a request or reply.
///
/// Similar to that of a struct, this is a comma-delimited group of fields, with
/// the difference being that these fields have a specified byte length
/// (defaulting to 1).
///
/// # Examples
/// ```rust
/// {}
///
/// { window: Window[4] }
///
/// {
///     window: Window[4],
///     cursor: Option<Cursor>[4],
/// }
/// ```
#[derive(Clone)]
pub struct Body {
	pub fields: Punctuated<Field, Token![,]>,
}

/// A shorthand definition of a request or reply with an optional single field.
///
/// # Examples
/// ```rust
/// ;                          // no fields
/// window: Window[4];         // one `window` field
/// cursor: Option<Cursor>[4]; // one `cursor` field
/// ?[24];                     // 24 unused bytes (no field)
/// ```
#[derive(Clone)]
pub struct Shorthand {
	pub field: Option<Field>,
}

// Parsing {{{

impl Parse for Body {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse curly brackets/braces, but don't save the tokens.
		let content;
		braced!(content in input);

		Ok(Self {
			fields: content.parse_terminated(Field::parse)?,
		})
	}
}

impl Parse for Shorthand {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse a single field, if present.
		let field: Option<Field> = input.parse().ok();
		// Parse a `;` token, but don't save it.
		input.parse::<Token![;]>()?;

		Ok(Self { field })
	}
}

// }}}
