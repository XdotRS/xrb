// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod datasize;
mod readable;
mod r#trait;
mod writable;

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
			Self::Struct(r#struct) => {
				r#struct.to_tokens(tokens);

				r#struct.impl_writable(tokens);
				r#struct.impl_readable(tokens);
				r#struct.impl_datasize(tokens);
			},

			Self::Enum(r#enum) => {
				r#enum.to_tokens(tokens);

				r#enum.impl_writable(tokens);
				r#enum.impl_readable(tokens);
				r#enum.impl_datasize(tokens);
			},

			Self::Request(request) => {
				request.to_tokens(tokens);
				request.impl_trait(tokens);

				request.impl_writable(tokens);
				request.impl_readable(tokens);
				request.impl_datasize(tokens);
			},

			Self::Reply(reply) => {
				reply.to_tokens(tokens);
				reply.impl_trait(tokens);

				reply.impl_writable(tokens);
				reply.impl_readable(tokens);
				reply.impl_datasize(tokens);
			},

			Self::Event(event) => {
				event.to_tokens(tokens);
				event.impl_trait(tokens);

				event.impl_writable(tokens);
				event.impl_readable(tokens);
				event.impl_datasize(tokens);
			},

			Self::Other(item) => item.to_tokens(tokens),
		}
	}
}

macro_rules! structlike_to_tokens {
	($Struct:ty) => {
		impl ToTokens for $Struct {
			fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
				for attribute in &self.attributes {
					attribute.to_tokens(tokens);
				}

				self.visibility.to_tokens(tokens);
				self.struct_token.to_tokens(tokens);
				self.ident.to_tokens(tokens);
				self.generics.to_tokens(tokens);
				self.content.to_tokens(tokens);
			}
		}
	};
}

structlike_to_tokens!(Struct);
structlike_to_tokens!(Request);
structlike_to_tokens!(Reply);
structlike_to_tokens!(Event);

impl ToTokens for Enum {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		self.visibility.to_tokens(tokens);
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
		self.content.to_tokens(tokens);

		if let Some((equals, expr)) = self.discriminant.as_ref() {
			equals.to_tokens(tokens);
			expr.to_tokens(tokens);
		}
	}
}
