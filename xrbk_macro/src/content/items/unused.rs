// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use syn::{bracketed, parenthesized, parse::ParseStream, token, Result, Token, Type};

use crate::content::Attribute;

use super::Source;

pub enum Unused {
	Unit {
		attribute: Option<Attribute>,
		unit_token: token::Paren,
	},

	// There is no guarantee the number of unused bytes returned by the
	// expression is `1`... so don't allow metabyte.
	Array(Box<Array>),
}

pub struct Array {
	pub bracket_token: token::Bracket,
	pub unit_token: token::Paren,
	pub semicolon_token: Token![;],
	pub source: Source,
}

impl Unused {
	pub const fn is_unit(&self) -> bool {
		matches!(self, Self::Unit { .. })
	}

	pub const fn is_array(&self) -> bool {
		matches!(self, Self::Array { .. })
	}

	pub const fn source(&self) -> Option<&Source> {
		match self {
			Self::Array(array) => Some(&array.source),
			Self::Unit { .. } => None,
		}
	}
}

// Parsing {{{

impl Unused {
	pub fn parse(input: ParseStream, map: &HashMap<String, Type>) -> Result<Self> {
		let look = input.lookahead1();

		if look.peek(token::Paren) {
			let _unit;

			Ok(Self::Unit {
				attribute: {
					if input.peek(Token![#]) {
						Some(Attribute::parse_metabyte(input)?)
					} else {
						None
					}
				},

				unit_token: parenthesized!(_unit in input),
			})
		} else if look.peek(token::Bracket) {
			Ok(Self::Array(Box::new(Array::parse(input, map)?)))
		} else {
			Err(look.error())
		}
	}
}

impl Array {
	pub fn parse(input: ParseStream, map: &HashMap<String, Type>) -> Result<Self> {
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
