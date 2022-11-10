// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::content::FmtIndexedIdent;
use quote::format_ident;
use std::collections::HashMap;
use syn::{bracketed, parenthesized, parse::ParseStream, token, Ident, Result, Token, Type};

use super::Source;

pub enum Unused {
	Unit(token::Paren),
	Array(Box<Array>),
}

pub struct Array {
	pub bracket_token: token::Bracket,
	pub unit_token: token::Paren,
	pub semicolon_token: Token![;],
	pub source: Source,
}

impl FmtIndexedIdent for Array {
	fn fmt_indexed_ident(&self, index: usize) -> Ident {
		format_ident!("_item{}_", index)
	}
}

impl Unused {
	#[allow(dead_code)]
	pub const fn is_unit(&self) -> bool {
		matches!(self, Self::Unit(_))
	}

	#[allow(dead_code)]
	pub const fn is_array(&self) -> bool {
		matches!(self, Self::Array(_))
	}

	#[allow(dead_code)]
	pub const fn source(&self) -> Option<&Source> {
		match self {
			Self::Array(array) => Some(&array.source),
			Self::Unit(_) => None,
		}
	}
}

// Expansion {{

impl Unused {
	#[allow(dead_code)]
	pub fn to_write_tokens(&self, _tokens: &mut proc_macro2::TokenStream) {
		// writer.unused(#unused#num(__data__))
	}

	#[allow(dead_code)]
	pub fn to_read_tokens(&self, _tokens: &mut proc_macro2::TokenStream) {
		// reader.advance(#unused#num(__data__))
	}
}

// }}}

// Parsing {{{

impl Unused {
	pub fn parse(input: ParseStream, map: &HashMap<Ident, Type>) -> Result<Self> {
		let look = input.lookahead1();

		if look.peek(token::Paren) {
			let _unit;

			Ok(Self::Unit(parenthesized!(_unit in input)))
		} else if look.peek(token::Bracket) {
			Ok(Self::Array(Box::new(Array::parse(input, map)?)))
		} else {
			Err(look.error())
		}
	}
}

impl Array {
	pub fn parse(input: ParseStream, map: &HashMap<Ident, Type>) -> Result<Self> {
		let (content, _unit);

		Ok(Self {
			bracket_token: bracketed!(content in input),
			unit_token: parenthesized!(_unit in content),
			semicolon_token: content.parse()?,
			source: Source::parse_without_receiver(&content, map)?,
		})
	}
}

// }}}
