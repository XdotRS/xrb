// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token;
use syn::{braced, Result, Token};

use proc_macro2::{Delimiter, Group, TokenStream as TokenStream2};
use quote::{ToTokens, TokenStreamExt};

use crate::parsing::fields::Field;

/// The definition of a request or body with zero or more fields.
///
/// # Examples
/// ```rust
/// ;                     // shorthand: no fields
/// window: Window[4];    // shorthand: one field
///
/// {}                    // full: no fields
/// { window: Window[4] } // full: one field
///
/// // full: many fields
/// {
///     window: Window[4],
///     cursor: Option<Cursor>[4],
///     x: i16[2],
///     y: i16[2],
///     width: u16[2],
///     height: u16[2],
/// }
/// ```
#[derive(Clone)]
pub enum Definition {
	Short(Shorthand),
	Full(Body),
}

impl Definition {
	#[allow(dead_code)]
	/// The wrapped [`Shorthand`] definition, if one is indeed wrapped.
	pub fn short(&self) -> Option<Shorthand> {
		match self {
			Self::Short(short) => Some(short.clone()),
			_ => None,
		}
	}

	#[allow(dead_code)]
	/// The wrapped [`Body`] definition, if one is indeed wrapped.
	pub fn full(&self) -> Option<Body> {
		match self {
			Self::Full(body) => Some(body.clone()),
			_ => None,
		}
	}
}

impl From<Shorthand> for Definition {
	fn from(short: Shorthand) -> Self {
		Self::Short(short)
	}
}

impl From<Body> for Definition {
	fn from(full: Body) -> Self {
		Self::Full(full)
	}
}

impl ToTokens for Definition {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Write `self` as tokens by calling the appropriate definition type's
		// `to_tokens` methods (see their definitions further on in this file).
		match self {
			Self::Full(body) => body.to_tokens(tokens),
			Self::Short(shorthand) => shorthand.to_tokens(tokens),
		}
	}
}

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

impl ToTokens for Body {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Appends a [`Group`] with the [`Brace`] [`Delimiter`] and the content
		// bring `self.fields`.
		//
		// `self.fields` is a `,`-punctuated list of fields; that means that it
		// will be written as each field separated by commas, like so:
		// ```rust
		// name1: Type1, name2: Type2, name3: Type3
		// ```
		//
		// The [`Delimiter::Brace`] refers to curly brackets/braces (`{` and
		// `}`), and for the [`Group`], means that it wraps the fields with
		// curly brackets. That means that this will turn out to be:
		// ```rust
		// { name1: Type1, name2: Type2, name3: Type3 }
		// ```
		tokens.append(Group::new(Delimiter::Brace, self.fields.to_token_stream()));
	}
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

impl ToTokens for Shorthand {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Appends a [`Group`] with the [`Brace`] [`Delimiter`] and the content
		// of `self.field`. That means that it wraps the token representation
		// of the field with curly brackets, a.k.a. braces:
		// ```rust
		// { field }
		// ```
		tokens.append(Group::new(Delimiter::Brace, self.field.to_token_stream()));
	}
}

// Parsing {{{

impl Parse for Definition {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(match input.lookahead1().peek(token::Brace) {
			// If the next token is `{`, parse as `Body`...
			true => input.parse::<Body>()?.into(),
			// Otherwise, parse as `Shorthand`...
			false => input.parse::<Shorthand>()?.into(),
		})
	}
}

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
