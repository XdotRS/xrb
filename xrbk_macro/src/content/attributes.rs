// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{
	braced, bracketed, parenthesized,
	parse::{Parse, ParseStream},
	token, Error, Path, Result, Token,
};

use super::general::Source;

pub struct Attribute {
	pub hash_token: Token![#],
	pub style: Option<Token![!]>,
	pub bracket_token: token::Bracket,
	pub content: AttrContent,
}

impl Attribute {
	pub const fn is_context(&self) -> bool {
		matches!(self.content, AttrContent::Context(..))
	}
}

pub enum AttrContent {
	Context(Path, Context),
	Other(Path, TokenStream2),
}

pub enum Context {
	Equals(Token![=], Source),
	Colon(Token![:], Source),
	Paren(token::Paren, Source),
	Bracket(token::Bracket, Source),
	Brace(token::Brace, Source),
}

// Expansion {{{

impl ToTokens for Attribute {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		if let AttrContent::Other(path, content) = self.content {
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

impl Parse for Attribute {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		let hash_token = input.parse()?;
		let style: Option<Token![!]> = input.parse().ok();
		let bracket_token = bracketed!(content in input);
		let content = content.parse()?;

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
}

impl Attribute {
	pub fn parse_outer(input: ParseStream) -> Result<Vec<Self>> {
		let mut attributes = vec![];

		while input.peek(Token![#]) && input.peek2(token::Bracket) {
			let attribute: Attribute = input.parse()?;

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

	pub fn parse_inner(input: ParseStream) -> Result<Vec<Self>> {
		let mut attributes = vec![];

		while input.peek(Token![#]) && input.peek2(token::Bracket) {
			let attribute: Attribute = input.parse()?;

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

impl Parse for AttrContent {
	fn parse(input: ParseStream) -> Result<Self> {
		let path: Path = input.parse()?;

		Ok(if path.is_ident("context") {
			Self::Context(path, input.parse()?)
		} else {
			Self::Other(path, input.parse()?)
		})
	}
}

impl Parse for Context {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;
		let look = input.lookahead1();

		if look.peek(Token![=]) {
			Ok(Self::Equals(input.parse()?, input.parse()?))
		} else if look.peek(Token![:]) {
			Ok(Self::Colon(input.parse()?, input.parse()?))
		} else if look.peek(token::Paren) {
			Ok(Self::Paren(
				parenthesized!(content in input),
				content.parse()?,
			))
		} else if look.peek(token::Bracket) {
			Ok(Self::Bracket(
				bracketed!(content in input),
				content.parse()?,
			))
		} else if look.peek(token::Brace) {
			Ok(Self::Brace(braced!(content in input), content.parse()?))
		} else {
			Err(look.error())
		}
	}
}

// }}}
