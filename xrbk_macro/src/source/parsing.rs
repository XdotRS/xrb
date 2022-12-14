// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::element::parsing::DefinitionType;
use quote::format_ident;
use std::collections::HashMap;
use syn::{
	parse::{discouraged::Speculative, Parse, ParseStream},
	spanned::Spanned,
	Error,
};

use super::*;
use crate::ext::{ParseWithContext, PsExt};

pub type IdentMap<'a> = &'a HashMap<String, Type>;
pub type IdentMapMut<'a> = &'a mut HashMap<String, Type>;

impl ParseWithContext for Arg {
	type Context<'a> = (IdentMap<'a>, Option<IdentMap<'a>>);

	fn parse_with(input: ParseStream, maps: Self::Context<'_>) -> syn::Result<Self>
	where
		Self: Sized,
	{
		let (let_map, field_map) = maps;

		let ident: Ident = input.parse()?;
		let formatted_ident = format_ident!("__{}__", ident);

		let r#type = if let Some(r#type) = let_map.get(&ident.to_string()) {
			Some(r#type.to_owned())
		} else if let Some(field_map) = field_map {
			match field_map.get(&ident.to_string()) {
				Some(r#type) => Some(r#type.to_owned()),
				None => {
					return Err(Error::new(
						ident.span(),
						"unrecognized source argument identifier",
					));
				},
			}
		} else {
			None
		};

		Ok(Self {
			ident,
			formatted_ident,
			r#type,
		})
	}
}

impl Parse for LengthArg {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let self_token = input.parse()?;
		let double_colon_token = input.parse()?;

		let length_token = {
			let ident: Ident = input.parse()?;

			if ident != "length" {
				return Err(Error::new(ident.span(), "expected message `length`"));
			}

			ident
		};
		let formatted_length_token = format_ident!("_{}_", length_token);

		Ok(Self {
			self_token,
			double_colon_token,
			length_token,

			formatted_length_token,
		})
	}
}

impl ParseWithContext for Args {
	type Context<'a> = ((IdentMap<'a>, Option<IdentMap<'a>>), DefinitionType);

	fn parse_with(input: ParseStream, context: Self::Context<'_>) -> syn::Result<Self>
	where
		Self: Sized,
	{
		let (maps, definition_type) = context;

		let mut args = Punctuated::new();
		let mut length_arg = None;

		while input.peek(Ident) || (definition_type.length_syntax() && input.peek(Token![self])) {
			if definition_type.length_syntax() && input.peek(Token![self]) {
				if length_arg.is_some() {
					let length_arg2: LengthArg = input.parse()?;

					return Err(Error::new(
						length_arg2.span(),
						"duplicate message length argument",
					));
				}

				length_arg = Some(input.parse()?);

				if input.peek(Token![,]) {
					input.parse::<Token![,]>()?;
				} else {
					break;
				}
			} else {
				args.push_value(input.parse_with(maps)?);

				if input.peek(Token![,]) {
					args.push_punct(input.parse()?);
				} else {
					break;
				}
			}
		}

		Ok(Self { args, length_arg })
	}
}

impl ParseWithContext for Source {
	type Context<'a> = ((IdentMap<'a>, Option<IdentMap<'a>>), DefinitionType);

	fn parse_with(input: ParseStream, context: Self::Context<'_>) -> syn::Result<Self>
	where
		Self: Sized,
	{
		let fork = &input.fork();
		let args = fork.parse_with(context);

		Ok(Self {
			args: if let Ok(args) = args && fork.peek(Token![=>]) {
				input.advance_to(fork);

				Some((args, input.parse::<Token![=>]>()?))
			} else {
				None
			},

			expr: input.parse()?,
		})
	}
}
