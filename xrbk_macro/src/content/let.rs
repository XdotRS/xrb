// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::{
	parse::{Parse, ParseStream},
	Ident, Token, Type,
};

use super::general::Source;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};

pub struct Let {
	pub let_token: Token![let],

	pub ident: Ident,
	pub colon_token: Token![:],
	pub r#type: Type,

	pub eq_token: Token![=],

	pub source: Source,
}

// Expansion {{{

impl ToTokens for Let {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.let_token.to_tokens(tokens);

		format_ident!("__{}__", self.ident).to_tokens(tokens);
		self.colon_token.to_tokens(tokens);
		self.r#type.to_tokens(tokens);

		self.eq_token.to_tokens(tokens);

		quote! {
			reader.read()?;
		}
		.to_tokens(tokens);
	}
}

impl Let {
	pub fn to_fn_tokens(&self, tokens: &mut TokenStream2) {
		self.source.to_tokens(tokens, self.ident, self.r#type);
	}

	pub fn to_write_tokens(&self, tokens: &mut TokenStream2) {
		let ident = self.ident;
		// e.g. `x: i32, y: i32` is turned into `__x__, __y__`.
		let params = self.source.params.to_call_token_stream();

		quote! {
			writer.write(self.#ident(#params));
		}
		.to_tokens(tokens);
	}
}

// }}}

// Parsing {{{

impl Parse for Let {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(Self {
			let_token: input.parse()?,
			ident: input.parse()?,
			colon_token: input.parse()?,
			r#type: input.parse()?,
			eq_token: input.parse()?,
			source: input.parse()?,
		})
	}
}

// }}}
