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
use crate::{definition::DefinitionType, source::IdentMap, ParseWithContext, PsExt};

/// Normal attributes and special XRBK attributes which were parsed.
pub struct ParsedAttributes {
	/// Normal attributes which were parsed.
	pub attributes: Vec<Attribute>,

	/// A context attribute, if one was parsed.
	pub context_attribute: Option<ContextAttribute>,
	/// A metabyte attribute, if one was parsed.
	pub metabyte_attribute: Option<MetabyteAttribute>,
	/// A sequence attribute, if one was parsed.
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

		// While there are still attributes remaining...
		while input.peek(Token![#]) && input.peek2(token::Bracket) {
			let content;

			// Parse the `#`.
			let hash_token = input.parse()?;
			// Parse the square brackets.
			let bracket_token = bracketed!(content in input);
			// Parse the attribute name.
			let path: Path = content.parse()?;

			// If the name is `context`, parse it as a context attribute.
			if path.is_ident("context") {
				// If a context attribute has already been parsed, generate an error.
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

					// Parse the context.
					context: content.parse_with(context)?,
				});
			// If the name is `metabyte`, parse it as a metabyte attribute.
			} else if path.is_ident("metabyte") {
				// If a metabyte attribute has already been parsed, generate an error.
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
			// If the name is `sequence`, parse it as a sequence attribute.
			} else if path.is_ident("sequence") {
				// If a sequence attribute has already been parsed, generate an error.
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
			// Otherwise, if the name was not `context`, `metabyte`, nor
			// `sequence`, parse the attribute as a normal attribute.
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
			// Normal brackets.

			let content;

			Self::Paren {
				paren_token: parenthesized!(content in input),
				source: content.parse_with(((let_map, Some(field_map)), definition_type))?,
			}
		} else if look.peek(Token![=]) {
			// Equals.

			Self::Equals {
				equals_token: input.parse()?,
				source: input.parse_with(((let_map, Some(field_map)), definition_type))?,
			}
		} else {
			// If neither normal brackets nor an equals token was found, generate an error.
			return Err(look.error());
		})
	}
}
