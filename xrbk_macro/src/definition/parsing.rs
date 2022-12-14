// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use crate::{ParseWithContext, PsExt};
use syn::{
	parse::{discouraged::Speculative, Parse, ParseStream},
	Attribute,
	Token,
	Visibility,
};

impl Parse for Definitions<'_> {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut definitions = Vec::new();

		while !input.is_empty() {
			definitions.push(input.parse()?);
		}

		Ok(Self(definitions))
	}
}

impl Parse for Definition<'_> {
	fn parse(input: ParseStream) -> Result<Self> {
		let fork = &input.fork();

		let attributes = fork.call(Attribute::parse_outer)?;
		let visibility = fork.parse::<Visibility>()?;

		Ok(if fork.peek(Token![struct]) {
			input.advance_to(fork);

			let metadata = input.parse_with((attributes, visibility))?;

			let content = input.parse_with(match metadata {
				Metadata::Struct(_) => false,

				Metadata::Request(_) => true,
				Metadata::Reply(_) => true,
				Metadata::Event(_) => false,
			})?;

			let semicolon = match content {
				Content::Struct { .. } => None,
				_ => Some(input.parse()?),
			};

			Self::Structlike(metadata, content, semicolon)
		} else if fork.peek(Token![enum]) {
			input.advance_to(fork);

			Self::Enum(input.parse_with((attributes, visibility))?)
		} else {
			Self::Other(input.parse()?)
		})
	}
}

impl ParseWithContext for Metadata {
	type Context = (Vec<Attribute>, Visibility);

	fn parse_with(input: ParseStream, context: Self::Context<'_>) -> Result<Self>
	where
		Self: Sized,
	{
		let (attributes, vis) = context;

		let struct_token = input.parse()?;
		let ident = input.parse()?;
		let generics = input.parse()?;

		Ok(if !input.peek(Token![:]) {
			Self::Struct(Box::new(Struct {
				attributes,
				visibility: vis,
				struct_token,
				ident,
				generics,
			}))
		} else {
			let colon_token = input.parse()?;
			let message_token: Ident = input.parse()?;

			match &*message_token.to_string() {
				"Request" => Self::Request(Box::new(input.parse_with::<Request>((
					attributes,
					vis,
					struct_token,
					ident,
					generics,
					colon_token,
					message_token,
				))?)),

				"Reply" => Self::Reply(Box::new(input.parse_with::<Reply>((
					attributes,
					vis,
					struct_token,
					ident,
					generics,
					colon_token,
					message_token,
				))?)),

				"Event" => Self::Event(Box::new(input.parse_with::<Event>((
					attributes,
					vis,
					struct_token,
					ident,
					generics,
					colon_token,
					message_token,
				))?)),

				_ => {
					return Err(Error::new(
						message_token.span(),
						"expected `Request`, `Reply`, or `Event` message type",
					))
				},
			}
		})
	}
}

type MetadataContext = (
	Vec<Attribute>,
	Visibility,
	Token![struct],
	Ident,
	Generics,
	Token![:],
	Ident,
);

impl ParseWithContext for Request {
	type Context = MetadataContext;

	fn parse_with(input: ParseStream, context: Self::Context<'_>) -> Result<Self>
	where
		Self: Sized,
	{
		let content;
		let (attributes, visibility, struct_token, ident, generics, colon_token, request_token) =
			context;

		Ok(Self {
			attributes,
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
		})
	}
}

impl ParseWithContext for Reply {
	type Context = MetadataContext;

	fn parse_with(input: ParseStream, context: Self::Context<'_>) -> Result<Self>
	where
		Self: Sized,
	{
		let (attributes, visibility, struct_token, ident, generics, colon_token, reply_token) =
			context;

		Ok(Self {
			attributes,
			visibility,
			struct_token,
			ident,
			generics,
			colon_token,
			reply_token,
			for_token: input.parse()?,
			request: input.parse()?,
		})
	}
}

impl ParseWithContext for Event {
	type Context = MetadataContext;

	fn parse_with(input: ParseStream, context: Self::Context<'_>) -> Result<Self>
	where
		Self: Sized,
	{
		let content;
		let (attributes, visibility, struct_token, ident, generics, colon_token, event_token) =
			context;

		Ok(Self {
			attributes,
			visibility,
			struct_token,
			ident,
			generics,
			colon_token,
			event_token,
			paren_token: parenthesized!(content in input),
			code: content.parse()?,
		})
	}
}

impl ParseWithContext for Enum<'_> {
	type Context = (Vec<Attribute>, Visibility);

	fn parse_with(input: ParseStream, context: Self::Context<'_>) -> Result<Self>
	where
		Self: Sized,
	{
		let content;
		let (attributes, visibility) = context;

		Ok(Self {
			attributes,
			visibility,
			enum_token: input.parse()?,
			ident: input.parse()?,
			generics: input.parse()?,
			brace_token: braced!(content in input),
			variants: content.parse_terminated(Variant::parse)?,
		})
	}
}

impl Parse for Variant<'_> {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			attributes: input.call(Attribute::parse_outer)?,
			ident: input.parse()?,
			content: input.parse_with(false)?,
			discriminant: if input.peek(Token![=]) {
				Some((input.parse()?, input.parse()?))
			} else {
				None
			},
		})
	}
}
