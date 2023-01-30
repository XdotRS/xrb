// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::{
	parse::{discouraged::Speculative, Parse, ParseStream},
	spanned::Spanned,
	Attribute,
	Token,
	Type,
	Visibility,
};

use crate::{definition::DefinitionType, ParseWithContext, PsExt};

use super::*;

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
					let message_path: Path = input.parse()?;

					let message_ident = match message_path.get_ident() {
						Some(ident) => ident,

						None => {
							return Err(
								input.error("expected `Request`, `Reply`, `Event`, or `Error`")
							);
						},
					};

					match &*message_ident.to_string() {
						"Request" => Self::Request(input.parse_with((
							item_attributes,
							visibility,
							struct_token,
							ident,
							generics,
							colon_token,
							message_path,
						))?),

						"Reply" => Self::Reply(input.parse_with((
							item_attributes,
							visibility,
							struct_token,
							ident,
							generics,
							colon_token,
							message_path,
						))?),

						"Event" => Self::Event(input.parse_with((
							item_attributes,
							visibility,
							struct_token,
							ident,
							generics,
							colon_token,
							message_path,
						))?),

						"Error" => Self::Error(input.parse_with((
							item_attributes,
							visibility,
							struct_token,
							ident,
							generics,
							colon_token,
							message_path,
						))?),

						_ => {
							return Err(syn::Error::new(
								message_path.span(),
								"expected `Request`, `Reply`, `Event`, or `Error` message type",
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
	Path,
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

		let paren_token = parenthesized!(content in input);

		let major_opcode = content.parse()?;
		let mut comma1 = None;
		let mut minor_opcode = None;
		let mut comma2 = None;
		let mut other_errors = None;
		let mut comma3 = None;

		if content.peek(Token![,]) {
			comma1 = content.parse()?;
			let fork = &content.fork();

			if let Ok(r#type) = fork.parse::<Type>() {
				if fork.peek(Token![,]) {
					let comma = fork.parse()?;

					if fork.is_empty() {
						content.advance_to(fork);
						other_errors = Some(r#type);
						comma3 = Some(comma);
					} else {
						minor_opcode = Some(content.parse::<Expr>()?);

						if content.peek(Token![,]) {
							comma2 = Some(content.parse()?);

							if !content.is_empty() {
								other_errors = Some(content.parse::<Type>()?);

								if content.peek(Token![,]) {
									comma3 = Some(content.parse()?);
								}
							}
						}
					}
				} else {
					content.advance_to(fork);
					other_errors = Some(r#type);
				}
			} else {
				minor_opcode = Some(content.parse::<Expr>()?);

				if content.peek(Token![,]) {
					comma2 = Some(content.parse()?);

					if !content.is_empty() {
						other_errors = Some(content.parse::<Type>()?);

						if content.peek(Token![,]) {
							comma3 = Some(content.parse()?);
						}
					}
				}
			}
		}

		Ok(Self {
			item_attributes,

			visibility,
			struct_token,
			ident,
			generics,

			colon_token,
			request_token,

			paren_token,

			major_opcode,
			comma1,

			minor_opcode,
			comma2,

			other_errors,
			comma3,

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
			event_code: content.parse()?,
			comma: if content.peek(Token![,]) {
				Some(content.parse()?)
			} else {
				None
			},

			content: input.parse_with(DefinitionType::Event)?,
		})
	}
}

impl ParseWithContext for Error {
	type Context<'a> = MetadataContext;

	fn parse_with(
		input: ParseStream,
		(item_attributes, visibility, struct_token, ident, generics, colon_token, error_token): MetadataContext,
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
			error_token,

			paren_token: parenthesized!(content in input),
			error_code: content.parse()?,
			comma: if content.peek(Token![,]) {
				Some(content.parse()?)
			} else {
				None
			},

			content: input.parse_with(DefinitionType::Error)?,
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

			discriminant_type: if input.peek(Token![:]) {
				Some((input.parse::<Token![:]>()?, input.parse::<Type>()?))
			} else {
				None
			},

			where_clause: input.parse()?,

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
				Some((input.parse::<Token![=]>()?, input.parse::<Expr>()?))
			} else {
				None
			},
		})
	}
}
