// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::{
	braced, parenthesized,
	parse::{Parse, ParseStream},
	token, Attribute, Error, Expr, Generics, Ident, Result, Token, Type, Visibility,
};

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::punctuated::Punctuated;

use crate::Items;

/// A list of [`Definition`]s.
pub struct Definitions(pub Vec<Definition>);

/// An [`Enum`] or [`Struct`] definition.
pub enum Definition {
	/// An [`Enum`] `Definition`.
	Enum(Box<Enum>),

	/// A [`Struct`] `Definition`.
	Struct(Box<Struct>),
}

/// A definition, as defined with the [`define!`] macro, for ordinary structs
/// and messages.
///
/// [`define!`]: crate::define
pub struct Struct {
	/// The metadata associated with the definition.
	///
	/// This defines the type of definition (i.e. enum, struct, event, request,
	/// or reply), as well as the additional information and tokens that starts
	/// that definition (`enum`, `struct`, the name, generics, the major opcode
	/// of a request, etc.).
	pub metadata: StructMetadata,

	/// The items defined within the definition.
	///
	/// This is the main feature of the [`define!`] macro: it's what allows
	/// additional serialization and deserialization code to be generated in a
	/// more concise way than could be achieved with a derive macro.
	pub items: Items,

	/// A semicolon token if `items` is [`Items::Unit`] or [`Items::Unnamed`].
	pub semicolon_token: Option<Token![;]>,
}

/// The type of definition and metadata associated with it.
pub enum StructMetadata {
	/// An ordinary struct definition.
	Struct(BasicStructMetadata),

	/// An event message struct.
	Event(Event),

	/// A request message struct.
	Request(Box<Request>),

	/// A reply message struct.
	Reply(Reply),
}

/// The definition of an enum.
pub struct Enum {
	/// Attributes associated with the enum, including doc comments.
	pub attributes: Vec<Attribute>,

	/// The visibility of the enum.
	pub vis: Visibility,
	/// The enum token: `enum`.
	pub enum_token: Token![enum],
	/// The name of the enum.
	pub ident: Ident,
	/// Generics (lifetimes and/or generic types) associated with the enum.
	pub generics: Generics,

	/// A pair of curly brackets (`{` and `}`) surrounding the enum variants.
	pub brace_token: token::Brace,
	/// The enum variants defined within the enum.
	pub variants: Punctuated<Variant, Token![,]>,
}

/// The definition of an enum variant.
pub struct Variant {
	/// Attributes associated with the enum variant, including doc comments.
	pub attributes: Vec<Attribute>,

	/// The name of the enum variant.
	pub ident: Ident,
	/// [`Items`] defined within the enum variant, if any.
	pub items: Items,

	/// An optional discriminant for the enum variant (this is used to
	/// serialize and deserialize the enum variant).
	pub discriminant: Option<(Token![=], Expr)>,
}

/// Metadata for a basic struct.
pub struct BasicStructMetadata {
	/// Attributes associated with the struct, including doc comments.
	pub attributes: Vec<Attribute>,
	/// The visibility of the struct.
	pub vis: Visibility,
	/// The struct token: `struct`.
	pub struct_token: Token![struct],
	/// The name of the struct.
	pub name: Ident,
	/// Generics (lifetimes and./or generic types) associated with the struct.
	pub generics: Generics,
}

/// Metadata for an event struct.
pub struct Event {
	/// Attributes associated with the event's struct, including doc comments.
	pub attributes: Vec<Attribute>,

	/// The visibility of the event's struct.
	pub vis: Visibility,
	/// The struct token: `struct`.
	pub struct_token: Token![struct],
	/// The name of the event.
	pub name: Ident,
	/// Generics (lifetimes and/or generic types) associated with the event's
	/// struct.
	pub generics: Generics,

	/// A colon token: `:`.
	pub colon_token: Token![:],
	/// Specifies that this is an event: `Event`.
	pub event_ident: Ident,

	/// A left arrow bracket token: `<`.
	pub lt_token: Token![<],
	/// An expression that evaluates to the code associated with the event.
	pub event_code_expr: Expr,
	/// A right arrow bracket token: `>`.
	pub gt_token: Token![>],
}

/// Metadata for a request struct.
pub struct Request {
	/// Attributes associated with the request's struct, including doc comments.
	pub attributes: Vec<Attribute>,

	/// The visibility of the request's struct.
	pub vis: Visibility,
	/// The struct token: `struct`.
	pub struct_token: Token![struct],
	/// The name of the request.
	pub name: Ident,
	/// Generics (lifetimes and/or generic types) associated with the request's
	/// struct.
	pub generics: Generics,

	/// A colon token: `:`.
	pub colon_token: Token![:],
	/// Specifies that this is a request: `Request`.
	pub request_ident: Ident,

	/// A left arrow bracket token: `<`.
	pub lt_token: Token![<],
	/// An expression that evaluates to the major opcode associated with the
	/// request.
	pub major_opcode_expr: Expr,
	/// An optional comma then expression that evaluates to the minor opcode
	/// associated with the request.
	pub minor_opcode: Option<(Token![,], Expr)>,
	/// A right arrow bracket token: `>`.
	pub gt_token: Token![>],

	/// An optional arrow followed by a type representing replies generated by
	/// the request.
	pub reply_ty: Option<(Token![->], Type)>,
}

/// Metadata for a reply struct.
pub struct Reply {
	/// Attributes associated with the reply's struct.
	pub attributes: Vec<Attribute>,

	/// The visibility of the reply's struct.
	pub vis: Visibility,
	/// The struct token: `struct`.
	pub struct_token: Token![struct],
	/// The name of the reply.
	pub name: Ident,
	/// Generics (lifetimes and/or generic types) associated with the reply's
	/// struct.
	pub generics: Generics,

	/// A colon token: `:`.
	pub colon_token: Token![:],
	/// Specifies that this is a reply: `Reply`.
	pub reply_ident: Ident,

	pub paren_token: Option<token::Paren>,
	pub question_token: Option<Token![?]>,
	pub sequence_token: Option<Ident>,

	/// A for token: `for`.
	pub for_token: Token![for],
	/// The type of request that generates this reply.
	pub request_ty: Type,
}

// Expansion {{{

impl ToTokens for Definitions {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		for definition in &self.0 {
			definition.to_tokens(tokens);
		}
	}
}

impl ToTokens for Definition {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Enum(r#enum) => r#enum.to_tokens(tokens),
			Self::Struct(r#struct) => r#struct.to_tokens(tokens),
		}
	}
}

impl ToTokens for Struct {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.metadata.to_tokens(tokens);
		self.items.to_tokens(tokens);
	}
}

impl ToTokens for Enum {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Attributes on the enum.
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// The enum's visibility.
		self.vis.to_tokens(tokens);
		// `enum`
		self.enum_token.to_tokens(tokens);
		// The name of the enum.
		self.ident.to_tokens(tokens);
		// Generics associated with the enum.
		self.generics.to_tokens(tokens);

		// Surround the enum's variants with its curly brackets (`{` and `}`).
		self.brace_token.surround(tokens, |tokens| {
			for variant in &self.variants {
				variant.to_tokens(tokens);
			}
		});
	}
}

impl ToTokens for Variant {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Attributes on the variant.
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// The name of the enum variant.
		self.ident.to_tokens(tokens);
		// The `Items` defined within the enum variant, if any.
		self.items.to_tokens(tokens);

		// The enum variant's discriminant, if any.
		if let Some((equals, expr)) = &self.discriminant {
			// `=`
			equals.to_tokens(tokens);
			// The actual expression for the discriminant.
			expr.to_tokens(tokens);
		}
	}
}

impl ToTokens for StructMetadata {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Struct(meta) => meta.to_tokens(tokens),
			Self::Event(meta) => meta.to_tokens(tokens),
			Self::Request(meta) => meta.to_tokens(tokens),
			Self::Reply(meta) => meta.to_tokens(tokens),
		}
	}
}

/// Implements [`ToTokens`] for metadata.
///
/// This is simply to avoid repetitive code. We can generate the same
/// implementation for every type of metadata, because what differentiates the
/// types of metadata is information that is not used to define the struct with
/// normal Rust syntax.
///
/// # Examples
/// Basic [`Struct`]:
/// ```ignore
/// pub struct MyStruct<'a, T>
/// ```
/// Events, requests, and replies:
/// ```ignore
/// pub struct MyEvent<'a, T>
///
/// pub struct MyRequest<'a, T>
///
/// pub struct MyReply<'a, T>
/// ```
macro_rules! struct_tokens {
	(for $Type:ty) => {
		impl ToTokens for $Type {
			fn to_tokens(&self, tokens: &mut TokenStream2) {
				// Attributes.
				for attribute in &self.attributes {
					attribute.to_tokens(tokens);
				}

				// Visibility.
				self.vis.to_tokens(tokens);
				// `struct`.
				self.struct_token.to_tokens(tokens);
				// The name of the struct.
				self.name.to_tokens(tokens);
				// The generics associated with the struct.
				self.generics.to_tokens(tokens);
			}
		}
	};
}

// Struct metadatas
struct_tokens!(for BasicStructMetadata);
struct_tokens!(for Event);
struct_tokens!(for Request);
struct_tokens!(for Reply);

// }}}

// Parsing {{{

impl Parse for Definitions {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut definitions = vec![];

		// As long as there are still tokens left, continue to parse them as
		// definitions.
		while !input.is_empty() {
			definitions.push(input.parse()?);
		}

		Ok(Self(definitions))
	}
}

impl Parse for Definition {
	fn parse(input: ParseStream) -> Result<Self> {
		// Since all definitions start with attributes and a visibility, we
		// parse those here.
		let attributes = input.call(Attribute::parse_outer)?;
		let vis = input.parse()?;

		let look = input.lookahead1();

		if look.peek(Token![enum]) {
			// If the next token is `enum`, parse this as an `Enum`.
			Ok(Self::Enum(Box::new(Enum::parse_with(
				input, attributes, vis,
			)?)))
		} else if look.peek(Token![struct]) {
			// If the next token is `struct`, parse this as a `Struct`.
			Ok(Self::Struct(Box::new(Struct::parse_with(
				input, attributes, vis,
			)?)))
		} else {
			// Otherwise, if the next token is neither `enum` nor `struct`,
			// generate an error:
			Err(look.error())
		}
	}
}

impl Parse for Struct {
	fn parse(input: ParseStream) -> Result<Self> {
		Self::parse_with(input, input.call(Attribute::parse_outer)?, input.parse()?)
	}
}

impl Struct {
	fn parse_with(input: ParseStream, attributes: Vec<Attribute>, vis: Visibility) -> Result<Self> {
		// Parse the struct's metadata.
		let metadata = StructMetadata::parse_with(input, attributes, vis)?;
		// Parse the struct's items.
		let items: Items = input.parse()?;

		// If this is a unit struct or tuple struct, require a semicolon,
		// otherwise forbid it.
		//
		// For example:
		// ```
		// pub struct Unit;
		// pub struct Tuple(i32, i32);
		//
		// pub struct Named {
		//     x: i32,
		//     y: i32,
		// }
		// ```
		let semicolon_token: Option<Token![;]> = match items {
			Items::Unit => Some(input.parse()?),
			Items::Unnamed { .. } => Some(input.parse()?),
			Items::Named { .. } => None,
		};

		Ok(Self {
			metadata,
			items,
			semicolon_token,
		})
	}
}

impl Enum {
	fn parse_with(input: ParseStream, attributes: Vec<Attribute>, vis: Visibility) -> Result<Self> {
		let content;

		Ok(Self {
			attributes,
			vis,

			// The enum token: `enum`.
			enum_token: input.parse()?,
			// The name of the enum.
			ident: input.parse()?,
			// Generics associated with the enum.
			generics: input.parse()?,

			// A pair of curly brackets (`{` and `}`) surrounding the enum's
			// variants.
			brace_token: braced!(content in input),
			// The enum's variants.
			variants: {
				let mut variants = Punctuated::new();

				// While there are still tokens left between the enum's curly
				// brackets, continue to parse enum variants.
				while !content.is_empty() {
					// Parse a variant and push it to the list.
					variants.push_value(content.parse()?);

					// If the token after the enum variant is not a comma,
					// then there should be no more variants left, so we break
					// from the loop.
					if !content.peek(Token![,]) {
						break;
					}

					// Otherwise, if the next token is a comma, we parse it and
					// add it to the punctuated list of variants.
					variants.push_punct(content.parse()?);
				}

				variants
			},
		})
	}
}

impl Parse for Enum {
	fn parse(input: ParseStream) -> Result<Self> {
		Self::parse_with(input, input.call(Attribute::parse_outer)?, input.parse()?)
	}
}

impl Parse for Variant {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// Parse attributes associated with the enum variant.
			attributes: input.call(Attribute::parse_outer)?,

			ident: input.parse()?,
			// Items associated with the enum variant.
			items: input.parse()?,

			// If the next token is an equals sign, parse the discriminant.
			discriminant: if input.peek(Token![=]) {
				Some((input.parse()?, input.parse()?))
			} else {
				None
			},
		})
	}
}

impl Parse for StructMetadata {
	fn parse(input: ParseStream) -> Result<Self> {
		Self::parse_with(input, input.call(Attribute::parse_outer)?, input.parse()?)
	}
}

impl StructMetadata {
	fn parse_with(input: ParseStream, attributes: Vec<Attribute>, vis: Visibility) -> Result<Self> {
		// All 'struct-based' definitions start with `struct`, a name, and
		// optional generics, so we can parse those straight away.
		let struct_token: Token![struct] = input.parse()?;
		let name: Ident = input.parse()?;
		let generics: Generics = input.parse()?;

		if !input.peek(Token![:]) {
			// If the next token is _not_ a colon, then this is just a
			// simple struct definition - requests, replies, and events have
			// a colon followed by which type of message they are.
			Ok(Self::Struct(BasicStructMetadata {
				// Attributes.
				attributes,
				// Visibility.
				vis,
				// `struct`.
				struct_token,
				// The name of the struct.
				name,
				// Generics associated with the struct.
				generics,
			}))
		} else {
			// All 'message' definitions (requests, replies, events) have a
			// colon, followed by an identifier which specifies which type
			// of message it is, so we read those at the start.
			let colon_token: Token![:] = input.parse()?;
			let message_ty_ident: Ident = input.parse()?;

			match message_ty_ident.to_string().as_str() {
				// "Event" => parse event metadata
				"Event" => Ok(Self::Event(Event {
					// Attributes.
					attributes,
					// Visibility.
					vis,
					// `struct`.
					struct_token,

					// The name of the event.
					name,
					// Generics associated with the event struct.
					generics,

					// `:`.
					colon_token,
					// `Event`.
					event_ident: message_ty_ident,

					// `<`.
					lt_token: input.parse()?,

					// An expression that evaluates to the event's code.
					event_code_expr: input.parse()?,

					// `>`.
					gt_token: input.parse()?,
				})),

				// "Request" => parse request metadata
				"Request" => Ok(Self::Request(Box::new(Request {
					// Attributes.
					attributes,
					// Visibility.
					vis,
					// `struct`.
					struct_token,

					// The name of the request.
					name,
					// Generics associated with the request struct.
					generics,

					// `:`.
					colon_token,
					// `Request`.
					request_ident: message_ty_ident,

					// `<`.
					lt_token: input.parse()?,

					// An expression that evaluates to the request's major
					// opcode.
					major_opcode_expr: input.parse()?,
					// An optional expression (preceded by a comma) that
					// evaluates to the request's minor opcode.
					minor_opcode: {
						// If the next token can be successfully parsed as a
						// comma...
						if let Ok(comma) = input.parse::<Token![,]>() {
							// Then evaluate `minor_opcode` to that comma and
							// a minor opcode expression.
							Some((comma, input.parse()?))
						} else {
							// Otherwise, if there is no comma, there is no
							// minor opcode.
							None
						}
					},

					// `>`.
					gt_token: input.parse()?,

					// Optional: `->` followed by a type that specifies a
					// type of reply generated by this request.
					reply_ty: {
						// If the next token can be successfully parsed as an
						// arrow...
						if let Ok(arrow) = input.parse::<Token![->]>() {
							// Then evaluate `reply_ty` to that arrow and a
							// reply type.
							Some((arrow, input.parse()?))
						} else {
							// Otherwise, if there is no arrow, there is no reply type.
							None
						}
					},
				}))),

				// "Reply" => parse reply metadata
				"Reply" => Ok({
					let content;

					let (paren_token, (question_token, sequence_token)) = {
						// If there is a normal bracket after the message type
						// identifier...
						if input.peek(token::Paren) {
							// Parse the normal bracket pair (`(` and `)`) to
							// `paren_token`.
							let paren_token = Some(parenthesized!(content in input));

							// Attempt to parse a question mark token.
							let question_token = content.parse().ok();
							// If there is a question mark token, require
							// `sequence` to follow it in order to opt out of
							// the `sequence` field.
							let sequence_token = if question_token.is_some() {
								let ident: Ident = content.parse()?;

								// If the identifier following the question
								// mark token is not `sequence`, return an
								// error.
								if ident != "sequence" {
									return Err(Error::new(
										ident.span(),
										"expected `sequence` after `?` to opt out of the `sequence` field",
									));
								}

								Some(ident)
							} else {
								// Otherwise, if there is no question mark
								// token, then don't parse the `sequence`
								// identifier either.
								None
							};

							(paren_token, (question_token, sequence_token))
						} else {
							// Otherwise, if there is no pair of normal
							// brackets, then do not parse `paren_token`,
							// `question_token`, nor `sequence_token`.
							(None, (None, None))
						}
					};

					Self::Reply(Reply {
						// Attributes.
						attributes,

						// Visibility.
						vis,
						// `struct`.
						struct_token,

						// The name of the reply struct.
						name,
						// Generics associated with the reply struct.
						generics,

						// `:`.
						colon_token,
						// `Reply`.
						reply_ident: message_ty_ident,

						// `(` and `)`.
						paren_token,
						// Optional: `?`.
						question_token,
						// Following `question_token` if Some: `sequence`.
						sequence_token,

						// `for`.
						for_token: input.parse()?,
						// The type of the request.
						request_ty: input.parse()?,
					})
				}),

				// Otherwise, if the identifier following the colon is not
				// `Event`, `Request`, nor `Reply`, then we generate an
				// error over the identifier.
				_ => Err(Error::new(
					message_ty_ident.span(),
					"expected a message type of `Event`, `Request`, or `Reply`",
				)),
			}
		}
	}
}

// }}}
