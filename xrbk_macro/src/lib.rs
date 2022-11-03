// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod content;
mod metadata;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, parse::{Parse, ParseStream, Result}};

use content::*;
use metadata::*;

struct Definitions(Vec<StructDefinition>);

impl Parse for Definitions {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut definitions = vec![];

		while !input.is_empty() {
			definitions.push(input.parse()?);
		}

		Ok(Self(definitions))
	}
}

impl ToTokens for Definitions {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		for definition in &self.0 {
			definition.to_tokens(tokens);
		}
	}
}

#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
	parse_macro_input!(input as Definitions).into_token_stream().into()
}
