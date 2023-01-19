// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, ToTokens};
use syn::{
	braced,
	bracketed,
	parenthesized,
	parse::{Parse, ParseStream, Result},
	punctuated::Punctuated,
	spanned::Spanned,
	AttrStyle,
	Attribute,
};

use super::*;
use crate::{definition::DefinitionType, source::IdentMap, ParseWithContext, PsExt, TsExt};

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
	/// A minor opcode attribute, if one was parsed.
	pub minor_opcode_attribute: Option<MinorOpcodeAttribute>,
	/// A major opcode attribute, if one was parsed.
	pub major_opcode_attribute: Option<MajorOpcodeAttribute>,
	/// An error data attribute, if one was parsed.
	pub error_data_attribute: Option<ErrorDataAttribute>,
	/// A hide attribute, if one was parsed.
	pub hide_attribute: Option<HideAttribute>,
}

pub struct ParsedItemAttributes {
	pub attributes: Vec<Attribute>,

	pub derive_x11_sizes: Punctuated<Path, Token![,]>,
	pub derive_constant_x11_sizes: Punctuated<Path, Token![,]>,
	pub derive_writables: Punctuated<Path, Token![,]>,
	pub derive_readables: Punctuated<Path, Token![,]>,
	pub derive_readable_with_contexts: Punctuated<Path, Token![,]>,
}

impl ParsedItemAttributes {
	pub fn contains_xrbk_derives(&self) -> bool {
		!self.derive_x11_sizes.is_empty()
			|| !self.derive_constant_x11_sizes.is_empty()
			|| !self.derive_writables.is_empty()
			|| !self.derive_readables.is_empty()
			|| !self.derive_readable_with_contexts.is_empty()
	}
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
		let mut minor_opcode_attribute = None;
		let mut major_opcode_attribute = None;
		let mut error_data_attribute = None;
		let mut hide_attribute = None;

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
					return Err(syn::Error::new(
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
					return Err(syn::Error::new(
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
					return Err(syn::Error::new(
						path.span(),
						"no more than one sequence attribute is allowed per element",
					));
				}

				sequence_attribute = Some(SequenceAttribute {
					hash_token,
					bracket_token,
					path,
				});
			// If the name is `minor_opcode`, parse it as a minor opcode
			// attribute.
			} else if path.is_ident("minor_opcode") {
				if minor_opcode_attribute.is_some() {
					return Err(syn::Error::new(
						path.span(),
						"no more than one minor opcode attribute is allowed per element",
					));
				}

				minor_opcode_attribute = Some(MinorOpcodeAttribute {
					hash_token,
					bracket_token,
					path,
				});
			// If the name is `major_opcode`, parse it as a major opcode
			// attribute.
			} else if path.is_ident("major_opcode") {
				if major_opcode_attribute.is_some() {
					return Err(syn::Error::new(
						path.span(),
						"no more than one major opcode attribute is allowed per element",
					));
				}

				major_opcode_attribute = Some(MajorOpcodeAttribute {
					hash_token,
					bracket_token,
					path,
				});
			// If the name is `error_data`, parse it as an error data attribute.
			} else if path.is_ident("error_data") {
				if error_data_attribute.is_some() {
					return Err(syn::Error::new(
						path.span(),
						"no more than one error data attribute is allowed per element",
					));
				}

				error_data_attribute = Some(ErrorDataAttribute {
					hash_token,
					bracket_token,
					path,
				});
			// If the name is `hide`, parse it as a hide attribute.
			} else if path.is_ident("hide") {
				if hide_attribute.is_some() {
					return Err(syn::Error::new(
						path.span(),
						"no more than one hide attribute is allowed per element",
					));
				}

				let inner_content;
				let paren_token = parenthesized!(inner_content in content);
				let hidden_traits = inner_content.parse_terminated(Path::parse)?;

				hide_attribute = Some(HideAttribute {
					hash_token,
					bracket_token,
					path,
					paren_token,
					hidden_traits,
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

		if let Some(hide_attribute) = &hide_attribute
			&& context_attribute.is_none()
			&& hide_attribute
				.hidden_traits
				.iter()
				.any(|r#trait| r#trait.is_ident(&format_ident!("Readable")))
			{
				return Err(syn::Error::new(
					hide_attribute.span(),
					"cannot hide this field when implementing Readable without a #[context(...)] attribute",
				));
		}

		Ok(Self {
			attributes,

			context_attribute,
			metabyte_attribute,
			sequence_attribute,
			minor_opcode_attribute,
			major_opcode_attribute,
			error_data_attribute,
			hide_attribute,
		})
	}
}

impl Parse for ParsedItemAttributes {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut attributes = Vec::new();

		let mut derive_x11_sizes = Punctuated::new();
		let mut derive_constant_x11_sizes = Punctuated::new();
		let mut derive_writables = Punctuated::new();
		let mut derive_readables = Punctuated::new();
		let mut derive_readable_with_contexts = Punctuated::new();

		while input.peek(Token![#]) && input.peek2(token::Bracket) {
			let content;

			let hash_token = input.parse()?;
			let bracket_token = bracketed!(content in input);
			let path = content.parse::<Path>()?;

			if path.is_ident("derive") {
				let inner;

				let paren_token = parenthesized!(inner in content);
				let mut paths = Punctuated::new();

				while !inner.is_empty() {
					let path = inner.parse::<Path>()?;

					let comma = if inner.peek(Token![,]) {
						Some(inner.parse()?)
					} else {
						None
					};
					let is_comma = comma.is_some();

					if path.is_ident("X11Size") {
						derive_x11_sizes.push_value(path);

						if let Some(comma) = comma {
							derive_x11_sizes.push_punct(comma);
						}
					} else if path.is_ident("ConstantX11Size") {
						derive_constant_x11_sizes.push_value(path);

						if let Some(comma) = comma {
							derive_constant_x11_sizes.push_punct(comma);
						}
					} else if path.is_ident("Writable") {
						derive_writables.push_value(path);

						if let Some(comma) = comma {
							derive_writables.push_punct(comma);
						}
					} else if path.is_ident("Readable") {
						derive_readables.push_value(path);

						if let Some(comma) = comma {
							derive_readables.push_punct(comma);
						}
					} else if path.is_ident("ReadableWithContext") {
						derive_readable_with_contexts.push_value(path);

						if let Some(comma) = comma {
							derive_readable_with_contexts.push_punct(comma);
						}
					} else {
						paths.push(path);

						if let Some(comma) = comma {
							paths.push_punct(comma);
						}
					}

					if !is_comma {
						break;
					}
				}

				attributes.push(Attribute {
					pound_token: hash_token,
					style: AttrStyle::Outer,
					bracket_token,
					path,

					tokens: TokenStream2::with_tokens(|tokens| {
						paren_token.surround(tokens, |tokens| {
							paths.to_tokens(tokens);
						});
					}),
				})
			} else {
				attributes.push(Attribute {
					pound_token: hash_token,
					style: AttrStyle::Outer,
					bracket_token,
					path,

					tokens: content.parse()?,
				})
			}
		}

		Ok(Self {
			attributes,

			derive_x11_sizes,
			derive_constant_x11_sizes,
			derive_writables,
			derive_readables,
			derive_readable_with_contexts,
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
		} else if look.peek(token::Brace) {
			// Curly brackets.

			let content;

			Self::Brace {
				brace_token: braced!(content in input),
				source: content.parse_with(((let_map, Some(field_map)), definition_type))?,
			}
		} else if look.peek(token::Bracket) {
			// Square brackets.

			let content;

			Self::Bracket {
				bracket_token: bracketed!(content in input),
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
