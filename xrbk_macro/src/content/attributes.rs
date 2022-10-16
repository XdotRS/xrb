// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
	braced, bracketed, parenthesized,
	parse::{Parse, ParseStream},
	punctuated::{Pair, Punctuated},
	token, Error, Expr, Ident, Path, Result, Token, Type,
};

/// A parameter of the form `ident: Type` in a [`Source`].
pub struct Param {
	pub ident: Ident,
	pub colon_token: Token![:],
	pub r#type: Type,
}

/// Zero, one, or more parameters in a [`Source`].
pub enum Params {
	/// Used when no parameters are required: just an underscore token (`_`).
	None(Token![_]),
	Some(Punctuated<Param, Token![,]>),
}

pub struct Source {
	pub params: Params,
	pub arrow_token: Option<Token![->]>,
	pub expr: Option<Expr>,
}

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

impl ToTokens for Param {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.ident.to_tokens(tokens);
		self.colon_token.to_tokens(tokens);
		self.r#type.to_tokens(tokens);
	}
}

impl ToTokens for Params {
	/// Converts the `Params` `to_tokens` for use in a function definition.
	///
	/// This preserves the identifiers and their types: `x: i32, y: i32` will
	/// be converted `to_tokens` as `x: i32, y: i32`.
	///
	/// See also: [`Params::to_call_tokens`]
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			// No parameters (represented by `_`) are simply not converted to
			// tokens.
			Self::None(_) => (),

			// Since the parameters are going to be converted to the exact same
			// tokens, we can simply call `Punctuated<Param, Token![,]>::to_tokens`.
			Self::Some(params) => params.to_tokens(tokens),
		}
	}
}

impl Params {
	/// Converts the `Params` to tokens for use when calling a generated
	/// function with pre-defined variables.
	///
	/// This surrounds the parameter identifiers with `__` on either side, and
	/// ignores the types: `x: i32, y: i32` is converted to `__x__, __y__`.
	///
	/// See also: [`Params::to_tokens`]
	pub fn to_call_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			// No paramters (represented by `_`) are simply not converted to
			// tokens.
			Self::None(_) => (),

			Self::Some(params) => {
				// For each `Param`:
				for pair in params.pairs() {
					// Let `ident` equal the `Param`'s identifier, and let
					// `comma` equal an `Option<Token![,]>` wrapping a comma if
					// one followed the `Param`.
					let (Param { ident, .. }, comma) = match pair {
						// If the `param` is `Punctuated` with a `comma`, return
						// a pair of `(param, Some(comma))`.
						Pair::Punctuated(param, comma) => (param, Some(comma)),

						// If the `param` is not punctuated with a comma, return
						// a pair of `(param, None)`.
						Pair::End(param) => (param, None),
					};

					// Surround the `ident` with `__` on either side (e.g. `x`
					// -> `__x__`) and write it `to_tokens`.
					format_ident!("__{}__", ident).to_tokens(tokens);
					// Write the comma, if any, `to_tokens`.
					comma.to_tokens(tokens);
				}
			}
		}
	}

	/// Returns a new [`TokenStream`] with the [call tokens].
	///
	/// [`TokenStream`]: TokenStream2
	/// [call tokens]: Self::to_call_tokens
	pub fn to_call_token_stream(&self) -> TokenStream2 {
		let mut tokens = TokenStream2::new();
		self.to_call_tokens(&mut tokens);

		tokens
	}
}

impl Source {
	/// Converts this `Source` to a function of the given `name` and return
	/// `type`.
	///
	/// For example, `_ -> 7` with the `name` `n` and the `type` `i32` would
	/// convert to the following:
	/// ```
	/// fn n() -> i32 {
	///     7
	/// }
	/// ```
	///
	/// And `x: i32, y: i32 -> x * y` with the `name` `mult` and the `type`
	/// `i32` would convert to the following:
	/// ```
	/// fn mult(x: i32, y: i32) -> i32 {
	///     x * y
	/// }
	/// ```
	pub fn to_tokens(&self, tokens: &mut TokenStream2, name: Ident, r#type: Type) {
		let params = self.params;
		let expr = self.expr;

		quote! {
			fn #name(#params) -> #r#type {
				#expr
			}
		}
		.to_tokens(tokens);
	}
}

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

impl Parse for Param {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(Self {
			// The identifier: e.g. `ident`.
			ident: input.parse()?,
			// The colon token: `:`.
			colon_token: input.parse()?,
			// The type: e.g. `i32`.
			r#type: input.parse()?,
		})
	}
}

impl Parse for Params {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let look = input.lookahead1();

		if input.peek(Token![_]) {
			// If the next token is `_`, then this is `Self::None`.
			Ok(Self::None(input.parse()?))
		} else if input.peek(Ident) {
			// Otherwise, if the next token is an `Ident`, then this is
			// `Self::Some` - there are one or more `Ident`s.
			Ok(Self::Some(input.parse_terminated(Param::parse)?))
		} else {
			// Otherwise, if the next  token is not an underscore nor an
			// `Ident`, then generate an error:
			Err(input.error("expected an underscore (denoting no parameters) or a parameter"))
		}
	}
}

impl Parse for Source {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let params: Params = input.parse()?;

		let arrow_token: Option<Token![->]> = input.parse().ok();
		// If `arrow_token` is `None`, then `expr` is not parsed and will be
		// `None`. If `arrow_token` is `Some`, then `expr` must be parsed and
		// will be `Some`.
		let expr: Option<Expr> = arrow_token
			.is_some()
			.then(|| Some(input.parse().ok()?))
			.flatten();

		Ok(Self {
			params,
			arrow_token,
			expr,
		})
	}
}

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
