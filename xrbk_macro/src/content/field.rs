// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use std::collections::HashMap;
use syn::{parse::ParseStream, Error, Ident, Result, Token, Type, Visibility};

use super::{AttrContent, Attribute, Context};

pub struct Field<'a> {
	pub attributes: Vec<Attribute<'a>>,
	pub vis: Visibility,
	pub ident: Option<Ident>,
	pub colon_token: Option<Token![:]>,
	pub r#type: Type,
}

impl<'a> Field<'a> {
	/// Returns whether this field has a name.
	pub const fn is_named(&self) -> bool {
		self.ident.is_some() && self.colon_token.is_some()
	}

	/// Returns whether this field does not have a name.
	pub const fn is_unnamed(&self) -> bool {
		self.ident.is_none() && self.colon_token.is_none()
	}

	/// Returns whether this field as a context attribute.
	pub const fn has_context(&self) -> bool {
		self.attributes
			.iter()
			.find(|attr| attr.is_context())
			.is_some()
	}

	/// Gets the context of this field if it has a context attribute.
	pub const fn context(&self) -> Option<Context<'a>> {
		self.attributes.iter().find_map(|attr| match attr.content {
			AttrContent::Context(_, context) => Some(context),
			_ => None,
		})
	}
}

// Expansion {{{

impl ToTokens for Field<'_> {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Convert every attribute (other than context attributes) on this field
		// to tokens.
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// Convert the field's visibiltiy to tokens.
		self.vis.to_tokens(tokens);
		// Convert the field's name to tokens.
		self.ident.to_tokens(tokens);
		// Convert the colon token between the field's name and its type to
		// tokens.
		self.colon_token.to_tokens(tokens);
		// Convert the field's type to tokens.
		self.r#type.to_tokens(tokens);
	}
}

// }}}

// Parsing {{{

impl Field<'_> {
	fn parse(input: ParseStream, map: HashMap<Ident, Type>) -> Result<Self> {
		let attributes = Attribute::parse_outer(input, map)?;
		let vis = input.parse()?;
		let ident = input.parse().ok();
		let colon_token = ident.map(|_| input.parse().ok()).flatten();
		let r#type = input.parse()?;

		Ok(Self {
			attributes,
			vis,
			ident,
			colon_token,
			r#type,
		})
	}

	pub fn parse_named(input: ParseStream, map: HashMap<Ident, Type>) -> Result<Self> {
		let field = Self::parse(input, map)?;

		// If this field does not have a name, generate an error:
		if field.is_unnamed() {
			return Err(input.error("expected named field"));
		}

		Ok(field)
	}

	pub fn parse_unnamed(input: ParseStream, map: HashMap<Ident, Type>) -> Result<Self> {
		let field = Self::parse(input, map)?;

		// If this field has a name, generate an error:
		if field.is_named() {
			return Err(Error::new(
				field.ident.unwrap().span(),
				"expected unnamed field",
			));
		}

		Ok(field)
	}
}

// }}}
