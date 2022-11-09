// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod attributes;
mod field;
mod r#let;
mod source;
mod unused;

pub use attributes::*;
pub use field::*;
pub use r#let::*;
pub use source::*;
pub use unused::*;

use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens, quote};
use std::collections::HashMap;
use syn::parse::Parse;
use syn::{
	braced, parenthesized,
	parse::{ParseStream, Result},
	punctuated::Punctuated,
	token, Token, Ident,
};
use syn::__private::TokenStream2;
use syn::punctuated::Pair;

pub enum Item {
	Field(Box<Field>),
	Let(Box<Let>),
	Unused(Unused),
}

pub enum Items {
	/// [`Item`]s surrounded by curly brackets (`{` and `}`), with names for
	/// [`Field`]s.
	Named(token::Brace, Punctuated<Item, Token![,]>),

	/// [`Item`]s surrounded by normal brackets (`(` and `)`), without names
	/// for [`Field`]s.
	Unnamed(token::Paren, Punctuated<Item, Token![,]>),

	/// No [`Item`]s at all.
	Unit,
}

impl Items {
	pub fn iter(&self) -> syn::punctuated::Iter<Item> {
		match self {
			Self::Named(_, items) => items.iter(),
			Self::Unnamed(_, items) => items.iter(),

			Self::Unit => <Punctuated<Item, Token![,]>>::default().iter(),
		}
	}
}

impl IntoIterator for Items {
	type Item = Item;
	type IntoIter = syn::punctuated::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		match self {
			Self::Named(_, items) => items.into_iter(),
			Self::Unnamed(_, items) => items.into_iter(),

			// If there are no items, create an empty iterator from
			// `Punctuated::default()`.
			Self::Unit => <Punctuated<Item, Token![,]>>::default().into_iter(),
		}
	}
}

// Expansion {{{

impl ToTokens for Item {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		// If `self` is a `Field`, convert it to tokens, otherwise don't - the
		// other items are used for generating the serialization and
		// deserialization code.
		if let Self::Field(field) = self {
			field.to_tokens(tokens);
		}
	}
}

impl Item {
	pub fn identifier(&self, item_index: usize) -> Option<Ident> {
		/// An internal-use function within [`Item::identifier`] to format a
		/// named field's `Ident`.
		fn fmt_named_ident(ident: &Ident) -> Ident {
			format_ident!("__{}__", ident)
		}

		/// An internal-use function within [`Item::Identifier`] to format an
		/// identifier for an item based on the item's index.
		fn fmt_ident(i: usize) -> Ident {
			format_ident!("_item_{}_", i)
		}

		match self {
			Self::Field(field) => Some(
				if let Some(ident) = &field.ident {
					// If this is a named field, format the field's identifier
					// with `fmt_named_ident` to avoid any possible name
					// conflicts.
					fmt_named_ident(ident)
				} else {
					// Otherwise, if this is an unnamed field, format an item
					// identifier with `fmt_ident`.
					fmt_ident(item_index)
				}
			),

			// If this is a let item, format the let item's identifier with
			// `fmt_named_ident`.
			Self::Let(r#let) => Some(fmt_named_ident(&r#let.ident)),

			Self::Unused(unused) => {
				if unused.is_array() {
					// If this is an 'array-type' unused item (meaning it has a
					// source function), format its identifier with `fmt_ident`.
					Some(fmt_ident(item_index))
				} else {
					// Otherwise, if this is a 'unit-type' unused item (meaning
					// it is always read and written as a single byte), there
					// is no source function, and therefore no identifier.
					None
				}
			},
		}
	}

	pub fn serialize_tokens(&self, tokens: &mut TokenStream, i: usize) {
		match self {
			Self::Field(field) => {
				let name = (field.ident.as_ref())
					.map_or_else(
						|| format_ident!("item{}", i),
						|ident| ident.to_owned(),
					);

				quote!(#name.write_to(writer)?;).to_tokens(tokens);
			}

			Self::Let(r#let) => {

			}

			Self::Unused(unused) => {

			}
		}
	}
}

impl ToTokens for Items {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		/// An internal-use function within `to_tokens` to reduce repeated
		/// code. This ensures that commas are only converted to tokens if
		/// their respective item is.
		fn items_to_tokens(items: &Punctuated<Item, Token![,]>, tokens: &mut TokenStream) {
			// For every pair of item and a possible comma...
			for pair in items.pairs() {
				// Unwrap the item and comma (which will be `None` if it is the
				// final item and there is no trailing comma).
				let (item, comma) = match pair {
					Pair::Punctuated(item, comma) => (item, Some(comma)),
					Pair::End(item) => (item, None),
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
			Self::Named(brace_token, items) => {
				brace_token.surround(tokens, |tokens| items_to_tokens(items, tokens));
			}

			// Surround unnamed items with their normal brackets.
			Self::Unnamed(paren_token, items) => {
				paren_token.surround(tokens, |tokens| items_to_tokens(items, tokens));
			}

			// Don't convert `Self::Unit` to any tokens at all.
			Self::Unit => (),
		}
	}
}

impl Items {
	/// Generates the pattern required to pattern-match against these items
	/// (e.g. in a `match` expression), returning a new [`TokenStream`].
	pub fn pattern_to_tokenstream(&self) -> TokenStream {
		let mut tokens = TokenStream::new();
		self.pattern_to_tokens(&mut tokens);

		tokens
	}

	/// Generates the pattern required to pattern-match against these items
	/// (e.g. in a `match` expression).
	pub fn pattern_to_tokens(&self, tokens: &mut TokenStream) {
		/// An internal-use function within `patterns_to_tokens` to reduce
		/// repeated code. This generates the pattern to match against the
		/// items.
		fn pattern(tokens: &mut TokenStream, items: &Punctuated<Item, Token![,]>) {
			// Enumerate every pair of item and a possible comma with an
			// index...
			for (i, pair) in items.pairs().enumerate() {
				// Unwrap the item and comma (which will be `None` if it is the
				// final item and there is no trailing comma).
				let (item, comma) = match pair {
					Pair::Punctuated(item, comma) => (item, Some(comma)),
					Pair::End(item) => (item, None),
				};

				// Only generate the pattern for fields.
				if let Item::Field(_) = item {
					// Use an appropriate identifier for the field (see
					// [`Item::identifier`] for more information).
					item.identifier(i).to_tokens(tokens);

					// Convert the comma after the field to tokens too.
					comma.to_tokens(tokens);
				}
			}
		}

		match self {
			// Surround named item patterns with their curly brackets.
			Self::Named(brace_token, items) => {
				brace_token.surround(tokens, |tokens| pattern(tokens, items));
			}

			// Surround unnamed items with their normal brackets.
			Self::Unnamed(paren_token, items) => {
				paren_token.surround(tokens, |tokens| pattern(tokens, items))
			}

			// Don't generate a pattern for `Self::Unit` at all.
			Self::Unit => {}
		}
	}

	pub fn destructure_tokens(&self, tokens: &mut TokenStream2) {
		let pat = self.pattern_to_tokenstream();
		quote!(let Self #pat = self;).to_tokens(tokens);
	}
}

// }}}

// Parsing {{{

impl Items {
	pub(self) fn parse_items(
		input: ParseStream,
		named: bool,
	) -> Result<Punctuated<Item, Token![,]>> {
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
				items.push_value(Item::Unused(Unused::parse(input, &map)?));
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
				items.push_value(Item::Let(Box::new(r#let)));
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

				// If the field is `named`, then we want to add its name to the
				// `map` of identifiers.
				// TODO: If it is unnamed, add the field's index to the map
				//       (e.g. 0 for the first unnamed field, 1 for the second,
				//       etc.)
				if named {
					map.insert(
						// TODO: We are copying the identifier and type
						//       here... do we need to?
						field.ident.to_owned().expect("this must be named"),
						field.r#type.to_owned(),
					);
				}

				// Then we push the `Item::Field` to the list of `items`.
				items.push_value(Item::Field(Box::new(field)));
			}

			// If the token following the item is not a comma, then it must be
			// the end of the list, so we break from the loop.
			if !input.peek(Token![,]) {
				break;
			}

			// Otherwise, if the next token is a comma, then the list can
			// continue: we add the comma to the list.
			items.push_punct(input.parse()?);
		}

		Ok(items)
	}

	/// Parse [`Items`] surrounded by curly brackets (`{` and `}`) and with
	/// named [`Field`s`](Field).
	pub fn parse_named(input: ParseStream) -> Result<Self> {
		let content;

		let brace_token = braced!(content in input);
		let items = Self::parse_items(&content, true)?;

		Ok(Self::Named(brace_token, items))
	}

	/// Parse [`Items`] surrounded by normal brackets (`(` and `)`) and with
	/// unnamed [`Field`s](Field).
	pub fn parse_unnamed(input: ParseStream) -> Result<Self> {
		let content;

		let paren_token = parenthesized!(content in input);
		let items = Self::parse_items(&content, false)?;

		Ok(Self::Unnamed(paren_token, items))
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
