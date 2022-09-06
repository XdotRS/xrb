// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{
	parenthesized, token, Attribute, Generics, Ident, LitInt, Result, Token, Type, Visibility,
};
use syn::punctuated::Punctuated;

use proc_macro2::{Delimiter, Group, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};

use crate::content::*;

/// A request or a reply generated from a request.
#[derive(Clone)]
pub struct Message {
	/// Attributes that will be associated with this message's `struct`.
	///
	/// This includes doc comments.
	pub attributes: Vec<Attribute>,
	/// The visibility that will be associated with this message's `struct`.
	pub vis: Visibility,
	pub struct_token: Token![struct],
	/// The name of the message.
	pub name: Ident,
	/// Generics. Ex: `<'a, T>`.
	pub generics: Generics,
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
		let mut content = TokenStream2::new();
		let mut fields: Punctuated<Field, Token![,]> = Punctuated::new();

		match &self.content {
			// If this is a shorthand definition and it has a field, write that
			// field's definition to `fields`.
			Content::Shorthand(shorthand) => {
				shorthand.field().map(|field| fields.push(field));
			}

			// If this is a longhand definition, write the definitions of any
			// and all fields to `fields`.
			Content::Longhand(longhand) => {
				for field in longhand.fields() {
					fields.push(field.clone());
				}
			}
		}

		// If this is a reply, append `major_opcode`, `minor_opcode`, and
		// `sequence` fields for the `Reply` trait implementation.
		match self.metadata {
			Metadata::Reply(_) => {
				quote! {
					major_op: u8,
					minor_op: u8,
					sequence_num: u16,
				}
				.to_tokens(&mut content);
			}
			_ => (),
		}

		fields.to_tokens(&mut content);

		// Surround the fields with `{` and `}` and append them to `tokens`.
		tokens.append(Group::new(Delimiter::Brace, content));
	}
}

impl Message {
	pub fn message_trait_impl(&self) -> TokenStream2 {
		let name = &self.name;

		let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

		match &self.metadata {
			Metadata::Request(request) => {
				let item_size = match &self.content {
					Content::Longhand(longhand) => longhand.items.iter().map(|item| item.size()).collect(),
					Content::Shorthand(shorthand) => {
						vec![shorthand.item.clone().map_or(quote!(0), |def| def.1.size())]
					}
				};

				let reply = request.reply.clone().map(|reply| reply.1);

				let major = request.major_opcode;
				let minor = match request.minor_opcode {
					Some((_, minor)) => quote!(Some(#minor)),
					None => quote!(None),
				};

				quote! {
					impl #impl_generics crate::Request<#reply> for #name #ty_generics #where_clause {
						fn major_opcode() -> u8 {
							#major
						}

						fn minor_opcode() -> Option<u8> {
							#minor
						}

						fn length(&self) -> u16 {
							1u16 #(+ ((#item_size) as u16))*
						}
					}
				}
			}
			Metadata::Reply(reply) => {
				let item_size = match &self.content {
					Content::Longhand(longhand) => longhand.items.iter().map(|item| item.size()).collect(),
					Content::Shorthand(shorthand) => {
						vec![shorthand.item.clone().map_or(quote!(0), |def| def.1.size())]
					}
				};

				let request_ty = &reply.request.1;

				quote! {
					impl #impl_generics crate::Reply<#request_ty> for #name #ty_generics #where_clause {
						fn sequence(&self) -> u16 {
							self.sequence_num
						}

						fn major_opcode(&self) -> u8 {
							self.major_op
						}

						fn minor_opcode(&self) -> u8 {
							self.minor_op
						}

						fn length(&self) -> u32 {
							0u32 #(+ ((#item_size) as u32))*
						}
					}
				}
			}
		}
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
	pub paren_token: token::Paren,
	/// The major opcode of this request.
	pub major_opcode: u8,
	/// The minor opcode of this request, if any. Only used in extensions.
	pub minor_opcode: Option<(Token![,], u8)>,
	/// The type of reply that is returned by this request, if any.
	pub reply: Option<(Token![->], Type)>,
}

/// Information specifically associated with replies, not requests.
#[derive(Clone)]
pub struct ReplyMetadata {
	/// The request which this reply is returned for.
	pub request: (Token![for], Type),
}

// Parsing {{{

impl Parse for Message {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			attributes: input.call(Attribute::parse_outer)?,
			vis: input.parse()?,
			struct_token: input.parse()?,
			// Parse the message's name.
			name: input.parse()?,
			// Parse any generic definitions, e.g. `'a` or `T`.
			generics: input.parse()?,
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

		Ok(Self {
			// `(` and `)`.
			paren_token: parenthesized!(content in input),
			// Major opcode.
			major_opcode: content.parse::<LitInt>()?.base10_parse()?,
			// Optional: `,` + minor opcode.
			minor_opcode: content
				.parse() // ,
				.ok()
				.map(|comma| (comma, content.parse::<LitInt>().unwrap().base10_parse().unwrap())),
			// Optional: `->` + reply type.
			reply: input.parse().ok().zip(input.parse().ok()),
		})
	}
}

impl Parse for ReplyMetadata {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// `for` + request type.
			request: (input.parse()?, input.parse()?),
		})
	}
}

// }}}
