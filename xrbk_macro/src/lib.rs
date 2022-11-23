// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(anonymous_lifetime_in_impl_trait)]

mod content;
mod definition;
mod impls;
mod ts_ext;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::parse_macro_input;

pub(crate) use content::*;
pub(crate) use definition::*;
pub(crate) use impls::*;
pub(crate) use ts_ext::*;

#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
	let definitions = parse_macro_input!(input as Definitions);

	let expanded = TokenStream2::with_tokens(|tokens| {
		definitions.to_tokens(tokens);
		definitions.impl_tokens(tokens);
	});

	expanded.into()
}
