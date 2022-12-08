// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Ident, Token, Type};

use crate::content::{AttrContent, Context};
use crate::{Attribute, ItemDeserializeTokens, ItemId, ItemSerializeTokens, TsExt};

use super::Source;

pub struct Let {
	/// Attributes associated with the let item and its [`Source`]'s function,
	/// if any.
	pub attributes: Vec<Attribute>,

	/// The let token: `let`.
	pub let_token: Token![let],

	/// The [`Ident`] used to refer to this data in [context attributes].
	///
	/// [context attributes]: super::attributes::Context
	pub ident: Ident,
	/// The colon token preceding the `type`: `:`.
	pub colon_token: Token![:],
	/// The [`Type`] used to `read` this data.
	pub r#type: Type,

	/// The equals token preceding the `expr`: `=`.
	pub eq_token: Token![=],

	/// The [`Source`] used in the generated function for this `Let` item.
	pub source: Source,
}

impl Let {
	/// Returns whether this let item has a context attribute.
	pub fn has_context(&self) -> bool {
		self.attributes
			.iter()
			.any(|attribute| attribute.is_context())
	}

	/// Returns whether this let item has a metabyte attribute.
	pub fn is_metabyte(&self) -> bool {
		self.attributes
			.iter()
			.any(|attribute| attribute.is_metabyte())
	}

	/// Gets the context of this let item if it has a context attribute.
	#[allow(clippy::borrowed_box)]
	pub fn context(&self) -> Option<&Box<Context>> {
		self.attributes
			.iter()
			.find_map(|attribute| match &attribute.content {
				AttrContent::Context(_, context) => Some(context),
				_ => None,
			})
	}
}

// Implementations {{{

impl ItemSerializeTokens for Let {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		let name = id.formatted();
		let r#type = &self.r#type;

		for attr in &self.attributes {
			if matches!(attr.content, AttrContent::Other(..)) {
				attr.to_tokens(tokens);
			}
		}

		let args = TokenStream2::with_tokens(|tokens| {
			self.source.args_to_tokens(tokens);
		});
		let formatted_args = TokenStream2::with_tokens(|tokens| {
			self.source.formatted_args_to_tokens(tokens);
		});

		let expr = &self.source.expr;

		tokens.append_tokens(|| {
			quote!(
				fn #name(#args) -> #r#type {
					#expr
				}
				let #name = #name(#formatted_args);

				<#r#type as cornflakes::Writable>::write_to(&#name, writer)?;
			)
		});
	}
}

impl ItemDeserializeTokens for Let {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		let name = id.formatted();
		let r#type = &self.r#type;

		tokens.append_tokens(|| {
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

					let #name = <r#type as cornflakes::ContextualReadable>::read_with(
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
