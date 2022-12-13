// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use syn::{
	braced,
	bracketed,
	parenthesized,
	parse::{Parse, ParseStream},
	spanned::Spanned,
	Error,
	Result,
};

use super::*;
use crate::{attribute::parsing::ParsedAttributes, source::Source, ParseWithContext, PsExt};

pub enum ElementType {
	Named,
	Unnamed,
}

impl ParseWithContext for Elements {
	type Context = bool;

	fn parse_with(input: ParseStream, length_allowed: bool) -> Result<Self> {
		let mut map = HashMap::new();

		let context = (Some(&mut map), length_allowed);

		Ok(if input.peek(token::Brace) {
			let content;

			Self::Struct {
				brace_token: braced!(content in input),
				elements: content.parse_terminated_with(|| (ElementType::Named, context))?,
			}
		} else if input.peek(token::Paren) {
			let content;

			Self::Tuple {
				paren_token: parenthesized!(content in input),
				elements: content.parse_terminated_with(|| (ElementType::Unnamed, context))?,
			}
		} else {
			Self::Unit
		})
	}
}

impl<'a> ParseWithContext for Element {
	type Context = (ElementType, Source::Context<'a>);

	fn parse_with(input: ParseStream, context: Self::Context<'a>) -> Result<Self> {
		let (element_type, context) = context;
		let parsed_attributes = crate::attribute::parsing::parse_attributes(input, context)?;

		Ok(if input.peek(Token![_]) {
			Self::SingleUnused(input.parse_with(parsed_attributes)?)
		} else if input.peek(token::Bracket) {
			Self::ArrayUnused(Box::new(input.parse_with((parsed_attributes, context))?))
		} else if input.peek(Token![let]) {
			let (_, length_allowed) = context;

			Self::Let(Box::new(
				input.parse_with((parsed_attributes, length_allowed))?,
			))
		} else {
			Self::Field(Box::new(
				input.parse_with((element_type, parsed_attributes))?,
			))
		})
	}
}

impl ParseWithContext for SingleUnused {
	type Context = ParsedAttributes;

	fn parse_with(input: ParseStream, context: Self::Context<'_>) -> Result<Self> {
		let ParsedAttributes {
			attributes,

			context_attribute,
			metabyte_attribute,
			sequence_attribute,
		} = context;

		if let Some(attribute) = attributes.first() {
			return Err(Error::new(
				attribute.span(),
				"normal attributes are not allowed for singular unused bytes elements",
			));
		}

		if let Some(attribute) = context_attribute {
			return Err(Error::new(
				attribute.span(),
				"context attributes are not allowed for singular unused bytes elements",
			));
		}

		if let Some(attribute) = sequence_attribute {
			return Err(Error::new(
				attribute.span(),
				"sequence attributes are not allowed for singular unused bytes elements",
			));
		}

		Ok(Self {
			attribute: metabyte_attribute,
			underscore_token: input.parse()?,
		})
	}
}

impl<'a> ParseWithContext for ArrayUnused {
	type Context = (ParsedAttributes, Source::Context<'a>);

	fn parse_with(input: ParseStream, context: Self::Context<'a>) -> Result<Self>
	where
		Self: Sized,
	{
		let (
			ParsedAttributes {
				attributes,
				metabyte_attribute,
				context_attribute,
				sequence_attribute,
			},
			context,
		) = context;

		if let Some(attribute) = metabyte_attribute {
			return Err(Error::new(
				attribute.span(),
				"metabyte attributes are not allowed for array-type unused bytes elements",
			));
		}

		if let Some(attribute) = context_attribute {
			return Err(Error::new(
				attribute.span(),
				"context attributes are not allowed for array-type unused bytes elements",
			));
		}

		if let Some(attribute) = sequence_attribute {
			return Err(Error::new(
				attribute.span(),
				"sequence attributes are not allowed for array-type unused bytes elements",
			));
		}

		let content;

		Ok(Self {
			attributes,

			bracket_token: bracketed!(content in input),
			underscore_token: content.parse()?,
			semicolon_token: content.parse()?,
			content: content.parse_with(context)?,
		})
	}
}

impl<'a> ParseWithContext for UnusedContent {
	type Context = Source::Context<'a>;

	fn parse_with(input: ParseStream, context: Self::Context<'a>) -> Result<Self>
	where
		Self: Sized,
	{
		Ok(if input.peek(Token![..]) {
			Self::Infer(input.parse()?)
		} else {
			Self::Source(Box::new(input.parse_with(context)?))
		})
	}
}

impl<'a> ParseWithContext for Let {
	type Context = (ParsedAttributes, bool);

	fn parse_with(input: ParseStream, context: Self::Context<'a>) -> Result<Self>
	where
		Self: Sized,
	{
		let (
			ParsedAttributes {
				attributes,
				metabyte_attribute,
				context_attribute,
				sequence_attribute,
			},
			context,
		) = context;

		if let Some(attribute) = sequence_attribute {
			return Err(Error::new(
				attribute.span(),
				"sequence attributes are not allowed for let elements",
			));
		}

		Ok(Self {
			attributes,
			metabyte_attribute,
			context_attribute,

			let_token: input.parse()?,

			ident: input.parse()?,
			colon_token: input.parse()?,
			r#type: input.parse()?,

			equals_token: input.parse()?,

			source: input.parse_with((None, context))?,
		})
	}
}

impl ParseWithContext for Field {
	type Context = (ElementType, ParsedAttributes);

	fn parse_with(input: ParseStream, context: Self::Context<'_>) -> Result<Self>
	where
		Self: Sized,
	{
		let (
			element_type,
			ParsedAttributes {
				attributes,
				metabyte_attribute,
				context_attribute,
				sequence_attribute,
			},
		) = context;

		Ok(Self {
			attributes,
			metabyte_attribute,
			context_attribute,
			sequence_attribute,

			visibility: input.parse()?,
			ident: match element_type {
				ElementType::Named => Some((input.parse()?, input.parse::<Token![:]>()?)),
				ElementType::Unnamed => None,
			},
			r#type: input.parse()?,
		})
	}
}
