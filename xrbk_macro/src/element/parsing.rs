// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::Span;
use std::collections::HashMap;
use syn::{
	braced,
	bracketed,
	parenthesized,
	parse::{discouraged::Speculative, ParseStream},
	spanned::Spanned,
	Result,
};

use super::*;
use crate::{
	attribute::parsing::ParsedAttributes,
	definition::DefinitionType,
	source::{IdentMap, IdentMapMut, SourceArgs},
	ParseWithContext,
	PsExt,
};

impl ParseWithContext for RegularContent {
	type Context<'a> = DefinitionType;

	fn parse_with(input: ParseStream, definition_type: DefinitionType) -> Result<Self>
	where
		Self: Sized,
	{
		let content;

		Ok(Self {
			brace_token: braced!(content in input),
			elements: content.parse_with((ElementType::Named, definition_type))?,
		})
	}
}

impl ParseWithContext for TupleContent {
	type Context<'a> = DefinitionType;

	fn parse_with(input: ParseStream, definition_type: DefinitionType) -> Result<Self>
	where
		Self: Sized,
	{
		let content;

		Ok(Self {
			paren_token: parenthesized!(content in input),
			elements: content.parse_with((ElementType::Unnamed, definition_type))?,
		})
	}
}

impl ParseWithContext for Content {
	type Context<'a> = DefinitionType;

	fn parse_with(input: ParseStream, definition_type: DefinitionType) -> Result<Self> {
		Ok(if input.peek(token::Brace) {
			Self::Regular(input.parse_with(definition_type)?)
		} else if input.peek(token::Paren) {
			Self::Tuple(input.parse_with(definition_type)?)
		} else {
			Self::Unit
		})
	}
}

impl ParseWithContext for StructlikeContent {
	type Context<'a> = DefinitionType;

	fn parse_with(input: ParseStream, definition_type: DefinitionType) -> Result<Self>
	where
		Self: Sized,
	{
		// Look at the next token.
		let look = input.lookahead1();

		// If the next token is `where`, then there is a where clause preceding the
		// content.
		Ok(if look.peek(Token![where]) {
			let where_clause = Some(input.parse()?);
			// Look at the token after the where clause.
			let look = input.lookahead1();

			// If the where clause precedes the content, then it can't be a tuple content.

			// If the next token is `{`, this is a regular content.
			if look.peek(token::Brace) {
				Self::Regular {
					where_clause,
					content: input.parse_with(definition_type)?,
				}
			// If the next token is `;`, this is a unit content.
			} else if look.peek(Token![;]) {
				Self::Unit {
					where_clause,
					semicolon: input.parse()?,
				}
			// Otherwise, if the next token is neither `{` nor `;`, then this is
			// a parsing error.
			} else {
				return Err(look.error());
			}
		// Otherwise, if the next token is `{`, then this is a regular content
		// with no where clause.
		} else if look.peek(token::Brace) {
			Self::Regular {
				where_clause: None,
				content: input.parse_with(definition_type)?,
			}
		// Otherwise, if the next token is `(`, then this is a tuple content.
		} else if look.peek(token::Paren) {
			Self::Tuple {
				content: input.parse_with(definition_type)?,
				where_clause: if input.peek(Token![where]) {
					Some(input.parse()?)
				} else {
					None
				},
				semicolon: input.parse()?,
			}
		// Otherwise, if the next token is `;`, then this is a unit content.
		} else if look.peek(Token![;]) {
			Self::Unit {
				where_clause: None,
				semicolon: input.parse()?,
			}
		// Otherwise, if the next token is not `where`, `{`, `(`, nor `;`, then
		// this is a parsing error.
		} else {
			return Err(look.error());
		})
	}
}

impl ParseWithContext for Elements {
	type Context<'a> = (ElementType, DefinitionType);

	fn parse_with(
		input: ParseStream, (element_type, definition_type): (ElementType, DefinitionType),
	) -> Result<Self>
	where
		Self: Sized,
	{
		let mut let_map = HashMap::new();
		let mut field_map = HashMap::new();

		let mut elements = Punctuated::new();

		let mut metabyte_element = None;
		let mut sequence_element = None;

		let mut minor_opcode_element = None;
		let mut major_opcode_element = None;
		let mut error_data_element = None;

		let mut field_index = 0;
		let mut unused_index = 0;

		while !input.is_empty() {
			let element: Element = input.parse_with((
				(field_index, unused_index),
				element_type,
				definition_type,
				(&mut let_map, &mut field_map),
			))?;

			match &element {
				Element::Field(_) => field_index += 1,
				Element::ArrayUnused(_) => unused_index += 1,
				_ => (),
			}

			if element.is_metabyte() {
				if metabyte_element.is_some() {
					return Err(syn::Error::new(
						element.span(),
						"no more than one metabyte element is allowed per message",
					));
				}

				metabyte_element = Some(element);
				elements.push_value(ElementsItem::Metabyte);
			} else if let Element::Field(field) = &element
				&& field.is_sequence()
			{
				if sequence_element.is_some() {
					return Err(syn::Error::new(
						field.span(),
						"no more than one sequence field is allowed per message",
					));
				}

				sequence_element = Some(element);
				elements.push_value(ElementsItem::Sequence);
			} else if let Element::Field(field) = &element
				&& field.is_minor_opcode()
			{
				if minor_opcode_element.is_some() {
					return Err(syn::Error::new(
						field.span(),
						"no more than one minor opcode field is allowed per error",
					));
				}

				minor_opcode_element = Some(element);
				elements.push(ElementsItem::MinorOpcode)
			} else if let Element::Field(field) = &element
				&& field.is_major_opcode()
			{
				if major_opcode_element.is_some() {
					return Err(syn::Error::new(
						field.span(),
						"no more than one major opcode field is allowed per error",
					));
				}

				major_opcode_element = Some(element);
				elements.push(ElementsItem::MajorOpcode);
			} else if let Element::Field(field) = &element
				&& field.is_error_data()
			{
				if error_data_element.is_some() {
					return Err(syn::Error::new(
						field.span(),
						"no more than one error data field is allowed per error",
					));
				}

				error_data_element = Some(element);
				elements.push(ElementsItem::ErrorData);
			} else {
				elements.push_value(ElementsItem::Element(element));
			}

			if input.peek(Token![,]) {
				elements.push_punct(input.parse()?);
			} else {
				break;
			}
		}

		for item in &mut elements {
			let r#let = if let ElementsItem::Element(Element::Let(r#let)) = item {
				Some(r#let)
			} else if let ElementsItem::Metabyte = item
				&& let Some(Element::Let(r#let)) = &mut metabyte_element
			{
				Some(r#let)
			} else {
				None
			};

			if let Some(r#let) = r#let {
				if let Some((SourceArgs { args, .. }, _)) = &mut r#let.source.args {
					for arg in args {
						if arg.r#type.is_none() {
							if let Some(r#type) = field_map.get(&arg.ident.to_string()) {
								arg.r#type = Some(r#type.to_owned());
								arg.formatted = Some(format_ident!("field_{}", arg.ident))
							} else {
								return Err(syn::Error::new(
									arg.ident.span(),
									"unrecognized source argument identifier",
								));
							}
						}
					}
				}
			}
		}

		// If the last element is an `ArrayUnused` element with `UnusedContent::Infer`,
		// then we can set the `UnusedContent::Infer`'s `last_element` field to `true`.
		if let Some(ElementsItem::Element(Element::ArrayUnused(unused))) = elements.last_mut() {
			if let UnusedContent::Infer { last_element, .. } = &mut unused.content {
				*last_element = true;
			}
		}

		let contains_infer = elements.iter().any(|item| {
			if let ElementsItem::Element(Element::ArrayUnused(unused)) = &item {
				matches!(unused.content, UnusedContent::Infer { .. })
			} else {
				false
			}
		});

		match (&definition_type, &sequence_element) {
			(DefinitionType::Basic | DefinitionType::Request, Some(sequence)) => {
				return Err(syn::Error::new(
					sequence.span(),
					"sequence fields are only allowed for replies, events, and errors",
				));
			},

			(DefinitionType::Reply, None) => {
				return Err(syn::Error::new(
					Span::call_site(),
					"replies must have a sequence field of type `u16`",
				));
			},

			(DefinitionType::Error, None) => {
				return Err(syn::Error::new(
					Span::call_site(),
					"errors must have a sequence field of type `u16`",
				));
			},

			_ => {},
		}

		match (&definition_type, &minor_opcode_element) {
			(DefinitionType::Error, Some(_)) => {},

			(_, Some(minor_opcode)) => {
				return Err(syn::Error::new(
					minor_opcode.span(),
					"minor opcode fields are only allowed for errors",
				));
			},

			(DefinitionType::Error, None) => {
				return Err(syn::Error::new(
					Span::call_site(),
					"errors must have a minor opcode field of type `u16`",
				));
			},

			_ => {},
		}

		match (&definition_type, &major_opcode_element) {
			(DefinitionType::Error, Some(_)) => {},

			(_, Some(major_opcode)) => {
				return Err(syn::Error::new(
					major_opcode.span(),
					"major opcode fields are only allowed for errors",
				));
			},

			(DefinitionType::Error, None) => {
				return Err(syn::Error::new(
					Span::call_site(),
					"errors must have a major opcode field of type `u8`",
				));
			},

			_ => {},
		}

		match (&definition_type, &error_data_element) {
			(DefinitionType::Error, Some(_)) => {},

			(_, Some(error_data)) => {
				return Err(syn::Error::new(
					error_data.span(),
					"error data fields are only allowed for errors",
				));
			},

			_ => {},
		}

		Ok(Self {
			elements,

			metabyte_element,
			sequence_element,

			minor_opcode_element,
			major_opcode_element,
			error_data_element,

			contains_infer,
		})
	}
}

impl ParseWithContext for Element {
	type Context<'a> = (
		(usize, usize),
		ElementType,
		DefinitionType,
		(IdentMapMut<'a>, IdentMapMut<'a>),
	);

	fn parse_with(
		input: ParseStream,
		((field_index, unused_index), element_type, definition_type, (let_map, field_map)): Self::Context<'_>,
	) -> Result<Self> {
		let parsed_attributes = input.parse_with(((&*let_map, &*field_map), definition_type))?;

		Ok(if input.peek(Token![_]) {
			Self::SingleUnused(input.parse_with(parsed_attributes)?)
		} else if input.peek(Token![let]) {
			Self::Let(Box::new(input.parse_with((
				parsed_attributes,
				let_map,
				definition_type,
			))?))
		} else {
			if input.peek(token::Bracket) {
				let content;
				let fork = &input.fork();

				let bracket_token = bracketed!(content in fork);

				if content.peek(Token![_]) {
					input.advance_to(fork);

					return Ok(Self::ArrayUnused(Box::new(content.parse_with((
						unused_index,
						parsed_attributes,
						bracket_token,
						(&*let_map, &*field_map),
						definition_type,
					))?)));
				}
			}

			Self::Field(Box::new(input.parse_with((
				field_index,
				element_type,
				parsed_attributes,
				field_map,
			))?))
		})
	}
}

impl ParseWithContext for SingleUnused {
	type Context<'a> = ParsedAttributes;

	fn parse_with(
		input: ParseStream,
		ParsedAttributes {
			attributes,
			context_attribute,
			metabyte_attribute,
			sequence_attribute,
			minor_opcode_attribute,
			major_opcode_attribute,
			error_data_attribute,
			hide_attribute,
		}: ParsedAttributes,
	) -> Result<Self> {
		if let Some(attribute) = attributes.first() {
			return Err(syn::Error::new(
				attribute.span(),
				"normal attributes are not allowed for singular unused bytes elements",
			));
		}

		if let Some(attribute) = context_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"context attributes are not allowed for singular unused bytes elements",
			));
		}

		if let Some(attribute) = sequence_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"sequence attributes are not allowed for singular unused bytes elements",
			));
		}

		if let Some(attribute) = minor_opcode_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"minor opcode attributes are not allowed for singular unused bytes elements",
			));
		}

		if let Some(attribute) = major_opcode_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"major opcode attributes are not allowed for singular unused bytes elements",
			));
		}

		if let Some(attribute) = error_data_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"error data attributes ar e not allowed for singular unused bytes elements",
			));
		}

		if let Some(attribute) = hide_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"hide attributes are not allowed for singular unused bytes elements",
			));
		}

		Ok(Self {
			attribute: metabyte_attribute,
			underscore_token: input.parse()?,
		})
	}
}

impl ParseWithContext for ArrayUnused {
	type Context<'a> = (
		usize,
		ParsedAttributes,
		token::Bracket,
		(IdentMap<'a>, IdentMap<'a>),
		DefinitionType,
	);

	fn parse_with(
		input: ParseStream,
		(
			unused_index,
			ParsedAttributes {
				attributes,
				metabyte_attribute,
				context_attribute,
				sequence_attribute,
				minor_opcode_attribute,
				major_opcode_attribute,
				error_data_attribute,
				hide_attribute,
			},
			bracket_token,
			maps,
			definition_type,
		): Self::Context<'_>,
	) -> Result<Self>
	where
		Self: Sized,
	{
		if let Some(attribute) = metabyte_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"metabyte attributes are not allowed for array-type unused bytes elements",
			));
		}

		if let Some(attribute) = context_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"context attributes are not allowed for array-type unused bytes elements",
			));
		}

		if let Some(attribute) = sequence_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"sequence attributes are not allowed for array-type unused bytes elements",
			));
		}

		if let Some(attribute) = minor_opcode_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"minor opcode attributes are not allowed for array-type unused bytes elements",
			));
		}

		if let Some(attribute) = major_opcode_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"major opcode attributes are not allowed for array-type unused bytes elements",
			));
		}

		if let Some(attribute) = error_data_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"error data attributes are not allowed for array-type unused bytes elements",
			));
		}

		if let Some(attribute) = hide_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"hide attributes are not allowed for array-type unused bytes elements",
			));
		}

		Ok(Self {
			formatted: format_ident!("unused_{}", unused_index),

			attributes,

			bracket_token,
			underscore_token: input.parse()?,
			semicolon_token: input.parse()?,
			content: input.parse_with((maps, definition_type))?,
		})
	}
}

impl ParseWithContext for UnusedContent {
	type Context<'a> = ((IdentMap<'a>, IdentMap<'a>), DefinitionType);

	fn parse_with(
		input: ParseStream, ((let_map, field_map), definition_type): Self::Context<'_>,
	) -> Result<Self>
	where
		Self: Sized,
	{
		Ok(if input.peek(Token![..]) {
			Self::Infer {
				double_dot_token: input.parse()?,

				// We don't know whether this is the last element or not until we have parsed the
				// comma following it (if does indeed follow it), so we initialize this as `false`
				// until then.
				last_element: false,
			}
		} else {
			Self::Source(Box::new(
				input.parse_with(((let_map, Some(field_map)), definition_type))?,
			))
		})
	}
}

impl ParseWithContext for Let {
	type Context<'a> = (ParsedAttributes, IdentMapMut<'a>, DefinitionType);

	fn parse_with(
		input: ParseStream,
		(
			ParsedAttributes {
				attributes,
				metabyte_attribute,
				context_attribute,
				sequence_attribute,
				minor_opcode_attribute,
				major_opcode_attribute,
				error_data_attribute,
				hide_attribute,
			},
			let_map,
			definition_type,
		): Self::Context<'_>,
	) -> Result<Self>
	where
		Self: Sized,
	{
		if let Some(attribute) = sequence_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"sequence attributes are not allowed for let elements",
			));
		}

		if let Some(attribute) = minor_opcode_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"minor opcode attributes are not allowed for let elements",
			));
		}

		if let Some(attribute) = major_opcode_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"major opcode attributes are not allowed for let elements",
			));
		}

		if let Some(attribute) = error_data_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"error data attributes are not allowed for let elements",
			));
		}

		if let Some(attribute) = hide_attribute {
			return Err(syn::Error::new(
				attribute.span(),
				"hide attributes ar e not allowed for let elements",
			));
		}

		let let_token = input.parse()?;

		let ident: Ident = input.parse()?;
		let colon_token = input.parse()?;
		let r#type: Type = input.parse()?;

		let equals_token = input.parse()?;

		let source = input.parse_with(((&*let_map, None), definition_type))?;

		let_map.insert(ident.to_string(), r#type.to_owned());

		Ok(Self {
			formatted: format_ident!("let_{}", ident),

			attributes,
			metabyte_attribute,
			context_attribute,

			let_token,

			ident,
			colon_token,
			r#type,

			equals_token,

			source,
		})
	}
}

impl ParseWithContext for Field {
	type Context<'a> = (usize, ElementType, ParsedAttributes, IdentMapMut<'a>);

	fn parse_with(
		input: ParseStream,
		(
			index,
			element_type,
			ParsedAttributes {
				attributes,
				metabyte_attribute,
				context_attribute,
				sequence_attribute,
				minor_opcode_attribute,
				major_opcode_attribute,
				error_data_attribute,
				hide_attribute,
			},
			map,
		): Self::Context<'_>,
	) -> Result<Self>
	where
		Self: Sized,
	{
		let visibility = input.parse()?;

		let id = match element_type {
			ElementType::Named => FieldId::Ident(input.parse()?),
			ElementType::Unnamed => FieldId::Index(Index::from(index)),
		};
		let colon_token = if let FieldId::Ident(_) = id {
			Some(input.parse()?)
		} else {
			None
		};

		let r#type: Type = input.parse()?;

		map.insert(id.to_string(), r#type.to_owned());

		Ok(Self {
			formatted: match &id {
				FieldId::Ident(ident) => format_ident!("field_{}", ident),
				FieldId::Index(index) => format_ident!("field_{}", index),
			},

			attributes,
			metabyte_attribute,
			context_attribute,
			sequence_attribute,
			minor_opcode_attribute,
			major_opcode_attribute,
			error_data_attribute,
			hide_attribute,

			visibility,
			id,
			colon_token,
			r#type,
		})
	}
}
