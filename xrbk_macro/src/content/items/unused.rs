// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::{parse::ParseStream, token, Result, Token};

use super::{Attribute, IdentMap, Source};

pub enum Unused {
	/// A unit token representing one single unused byte.
	Unit {
		/// An optional [metabyte attribute] which denotes the metabyte
		/// position as being a single unused byte.
		///
		/// This is exactly the same as the default for the metabyte position;
		/// if no item is annotated with a [metabyte attribute], it will
		/// default to a single unused byte.
		///
		/// [metabyte attribute]: crate::content::AttrContent::Metabyte
		attribute: Option<Attribute>,

		/// A unit token: `()`.
		unit_token: token::Paren,
	},

	// There is no guarantee the number of unused bytes returned by the
	// expression is `1`... so don't allow metabyte.
	//
	/// A syntax that allows the number of unused bytes read or written to be
	/// determined by a [`Source`].
	Array(Box<Array>),
}

pub struct Array {
	/// A pair of square brackets: `[` and `]`.
	pub bracket_token: token::Bracket,
	/// A unit token: `()`.
	pub unit_token: token::Paren,
	/// A semicolon token: `;`.
	pub semicolon_token: Token![;],
	/// The [content] of the `Array` that provides the number of unused bytes.
	///
	/// [content]: ArrayContent
	pub content: ArrayContent,
}

pub enum ArrayContent {
	/// Infer the number of unused bytes.
	Infer(Token![..]),
	/// Evaluate a [`Source`] for the number of unused bytes.
	Source(Box<Source>),
}

impl Unused {
	/// Returns whether this is the [`Unused::Unit`] form.
	pub const fn is_unit(&self) -> bool {
		matches!(self, Self::Unit { .. })
	}

	/// Returns whether this is the [`Unused::Array`] form.
	pub const fn is_array(&self) -> bool {
		matches!(self, Self::Array { .. })
	}

	/// Returns the contained [`Source`] if this is [`Unused::Array`] with
	/// content [`AttrContent::Source`].
	pub const fn source(&self) -> Option<&Source> {
		match self {
			Self::Array(array) => {
				if let ArrayContent::Source(source) = &array.content {
					Some(source)
				} else {
					None
				}
			}

			Self::Unit { .. } => None,
		}
	}
}

// Parsing {{{

impl ArrayContent {
	pub fn parse(input: ParseStream, map: IdentMap) -> Result<Self> {
		Ok(if input.peek(Token![..]) {
			Self::Infer(input.parse()?)
		} else {
			Self::Source(Box::new(Source::parse(input, map)?))
		})
	}
}

// }}}
