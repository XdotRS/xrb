// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use crate::content::ParseWithContext;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
	parse::{discouraged::Speculative, ParseStream, Result},
	punctuated::Punctuated,
	Error,
	Expr,
	Ident,
	Token,
	Type,
};

use crate::TsExt;

pub type IdentMap<'a> = &'a HashMap<String, Type>;

pub struct Arg(
	pub Option<(Token![self], Token![::])>,
	pub Ident,
	pub Option<Type>,
);

pub struct Args(pub Punctuated<Arg, Token![,]>);

pub struct Source {
	/// Arguments to the `Source`, if any.
	pub args: Option<Args>,
	/// An arrow token that denotes this as a function with arguments: `=>`.
	pub arrow_token: Option<Token![=>]>,
	/// The `Source` function's body.
	pub expr: Expr,
}

#[derive(Copy, Clone)]
pub enum LengthMode {
	Disallowed,
	Request,
	Reply,
}

impl Arg {
	pub fn format(&self) -> Ident {
		let Self(length_syntax, ident, _) = self;

		if length_syntax.is_some() {
			format_ident!("_{}_", ident)
		} else {
			format_ident!("__{}__", ident)
		}
	}
}

impl Args {
	pub fn format(&self) -> Vec<Ident> {
		let Self(args) = self;

		args.iter().map(|arg| arg.format()).collect()
	}
}

impl Source {
	pub fn args_to_tokens(&self, tokens: &mut TokenStream2) {
		if let Some(Args(args)) = &self.args {
			for Arg(_, ident, r#type) in args {
				tokens.append_tokens(|| quote!(#ident: &#r#type,));
			}
		}
	}

	pub fn formatted_args_to_tokens(&self, tokens: &mut TokenStream2) {
		if let Some(args) = &self.args {
			let args = args.format();

			for ident in args {
				tokens.append_tokens(|| quote!(&#ident,));
			}
		}
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
		let Self(_, ident, r#type) = self;

		ident.to_tokens(tokens);
		quote!(:).to_tokens(tokens);
		r#type.to_tokens(tokens);
	}
}

// }}}

// Parsing {{{

impl ParseWithContext for Source {
	type Context = (Option<IdentMap>, LengthMode);

	fn parse(input: ParseStream, map: Option<IdentMap>, mode: &LengthMode) -> Result<Self> {
		let fork = &input.fork();
		let args = if let Some(map) = map {
			Args::parse_mapped(fork, map, mode)
		} else {
			Args::parse_unmapped(fork, mode)
		};

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

	pub fn parse_mapped(input: ParseStream, map: IdentMap, mode: &LengthMode) -> Result<Self> {
		Self::parse(input, Some(map), mode)
	}

	pub fn parse_unmapped(input: ParseStream, mode: &LengthMode) -> Result<Self> {
		Self::parse(input, None, mode)
	}
}

impl Arg {
	pub fn parse_mapped(input: ParseStream, map: IdentMap, mode: &LengthMode) -> Result<Self> {
		Ok(if let LengthMode::Request | LengthMode::Reply = mode && input.peek(Token![self]) {
			// If `self::length` syntax is allowed, and this `Arg` begins with
			// `self`...

			// Parse the `self` token.
			let self_token: Token![self] = input.parse()?;
			// Parse the following `::` token.
			let double_colon_token: Token![::] = input.parse()?;

			// Parse the `length` identifier following `self::`.
			let ident: Ident = input.parse()?;
			// If the `ident` is not `length`, generate an error.
			if ident != "length" {
				return Err(Error::new(
					ident.span(),
					"only `self::length` syntax is allowed with `self::`",
				));
			}

			Self(Some((self_token, double_colon_token)), ident, Some(Type::Verbatim(match mode {
				// Requests use `u16` lengths.
				LengthMode::Request => quote!(u16),
				// Replies use `u32` lengths.
				LengthMode::Reply => quote!(u32),

				_ => unreachable!(),
			})))
		} else {
			// Otherwise, if `self::length` syntax is not allowed, or if this
			// is not a `self::length` `Arg`...

			let ident: Ident = input.parse()?;

			if let Some(r#type) = map.get(&ident.to_string()) {
				Self(None, ident, Some(r#type.to_owned()))
			} else {
				return Err(Error::new(
					ident.span(),
					"unrecognized source argument identifier",
				));
			}
		})
	}

	pub fn parse_unmapped(input: ParseStream, mode: &LengthMode) -> Result<Self> {
		Ok(if let LengthMode::Request | LengthMode::Reply = mode && input.peek(Token![self]) {
			let self_token: Token![self] = input.parse()?;
			let double_colon_token: Token![::] = input.parse()?;

			Self(Some((self_token, double_colon_token)), input.parse()?, Some(Type::Verbatim(match mode {
				LengthMode::Request => quote!(u16),
				LengthMode::Reply => quote!(u32),
				_ => unreachable!(),
			})))
		} else {
			Self(None, input.parse()?, None)
		})
	}
}

impl Args {
	fn parse(input: ParseStream, map: Option<IdentMap>, mode: &LengthMode) -> Result<Self> {
		let mut args = Punctuated::new();

		while input.peek(Ident) {
			if let Some(map) = map {
				args.push_value(Arg::parse_mapped(input, map, mode)?);
			} else {
				args.push_value(Arg::parse_unmapped(input, mode)?);
			}

			if input.peek(Token![,]) {
				args.push_punct(input.parse()?);
			} else {
				break;
			}
		}

		Ok(Self(args))
	}

	pub fn parse_mapped(input: ParseStream, map: IdentMap, mode: &LengthMode) -> Result<Self> {
		Self::parse(input, Some(map), mode)
	}

	pub fn parse_unmapped(input: ParseStream, mode: &LengthMode) -> Result<Self> {
		Self::parse(input, None, mode)
	}
}

// }}}
