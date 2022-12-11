// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Ident, Token, Type, Visibility};

use crate::{
	AttrContent,
	Attribute,
	Context,
	ItemDeserializeTokens,
	ItemId,
	ItemSerializeTokens,
	TsExt,
};

pub struct Field {
	pub attributes: Vec<Attribute>,
	pub vis: Visibility,
	pub ident: Option<Ident>,
	pub colon_token: Option<Token![:]>,
	pub r#type: Type,
}

impl Field {
	/// Returns whether this field has a name.
	pub const fn is_named(&self) -> bool {
		self.ident.is_some() && self.colon_token.is_some()
	}

	/// Returns whether this field does not have a name.
	pub const fn is_unnamed(&self) -> bool {
		self.ident.is_none() && self.colon_token.is_none()
	}

	/// Returns whether this field has a context attribute.
	pub fn has_context(&self) -> bool {
		self.attributes.iter().any(|attr| attr.is_context())
	}

	/// Returns whether this field has a metabyte attribute.
	pub fn is_metabyte(&self) -> bool {
		self.attributes
			.iter()
			.any(|attribute| attribute.is_metabyte())
	}

	/// Gets the context of this field if it has a context attribute.
	#[allow(clippy::borrowed_box)]
	pub fn context(&self) -> Option<&Box<Context>> {
		self.attributes.iter().find_map(|attr| match &attr.content {
			AttrContent::Context(_, context) => Some(context),
			_ => None,
		})
	}
}

// Expansion {{{

impl ToTokens for Field {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Convert every attribute (other than context attributes) on this field
		// to tokens.
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// Convert the field's visibility to tokens.
		self.vis.to_tokens(tokens);
		// Convert the field's name to tokens.
		self.ident.to_tokens(tokens);
		// Convert the colon token between the field's name and its type to
		// tokens.
		self.colon_token.to_tokens(tokens);
		// Convert the field's type to tokens.
		self.r#type.to_tokens(tokens);
	}
}

// }}}

// Implementations {{{

impl ItemSerializeTokens for Field {
	// Tokens to serialize a field.
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		let name = id.formatted();
		let r#type = &self.r#type;

		tokens
			.append_tokens(|| quote!(<#r#type as cornflakes::Writable>::write_to(#name, writer)?;));
	}
}

impl ItemDeserializeTokens for Field {
	// Tokens to deserialize a field.
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		let name = id.formatted();
		let r#type = &self.r#type;

		tokens.append_tokens(|| {
			// If this is a contextual field, that context must be provided.
			if let Some(context) = self.context() {
				let args = TokenStream2::with_tokens(|tokens| {
					context.source().args_to_tokens(tokens);
				});
				let formatted_args = TokenStream2::with_tokens(|tokens| {
					context.source().formatted_args_to_tokens(tokens);
				});

				let expr = &context.source().expr;

				quote!(
					fn #name(#args) -> <#r#type as cornflakes::ContextualReadable>::Context {
						#expr
					}

					let #name = <#r#type as cornflakes::ContextualReadable>::read_with(
						reader,
						&#name(#formatted_args),
					)?;
				)
			} else {
				quote!(
					let #name = <#r#type as cornflakes::Readable>::read_from(reader)?;
				)
			}
		});
	}
}

// }}}
