// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::{
	parse::{Parse, ParseStream},
	punctuated::{Punctuated, Pair},
	Expr, Ident, Token, Type,
};

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, format_ident};

pub struct Param {
	pub ident: Ident,
	pub colon_token: Token![:],
	pub r#type: Type,
}

pub enum Params {
	None(Token![_]),
	One(Param),
	Some(Punctuated<Param, Token![,]>),
}

pub struct Source {
	pub params: Params,
	pub arrow_token: Option<Token![->]>,
	pub expr: Option<Expr>,
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
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::None(_) => (),
			Self::One(param) => param.to_tokens(tokens),
			Self::Some(params) => params.to_tokens(tokens),
		}
	}
}

impl Params {
	pub fn to_call_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::None(_) => (),
			Self::One(Param { ident, .. }) => ident.to_tokens(tokens),
			Self::Some(params) => {
				for pair in params.pairs() {
					let (Param { ident, .. }, comma) = match pair {
						Pair::Punctuated(param, comma) => (param, Some(comma)),
						Pair::End(param) => (param, None),
					};

					format_ident!("__{}__", ident).to_tokens(tokens);
					comma.to_tokens(tokens);
				}
			}
		}
	}

	pub fn to_call_token_stream(&self) -> TokenStream2 {
		let mut tokens = TokenStream2::new();
		self.to_call_tokens(&mut tokens);

		tokens
	}
}

impl Source {
	/// Converts this `Source` to a function of the given `name` and return
	/// `ty`pe.
	///
	/// For example, `_ -> 7` with the `name` `n` and the `ty`pe `i32` would
	/// convert to the following:
	/// ```
	/// fn n() -> i32 {
	///     7
	/// }
	/// ```
	pub fn to_tokens(&self, tokens: &mut TokenStream2, name: Ident, r#type: Type) {
		let params = self.params;
		let params_call = self.params.to_call_token_stream();
		// TODO: expr

		quote! {
			fn #name(#params) -> #r#type {
				// #expr
			}
		}
		.to_tokens(tokens);
	}
}

// }}}

// Parsing {{{

impl Parse for Param {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(Self {
			ident: input.parse()?,
			colon_token: input.parse()?,
			r#type: input.parse()?,
		})
	}
}

impl Parse for Params {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let look = input.lookahead1();

		if input.peek(Token![_]) {
			Ok(Self::None(input.parse()?))
		} else if input.peek(Ident) {
			Ok(Self::Some(input.parse_terminated(Param::parse)?))
		} else {
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

// }}}
