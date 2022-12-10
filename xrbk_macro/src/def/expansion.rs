// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use proc_macro2::TokenStream;
use quote::ToTokens;

impl ToTokens for Definitions {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let Self(definitions) = self;

		for definition in definitions {
			definition.to_tokens(tokens);
		}
	}
}

impl ToTokens for Definition {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Self::Structlike(metadata, items, semicolon) => {
				metadata.to_tokens(tokens);
				items.to_tokens(tokens);
				semicolon.to_tokens(tokens);
			}

			Self::Enum(r#enum) => r#enum.to_tokens(tokens),

			Self::Other(item) => item.to_tokens(tokens),
		}
	}
}

impl ToTokens for Metadata {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let (attributes, vis, struct_token, ident, generics) = match self {
			Self::Struct(r#struct) => (
				r#struct.attributes,
				r#struct.vis,
				r#struct.struct_token,
				r#struct.ident,
				r#struct.generics,
			),

			Self::Request(request) => (
				request.attributes,
				request.vis,
				request.struct_token,
				request.ident,
				request.generics,
			),
			Self::Reply(reply) => (
				reply.attributes,
				reply.vis,
				reply.struct_token,
				reply.ident,
				reply.generics,
			),
			Self::Event(event) => (
				event.attributes,
				event.vis,
				event.struct_token,
				event.ident,
				event.generics,
			),
		};

		for attribute in attributes {
			attribute.to_tokens(tokens);
		}

		vis.to_tokens(tokens);
		struct_token.to_tokens(tokens);
		ident.to_tokens(tokens);
		generics.to_tokens(tokens);
	}
}

impl ToTokens for Enum {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		self.vis.to_tokens(tokens);
		self.enum_token.to_tokens(tokens);
		self.ident.to_tokens(tokens);
		self.generics.to_tokens(tokens);

		self.brace_token.surround(tokens, |tokens| {
			self.variants.to_tokens(tokens);
		});
	}
}

impl ToTokens for Variant {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		self.ident.to_tokens(tokens);
		self.items.to_tokens(tokens);
		self.discriminant.map(|(equals, expr)| {
			equals.to_tokens(tokens);
			expr.to_tokens(tokens);
		});
	}
}
