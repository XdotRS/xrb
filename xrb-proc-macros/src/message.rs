// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, token, Attribute, Ident, LitInt, Result, Token, Type, Visibility, Generics};

use proc_macro2::{Delimiter, Group, Span, TokenStream as TokenStream2};
use quote::{ToTokens, TokenStreamExt};

use crate::content::Content;

/// A request or a reply generated from a request.
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
	pub generics: Option<Generics>,
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
		// Generics
		self.generics.to_tokens(tokens);
		// Fields
		Group::new(Delimiter::Brace, self.content.fields_to_tokenstream()).to_tokens(tokens);
	}
}

/// Specific metadata about the message, depending on whether it is a request or
/// a reply.
#[derive(Clone)]
pub enum Metadata {
	/// Information specific to a request.
	///
	/// Requries a major opcode, and optionally allows a minor opcode and reply
	/// type declaration.
	Request(RequestMetadata),
	/// Information specific to a reply.
	///
	/// Requries a request type declaration.
	Reply(ReplyMetadata),
}

/// Information specifically associated with requests, not replies.
#[derive(Clone)]
pub struct RequestMetadata {
	/// The major opcode of this request.
	pub major_opcode: u8,
	/// The minor opcode of this request, if any. Only used in extensions.
	pub minor_opcode: Option<u8>,
	/// The type of reply that is returned by this request, if any.
	pub reply: Option<Type>,
}

/// Information specifically associated with replies, not requests.
#[derive(Clone)]
pub struct ReplyMetadata {
	/// The request which this reply is returned for.
	pub request: Type,
}

// Parsing {{{

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
			// Parse any generic definitions, e.g. `'a` or `T`.
			generics: input.parse().ok(),
			// Parse the message's metadata (specific to the type of message).
			metadata: input.parse()?,
			// Parse the message's content. Includes fields, unused bytes, etc.
			content: input.parse()?,
		})
	}
}

impl Parse for Metadata {
	fn parse(input: ParseStream) -> Result<Self> {
		let look = input.lookahead1();

		if look.peek(token::Paren) {
			// If `(` is next, map this as a request.
			input.parse().map(Metadata::Request)
		} else if look.peek(Token![for]) {
			// If `for` is next, map this as a reply.
			input.parse().map(Metadata::Reply)
		} else {
			// Construct an error along the lines of 'expected `(` or `for`'.
			Err(look.error())
		}
	}
}

impl Parse for RequestMetadata {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parnetheses (`(` and `)`) for the opcodes.
		let content;
		parenthesized!(content in input);

		Ok(Self {
			// Major opcode (required)
			major_opcode: content.parse::<LitInt>()?.base10_parse()?,
			// Minor opcode (optional, requires a comma token preceding it if so)
			minor_opcode: content.parse::<Token![,]>().ok().and(
				content
					.parse::<LitInt>()
					.ok()
					.map(|lit| lit.base10_parse().ok())
					.flatten(),
			),
			// Reply type (if `->` is not given, it is `None`, but if `->` is
			// given then panic if the type is not also given).
			reply: input
				.parse::<Token![->]>()
				.ok()
				.map(|_| input.parse::<Type>().unwrap()),
		})
	}
}

impl Parse for ReplyMetadata {
	fn parse(input: ParseStream) -> Result<Self> {
		// Require the `for` token for the request type declaration
		// in replies.
		input.parse::<Token![for]>()?;

		Ok(Self {
			// Require the request type.
			request: input.parse()?,
		})
	}
}

// }}}
