// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use syn::{
	parse::{discouraged::Speculative, ParseStream, Result},
	punctuated::Punctuated,
	Error, Expr, Ident, Receiver, Token, Type,
};

pub struct Arg<'a>(Ident, &'a Type);

pub struct Source<'a> {
	pub receiver: Option<Receiver>,
	pub comma_token: Option<Token![,]>,
	/// Arguments to the `Source`, if any.
	pub args: Option<Punctuated<Arg<'a>, Token![,]>>,
	/// An arrow token that denotes this as a function with arguments: `=>`.
	pub arrow_token: Option<Token![=>]>,
	/// The `Source` function's body.
	pub expr: Expr,
}

// Parsing {{{

impl Source<'_> {
	/// Parse a `Source` that can have zero or more [`Arg`s](Arg).
	pub fn parse_with_args(input: ParseStream, map: HashMap<Ident, Type>) -> Result<Self> {
		let fork = &input.fork();

		// Parse a receiver (e.g. `self`, `&self`).
		let receiver: Option<Receiver> = fork.parse().ok();
		let comma_token: Option<Token![,]> = receiver.map(|_| fork.parse().ok()).flatten();

		// If there is EITHER:
		// - no receiver; or
		// - a receiver _and_ a comma following it
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
	pub fn parse_without_receiver(input: ParseStream, map: HashMap<Ident, Type>) -> Result<Self> {
		let source = Self::parse_with_args(input, map)?;

		source.receiver.map_or_else(
			// If there is no receiver, return the source.
			|| Ok(source),
			// But if there is a receiver, generate an error:
			|receiver| Err(Error::new(receiver.self_token.span, "no receiver allowed")),
		)
	}

	/// Parse a `Source` without the option for any [`Arg`s](Arg).
	pub fn parse_receiver(input: ParseStream) -> Result<Self> {
		let fork = input.fork();

		// Parse a receiver (e.g. `self`, `&self`).
		let receiver: Option<Receiver> = fork.parse().ok();
		let comma_token: Option<Token![,]> = receiver.map(|_| fork.parse().ok()).flatten();

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

impl Arg<'_> {
	/// Parses a single `Arg`: an `Ident` that is contained in the `map`.
	pub fn parse(input: ParseStream, map: HashMap<Ident, Type>) -> Result<Self> {
		// Parse an identifier.
		let ident = input.parse()?;

		// Attempt to get the type of the identifier from the map of known
		// identifiers...
		map.get(&ident).map_or_else(
			// If no type was found, generate an error:
			|| Err(Error::new(ident.span(), "unrecognized identifier")),
			// Otherwise, return `Self(ident, r#type)`.
			|r#type| Ok(Self(ident, r#type)),
		)
	}

	/// Parse a [`Punctuated`] (by commas) list of `Arg`s.
	///
	/// See also: [Self::parse]
	pub fn parse_args(
		input: ParseStream,
		map: HashMap<Ident, Type>,
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
