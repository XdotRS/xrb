// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use crate::TsExt;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use syn::{punctuated::Pair, Attribute};

impl ToTokens for SourceLengthArg {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.self_token.to_tokens(tokens);
		self.double_colon_token.to_tokens(tokens);
		self.length_token.to_tokens(tokens);
	}
}

impl ToTokens for SourceArg {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		let r#type = &self.r#type;

		if let Some((_, pattern)) = &self.pattern {
			quote!(#pattern: &#r#type).to_tokens(tokens);
		} else {
			let ident = &self.ident;

			quote!(#ident: &#r#type).to_tokens(tokens);
		}
	}
}

impl ToTokens for SourceArgs {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		if let Some((_, r#type)) = &self.length_arg {
			tokens.append_tokens(quote!(length: #r#type, ));
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

impl SourceArgs {
	pub fn formatted_tokens(&self, tokens: &mut TokenStream2) {
		if let Some((SourceLengthArg { length_token, .. }, ..)) = &self.length_arg {
			quote_spanned!(length_token.span()=> #length_token,).to_tokens(tokens);
		}

		for pair in self.args.pairs() {
			let (arg, comma) = match pair {
				Pair::Punctuated(arg, comma) => (arg, Some(comma)),
				Pair::End(arg) => (arg, None),
			};

			let formatted = &arg.formatted;

			quote!(&#formatted).to_tokens(tokens);
			comma.to_tokens(tokens);
		}
	}
}

impl Source {
	pub fn function_to_tokens(
		&self, tokens: &mut TokenStream2, attributes: Option<&Vec<Attribute>>, ident: &Ident,
		return_type: TokenStream2,
	) {
		let args = self.args.as_ref().map(|(args, ..)| args);
		let expr = &self.expr;

		if let Some(attributes) = attributes {
			for attribute in attributes {
				attribute.to_tokens(tokens);
			}
		}

		tokens.append_tokens({
			quote_spanned!(ident.span()=>
				fn #ident(#args) -> #return_type {
					#expr
				}
			)
		});
	}

	pub fn call_to_tokens(&self, tokens: &mut TokenStream2, ident: &Ident) {
		let args = TokenStream2::with_tokens(|tokens| {
			if let Some((args, ..)) = self.args.as_ref() {
				args.formatted_tokens(tokens)
			}
		});

		quote_spanned!(ident.span()=> #ident(#args)).to_tokens(tokens);
	}
}
