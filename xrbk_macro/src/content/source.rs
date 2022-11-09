// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
	parse::{discouraged::Speculative, ParseStream, Result},
	punctuated::Punctuated,
	Error, Expr, Ident, Receiver, Token, Type,
};

pub struct Arg(pub Ident, pub Type);

pub struct Source {
	pub receiver: Option<Receiver>,
	pub comma_token: Option<Token![,]>,
	/// Arguments to the `Source`, if any.
	pub args: Option<Punctuated<Arg, Token![,]>>,
	/// An arrow token that denotes this as a function with arguments: `=>`.
	pub arrow_token: Option<Token![=>]>,
	/// The `Source` function's body.
	pub expr: Expr,
}

// Expansion {{{

impl Source {
	// TODO
	pub fn fn_to_tokens(&self, tokens: &mut TokenStream2, ident: &Ident, r#type: &Type) {
		let receiver = &self.receiver;
		let comma = &self.comma_token;
		let arg: &Vec<_> = &self.args.iter().flatten().collect();
		let expr = &self.expr;

		quote!(
			fn #ident(#receiver #comma #(#arg)*) -> #r#type {
				#expr
			}
		).to_tokens(tokens)
	}
}

impl ToTokens for Arg {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.0.to_tokens(tokens);
		quote!(:).to_tokens(tokens);
		self.1.to_tokens(tokens);
	}
}

// }}}

// Parsing {{{

impl Source {
	/// Parse a `Source` that can have zero or more [`Arg`s](Arg) and a receiver.
	pub fn parse(input: ParseStream, map: &HashMap<Ident, Type>) -> Result<Self> {
		let fork = &input.fork();

		// Parse a receiver (e.g. `self`, `&self`).
		let receiver: Option<Receiver> = fork.parse().ok();
		let comma_token: Option<Token![,]> = receiver.as_ref().and_then(|_| fork.parse().ok());

		// If there is EITHER:
		// - no receiver; or
		// - a receiver _and_ a comma following it,
		// parse additional `Arg`s.
		let args = if receiver.is_none() || comma_token.is_some() {
			Some(Arg::parse_args(fork, map)?)
		} else {
			None
		};

		// If the next token is `=>`, then this is actually a `Source` with
		// parameters, so we can advance `input` to the position of our `fork`
		// and construct `Self` with the parsed parameters.
		Ok(if fork.peek(Token![=>]) {
			input.advance_to(fork);

			Self {
				receiver,
				comma_token,
				args,
				arrow_token: input.parse()?,

				expr: input.parse()?,
			}
		} else {
			// Otherwise, we simply parse a single expression.
			Self {
				receiver: None,
				comma_token: None,
				args: None,
				arrow_token: None,

				expr: input.parse()?,
			}
		})
	}

	/// Parse a `Source` that can have zero or more [`Arg`s](Arg) but no receiver.
	pub fn parse_without_receiver(input: ParseStream, map: &HashMap<Ident, Type>) -> Result<Self> {
		let source = Self::parse(input, map)?;

		if let Some(receiver) = source.receiver {
			// If there is a receiver, generate an error:
			Err(Error::new(receiver.self_token.span, "no receiver allowed"))
		} else {
			// If there is no receiver, return the source.
			Ok(source)
		}
	}

	/// Parse a `Source` without the option for any [`Arg`s](Arg).
	pub fn parse_without_args(input: ParseStream) -> Result<Self> {
		let fork = input.fork();

		// Parse a receiver (e.g. `self`, `&self`).
		let receiver: Option<Receiver> = fork.parse().ok();
		let comma_token: Option<Token![,]> = receiver.as_ref().and_then(|_| fork.parse().ok());

		// If the next token is `=>`, then this is actually a `Source` with
		// a receiver, so we can advance `input` to the position of our `fork`
		// and construct `Self` with the parsed receiver.
		Ok(if fork.peek(Token![=>]) {
			input.advance_to(&fork);

			Self {
				receiver,
				comma_token,
				args: None,
				arrow_token: input.parse()?,

				expr: input.parse()?,
			}
		} else {
			// Otherwise, we simply parse a single expression.
			Self {
				receiver: None,
				comma_token: None,
				args: None,
				arrow_token: None,

				expr: input.parse()?,
			}
		})
	}
}

impl Arg {
	/// Parses a single `Arg`: an `Ident` that is contained in the `map`.
	pub fn parse(input: ParseStream, map: &HashMap<Ident, Type>) -> Result<Self> {
		// Parse an identifier.
		let ident = input.parse()?;

		// Attempt to get the type of the identifier from the map of known
		// identifiers...
		if let Some(r#type) = map.get(&ident) {
			Ok(Self(ident, r#type.to_owned()))
		} else {
			Err(Error::new(ident.span(), "unrecognized identifier"))
		}
	}

	/// Parse a [`Punctuated`] (by commas) list of `Arg`s.
	///
	/// See also: [Self::parse]
	pub fn parse_args(
		input: ParseStream,
		map: &HashMap<Ident, Type>,
	) -> Result<Punctuated<Self, Token![,]>> {
		let mut args = Punctuated::new();

		// While the next token is an `Ident`, there are still `Arg`s to parse:
		while input.peek(Ident) {
			// We know the next token is an `Arg`, so parse it and add it to the
			// list.
			args.push_value(Self::parse(input, map)?);

			// If the token following that `Arg` is not a comma, then we have
			// reached the end of the list.
			if !input.peek(Token![,]) {
				break;
			} else {
				// Otherwise, we parse and push a comma to the list.
				args.push_punct(input.parse()?);
			}
		}

		Ok(args)
	}
}

// }}}
