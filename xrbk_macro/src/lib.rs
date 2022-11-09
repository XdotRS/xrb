// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(anonymous_lifetime_in_impl_trait)]

mod content;
mod definition;
mod impls;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::parse_macro_input;

use content::*;
use definition::*;

#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as Definitions);

	let mut expanded = TokenStream2::new();

	input.to_tokens(&mut expanded);
	//input.impl_tokens(&mut expanded);

	expanded.into()
}
