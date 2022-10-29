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

pub struct Attribute<'a> {
	pub hash_token: Token![#],
	pub style: Option<Token![!]>,
	pub bracket_token: token::Bracket,
	pub content: AttrContent<'a>,
}

impl Attribute<'_> {
	pub const fn is_context(&self) -> bool {
		matches!(self.content, AttrContent::Context(..))
	}
}

pub enum AttrContent<'a> {
	Context(Path, Context<'a>),
	Other(Path, TokenStream2),
}

pub enum Context<'a> {
	Equals(Token![=], Source<'a>),
	Colon(Token![:], Source<'a>),
	Paren(token::Paren, Source<'a>),
	Bracket(token::Bracket, Source<'a>),
	Brace(token::Brace, Source<'a>),
}

// Expansion {{{

impl ToTokens for Attribute<'_> {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		if let AttrContent::Other(path, content) = &self.content {
			self.hash_token.to_tokens(tokens);
			self.style.to_tokens(tokens);

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

		if style.is_some() {
			if let AttrContent::Context(..) = content {
				return Err(Error::new(
					style.expect("already checked for this").span,
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
			Ok(Self::Equals(input.parse()?, parse_source()?))
		} else if look.peek(Token![:]) {
			Ok(Self::Colon(input.parse()?, parse_source()?))
		} else if look.peek(token::Paren) {
			Ok(Self::Paren(
				parenthesized!(content in input),
				parse_source()?,
			))
		} else if look.peek(token::Bracket) {
			Ok(Self::Bracket(bracketed!(content in input), parse_source()?))
		} else if look.peek(token::Brace) {
			Ok(Self::Brace(braced!(content in input), parse_source()?))
		} else {
			Err(look.error())
		}
	}
}

// }}}
