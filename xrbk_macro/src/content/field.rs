// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use std::collections::HashMap;
use syn::{parse::ParseStream, Error, Ident, Result, Token, Type, Visibility};

use super::{AttrContent, Attribute, Context};

pub struct Field {
	pub attributes: Vec<Attribute>,
	pub vis: Visibility,
	pub ident: Option<Ident>,
	pub colon_token: Option<Token![:]>,
	pub r#type: Type,
}

impl Field {
	/// Returns whether this field has a name.
	#[allow(dead_code)]
	pub const fn is_named(&self) -> bool {
		self.ident.is_some() && self.colon_token.is_some()
	}

	/// Returns whether this field does not have a name.
	pub const fn is_unnamed(&self) -> bool {
		self.ident.is_none() && self.colon_token.is_none()
	}

	/// Returns whether this field as a context attribute.
	#[allow(dead_code)]
	pub fn has_context(&self) -> bool {
		self.attributes
			.iter()
			.any(|attr| attr.is_context())
	}

	/// Gets the context of this field if it has a context attribute.
	#[allow(dead_code, clippy::borrowed_box)]
	pub fn context(&self) -> Option<&Box<Context>> {
		self.attributes.iter().find_map(|attr| match &attr.content {
			AttrContent::Context(_, context) => Some(context),
			_ => None,
		})
	}
}

// Expansion {{{

impl ToTokens for Field {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Convert every attribute (other than context attributes) on this field
		// to tokens.
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// Convert the field's visibility to tokens.
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

impl Field {
	fn parse(input: ParseStream, map: &HashMap<Ident, Type>) -> Result<Self> {
		let attributes = Attribute::parse_outer(input, map)?;
		let vis = input.parse()?;
		let ident = input.parse().ok();
		let colon_token = ident.as_ref().and_then(|_| input.parse().ok());
		let r#type = input.parse()?;

		Ok(Self {
			attributes,
			vis,
			ident,
			colon_token,
			r#type,
		})
	}

	pub fn parse_named(input: ParseStream, map: &HashMap<Ident, Type>) -> Result<Self> {
		let field = Self::parse(input, map)?;

		// If this field does not have a name, generate an error:
		if field.is_unnamed() {
			return Err(input.error("expected named field"));
		}

		Ok(field)
	}

	pub fn parse_unnamed(input: ParseStream, map: &HashMap<Ident, Type>) -> Result<Self> {
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
