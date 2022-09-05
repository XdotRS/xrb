// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod content;
mod message;

use proc_macro::TokenStream;

use quote::quote;

use syn::{parse_macro_input, Result};
use syn::parse::{Parse, ParseStream};

use crate::message::Message;
use crate::content::Enum;

struct Messages {
	pub messages: Vec<Message>,
}

impl Parse for Messages {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut messages: Vec<Message> = vec![];

		while !input.is_empty() {
			messages.push(input.parse()?);
		}

		Ok(Self { messages })
	}
}

#[proc_macro]
pub fn requests(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as Messages);

	let messages = input.messages;
	let enum_defs: Vec<Enum> = messages
		.iter()
		.flat_map(|message| message.content.enum_definitions())
		.collect();

	let expanded = quote! {
		#(#messages)*
		#(#enum_defs)*
	};

	expanded.into()
}
