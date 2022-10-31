// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use syn::{bracketed, parenthesized, parse::ParseStream, token, Ident, Result, Token, Type};

use super::Source;

pub enum Unused<'a> {
	Unit(token::Paren),
	Array(Array<'a>),
}

pub struct Array<'a> {
	pub bracket_token: token::Bracket,
	pub unit_token: token::Paren,
	pub semicolon_token: Token![;],
	pub source: Source<'a>,
}

impl<'a> Unused<'a> {
	pub const fn is_unit(&self) -> bool {
		matches!(self, Self::Unit(_))
	}

	pub const fn is_array(&self) -> bool {
		matches!(self, Self::Array(_))
	}

	pub const fn source(&self) -> Option<&Source<'a>> {
		match self {
			Self::Array(Array { source, .. }) => Some(source),
			Self::Unit(_) => None,
		}
	}
}

// Expansion {{

impl Unused<'_> {
	pub fn to_write_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		// writer.unused(#unused#num(__data__))
	}

	pub fn to_read_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		// reader.advance(#unused#num(__data__))
	}
}

// }}}

// Parsing {{{

impl Unused<'_> {
	pub fn parse(input: ParseStream, map: HashMap<Ident, Type>) -> Result<Self> {
		let look = input.lookahead1();

		if look.peek(token::Paren) {
			let content;

			Ok(Self::Unit(parenthesized!(content in input)))
		} else if look.peek(token::Bracket) {
			Ok(Self::Array(Array::parse(input, map)?))
		} else {
			Err(look.error())
		}
	}
}

impl Array<'_> {
	pub fn parse(input: ParseStream, map: HashMap<Ident, Type>) -> Result<Self> {
		let (content, _unit);

		Ok(Self {
			bracket_token: bracketed!(content in input),
			unit_token: parenthesized!(_unit in content),
			semicolon_token: content.parse()?,
			source: Source::parse_without_receiver(input, map)?,
		})
	}
}

// }}}
