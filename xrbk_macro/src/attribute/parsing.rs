// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::{
	bracketed,
	parenthesized,
	parse::{ParseStream, Result},
	spanned::Spanned,
	AttrStyle,
	Attribute,
	Error,
};

use super::*;
use crate::{element::parsing::DefinitionType, source::parsing::IdentMap, ParseWithContext, PsExt};

pub struct ParsedAttributes {
	pub attributes: Vec<Attribute>,

	pub context_attribute: Option<ContextAttribute>,
	pub metabyte_attribute: Option<MetabyteAttribute>,
	pub sequence_attribute: Option<SequenceAttribute>,
}

impl ParseWithContext for ParsedAttributes {
	type Context<'a> = <Context as ParseWithContext>::Context<'a>;

	fn parse_with(input: ParseStream, context: Self::Context<'_>) -> Result<Self>
	where
		Self: Sized,
	{
		let mut attributes = Vec::new();

		let mut context_attribute = None;
		let mut metabyte_attribute = None;
		let mut sequence_attribute = None;

		while input.peek(Token![#]) && input.peek2(token::Bracket) {
			let content;

			let hash_token = input.parse()?;
			let bracket_token = bracketed!(content in input);
			let path: Path = content.parse()?;

			if path.is_ident("context") {
				if context_attribute.is_some() {
					return Err(Error::new(
						path.span(),
						"no more than one context attribute is allowed per element",
					));
				}

				context_attribute = Some(ContextAttribute {
					hash_token,
					bracket_token,
					path,

					context: content.parse_with(context)?,
				});
			} else if path.is_ident("metabyte") {
				if metabyte_attribute.is_some() {
					return Err(Error::new(
						path.span(),
						"no more than one metabyte attribute is allowed per element",
					));
				}

				metabyte_attribute = Some(MetabyteAttribute {
					hash_token,
					bracket_token,
					path,
				});
			} else if path.is_ident("sequence") {
				if sequence_attribute.is_some() {
					return Err(Error::new(
						path.span(),
						"no more than one sequence attribute is allowed per element",
					));
				}

				sequence_attribute = Some(SequenceAttribute {
					hash_token,
					bracket_token,
					path,
				});
			} else {
				attributes.push(Attribute {
					pound_token: hash_token,
					style: AttrStyle::Outer,
					bracket_token,
					path,

					tokens: content.parse()?,
				});
			}
		}

		Ok(Self {
			attributes,

			context_attribute,
			metabyte_attribute,
			sequence_attribute,
		})
	}
}

impl ParseWithContext for Context {
	type Context<'a> = ((IdentMap<'a>, IdentMap<'a>), DefinitionType);

	fn parse_with(input: ParseStream, context: Self::Context<'_>) -> Result<Self> {
		let ((let_map, field_map), definition_type) = context;
		let look = input.lookahead1();

		Ok(if look.peek(token::Paren) {
			let content;

			Self::Paren {
				paren_token: parenthesized!(content in input),
				source: content.parse_with(((let_map, Some(field_map)), definition_type))?,
			}
		} else if look.peek(Token![=]) {
			Self::Equals {
				equals_token: input.parse()?,
				source: input.parse_with(((let_map, Some(field_map)), definition_type))?,
			}
		} else {
			return Err(look.error());
		})
	}
}
