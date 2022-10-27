// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{
	parse::{discouraged::Speculative, Parse, ParseStream, Result},
	punctuated::Punctuated,
	Expr, Ident, Token, Type,
};

pub struct Source {
	pub params: Punctuated<Ident, Token![,]>,
	pub arrow_token: Option<Token![=>]>,
	// TODO: Allow just a single parameter to fill in for the expression. That
	// probably would be best implemented by keeping track of a map of the
	// already known `Ident`s by this point in parsing (and map them to their
	// `Type`s too, because that would mean we could infer the types and improve
	// ergonomics), so that we can parse a matching `Ident` as a parameter, but
	// an `Ident` that doesn't match as an `Expr` (e.g. if you wanted to refer
	// to a constant or use some certain keywords).
	pub expr: Expr,
}

// Expansion {{{

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
		let params = &self.params;
		let expr = &self.expr;

		quote! {
			fn #name(#params) -> #r#type {
				#expr
			}
		}
		.to_tokens(tokens);
	}
}

// }}}

// Parsing {{{

impl Source {
	pub fn parse(input: ParseStream, map: HashMap<Ident, Type>) -> Result<Self> {
		let ahead = input.fork();
		let idents: Punctuated<Ident, Token![,]> = ahead.parse_terminated(Ident::parse)?;

		if !idents.is_empty() {
			if idents.len() == 1 {
				let ident = idents.first().expect("length of 1 means this exists");

				if map.contains_key(ident) {
					// This means the source was given as a single `Ident`, but
					// that `Ident` was contained in the `map` of
					// already-defined `Ident`s, so it was picked up as a
					// function.
					input.advance_to(&ahead);

					// TODO: clean this up
					return Ok(Self {
						expr: Expr::Verbatim(ident.to_token_stream()),
						arrow_token: None,
						params: idents,
					});
				}
			} else if ahead.peek(Token![=>]) {
				// This means the source was given in the form `a, b, c =>`,
				// which cannot be an expression, and therefore must be a
				// function, so we can safely parse it as such.
				input.advance_to(&ahead);

				return Ok(Self {
					params: idents,
					arrow_token: input.parse()?,
					expr: input.parse()?,
				});
			}
		}

		// If we get to this point then we know that the source has not been
		// picked up as a function, and therefore must be an expression
		// directly.
		Ok(Self {
			params: idents,
			arrow_token: None,
			expr: input.parse()?,
		})
	}
}

impl Parse for Source {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if !input.peek(Ident) {
			// If the next token is not an `Ident`, then this is definitely not
			// a 'function source' - simply parse it as an `Expr`.
			Self {
				params: Punctuated::new(),
				arrow_token: None,
				expr: input.parse()?,
			}
		} else {
			// Fork the `ParseStream` so we can try to parse parameters and a
			// `=>` token, but can revert back to this point if we find out this
			// is actually just an `Expr`.
			let ahead = input.fork();

			let mut params: Punctuated<Ident, Token![,]> = Punctuated::new();
			// While the next token is an `Ident`, continue parsing `Param`s and
			// commas to `params`.
			while ahead.peek(Ident) {
				// Parse a `Param`.
				params.push_value(ahead.parse()?);

				// If the next token is not a comma, then this must be the end
				// of the `params`, so we break. Note that this works if the
				// final `Param` has a comma too: the loop won't start again
				// because `ahead.peek(Ident)` will be `false`.
				if !ahead.peek(Token![,]) {
					break;
				}

				// At this point, we know there is a comma, and so we can push
				// it to `params`.
				params.push_punct(ahead.parse()?);
			}

			if ahead.peek(Token![=>]) {
				// If the next token is `=>`, then this is a function source,
				// not an expression, so we can advance `input` to `ahead`
				// (because we are going to accept `ahead` as the correct
				// parsing branch).
				input.advance_to(&ahead);

				Self {
					// Pass the parameters.
					params,
					// Arrow token: `=>`.
					arrow_token: Some(input.parse()?),
					// The expression.
					expr: input.parse()?,
				}
			} else {
				Self {
					params: Punctuated::new(),
					arrow_token: None,
					expr: input.parse()?,
				}
			}
		})
	}
}

// }}}
