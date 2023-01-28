// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::definition::DefinitionType;
use quote::format_ident;
use syn::{
	parse::{discouraged::Speculative, Parse, ParseStream},
	spanned::Spanned,
	Error,
};

use super::*;
use crate::ext::{ParseWithContext, PsExt};

impl ParseWithContext for SourceArg {
	type Context<'a> = (IdentMap<'a>, Option<IdentMap<'a>>);

	fn parse_with(input: ParseStream, maps: Self::Context<'_>) -> syn::Result<Self>
	where
		Self: Sized,
	{
		let (let_map, field_map) = maps;

		let ident: Ident = input.parse()?;

		let (r#type, formatted) = if let Some(r#type) = let_map.get(&ident.to_string()) {
			(
				Some(r#type.to_owned()),
				Some(format_ident!("let_{}", ident)),
			)
		} else if let Some(field_map) = field_map {
			match field_map.get(&ident.to_string()) {
				Some(r#type) => (
					Some(r#type.to_owned()),
					Some(format_ident!("field_{}", ident)),
				),
				None => {
					return Err(Error::new(
						ident.span(),
						"unrecognized source argument identifier",
					));
				},
			}
		} else {
			(None, None)
		};

		let pattern = if input.peek(Token![:]) {
			Some((input.parse::<Token![:]>()?, input.parse::<Pat>()?))
		} else {
			None
		};

		Ok(Self {
			ident,
			formatted,
			r#type,
			pattern,
		})
	}
}

impl Parse for SourceRemainingArg {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let self_token = input.parse()?;
		let double_colon_token = input.parse()?;

		let remaining_token = {
			let ident: Ident = input.parse()?;

			if ident != "remaining" {
				return Err(Error::new(ident.span(), "expected `remaining` bytes"));
			}

			ident
		};

		Ok(Self {
			self_token,
			double_colon_token,
			remaining_token,
		})
	}
}

impl ParseWithContext for SourceArgs {
	type Context<'a> = ((IdentMap<'a>, Option<IdentMap<'a>>), DefinitionType);

	fn parse_with(input: ParseStream, context: Self::Context<'_>) -> syn::Result<Self>
	where
		Self: Sized,
	{
		let (maps, definition_type) = context;

		let mut args = Punctuated::new();
		let mut remaining_arg = None;

		while input.peek(Ident) || (definition_type.remaining_syntax() && input.peek(Token![self]))
		{
			if definition_type.remaining_syntax() && input.peek(Token![self]) {
				if remaining_arg.is_some() {
					let remaining_arg2: SourceRemainingArg = input.parse()?;

					return Err(Error::new(
						remaining_arg2.span(),
						"duplicate remaining bytes argument",
					));
				}

				remaining_arg = Some((input.parse()?, definition_type));

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

		Ok(Self {
			args,
			remaining_arg,
		})
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
