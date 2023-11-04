// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod xrbk;

use super::*;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};

impl ToTokens for FieldId {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Ident(ident) => ident.to_tokens(tokens),
			Self::Index(index) => index.to_tokens(tokens),
		}
	}
}

impl ToTokens for RegularContent {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.brace_token.surround(tokens, |tokens| {
			self.elements.to_tokens(tokens);
		});
	}
}

impl RegularContent {
	pub fn pat_cons_to_tokens(&self, tokens: &mut TokenStream2) {
		self.brace_token.surround(tokens, |tokens| {
			self.elements.pat_cons_to_tokens(tokens);
		});
	}
}

impl ToTokens for TupleContent {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.paren_token.surround(tokens, |tokens| {
			self.elements.to_tokens(tokens);
		});
	}
}

impl TupleContent {
	pub fn pat_cons_to_tokens(&self, tokens: &mut TokenStream2) {
		self.paren_token.surround(tokens, |tokens| {
			self.elements.pat_cons_to_tokens(tokens);
		});
	}
}

impl ToTokens for Content {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Regular(content) => content.to_tokens(tokens),
			Self::Tuple(content) => content.to_tokens(tokens),

			Self::Unit => {},
		}
	}
}

impl Content {
	pub fn pat_cons_to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Regular(content) => content.pat_cons_to_tokens(tokens),
			Self::Tuple(content) => content.pat_cons_to_tokens(tokens),

			Self::Unit => {},
		}
	}
}

impl ToTokens for StructlikeContent {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Regular {
				where_clause,
				content,
			} => {
				// TODO: where is the place to add generic bounds?
				where_clause.to_tokens(tokens);
				content.to_tokens(tokens);
			},

			Self::Tuple {
				content,
				where_clause,
				semicolon,
			} => {
				content.to_tokens(tokens);
				// TODO: where is the place to add generic bounds?
				where_clause.to_tokens(tokens);
				semicolon.to_tokens(tokens);
			},

			Self::Unit {
				where_clause,
				semicolon,
			} => {
				// TODO: where is the place to add generic bounds?
				where_clause.to_tokens(tokens);
				semicolon.to_tokens(tokens);
			},
		}
	}
}

impl StructlikeContent {
	pub fn pat_cons_to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Regular { content, .. } => content.pat_cons_to_tokens(tokens),
			Self::Tuple { content, .. } => content.pat_cons_to_tokens(tokens),

			Self::Unit { .. } => {},
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
	pub fn pat_cons_to_tokens(&self, tokens: &mut TokenStream2) {
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

		if let FieldId::Ident(ident) = &self.id
			&& let Some(colon) = &self.colon_token
		{
			ident.to_tokens(tokens);
			colon.to_tokens(tokens);
		}

		self.r#type.to_tokens(tokens);
	}
}

impl ToTokens for Let {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		self.let_token.to_tokens(tokens);
		self.ident.to_tokens(tokens);
		quote_spanned!(self.equals_token.span=> ;).to_tokens(tokens);
	}
}

impl ToTokens for SingleUnused {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.attribute.to_tokens(tokens);
		self.underscore_token.to_tokens(tokens);
	}
}

impl ToTokens for ArrayUnused {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		self.bracket_token.surround(tokens, |tokens| {
			self.underscore_token.to_tokens(tokens);
			self.semicolon_token.to_tokens(tokens);
			self.content.to_tokens(tokens);
		})
	}
}

impl ToTokens for UnusedContent {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Infer {
				double_dot_token, ..
			} => double_dot_token.to_tokens(tokens),
			Self::Source(_) => {},
		}
	}
}
