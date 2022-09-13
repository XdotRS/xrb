// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod field;
mod length;
mod unused;

pub use field::*;
pub use length::*;
pub use unused::*;

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, token, Result, Token};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Content {
	Shorthand(Box<Shorthand>),
	Longhand(Longhand),
}

impl<'a> Content {
	/// Gets the metabyte [`Item`] if one is declared.
	pub fn metabyte(&'a self) -> Option<&'a Item> {
		match self {
			Self::Shorthand(shorthand) => shorthand
				.item
				.as_ref()
				// Pattern match the pair of `Token![,]` and the item to get
				// just the item.
				.map(|(_, item)| item)
				// If the item is not declared for the metabyte position, filter
				// it out.
				.filter(|item| item.is_metabyte()),

			Self::Longhand(longhand) => longhand
				.items
				.iter()
				// Find a metabyte item from the longhand definition's list of
				// items, if there is any.
				.find(|item| item.is_metabyte()),
		}
	}

	/// Gets a [`Vec`] of the content's declared [`Item`]s without the metabyte
	/// [`Item`].
	pub fn items_sans_metabyte(&'a self) -> Vec<&'a Item> {
		match self {
			Self::Shorthand(shorthand) => shorthand
				.item
				.as_ref()
				// If the item is declared for the metabyte position, filter it
				// out.
				.filter(|(_, item)| !item.is_metabyte())
				// Return either an empty `Vec` if there is no non-metabyte
				// item, or a `Vec` with the item if there is.
				.map_or_else(std::vec::Vec::new, |(_, item)| vec![item]),

			Self::Longhand(longhand) => longhand
				.items
				.iter()
				// Filter out the metabyte item.
				.filter(|item| !item.is_metabyte())
				.collect(),
		}
	}

	/// Gets a <code>Vec<&'a [Item]></code> of the declared items.
	pub fn items(&'a self) -> Vec<&'a Item> {
		match self {
			Self::Shorthand(shorthand) => shorthand
				.item
				.as_ref()
				// If there is an item declared, return a `Vec` containing that
				// item, otherwise return an empty `Vec`.
				.map_or_else(std::vec::Vec::new, |(_, item)| vec![item]),

			Self::Longhand(longhand) => longhand.items.iter().collect(),
		}
	}

	/// Filters the declared items to a `Vec` of just fields.
	pub fn fields(&'a self) -> Vec<&'a Field> {
		match self {
			Self::Shorthand(shorthand) => {
				shorthand
					.item
					.as_ref()
					// If there is no item declaration, return an empty `Vec`,
					// otherwise if there is, then if it is a field, return a
					// `Vec` with that item, or, if it isn't a field, return an
					// empty `Vec` too.
					.map_or_else(std::vec::Vec::new, |(_, item)| match item {
						Item::Field(field) => vec![field],
						_ => vec![],
					})
			}

			Self::Longhand(longhand) => longhand
				.items
				.iter()
				// Filter the `Vec` of `Item`s to a `Vec` of `Field`s.
				.filter_map(|item| match item {
					Item::Field(field) => Some(field),
					_ => None,
				})
				.collect(),
		}
	}
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Shorthand {
	/// An optional item declaration preceded by a colon (`:`).
	///
	/// # Examples
	/// ```ignore
	/// use xrb_proc_macros::messages;
	/// use xrb::Window;
	///
	/// messages! {
	///     // With the optional item declaration:
	///     pub struct DestroyWindow(4): pub target: Window;
	///
	///     // Without:
	///     pub struct GrabServer(36);
	/// }
	/// ```
	pub item: Option<(Token![:], Item)>,
	/// The trailing semicolon (`;`) that is requried for a shorthand definition.
	pub semicolon_token: Token![;],
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Longhand {
	/// The braces (`{` and `}`) that surround the declared `items`.
	pub brace_token: token::Brace,
	/// A list of [`Item`]s punctuated with commas. The final comma is optional.
	pub items: Punctuated<Item, Token![,]>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Item {
	UnusedBytes(UnusedBytes),
	FieldLength(FieldLength),
	Field(Field),
}

impl Item {
	/// Returns whether this [`Item`] is defined for the metabyte position.
	pub fn is_metabyte(&self) -> bool {
		match self {
			// If a `field` item has a metabyte token, it is a metabyte item.
			Self::Field(field) => field.metabyte_token.is_some(),

			// If a `field_len` item has a metabyte token, it is a metabyte item.
			Self::FieldLength(field_len) => field_len.metabyte_token.is_some(),

			Self::UnusedBytes(unused) => match unused {
				// If a single `unused` byte item has a metabyte token, it is a
				// metabyte item.
				UnusedBytes::Single((metabyte_token, _)) => metabyte_token.is_some(),
				// A fully specified unused bytes item can't have a metabyte
				// token.
				UnusedBytes::FullySpecified(_) => false,
			},
		}
	}
}

// Expansion {{{

impl ToTokens for Item {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Field(field) => field.to_tokens(tokens),
			Self::FieldLength(field_len) => field_len.to_tokens(tokens),
			Self::UnusedBytes(unused) => unused.to_tokens(tokens),
		}
	}
}

// }}}

// Parsing {{{

impl Parse for Content {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(token::Brace) {
			Self::Longhand(input.parse()?)
		} else {
			Self::Shorthand(input.parse()?)
		})
	}
}

impl Parse for Shorthand {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// Optional: `(Token![:], Item)`:
			item: input.parse().ok().zip(input.parse().ok()),
			// `;`.
			semicolon_token: input.parse()?,
		})
	}
}

impl Parse for Longhand {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		Ok(Self {
			brace_token: braced!(content in input),
			items: content.parse_terminated(Item::parse)?,
		})
	}
}

impl Parse for Item {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(
			if (input.peek(Token![#]) && !input.peek2(token::Bracket))
				|| (input.peek(Token![$]) && input.peek2(Token![#]))
			{
				Self::FieldLength(input.parse()?)
			} else if input.peek(token::Paren)
				|| input.peek(token::Bracket)
				|| (input.peek(Token![$]) && input.peek2(token::Paren))
			{
				Self::UnusedBytes(input.parse()?)
			} else {
				Self::Field(input.parse()?)
			},
		)
	}
}

// }}}
