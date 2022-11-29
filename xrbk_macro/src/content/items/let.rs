// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, Token, Type};

use crate::{Attribute, ItemDeserializeTokens, ItemId, ItemSerializeTokens, TsExt};

use super::Source;

pub struct Let {
	/// An optional metabyte attribute associated with the `Let` item.
	pub attribute: Option<Attribute>,

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

// Expansion {{{

impl ToTokens for Let {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// `let`
		self.let_token.to_tokens(tokens);

		// `product` -> `__product__`
		format_ident!("__{}__", self.ident).to_tokens(tokens);
		// `:`
		self.colon_token.to_tokens(tokens);
		// `Type`
		self.r#type.to_tokens(tokens);

		// `=`
		self.eq_token.to_tokens(tokens);

		quote! (
			reader.read()?;
		)
		.to_tokens(tokens);
	}
}

// }}}

// Implementations {{{

impl ItemSerializeTokens for Let {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		let name = id.formatted();
		let r#type = &self.r#type;

		let args = TokenStream2::with_tokens(|tokens| {
			self.source.args_to_tokens(tokens);
		});
		let formatted_args = TokenStream2::with_tokens(|tokens| {
			self.source.formatted_args_to_tokens(tokens);
		});

		let expr = &self.source.expr;

		quote!(
			fn #name(#args) -> #r#type {
				#expr
			}

			<#r#type as cornflakes::Writable>::write_to(
				&#name(#formatted_args),
				writer,
			)?;
		)
		.to_tokens(tokens);
	}
}

impl ItemDeserializeTokens for Let {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		let name = id.formatted();
		let r#type = &self.r#type;

		tokens.append_tokens(
			|| quote!(let #name = <#r#type as cornflakes::Readable>::read_from(reader)?;),
		);
	}
}

// }}}
