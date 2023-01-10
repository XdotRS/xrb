// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use crate::{definition::DefinitionType, ParseWithContext, PsExt};
use syn::{
	parse::{discouraged::Speculative, Parse, ParseStream},
	Attribute,
	Token,
	Visibility,
};

impl Parse for Definitions {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut definitions = Vec::new();

		while !input.is_empty() {
			definitions.push(input.parse()?);
		}

		Ok(Self(definitions))
	}
}

impl Parse for Definition {
	fn parse(input: ParseStream) -> Result<Self> {
		let fork = &input.fork();

		let item_attributes = fork.parse::<ParsedItemAttributes>()?;
		let visibility = fork.parse::<Visibility>()?;

		Ok(if item_attributes.contains_xrbk_derives() {
			if fork.peek(Token![struct]) {
				input.advance_to(fork);

				let struct_token = input.parse()?;
				let ident = input.parse()?;
				let generics = input.parse()?;

				if !input.peek(Token![:]) {
					Self::Struct(Struct {
						item_attributes,
						visibility,
						struct_token,
						ident,
						generics,
						content: input.parse_with(DefinitionType::Basic)?,
					})
				} else {
					let colon_token = input.parse()?;
					let message_token: Ident = input.parse()?;

					match &*message_token.to_string() {
						"Request" => Self::Request(input.parse_with((
							item_attributes,
							visibility,
							struct_token,
							ident,
							generics,
							colon_token,
							message_token,
						))?),

						"Reply" => Self::Reply(input.parse_with((
							item_attributes,
							visibility,
							struct_token,
							ident,
							generics,
							colon_token,
							message_token,
						))?),

						"Event" => Self::Event(input.parse_with((
							item_attributes,
							visibility,
							struct_token,
							ident,
							generics,
							colon_token,
							message_token,
						))?),

						_ => {
							return Err(Error::new(
								message_token.span(),
								"expected `Request`, `Reply`, or `Event` message type",
							))
						},
					}
				}
			} else if fork.peek(Token![enum]) {
				input.advance_to(fork);

				Self::Enum(input.parse_with((item_attributes, visibility))?)
			} else {
				Self::Other(input.parse()?)
			}
		} else {
			Self::Other(input.parse()?)
		})
	}
}

type MetadataContext = (
	ParsedItemAttributes,
	Visibility,
	Token![struct],
	Ident,
	Generics,
	Token![:],
	Ident,
);

impl ParseWithContext for Request {
	type Context<'a> = MetadataContext;

	fn parse_with(
		input: ParseStream,
		(item_attributes, visibility, struct_token, ident, generics, colon_token, request_token): MetadataContext,
	) -> Result<Self>
	where
		Self: Sized,
	{
		let content;

		Ok(Self {
			item_attributes,
			visibility,
			struct_token,
			ident,
			generics,
			colon_token,
			request_token,
			paren_token: parenthesized!(content in input),
			major_opcode: content.parse()?,
			minor_opcode: if content.peek(Token![,]) {
				Some((content.parse()?, content.parse()?))
			} else {
				None
			},
			reply: if input.peek(Token![->]) {
				Some((input.parse()?, input.parse()?))
			} else {
				None
			},
			content: input.parse_with(DefinitionType::Request)?,
		})
	}
}

impl ParseWithContext for Reply {
	type Context<'a> = MetadataContext;

	fn parse_with(
		input: ParseStream,
		(item_attributes, visibility, struct_token, ident, generics, colon_token, reply_token): MetadataContext,
	) -> Result<Self>
	where
		Self: Sized,
	{
		Ok(Self {
			item_attributes,
			visibility,
			struct_token,
			ident,
			generics,
			colon_token,
			reply_token,
			for_token: input.parse()?,
			request: input.parse()?,
			content: input.parse_with(DefinitionType::Reply)?,
		})
	}
}

impl ParseWithContext for Event {
	type Context<'a> = MetadataContext;

	fn parse_with(
		input: ParseStream,
		(item_attributes, visibility, struct_token, ident, generics, colon_token, event_token): MetadataContext,
	) -> Result<Self>
	where
		Self: Sized,
	{
		let content;

		Ok(Self {
			item_attributes,
			visibility,
			struct_token,
			ident,
			generics,
			colon_token,
			event_token,
			paren_token: parenthesized!(content in input),
			code: content.parse()?,
			content: input.parse_with(DefinitionType::Event)?,
		})
	}
}

impl ParseWithContext for Enum {
	type Context<'a> = (ParsedItemAttributes, Visibility);

	fn parse_with(
		input: ParseStream, (item_attributes, visibility): (ParsedItemAttributes, Visibility),
	) -> Result<Self>
	where
		Self: Sized,
	{
		let content;

		Ok(Self {
			item_attributes,
			visibility,
			enum_token: input.parse()?,
			ident: input.parse()?,
			generics: input.parse()?,
			brace_token: braced!(content in input),
			variants: content.parse_terminated(Variant::parse)?,
		})
	}
}

impl Parse for Variant {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			attributes: input.call(Attribute::parse_outer)?,
			ident: input.parse()?,
			content: input.parse_with(DefinitionType::Basic)?,
			discriminant: if input.peek(Token![=]) {
				Some((input.parse()?, input.parse()?))
			} else {
				None
			},
		})
	}
}
