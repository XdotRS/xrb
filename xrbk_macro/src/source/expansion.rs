// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use crate::TsExt;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::punctuated::Pair;

impl ToTokens for LengthArg {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.self_token.to_tokens(tokens);
		self.double_colon_token.to_tokens(tokens);
		self.length_token.to_tokens(tokens);
	}
}

impl ToTokens for Arg {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		let ident = &self.ident;
		let r#type = &self.r#type;

		quote!(#ident: #r#type).to_tokens(tokens);
	}
}

impl ToTokens for Args {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		if let Some((_, r#type)) = &self.length_arg {
			tokens.append_tokens(|| quote!(length: #r#type, ));
		}

		for pair in self.args.pairs() {
			let (arg, comma) = match pair {
				Pair::Punctuated(arg, comma) => (arg, Some(comma)),
				Pair::End(arg) => (arg, None),
			};

			arg.to_tokens(tokens);
			comma.to_tokens(tokens);
		}
	}
}

impl Source {
	pub fn function_to_tokens(&self, tokens: &mut TokenStream2, ident: &Ident, return_type: &Type) {
		let args = self.args.map(|(args, ..)| args);
		let expr = &self.expr;

		tokens.append_tokens(|| {
			quote!(
				fn #ident(#args) -> #return_type {
					#expr
				}
			)
		});
	}
}
