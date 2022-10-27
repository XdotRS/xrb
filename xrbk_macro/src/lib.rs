// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod content;
//mod metadata;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, Ident, Type};

use quote::quote;

use content::Source;

#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as Source);

	let mut tokens = proc_macro2::TokenStream::new();
	input.to_tokens(
		&mut tokens,
		Ident::new("test", Span::call_site()),
		Type::Verbatim(quote!(u32)).into(),
	);

	tokens.into()
}

// TODO: Attribute macros are simply allowed to replace (or modify) the `item`
// `TokenStream` they are given. I _think_ they would alaso be expanded _after_
// the function-like macro? Unless there's a way to change that. But in that
// case: what code do you generate? Is there a way to access the content of the
// context macro attribute in code?
//
// Of course, the other approach would be to 'intercept' the context attribute;
// it wouldn't, in actual fact, be an attribute at all. You could parse _either_
// `#[context(...)]` _or_ another attribute when parsing attributes.

//#[proc_macro_attribute]
//pub fn context(attr: TokenStream, item: TokenStream) -> TokenStream {
//	let attr = parse_macro_input!(attr as Source);
//
//	item
//}

macro_rules! ignore {
	($($tt:tt)*) => {};
}

ignore! {
	#[context(fn(comma_token) comma_token)]
	#[derive(Debug)]
	pub x: i32,
}
