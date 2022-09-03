// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{Result, braced, Attribute, Visibility, Ident, Type, Token, Expr, token, bracketed, parenthesized, LitInt};
use syn::punctuated::Punctuated;

use proc_macro2::{TokenStream as TokenStream2, Span, Group, Delimiter, Punct, Spacing};
use quote::{ToTokens, TokenStreamExt};

#[derive(Clone)]
pub struct Content {
	pub metabyte: Option<Item>,
	pub items: Vec<Item>,
}

#[derive(Clone)]
pub enum Item {
	UnusedBytes(UnusedByteSize),
	FieldLength(Ident, Option<Type>),
	Field {
		attributes: Vec<Attribute>,
		vis: Option<Visibility>,
		name: Ident,
		ty: Type,
		enum_definition: Option<Enum>,
	},
}

#[derive(Clone)]
pub enum UnusedByteSize {
	Number(u8),
	Padding(Ident),
}

#[derive(Clone)]
pub struct Enum {
	pub name: Ident,
	pub variants: Punctuated<Variant, Token![,]>,
}

impl ToTokens for Enum {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		tokens.append(Ident::new("pub", Span::call_site()));
		tokens.append(Ident::new("enum", Span::call_site()));
		self.name.to_tokens(tokens);
		tokens.append(Group::new(Delimiter::Brace, self.variants.to_token_stream()));
	}
}

#[derive(Clone)]
pub struct Variant {
	pub name: Ident,
	pub value: Expr,
}

impl ToTokens for Variant {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.name.to_tokens(tokens);
		tokens.append(Punct::new('=', Spacing::Alone));
		self.value.to_tokens(tokens);
	}
}

// Parsing {{{

impl Parse for Content {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;
		braced!(content in input);

		let mut metabyte: Option<Item> = None;
		let mut items: Vec<Item> = vec![];

		while !content.is_empty() {
			if metabyte.is_none() && content.peek(Token![$]) {
				// If the metabyte is not already set and there is a `$` token,
				// read the metabyte item.
				content.parse::<Token![$]>()?;
				metabyte = content.parse().ok();
			} else {
				// Otherwise, read the item like normal...
				items.push(content.parse()?);
			}

			// Require a comma after every item, even the last one, for simplicity.
			input.parse::<Token![,]>()?;
		}

		Ok(Self {
			metabyte,
			items,
		})
	}
}

impl Parse for Item {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(Token![#]) {
			input.parse::<Token![#]>()?;
			let name: Ident = input.parse()?;

			let ty = if input.peek(token::Bracket) {
				let content;
				bracketed!(content in input);

				Some(content.parse::<Type>()?)
			} else {
				None
			};

			Ok(Self::FieldLength(name, ty))
		} else if input.peek(token::Paren) {
			let content;
			parenthesized!(content in input);

			assert!(content.is_empty());

			let content;
			bracketed!(content in input);

			let look = content.lookahead1();

			if look.peek(LitInt) {
				Ok(Self::UnusedBytes(
					UnusedByteSize::Number(
						content.parse::<LitInt>()?.base10_parse()?
					)
				))
			} else if look.peek(Ident) && content.parse::<Ident>()?.to_string() == "padding" {
				let param;
				parenthesized!(param in content);

				Ok(Self::UnusedBytes(UnusedByteSize::Padding(param.parse()?)))
			} else {
				Err(look.error())
			}
		} else {
			let attributes = input.call(Attribute::parse_outer)?;
			let vis: Option<Visibility> = input.parse::<Visibility>().ok();
			let name: Ident = input.parse()?;
			input.parse::<Token![:]>()?;

			let mut enum_def: Option<Enum> = None;
			let look = input.lookahead1();

			let ty = if look.peek(Token![enum]) {
				enum_def = Some(input.parse::<Enum>()?);

				Type::Verbatim(enum_def.clone().map(|en| en.name).unwrap().to_token_stream())
			} else {
				input.parse()?
			};

			Ok(Self::Field {
				attributes,
				vis,
				name,
				ty,
				enum_definition: enum_def,
			})
		}
	}
}

impl Parse for Enum {
	fn parse(input: ParseStream) -> Result<Self> {
		// enum Name { Variant = 0 }

		input.parse::<Token![enum]>()?;
		let name: Ident = input.parse()?;

		let content;
		braced!(content in input);

		let variants: Punctuated<Variant, Token![,]> = Punctuated::parse_terminated(&content)?;

		Ok(Self {
			name,
			variants,
		})
	}
}

impl Parse for Variant {
	fn parse(input: ParseStream) -> Result<Self> {
		// Variant = 0

		let name: Ident = input.parse()?;
		input.parse::<Token![=]>()?;
		let value: Expr = input.parse()?;

		Ok(Self { name, value })
	}
}

// }}}
