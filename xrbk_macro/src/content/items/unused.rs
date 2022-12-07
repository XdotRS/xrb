// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse::ParseStream, token, Result, Token};

use crate::content::LengthMode;
use crate::{
	Attribute, IdentMap, ItemDeserializeTokens, ItemId, ItemSerializeTokens, Source, TsExt,
};

pub enum Unused {
	/// One single unused byte.
	Single {
		/// An optional [metabyte attribute] which denotes the metabyte
		/// position as being a single unused byte.
		///
		/// This is exactly the same as the default for the metabyte position;
		/// if no item is annotated with a [metabyte attribute], it will
		/// default to a single unused byte.
		///
		/// [metabyte attribute]: crate::content::AttrContent::Metabyte
		attribute: Option<Attribute>,

		/// An underscore token: `_`.
		underscore_token: Token![_],
	},

	// There is no guarantee the number of unused bytes returned by the
	// expression is `1`... so don't allow metabyte.
	//
	/// A syntax that allows the number of unused bytes read or written to be
	/// determined by a [`Source`].
	Array(Box<Array>),
}

pub struct Array {
	/// Attributes associated with the unused bytes item's [`Source`] function,
	/// if any.
	pub attributes: Vec<Attribute>,

	/// A pair of square brackets: `[` and `]`.
	pub bracket_token: token::Bracket,

	/// An underscore token: `_`.
	pub underscore_token: Token![_],
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
	/// Returns whether this is the [`Unused::Single`] form.
	pub const fn is_single(&self) -> bool {
		matches!(self, Self::Single { .. })
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

			Self::Single { .. } => None,
		}
	}
}

// Parsing {{{

impl ArrayContent {
	pub fn parse(input: ParseStream, map: IdentMap, mode: LengthMode) -> Result<Self> {
		Ok(if input.peek(Token![..]) {
			Self::Infer(input.parse()?)
		} else {
			Self::Source(Box::new(Source::parse_mapped(input, map, mode)?))
		})
	}
}

// }}}

// Implementations {{{

impl ItemSerializeTokens for Unused {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		tokens.append_tokens(|| {
			match self {
				Self::Single { .. } => {
					// 0u8.write_to(writer)?;
					quote!(
						writer.put_u8(0);
					)
				}

				Self::Array(array) => {
					let name = id.formatted();

					match &array.content {
						ArrayContent::Source(source) => {
							let args = TokenStream2::with_tokens(|tokens| {
								source.args_to_tokens(tokens);
							});
							let formatted_args = TokenStream2::with_tokens(|tokens| {
								source.formatted_args_to_tokens(tokens);
							});

							let expr = &source.expr;

							quote!(
								fn #name(#args) -> usize {
									#expr
								}

								writer.put_bytes(
									0u8,
									#name(#formatted_args),
								);
							)
						}

						ArrayContent::Infer(_) => {
							quote!(
								writer.put_bytes(
									0u8,
									// TODO: use padding function
								);
							)
						}
					}
				}
			}
		});
	}
}

impl ItemDeserializeTokens for Unused {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		tokens.append_tokens(|| {
			match self {
				Self::Array(array) => {
					let name = id.formatted();

					match &array.content {
						ArrayContent::Source(source) => {
							let args = TokenStream2::with_tokens(|tokens| {
								source.args_to_tokens(tokens);
							});
							let formatted_args = TokenStream2::with_tokens(|tokens| {
								source.formatted_args_to_tokens(tokens);
							});

							let expr = &source.expr;

							quote!(
								fn #name(#args) -> usize {
									#expr
								}

								reader.advance(#name(#formatted_args));
							)
						}

						ArrayContent::Infer(_) => {
							quote!(
								reader.advance(
									// TODO: use padding function
								);
							)
						}
					}
				}

				Self::Single { .. } => {
					// reader.advance(1);
					quote!(reader.advance(1);)
				}
			}
		});
	}
}

// }}}
