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

impl Shorthand {
	/// If a [`Field`] is declared, returns `Some` of that [`Field`].
	pub fn field(&self) -> Option<Field> {
		self.item
			.clone()
			.and_then(|declaration| match declaration.1 {
				Item::Field(field) => Some(field),
				_ => None,
			})
	}
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Longhand {
	/// The braces (`{` and `}`) that surround the declared `items`.
	pub brace_token: token::Brace,
	/// A list of [`Item`]s punctuated with commas. The final comma is optional.
	pub items: Punctuated<Item, Token![,]>,
}

impl Longhand {
	/// Returns a list of the [`Field`] items contained within `self.items`.
	pub fn fields(&self) -> Vec<&Field> {
		self.items
			.iter()
			.filter_map(|item| match item {
				Item::Field(field) => Some(field),
				_ => None,
			})
			.collect()
	}

	/// Returns the item within `self.items` that defines the metabyte position,
	/// if any.
	#[allow(dead_code)]
	pub fn metabyte(&self) -> Option<&Item> {
		self.items.iter().find(|item| item.is_metabyte())
	}

	/// Returns `self.items` with metabyte items removed.
	#[allow(dead_code)]
	pub fn items_sans_metabyte(&self) -> Vec<&Item> {
		self.items
			.iter()
			.filter(|item| !item.is_metabyte())
			.collect()
	}
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
