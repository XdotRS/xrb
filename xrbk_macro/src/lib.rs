// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(let_chains)]

mod attribute;
mod definition;
mod element;
mod ext;
mod source;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

pub(crate) use definition::*;
pub(crate) use ext::*;
pub(crate) use source::*;

#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
	let definitions = parse_macro_input!(input as Definitions);

	let expanded = definitions.into_token_stream();

	expanded.into()
}
