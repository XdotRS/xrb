// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream;
use quote::ToTokens;

use super::*;

impl ToTokens for Elements {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Self::Struct {
				brace_token,
				elements,
			} => {
				brace_token.surround(tokens, |tokens| {
					elements.to_tokens(tokens);
				});
			},

			Self::Tuple {
				paren_token,
				elements,
			} => {
				paren_token.surround(tokens, |tokens| {
					elements.to_tokens(tokens);
				});
			},

			Self::Unit => {},
		}
	}
}

impl ToTokens for Element {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		if let Element::Field(field) = self {
			field.to_tokens(tokens);
		}
	}
}

impl ToTokens for Field {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		self.visibility.to_tokens(tokens);

		self.ident.map(|(ident, colon)| {
			ident.to_tokens(tokens);
			colon.to_tokens(tokens);
		});

		self.r#type.to_tokens(tokens);
	}
}
