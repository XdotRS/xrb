// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
	parse::{Parse, ParseStream},
	Ident, Result, Token, Type,
};

use crate::{Attribute, TsExt};

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

impl Let {
	pub fn to_fn_tokens(&self, tokens: &mut TokenStream2) {
		self.source
			.fn_to_tokens(tokens, &format_ident!("__{}__", self.ident), &self.r#type);
	}

	pub fn to_write_tokens(&self, tokens: &mut TokenStream2) {
		let name = format_ident!("__{}__", self.ident);

		tokens.append_tokens(|| {
			quote!(
				writer.write(self.#name());
			)
		});
	}
}

// }}}

// Parsing {{{

impl Parse for Let {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			attribute: {
				if input.peek(Token![#]) {
					Some(Attribute::parse_metabyte(input)?)
				} else {
					None
				}
			},

			let_token: input.parse()?,

			ident: input.parse()?,
			colon_token: input.parse()?,
			r#type: input.parse()?,

			eq_token: input.parse()?,

			source: Source::parse_without_args(input)?,
		})
	}
}

// }}}
