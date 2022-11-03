// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod content;
mod definition;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

use content::*;
use definition::*;

#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
	parse_macro_input!(input as Definitions)
		.into_token_stream()
		.into()
}
