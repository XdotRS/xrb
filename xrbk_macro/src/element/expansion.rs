// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod cornflakes;

use super::*;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

impl ToTokens for FieldId {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Ident(ident) => ident.to_tokens(tokens),
			Self::Index(index) => index.to_tokens(tokens),
		}
	}
}

impl ToTokens for Content {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
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

impl Content {
	pub fn fields_to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Struct {
				brace_token,
				elements,
			} => {
				brace_token.surround(tokens, |tokens| {
					elements.fields_to_tokens(tokens);
				});
			},

			Self::Tuple {
				paren_token,
				elements,
			} => {
				paren_token.surround(tokens, |tokens| {
					elements.fields_to_tokens(tokens);
				});
			},

			Self::Unit => {},
		}
	}
}

impl ToTokens for Elements {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		for (element, comma) in self.pairs() {
			if let Element::Field(field) = element {
				field.to_tokens(tokens);
				comma.to_tokens(tokens);
			}
		}
	}
}

impl Elements {
	pub fn fields_to_tokens(&self, tokens: &mut TokenStream2) {
		for (element, comma) in self.pairs() {
			if let Element::Field(field) = element {
				let formatted = &field.formatted;

				match &field.id {
					FieldId::Ident(ident) => quote!(#ident: #formatted).to_tokens(tokens),
					FieldId::Index(_) => formatted.to_tokens(tokens),
				}

				comma.to_tokens(tokens);
			}
		}
	}
}

impl ToTokens for Element {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		if let Element::Field(field) = self {
			field.to_tokens(tokens);
		}
	}
}

impl ToTokens for Field {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		self.visibility.to_tokens(tokens);

		if let FieldId::Ident(ident) = &self.id && let Some(colon) = &self.colon_token {
			ident.to_tokens(tokens);
			colon.to_tokens(tokens);
		}

		self.r#type.to_tokens(tokens);
	}
}
