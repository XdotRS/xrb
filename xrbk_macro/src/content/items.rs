// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod iter;

use crate::*;

use std::collections::HashMap;

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, ToTokens};
use syn::{
	braced, parenthesized,
	parse::{Parse, ParseStream, Result},
	punctuated::{Pair, Punctuated},
	token, Ident, Token,
};

pub enum Items {
	/// [`Item`]s surrounded by curly brackets (`{` and `}`), with names for
	/// [`Field`]s.
	Named {
		brace_token: token::Brace,
		items: Punctuated<ItemWithId, Token![,]>,
	},

	/// [`Item`]s surrounded by normal brackets (`(` and `)`), without names
	/// for [`Field`]s.
	Unnamed {
		paren_token: token::Paren,
		items: Punctuated<ItemWithId, Token![,]>,
	},

	/// No [`Item`]s at all.
	Unit,
}

type ItemWithId = (ItemId, Item);

pub enum ItemId {
	/// An `ItemId` associated with fields.
	///
	/// Named fields have a [`FieldId::Ident`] ID, unnamed fields have a
	/// [`FieldId::Id`] ID.
	Field(FieldId),

	/// An `ItemId` associated with unused bytes items.
	///
	/// 'Array-type' unused bytes items have a `usize` ID, 'unit-type' unused
	/// bytes items do not.
	Unused(Option<usize>),

	/// An `ItemId` associated with let-items.
	Let(Ident),
}

pub enum FieldId {
	Ident(Ident),
	Id(usize),
}

impl ItemId {
	pub fn formatted(&self) -> Option<Ident> {
		match self {
			Self::Field(id) => Some(id.formatted()),

			Self::Unused(id) => id.map(|id| format_ident!("_unused_{}_", id)),

			Self::Let(id) => Some(format_ident!("__{}__", id)),
		}
	}
}

impl FieldId {
	pub fn formatted(&self) -> Ident {
		match self {
			Self::Ident(id) => format_ident!("__{}__", id),
			Self::Id(id) => format_ident!("__{}__", id),
		}
	}
}

// Expansion {{{

impl ToTokens for Items {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		/// An internal-use function within `to_tokens` to reduce repeated
		/// code. This ensures that commas are only converted to tokens if
		/// their respective item is.
		fn items_to_tokens(items: &Punctuated<ItemWithId, Token![,]>, tokens: &mut TokenStream2) {
			// For every pair of item and a possible comma...
			for pair in items.pairs() {
				// Unwrap the item and comma (which will be `None` if it is the
				// final item and there is no trailing comma).
				let (item, comma) = match pair {
					Pair::Punctuated((_, item), comma) => (item, Some(comma)),
					Pair::End((_, item)) => (item, None),
				};

				// If this is a field, convert the  field and the comma to
				// tokens, otherwise... don't.
				if let Item::Field(field) = item {
					field.to_tokens(tokens);
					comma.to_tokens(tokens);
				}
			}
		}

		match self {
			// Surround named items with their curly brackets.
			Self::Named { brace_token, items } => {
				brace_token.surround(tokens, |tokens| items_to_tokens(items, tokens));
			}

			// Surround unnamed items with their normal brackets.
			Self::Unnamed { paren_token, items } => {
				paren_token.surround(tokens, |tokens| items_to_tokens(items, tokens));
			}

			// Don't convert `Self::Unit` to any tokens at all.
			Self::Unit => (),
		}
	}
}

impl Items {
	/// Generates the pattern required to pattern-match against these items
	/// (e.g. in a `match` expression).
	pub fn pattern_to_tokens(&self, tokens: &mut TokenStream2) {
		/// An internal-use function within `patterns_to_tokens` to reduce
		/// repeated code. This generates the pattern to match against the
		/// items.
		fn pattern(tokens: &mut TokenStream2, items: &Punctuated<ItemWithId, Token![,]>) {
			for pair in items.pairs() {
				// Unwrap the item ID, item, and comma (which will be `None`
				// if it is the final item and there is no trailing comma).
				let ((id, item), comma) = match pair {
					Pair::Punctuated((id, item), comma) => ((id, item), Some(comma)),
					Pair::End((id, item)) => ((id, item), None),
				};

				// Only generate the pattern for fields.
				if let Item::Field(_) = item {
					// Convert the field's formatted identifier to tokens.
					id.formatted().to_tokens(tokens);

					// Convert the comma after the field to tokens too.
					comma.to_tokens(tokens);
				}
			}
		}

		match self {
			// Surround named item patterns with their curly brackets.
			Self::Named { brace_token, items } => {
				brace_token.surround(tokens, |tokens| pattern(tokens, items));
			}

			// Surround unnamed items with their normal brackets.
			Self::Unnamed { paren_token, items } => {
				paren_token.surround(tokens, |tokens| pattern(tokens, items))
			}

			// Don't generate a pattern for `Self::Unit` at all.
			Self::Unit => {}
		}
	}
}

// }}}

// Parsing {{{

impl Items {
	pub(self) fn parse_items(
		input: ParseStream,
		named: bool,
	) -> Result<Punctuated<ItemWithId, Token![,]>> {
		let mut unused_index: usize = 0;
		let mut field_index: usize = 0;

		let mut items = Punctuated::new();
		// Keep track of the identifiers defined thus far and which types they
		// correspond to. This is used to parse `Source`s.
		let mut map = HashMap::new();

		// While there are still tokens left in the `input` stream, we continue
		// to parse items.
		while !input.is_empty() {
			if input.peek(token::Bracket) || input.peek(token::Paren) {
				// If the next token (i.e. the start of a new item) is a square
				// bracket or a normal bracket, then this must be an unused
				// bytes item (either in the form `[(); source]`, or just `()`).

				let unused = Unused::parse(input, &map)?;

				let id = match unused {
					Unused::Array(_) => {
						// 'Save' the current `unused_index` to return it.
						let index = unused_index;

						// If this is an `Unused::Array`, it will use the
						// `unused_index`, which must therefore be incremented
						// by one:
						unused_index += 1;

						Some(index)
					}

					// `Unused::Unit` uses no index because it does not
					// generate a source function that is to be referred to.
					Unused::Unit(_) => None,
				};

				items.push_value((ItemId::Unused(id), Item::Unused(unused)));
			} else if input.peek(Token![let]) {
				// Otherwise, if the next token is `Let`, then this must be a
				// `Let` item. Note that this won't work if support for
				// attributes is added to `Let` items: in that case we would
				// have to parse all of the attributes before we could work out
				// if it was a `Field` item or a `Let` item.
				let r#let: Let = input.parse()?;

				// We insert the name of the `Let` item into the `map`, since
				// it will be able to be referred to by name in `Source`s, and
				// we'll want to know its type.
				map.insert(r#let.ident.to_owned(), r#let.r#type.to_owned());

				// Push the new `Item::Let` to the list of `items`.
				items.push_value((
					ItemId::Let(r#let.ident.to_owned()),
					Item::Let(Box::new(r#let)),
				));
			} else {
				// Otherwise, if this is not an unused bytes item, nor a `Let`
				// item, we assume it is a `Field` and parse it accordingly.

				let field = if named {
					// If we are to parse the items as `named`, then we parse
					// the `field` as as named:
					Field::parse_named(input, &map)?
				} else {
					// Otherwise, we parse the field as unnamed:
					Field::parse_unnamed(input, &map)?
				};

				let id = if let Some(ident) = &field.ident {
					FieldId::Ident(ident.to_owned())
				} else {
					let index = field_index;
					field_index += 1;

					FieldId::Id(index)
				};

				match &id {
					FieldId::Ident(ident) => {
						map.insert(ident.to_owned(), field.r#type.to_owned());
					}

					FieldId::Id(id) => {
						map.insert(
							Ident::new(&id.to_string(), Span::call_site()),
							field.r#type.to_owned(),
						);
					}
				}

				// Then we push the `Item::Field` to the list of `items`.
				items.push_value((ItemId::Field(id), Item::Field(Box::new(field))));
			}

			// If the token following the item is not a comma, then it must be
			// the end of the list, so we break from the loop.
			if !input.peek(Token![,]) {
				break;
			} else {
				// Otherwise, if the next token is a comma, then the list can
				// continue: we add the comma to the list.
				items.push_punct(input.parse()?);
			}
		}

		Ok(items)
	}

	/// Parse [`Items`] surrounded by curly brackets (`{` and `}`) and with
	/// named [`Field`s`](Field).
	pub fn parse_named(input: ParseStream) -> Result<Self> {
		let content;

		let brace_token = braced!(content in input);
		let items = Self::parse_items(&content, true)?;

		Ok(Self::Named { brace_token, items })
	}

	/// Parse [`Items`] surrounded by normal brackets (`(` and `)`) and with
	/// unnamed [`Field`s](Field).
	pub fn parse_unnamed(input: ParseStream) -> Result<Self> {
		let content;

		let paren_token = parenthesized!(content in input);
		let items = Self::parse_items(&content, false)?;

		Ok(Self::Unnamed { paren_token, items })
	}
}

impl Parse for Items {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(token::Brace) {
			// If the next token is a curly bracket (`{`), parse as named
			// `Item`s.
			Self::parse_named(input)
		} else if input.peek(token::Paren) {
			// Otherwise, if the next token is a normal bracket (`(`), parse as
			// unnamed `Item`s.
			Self::parse_unnamed(input)
		} else {
			// Otherwise, if the next token is neither a curly bracket (`{`),
			// nor a normal bracket (`(`), there are no items; simply return
			// `Self::Unit`.
			Ok(Self::Unit)
		}
	}
}

// }}}
