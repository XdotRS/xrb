// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Ident, Result, Token, Type, Visibility};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Field {
	/// Attributes, including doc comments. Ex: `#[error("unsupported feature")]`
	pub attributes: Vec<Attribute>,
	/// Visibility. Ex: `pub`.
	pub vis: Visibility,
	/// `$`. Indicates that this field should be placed in the metabyte.
	pub metabyte_token: Option<Token![$]>,
	/// Name. Ex: `mode`.
	pub name: Ident,
	/// `:`.
	pub colon_token: Token![:],
	/// Type. Ex: `u32`.
	pub ty: Type,
}

// Expansion {{{

impl ToTokens for Field {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Attributes, including doc comments.
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// Visibility. Ex: `pub`.
		self.vis.to_tokens(tokens);
		// Name. Ex: `mode`.
		self.name.to_tokens(tokens);
		// `:`.
		self.colon_token.to_tokens(tokens);
		// Type. Ex: `u32`.
		self.ty.to_tokens(tokens);
	}
}

impl Field {
	#[allow(dead_code)]
	pub fn write_to(&self, tokens: &mut TokenStream2) {
		// Field name.
		let name = &self.name;
		// Field type.
		let ty = &self.ty;

		quote! {
			// Equivalent to:
			// ```
			// writer.write(self.#name)?;
			// ```
			<#ty as cornflakes::ToBytes>::write_to(self.#name, writer)?;
		}
		.to_tokens(tokens);
	}
}

// }}}

// Parsing {{{

impl Parse for Field {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			attributes: input.call(Attribute::parse_outer)?,
			vis: input.parse()?,
			metabyte_token: input.parse().ok(),
			name: input.parse()?,
			colon_token: input.parse()?,
			ty: input.parse()?,
		})
	}
}

// }}}
