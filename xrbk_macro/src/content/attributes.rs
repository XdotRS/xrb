// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{
	braced, bracketed, parenthesized, parse::ParseStream, spanned::Spanned, token, Error, Path,
	Result, Token, Type,
};

use super::source::Source;

/// An attribute, reimplemented to allow for [`Context`] and metabyte attributes.
pub struct Attribute {
	/// A hash token: `#`.
	pub hash_token: Token![#],
	/// The style of attribute (outer or inner, the latter denoted by `!`).
	pub style: Option<Token![!]>,
	/// A pair of square bracket tokens (`[` and `]`).
	pub bracket_token: token::Bracket,
	/// The content of the attribute.
	pub content: AttrContent,
}

impl Attribute {
	/// Whether this is an [`AttrContent::Context`] attribute.
	pub const fn is_context(&self) -> bool {
		matches!(self.content, AttrContent::Context(..))
	}

	/// Whether this is an [`AttrContent::Metabyte`] attribute.
	pub const fn is_metabyte(&self) -> bool {
		matches!(self.content, AttrContent::Metabyte(..))
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
pub enum AttrContent {
	Context(Path, Box<Context>),
	Metabyte(Path),

	Other(Path, TokenStream2),
}

/// An attribute that provides context for the deserialization of an `Item`.
pub enum Context {
	/// ```ignore
	/// #[context = data_len => data_len]
	/// ```
	Equals(Token![=], Source),
	/// ```ignore
	/// #[context: data_len => data_len]
	/// ```
	Colon(Token![:], Source),
	/// ```ignore
	/// #[context(data_len => data_len)]
	/// ```
	Paren(token::Paren, Source),
	/// ```ignore
	/// #[context[data_len => data_len]]
	/// ```
	Bracket(token::Bracket, Source),
	/// ```ignore
	/// #[context {
	///     data_len => data_len
	/// }]
	/// ```
	Brace(token::Brace, Source),
}

impl Context {
	pub fn source(&self) -> &Source {
		match self {
			Self::Equals(_, source) => source,
			Self::Colon(_, source) => source,
			Self::Paren(_, source) => source,
			Self::Bracket(_, source) => source,
			Self::Brace(_, source) => source,
		}
	}
}

// Expansion {{{

impl ToTokens for Attribute {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// If this is `AttrContent::Other`, convert it to tokens.
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

impl Attribute {
	pub(self) fn parse(input: ParseStream, map: &HashMap<String, Type>) -> Result<Self> {
		let content;

		let hash_token = input.parse()?;
		let style: Option<Token![!]> = input.parse().ok();
		let bracket_token = bracketed!(content in input);
		let attr_content = AttrContent::parse(&content, map)?;

		// If this is an inner context attribute, generate an error:
		if let Some(style) = style {
			if let AttrContent::Context(..) = attr_content {
				return Err(Error::new(
					style.span,
					"inner context attributes are not allowed",
				));
			}
		}

		Ok(Self {
			hash_token,
			style,
			bracket_token,
			content: attr_content,
		})
	}

	pub fn parse_metabyte(input: ParseStream) -> Result<Self> {
		let content;

		Ok(Self {
			hash_token: input.parse()?,
			style: None,
			bracket_token: bracketed!(content in input),
			content: AttrContent::parse_metabyte(&content)?,
		})
	}

	pub fn parse_outer(input: ParseStream, map: &HashMap<String, Type>) -> Result<Vec<Self>> {
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

	#[allow(dead_code)]
	pub fn parse_inner(input: ParseStream, map: &HashMap<String, Type>) -> Result<Vec<Self>> {
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

impl AttrContent {
	fn parse(input: ParseStream, map: &HashMap<String, Type>) -> Result<Self> {
		let path: Path = input.parse()?;

		Ok(if path.is_ident("context") {
			Self::Context(path, Box::new(Context::parse(input, map)?))
		} else if path.is_ident("metabyte") {
			Self::Metabyte(path)
		} else {
			Self::Other(path, input.parse()?)
		})
	}

	fn parse_metabyte(input: ParseStream) -> Result<Self> {
		let path: Path = input.parse()?;

		if !path.is_ident("metabyte") {
			return Err(Error::new(
				path.span(),
				"only metabyte attribute is allowed in this position",
			));
		}

		Ok(Self::Metabyte(path))
	}
}

impl Context {
	fn parse(input: ParseStream, map: &HashMap<String, Type>) -> Result<Self> {
		let content;
		let look = input.lookahead1();

		if look.peek(Token![=]) {
			// Equals sign context (`=`)
			Ok(Self::Equals(
				input.parse()?,
				Source::parse_mapped(input, map)?,
			))
		} else if look.peek(Token![:]) {
			// Colon context (`:`)
			Ok(Self::Colon(
				input.parse()?,
				Source::parse_mapped(input, map)?,
			))
		} else if look.peek(token::Paren) {
			// Normal bracket context (`(...)`)
			Ok(Self::Paren(
				parenthesized!(content in input),
				Source::parse_mapped(&content, map)?,
			))
		} else if look.peek(token::Bracket) {
			// Square bracket context (`[...]`)
			Ok(Self::Bracket(
				bracketed!(content in input),
				Source::parse_mapped(&content, map)?,
			))
		} else if look.peek(token::Brace) {
			// Curly bracket context (`{...}`)
			Ok(Self::Brace(
				braced!(content in input),
				Source::parse_mapped(&content, map)?,
			))
		} else {
			// Otherwise, if the next token after `context` is none of those,
			// generate an error.
			Err(look.error())
		}
	}
}

// }}}
