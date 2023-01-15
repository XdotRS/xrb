// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod readable;
mod r#trait;
mod writable;
mod x11_size;

use super::*;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::spanned::Spanned;

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

				let attrs = &r#struct.item_attributes;

				for path in &attrs.derive_writables {
					r#struct.impl_writable(tokens, path);
				}

				for path in &attrs.derive_readables {
					r#struct.impl_readable(tokens, path);
				}

				for path in &attrs.derive_x11_sizes {
					r#struct.impl_x11_size(tokens, path);
				}
			},

			Self::Enum(r#enum) => {
				r#enum.to_tokens(tokens);

				let attrs = &r#enum.item_attributes;

				for path in &attrs.derive_writables {
					r#enum.impl_writable(tokens, path);
				}

				for path in &attrs.derive_readables {
					r#enum.impl_readable(tokens, path);
				}

				for path in &attrs.derive_x11_sizes {
					r#enum.impl_x11_size(tokens, path);
				}
			},

			Self::Request(request) => {
				request.to_tokens(tokens);
				request.impl_trait(tokens);

				let attrs = &request.item_attributes;

				for path in &attrs.derive_writables {
					request.impl_writable(tokens, path);
				}

				for path in &attrs.derive_readables {
					request.impl_readable(tokens, path);
				}

				for path in &attrs.derive_x11_sizes {
					request.impl_x11_size(tokens, path);
				}
			},

			Self::Reply(reply) => {
				reply.to_tokens(tokens);
				reply.impl_trait(tokens);

				let attrs = &reply.item_attributes;

				for path in &attrs.derive_writables {
					reply.impl_writable(tokens, path);
				}

				for path in &attrs.derive_readables {
					reply.impl_readable(tokens, path);
				}

				for path in &attrs.derive_x11_sizes {
					reply.impl_x11_size(tokens, path);
				}
			},

			Self::Event(event) => {
				event.to_tokens(tokens);
				event.impl_trait(tokens);

				let attrs = &event.item_attributes;

				for path in &attrs.derive_writables {
					event.impl_writable(tokens, path);
				}

				for path in &attrs.derive_readables {
					event.impl_readable(tokens, path);
				}

				for path in &attrs.derive_x11_sizes {
					event.impl_x11_size(tokens, path);
				}
			},

			Self::Error(error) => {
				error.to_tokens(tokens);
				error.impl_trait(tokens);

				let attrs = &error.item_attributes;

				for _path in &attrs.derive_writables {
					// TODO: error.impl_writable(tokens, path);
				}

				for path in &attrs.derive_readables {
					error.impl_readable(tokens, path);
				}

				for _path in &attrs.derive_x11_sizes {
					// TODO: error.impl_x11_size(tokens, path);
				}
			},

			Self::Other(item) => item.to_tokens(tokens),
		}
	}
}

macro_rules! structlike_to_tokens {
	($Struct:ty) => {
		impl ToTokens for $Struct {
			fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
				for attribute in &self.item_attributes.attributes {
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
structlike_to_tokens!(Error);

impl ToTokens for Enum {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		for attribute in &self.item_attributes.attributes {
			attribute.to_tokens(tokens);
		}

		self.visibility.to_tokens(tokens);
		self.enum_token.to_tokens(tokens);
		self.ident.to_tokens(tokens);
		self.generics.to_tokens(tokens);
		self.where_clause.to_tokens(tokens);

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
