// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
	parse::{discouraged::Speculative, ParseStream, Result},
	punctuated::Punctuated,
	Error, Expr, Ident, Token, Type,
};

pub type IdentMap<'a> = &'a HashMap<String, Type>;

pub struct Arg(pub Ident, pub Type);
// TODO: iterator?
pub struct Args(pub Punctuated<Arg, Token![,]>);

// TODO: let items need to be able to use any field name (as well as any
//       previously defined let item?), while context attributes need to be
//       able to use any previously defined field name or let item name.
pub struct Source {
	/// Arguments to the `Source`, if any.
	pub args: Option<Args>,
	/// An arrow token that denotes this as a function with arguments: `=>`.
	pub arrow_token: Option<Token![=>]>,
	/// The `Source` function's body.
	pub expr: Expr,
}

impl Arg {
	pub fn format(&self) -> Ident {
		let Self(ident, _) = self;

		format_ident!("__{}__", ident)
	}
}

impl Args {
	pub fn format(&self) -> Vec<Ident> {
		let Self(args) = self;

		args.iter().map(|arg| arg.format()).collect()
	}
}

// Expansion {{{

impl ToTokens for Args {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		let Self(args) = self;

		args.to_tokens(tokens);
	}
}

impl ToTokens for Arg {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		let Self(ident, r#type) = self;

		ident.to_tokens(tokens);
		quote!(:).to_tokens(tokens);
		r#type.to_tokens(tokens);
	}
}

// }}}

// Parsing {{{

impl Source {
	pub fn parse(input: ParseStream, map: IdentMap) -> Result<Self> {
		let fork = &input.fork();
		let args = Args::parse(fork, map);

		Ok(if let Ok(args) = args && fork.peek(Token![=>]) {
			// If we were able to successfully parse the `args` and there is a
			// `=>` token following them, then we know that this `Source` has
			// arguments.

			input.advance_to(fork);

			Self {
				args: Some(args),
				arrow_token: Some(input.parse()?),

				expr: input.parse()?,
			}
		} else {
			Self {
				args: None,
				arrow_token: None,

				expr: input.parse()?,
			}
		})
	}
}

impl Arg {
	/// Parses a single `Arg`.
	pub fn parse(input: ParseStream, map: IdentMap) -> Result<Self> {
		let ident: Ident = input.parse()?;

		if let Some(r#type) = map.get(&ident.to_string()) {
			Ok(Self(ident, r#type.to_owned()))
		} else {
			Err(Error::new(
				ident.span(),
				"unrecognized source argument identifier",
			))
		}
	}
}

impl Args {
	pub fn parse(input: ParseStream, map: IdentMap) -> Result<Self> {
		let mut args = Punctuated::new();

		while input.peek(Ident) {
			args.push_value(Arg::parse(input, map)?);

			if input.peek(Token![,]) {
				args.push_punct(input.parse()?);
			} else {
				break;
			}
		}

		Ok(Self(args))
	}
}

// }}}
