// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod reply;
mod request;

pub use reply::*;
pub use request::*;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{token, Attribute, Generics, Ident, Result, Token, Visibility};

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

#[derive(Clone)]
pub enum Metadata {
	Request(RequestMetadata),
	Reply(ReplyMetadata),
}

// Expansion {{{

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

		// Punctuate `self.content.fields()` with commas.
		let fields: Punctuated<_, Token![,]> = self.content.fields().into_iter().collect();

		let mut content = TokenStream2::new();

		// If this is a reply, append `major_opcode`, `minor_opcode`, and
		// `sequence` fields for the `Reply` trait implementation.
		if let Metadata::Reply(_) = self.metadata {
			quote! {
				/// The sequence number associated with the request that
				/// generated this reply.
				///
				/// This is generated in the implementation of
				/// [`Reply::sequence()`].
				///
				/// [`Reply::sequence()`]: crate::Reply::sequence
				__sequence: u16,
				/// The major opcode, if any, associated with the request
				/// that generated this reply.
				///
				/// This is generated in the implementation of
				/// [`Reply::major_opcode()`].
				///
				/// [`Reply::major_opcode()`]: crate::Reply::major_opcode
				__major_opcode: Option<u8>,

				// TODO: Opt-in to minor opcodes? Surely those replies that
				// define their metabyte position cannot have minor opcodes? And
				// surely replies that are only generated for requests that do
				// not have minor opcodes cannot have minor opcodes?
				//
				// I imagine the best way to check for this is to check whether
				// the associated request has a minor opcode...? Though, I guess
				// that might require keeping track of the requests at compile
				// time, since we can't make use of its `Request` trait impl
				// afaik.
				/// The minor opcode, if any, associated with the request
				/// that generated this reply.
				///
				/// This is generated in the implementation of
				/// [`Reply::minor_opcode()`].
				///
				/// [`Reply::minor_opcode()`]: crate::Reply::minor_opcode
				__minor_opcode: Option<u8>,
			}
			.to_tokens(&mut content);
		}

		fields.to_tokens(&mut content);

		// Surround the fields with `{` and `}` and append them to `tokens`.
		tokens.append(Group::new(Delimiter::Brace, content));
	}
}

impl Message {
	/// Generate an implementation for the trait associated with this type of
	/// message.
	pub fn message_trait_impl(&self) -> TokenStream2 {
		let name = &self.name;

		let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

		match &self.metadata {
			Metadata::Request(request) => {
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
							1u16 // TODO: actually calculate this
						}
					}
				}
			}
			Metadata::Reply(reply) => {
				let request_ty = &reply.request.1;

				quote! {
					impl #impl_generics crate::Reply<#request_ty> for #name #ty_generics #where_clause {
						fn sequence(&self) -> u16 {
							self.__sequence
						}

						fn major_opcode(&self) -> Option<u8> {
							self.__major_opcode
						}

						fn minor_opcode(&self) -> Option<u8> {
							self.__minor_opcode
						}

						fn length(&self) -> u32 {
							0u32 // TODO: actually calculate this
						}
					}
				}
			}
		}
	}
}

// }}}

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

// }}}
