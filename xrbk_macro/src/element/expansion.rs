// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod rw;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::punctuated::Pair;

use super::*;

impl ToTokens for FieldId<'_> {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			FieldId::Ident { ident, .. } => ident.to_tokens(tokens),
			FieldId::Index { index, .. } => index.to_tokens(tokens),
		}
	}
}

impl ToTokens for LetId<'_> {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.ident.to_tokens(tokens)
	}
}

impl ToTokens for UnusedId {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.index.to_tokens(tokens)
	}
}

impl ToTokens for Content<'_> {
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

impl Content<'_> {
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

impl ToTokens for Elements<'_> {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.elements.to_tokens(tokens);
	}
}

impl Elements<'_> {
	pub fn fields_to_tokens(&self, tokens: &mut TokenStream2) {
		for pair in self.fields.pairs() {
			let (field, comma) = match pair {
				Pair::Punctuated(field, comma) => (field, Some(comma)),
				Pair::End(field) => (field, None),
			};

			match &field.id {
				FieldId::Ident { ident, formatted } => {
					quote!(#ident: #formatted).to_tokens(tokens);
					comma.to_tokens(tokens);
				},

				FieldId::Index { formatted, .. } => {
					formatted.to_tokens(tokens);
					comma.to_tokens(tokens);
				},
			}
		}
	}
}

impl ToTokens for Element<'_> {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		if let Element::Field(field) = self {
			field.to_tokens(tokens);
		}
	}
}

impl ToTokens for Field<'_> {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
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
