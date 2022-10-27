// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use syn::{
	parse::{discouraged::Speculative, ParseStream, Result},
	punctuated::Punctuated,
	Expr, Ident, Receiver, Token, Type,
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
	pub fn parse_with_args(input: ParseStream, map: HashMap<Ident, Type>) -> Result<Self> {
		let fork = input.fork();
		let mut args: Punctuated<Arg, Token![,]> = Punctuated::new();

		let receiver: Option<Receiver> = fork.parse().ok();
		let comma_token: Option<Token![,]> = receiver.map(|_| fork.parse().ok()).flatten();

		if receiver.is_none() || comma_token.is_some() {
			while fork.peek(Ident) {
				let ident = fork.parse()?;

				map.get(&ident)
					.map(|r#type| args.push_value(Arg(ident, r#type)));

				if !fork.peek(Token![,]) {
					break;
				}

				args.push_punct(fork.parse()?);
			}
		}

		Ok(if fork.peek(Token![=>]) {
			input.advance_to(&fork);

			Self {
				receiver,
				comma_token,
				args: Some(args),
				arrow_token: input.parse()?,

				expr: input.parse()?,
			}
		} else {
			Self {
				receiver: None,
				comma_token: None,
				args: None,
				arrow_token: None,

				expr: input.parse()?,
			}
		})
	}

	pub fn parse_receiver(input: ParseStream) -> Result<Self> {
		let fork = input.fork();

		let receiver: Option<Receiver> = fork.parse().ok();
		let comma_token: Option<Token![,]> = receiver.map(|_| fork.parse().ok()).flatten();

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

// }}}
