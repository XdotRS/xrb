// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use crate::PsExt;
use syn::{
	parse::{discouraged::Speculative, Parse, ParseStream},
	token, Attribute, Token, Visibility,
};

impl Parse for Definition {
	fn parse(input: ParseStream) -> Result<Self> {
		let fork = &input.fork();

		let attributes = fork.call(Attribute::parse_outer)?;
		let visibility = fork.parse::<Visibility>()?;

		Ok(if fork.peek(Token![struct]) {
			input.advance_to(fork);

			let metadata = input.parse_with((attributes, visibility))?;
			let items = input.parse_with(Some(&metadata))?;
			let semicolon = match items {
				Items::Named { .. } => None,
				_ => Some(input.parse()?),
			};

			Self::Structlike(metadata, items, semicolon)
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

	fn parse_with(input: ParseStream, context: Self::Context) -> Result<Self>
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
				vis,
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
				}
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

	fn parse_with(input: ParseStream, context: Self::Context) -> Result<Self>
	where
		Self: Sized,
	{
		let content;
		let (attributes, vis, struct_token, ident, generics, colon_token, request_token) = context;

		Ok(Self {
			attributes,
			vis,
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

	fn parse_with(input: ParseStream, context: Self::Context) -> Result<Self>
	where
		Self: Sized,
	{
		let content;
		let (attributes, vis, struct_token, ident, generics, colon_token, reply_token) = context;

		let paren_token = if input.peek(token::Paren) {
			Some(parenthesized!(content in input))
		} else {
			None
		};
		let question_token = if paren_token.is_some() {
			Some(content.parse()?)
		} else {
			None
		};
		let sequence_token = if paren_token.is_some() {
			let sequence_token: Ident = content.parse()?;

			if sequence_token != "sequence" {
				return Err(Error::new(sequence_token.span(), "expected `sequence`"));
			}

			Some(sequence_token)
		} else {
			None
		};

		let for_token = input.parse()?;
		let request = input.parse()?;

		Ok(Self {
			attributes,
			vis,
			struct_token,
			ident,
			generics,
			colon_token,
			reply_token,
			paren_token,
			question_token,
			sequence_token,
			for_token,
			request,
		})
	}
}

impl ParseWithContext for Event {
	type Context = MetadataContext;

	fn parse_with(input: ParseStream, context: Self::Context) -> Result<Self>
	where
		Self: Sized,
	{
		let content;
		let (attributes, vis, struct_token, ident, generics, colon_token, event_token) = context;

		Ok(Self {
			attributes,
			vis,
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

impl ParseWithContext for Enum {
	type Context = (Vec<Attribute>, Visibility);

	fn parse_with(input: ParseStream, context: Self::Context) -> Result<Self>
	where
		Self: Sized,
	{
		let content;
		let (attributes, vis) = context;

		Ok(Self {
			attributes,
			vis,
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
			items: input.parse_with(None)?,
			discriminant: if input.peek(Token![=]) {
				Some((input.parse()?, input.parse()?))
			} else {
				None
			},
		})
	}
}
