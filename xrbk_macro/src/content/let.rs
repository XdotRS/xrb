// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::{
	parse::{Parse, ParseStream},
	Expr, Ident, Result, Token, Type,
};

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};

pub struct Let {
	/// The let token: `let`.
	pub let_token: Token![let],

	/// The [`Ident`] used to refer to this data in [context attributes].
	///
	/// [context attributes]: super::attributes::Context
	pub ident: Ident,
	/// The colon token preceding the `type`: `:`.
	pub colon_token: Token![:],
	/// The [`Type`] used to `read` this data.
	pub r#type: Type,

	/// The equals token preceding the `expr`: `=`.
	pub eq_token: Token![=],

	/// The [`Expr`] used in the generated function for this `let` item.
	pub expr: Expr,
}

// Expansion {{{

impl ToTokens for Let {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// `let`
		self.let_token.to_tokens(tokens);

		// `product` -> `__product__`
		format_ident!("__{}__", self.ident).to_tokens(tokens);
		// `:`
		self.colon_token.to_tokens(tokens);
		// `Type`
		self.r#type.to_tokens(tokens);

		// `=`
		self.eq_token.to_tokens(tokens);

		// TODO: Allow context for `Let` items?
		quote! {
			reader.read()?;
		}
		.to_tokens(tokens);
	}
}

impl Let {
	pub fn to_fn_tokens(&self, tokens: &mut TokenStream2) {
		let name = self.ident;
		let ty = self.r#type;
		let expr = self.expr;

		quote! {
			fn #name(&self) -> #ty {
				#expr
			}
		}
		.to_tokens(tokens);
	}

	pub fn to_write_tokens(&self, tokens: &mut TokenStream2) {
		let name = self.ident;

		quote! {
			writer.write(self.#name());
		}
		.to_tokens(tokens);
	}
}

// }}}

// Parsing {{{

impl Parse for Let {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			let_token: input.parse()?,

			ident: input.parse()?,
			colon_token: input.parse()?,
			r#type: input.parse()?,

			eq_token: input.parse()?,

			expr: input.parse()?,
		})
	}
}

// }}}
