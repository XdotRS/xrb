// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Ident, Result, Token, Visibility};

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, TokenStreamExt};

#[derive(Clone)]
pub struct Message {
	/// Attributes that will be associated with this message's `struct`.
	///
	/// This includes doc comments.
	pub attributes: Vec<Attribute>,
	/// The visibility that will be associated with this message's `struct`.
	pub vis: Option<Visibility>,
	/// The name of the message.
	pub name: Ident,
	/// The metadata relevant to this particular type of message.
	///
	/// Requests and replies have different metadata associated with them.
	pub metadata: Metadata,
	/// The content definition of this message.
	///
	/// This is used for the (de)serialization of the message and its `struct`
	/// fields. Not all of the message content is fields: unused bytes and
	/// lengths of list fields are only used for (de)serialization.
	pub content: Content,
}

impl ToTokens for Message {
	// This writes the `struct` definition of this message.
	//
	// It doesn't include the message metadata, and only the fields of the
	// `content` are written.
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Attributes
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// Visibility
		self.vis.to_tokens(tokens);
		// `struct` is actually a special [`Ident`]: append the `struct` keyword.
		tokens.append(Ident::new("struct", Span::call_site()));
		// Name
		self.name.to_tokens(tokens);
		// Fields
		self.content.fields_to_tokens(tokens);
	}
}

#[derive(Clone)]
pub enum Metadata {
	Request(RequestMetadata),
	Reply(ReplyMetadata),
}

#[derive(Clone)]
pub struct Content;

impl Content {
	pub fn fields_to_tokens(&self, _tokens: &mut TokenStream2) {}
}

#[derive(Clone)]
pub struct RequestMetadata;
#[derive(Clone)]
pub struct ReplyMetadata;

impl Parse for Message {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse attributes for the message.
		let attributes: Vec<Attribute> = input.call(Attribute::parse_outer)?;
		// Parse the message's visibility.
		let vis: Option<Visibility> = input.parse().ok();
		// Require a `struct` token to precede the `name`.
		input.parse::<Token![struct]>()?;

		Ok(Self {
			attributes,
			vis,
			// Parse the message's name.
			name: input.parse()?,
			// Parse the message's metadata (specific to the type of message).
			metadata: input.parse()?,
			// Parse the message's content. Includes fields, unused bytes, etc.
			content: input.parse()?,
		})
	}
}

impl Parse for Metadata {
	fn parse(_input: ParseStream) -> Result<Self> {
		Ok(Self::Request(RequestMetadata))
	}
}

impl Parse for Content {
	fn parse(_input: ParseStream) -> Result<Self> {
		Ok(Self)
	}
}
