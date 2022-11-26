// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::TsExt;

use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
	parse::{discouraged::Speculative, ParseStream, Result},
	punctuated::Punctuated,
	Error, Expr, Ident, Receiver, Token, Type,
};

pub type IdentMap<'a> = &'a HashMap<String, Type>;
type ArgList = Punctuated<Arg, Token![,]>;

pub enum Arg {
	Receiver(Receiver),
	Ident(Ident, Box<Type>),
}

pub struct Source {
	/// Arguments to the `Source`, if any.
	pub args: Option<ArgList>,
	/// An arrow token that denotes this as a function with arguments: `=>`.
	pub arrow_token: Option<Token![=>]>,
	/// The `Source` function's body.
	pub expr: Expr,
}

impl Source {
	pub fn fmt_args(&self) -> Vec<Ident> {
		self.args
			.iter()
			.flatten()
			.filter_map(|arg| {
				if let Arg::Ident(ident, _) = arg {
					Some(format_ident!("__{}__", ident))
				} else {
					None
				}
			})
			.collect()
	}
}

// Expansion {{{

impl Source {
	pub fn fn_to_tokens(&self, tokens: &mut TokenStream2, ident: &Ident, r#type: &Type) {
		let args: &Vec<_> = &self.args.iter().flatten().collect();
		let expr = &self.expr;

		tokens.append_tokens(|| {
			quote!(
				fn #ident(#(#args)*) -> #r#type {
					#expr
				}
			)
		});
	}
}

impl ToTokens for Arg {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Receiver(receiver) => {
				receiver.to_tokens(tokens);
			}

			Self::Ident(ident, r#type) => {
				ident.to_tokens(tokens);
				quote!(:).to_tokens(tokens);
				r#type.to_tokens(tokens);
			}
		}
	}
}

// }}}

// Parsing {{{

impl Source {
	fn parse_with_args(
		args: Result<ArgList>,
		input: ParseStream,
		fork: ParseStream,
	) -> Result<Self> {
		Ok(if let Ok(args) = args && fork.peek(Token![=>]) {
			// If we were able to successfully parse the `args` and there is a
			// `=>` token following them, then we know that this `Source` has
			// arguments.

			input.advance_to(fork);

			Self {
				args: Some(args),
				arrow_token: input.parse()?,

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

	/// Parses a `Source` with zero or one [`Arg::Receiver`]s and zero or more
	/// [`Arg::Ident`]s.
	pub fn parse(input: ParseStream, map: IdentMap) -> Result<Self> {
		let fork = &input.fork();

		Self::parse_with_args(Arg::parse_args(fork, map), input, fork)
	}

	/// Parses a `Source` with zero or more [`Arg::Ident`]s.
	pub fn parse_with_idents(input: ParseStream, map: IdentMap) -> Result<Self> {
		let fork = &input.fork();

		Self::parse_with_args(Arg::parse_idents(fork, map), input, fork)
	}

	/// Parses a `Source` with zero or one [`Arg::Receiver`]s.
	pub fn parse_with_receivers(input: ParseStream) -> Result<Self> {
		let fork = &input.fork();

		Self::parse_with_args(Arg::parse_receivers(fork), input, fork)
	}
}

impl Arg {
	/// Parses a single `Arg` (either a [`Receiver`] or an [`Ident`]).
	///
	/// [`Receiver`]: Self::Receiver
	/// [`Ident`]: Self::Ident
	pub fn parse(input: ParseStream, map: IdentMap) -> Result<Self> {
		let fork = &input.fork();

		if let Ok(receiver) = fork.parse() {
			input.advance_to(fork);

			Ok(Self::Receiver(receiver))
		} else {
			Self::parse_ident(input, map)
		}
	}

	/// Parses a single [`Arg::Receiver`].
	pub fn parse_receiver(input: ParseStream) -> Result<Self> {
		Ok(Self::Receiver(input.parse()?))
	}

	/// Parses a single [`Arg::Ident`].
	pub fn parse_ident(input: ParseStream, map: IdentMap) -> Result<Self> {
		let ident: Ident = input.parse()?;

		if let Some(r#type) = map.get(&ident.to_string()) {
			Ok(Self::Ident(ident, Box::new(r#type.to_owned())))
		} else {
			Err(Error::new(
				ident.span(),
				"unrecognized source argument identifier",
			))
		}
	}

	fn parse_idents_to(args: &mut ArgList, input: ParseStream, map: IdentMap) -> Result<()> {
		while input.peek(Ident) {
			args.push_value(Self::parse_ident(input, map)?);

			if input.peek(Token![,]) {
				args.push_punct(input.parse()?);
			} else {
				break;
			}
		}

		Ok(())
	}

	/// Parses a [`Punctuated`] list of `Arg`s.
	///
	/// Begins with zero or one [`Arg::Receiver`]s, followed by zero or more
	/// [`Arg::Ident`]s.
	pub fn parse_args(input: ParseStream, map: IdentMap) -> Result<ArgList> {
		let mut args = Punctuated::new();

		let fork = input.fork();
		if let Ok(receiver) = fork.parse::<Receiver>() {
			input.advance_to(&fork);

			args.push_value(Self::Receiver(receiver));

			if input.peek(Token![,]) {
				args.push_punct(input.parse()?);
			} else {
				return Ok(args);
			}
		}

		Self::parse_idents_to(&mut args, input, map)?;

		Ok(args)
	}

	/// Parses a [`Punctuated`] list of [`Arg::Ident`]s.
	pub fn parse_idents(input: ParseStream, map: IdentMap) -> Result<ArgList> {
		let mut args = Punctuated::new();

		Self::parse_idents_to(&mut args, input, map)?;

		Ok(args)
	}

	/// Parses a [`Punctuated`] list with one [`Arg::Receiver`] and zero or one
	/// trailing commas.
	pub fn parse_receivers(input: ParseStream) -> Result<ArgList> {
		let mut args = Punctuated::new();

		args.push_value(Self::parse_receiver(input)?);

		if input.peek(Token![,]) {
			args.push_punct(input.parse()?);
		}

		Ok(args)
	}
}

// }}}
