// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result, Token, Type};

/// `#fieldname` or `#$fieldname` or `#fieldname[2]`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FieldLength {
	/// `$`. Indicates that this data should be placed in the metabyte.
	pub metabyte_token: Option<Token![$]>,
	/// `#`.
	pub number_sign_token: Token![#],
	/// The name of the field which this reads or writes the length of.
	pub field_name: Ident,
	pub colon_token: Token![:],
	pub ty: Type,
}

// Expansion {{{

impl ToTokens for FieldLength {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Field name.
		let name = &self.field_name;
		let ty = &self.ty;

		quote! {
			// Equivalent to:
			// ```
			// writer.write(self.#name.len() as #ty)?;
			// ```
			cornflakes::ToBytes::write_to(self.#name.len() as #ty, writer)?;
		}
		.to_tokens(tokens);
	}
}

// }}}

// Parsing {{{

impl Parse for FieldLength {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// Optional: `$`.
			metabyte_token: input.parse().ok(),
			// `#`.
			number_sign_token: input.parse()?,
			// Name of the field which `self` represents the length of.
			field_name: input.parse()?,
			// `:`.
			colon_token: input.parse()?,
			// The numerical type to use to represent this length.
			ty: input.parse()?,
		})
	}
}

// }}}
