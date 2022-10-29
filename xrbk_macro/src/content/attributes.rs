// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use std::collections::HashMap;
use syn::{
	braced, bracketed, parenthesized, parse::ParseStream, token, Error, Ident, Path, Result, Token,
	Type,
};

use super::source::Source;

/// An attribute, reimplemented to allow for [`Context`].
pub struct Attribute<'a> {
	/// A hash token: `#`.
	pub hash_token: Token![#],
	/// The style of attribute (outer or inner, the latter denoted by `!`).
	pub style: Option<Token![!]>,
	/// A pair of square bracket tokens (`[` and `]`).
	pub bracket_token: token::Bracket,
	/// The content of the attribute.
	pub content: AttrContent<'a>,
}

impl Attribute<'_> {
	/// Whether this is a [`AttrContent::Context`] attribute.
	pub const fn is_context(&self) -> bool {
		matches!(self.content, AttrContent::Context(..))
	}

	/// Whether this is an inner style attribute.
	pub const fn is_inner(&self) -> bool {
		self.style.is_some()
	}

	/// Whether this is an outer style attribute.
	pub const fn is_outer(&self) -> bool {
		self.style.is_none()
	}
}

/// The content of an [`Attribute`] (what is between the square brackets).
pub enum AttrContent<'a> {
	Context(Path, Context<'a>),
	Other(Path, TokenStream2),
}

/// An attribute that provides context for the deserialization of an `Item`.
pub enum Context<'a> {
	/// ```
	/// #[context = data_len => data_len]
	/// ```
	Equals(Token![=], Source<'a>),
	/// ```
	/// #[context: data_len => data_len]
	/// ```
	Colon(Token![:], Source<'a>),
	/// ```
	/// #[context(data_len => data_len)]
	/// ```
	Paren(token::Paren, Source<'a>),
	/// ```
	/// #[context[data_len => data_len]]
	/// ```
	Bracket(token::Bracket, Source<'a>),
	/// ```
	/// #[context {
	///     data_len => data_len
	/// }]
	/// ```
	Brace(token::Brace, Source<'a>),
}

// Expansion {{{

impl ToTokens for Attribute<'_> {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// If this is `AttrContent::Other`, convert it to tokens. Otherwise, in
		// the case of `AttrContent::Context`, it simply provides context when
		// expanding the deserialization code, and so won't actually exist as
		// an attribute on the item.
		if let AttrContent::Other(path, content) = &self.content {
			// `#`
			self.hash_token.to_tokens(tokens);
			// Optional `!`
			self.style.to_tokens(tokens);

			// Surround the content with the square brackets
			self.bracket_token.surround(tokens, |tokens| {
				path.to_tokens(tokens);
				content.to_tokens(tokens);
			});
		}
	}
}

// }}}

// Parsing {{{

impl Attribute<'_> {
	fn parse(input: ParseStream, map: HashMap<Ident, Type>) -> Result<Self> {
		let content;

		let hash_token = input.parse()?;
		let style: Option<Token![!]> = input.parse().ok();
		let bracket_token = bracketed!(content in input);
		let content = AttrContent::parse(input, map)?;

		// If this is an inner context attribute, generate an error:
		if style.is_some() {
			if let AttrContent::Context(..) = content {
				return Err(Error::new(
					style.unwrap().span,
					"inner context attributes are not allowed",
				));
			}
		}

		Ok(Self {
			hash_token,
			style,
			bracket_token,
			content,
		})
	}

	pub fn parse_outer(input: ParseStream, map: HashMap<Ident, Type>) -> Result<Vec<Self>> {
		let mut attributes = vec![];

		while input.peek(Token![#]) && input.peek2(token::Bracket) {
			let attribute: Attribute = Self::parse(input, map)?;

			// If this is an inner attribute, generate an error:
			if attribute.style.is_some() {
				return Err(Error::new(
					attribute.style.expect("already checked for this").span,
					"inner attribute style not allowed in this position",
				));
			}

			attributes.push(attribute);
		}

		Ok(attributes)
	}

	pub fn parse_inner(input: ParseStream, map: HashMap<Ident, Type>) -> Result<Vec<Self>> {
		let mut attributes = vec![];

		while input.peek(Token![#]) && input.peek2(token::Bracket) {
			let attribute: Attribute = Self::parse(input, map)?;

			// If this is an outer attribute, generate an error:
			if attribute.style.is_none() {
				return Err(Error::new(
					attribute.bracket_token.span,
					"expected inner attribute style in this position",
				));
			}

			attributes.push(attribute);
		}

		Ok(attributes)
	}
}

impl AttrContent<'_> {
	fn parse(input: ParseStream, map: HashMap<Ident, Type>) -> Result<Self> {
		let path: Path = input.parse()?;

		Ok(if path.is_ident("context") {
			Self::Context(path, Context::parse(input, map)?)
		} else {
			Self::Other(path, input.parse()?)
		})
	}
}

impl Context<'_> {
	fn parse(input: ParseStream, map: HashMap<Ident, Type>) -> Result<Self> {
		let content;
		let look = input.lookahead1();

		let parse_source = || Source::parse_without_receiver(input, map);

		if look.peek(Token![=]) {
			// Equals sign context (`=`)
			Ok(Self::Equals(input.parse()?, parse_source()?))
		} else if look.peek(Token![:]) {
			// Colon context (`:`)
			Ok(Self::Colon(input.parse()?, parse_source()?))
		} else if look.peek(token::Paren) {
			// Normal bracket context (`(...)`)
			Ok(Self::Paren(
				parenthesized!(content in input),
				parse_source()?,
			))
		} else if look.peek(token::Bracket) {
			// Square bracket context (`[...]`)
			Ok(Self::Bracket(bracketed!(content in input), parse_source()?))
		} else if look.peek(token::Brace) {
			// Curly bracket context (`{...}`)
			Ok(Self::Brace(braced!(content in input), parse_source()?))
		} else {
			// Otherwise, if the next token after `context` is none of those,
			// generate an error.
			Err(look.error())
		}
	}
}

// }}}
