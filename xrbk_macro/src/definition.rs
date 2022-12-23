// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod expansion;
mod parsing;

use syn::{
	braced,
	parenthesized,
	token,
	Attribute,
	Error,
	Expr,
	Generics,
	Ident,
	Result,
	Token,
	Type,
	Visibility,
};

use quote::quote;
use syn::punctuated::Punctuated;

use crate::element::Content;

/// Multiple [`Definition`]s.
///
/// > **<sup>Syntax</sup>**\
/// > _Definitions_ :\
/// > &nbsp;&nbsp; [_Definition_]<sup>\*</sup>
/// >
/// > [_Definition_]: Definition
pub struct Definitions(Vec<Definition>);

/// A definition within the `define!` macro.
///
/// > **<sup>Syntax</sup>**\
/// > _Definition_ :\
/// > &nbsp;&nbsp; _Structlike_ | [_Enum_] | [_Item_][^other]
/// >
/// > _Structlike_ :\
/// > &nbsp;&nbsp; [_Metadata_] [_Content_] `;`<sup>?</sup>[^semicolon]
/// >
/// > [_Enum_]: Enum
/// >
/// > [_Item_]: syn::Item
/// > [^other]: Except [_Struct_]s and [_Enumeration_]s.
/// >
/// > [_Metadata_]: Metadata
/// > [_Content_]: Content
/// >
/// > [^semicolon]: The semicolon is present for [`Content::Tuple`] and
/// > [`Content::Unit`], but not for [`Content::Struct`].
/// >
/// > [_Struct_]: Struct
/// > [_Enumeration_]: https://doc.rust-lang.org/reference/items/enumerations.html
pub enum Definition {
	Structlike(Metadata, Content, Option<Token![;]>),
	Enum(Enum),

	/// Any other item allowed in Rust that isn't a struct nor an enum.
	Other(syn::Item),
}

/// Metadata associated with a [`Structlike` `Definition`].
///
/// > **<sup>Syntax</sup>**\
/// > _Metadata_ :\
/// > &nbsp;&nbsp; [_Struct_] | [_Request_] | [_Reply_] | [_Event_]
/// >
/// > [_Struct_]: Struct
/// > [_Request_]: Request
/// > [_Reply_]: Reply
/// > [_Event_]: Event
///
/// [`Structlike` `Definition`]: Definition::Structlike
pub enum Metadata {
	Struct(Box<Struct>),

	Request(Box<Request>),
	Reply(Box<Reply>),
	Event(Box<Event>),
}

/// Metadata for a struct.
///
/// > **<sup>Syntax</sup>**\
/// > _Struct_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> [_Visibility_]<sup>?</sup>
/// > `struct` [IDENTIFIER] [_GenericParams_]
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [_Visibility_]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
/// > [_GenericParams_]: https://doc.rust-lang.org/reference/items/generics.html
/// > [IDENTIFIER]: https://doc.rust-lang.org/reference/identifiers.html
pub struct Struct {
	/// Attributes associated with the struct, including doc comments.
	pub attributes: Vec<Attribute>,

	/// The visibility of the struct.
	pub visibility: Visibility,
	/// The struct token: `struct`.
	pub struct_token: Token![struct],
	/// The name of the struct.
	pub ident: Ident,
	/// Generics (lifetimes and/or generic types) associated with the struct.
	pub generics: Generics,
}

/// Metadata for a request struct.
///
/// > **<sup>Syntax</sup>**\
/// > _Request_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> [_Visibility_]<sup>?</sup>
/// > `struct` [IDENTIFIER] [_GenericParams_]<sup>?</sup>\
/// > &nbsp;&nbsp; `:` `Request` `(` _Opcodes_ `)` _ReplyType_<sup>?</sup>
/// >
/// > _Opcodes_ :\
/// > &nbsp;&nbsp; [_Expression_] (`,` [_Expression_])<sup>?</sup>
/// >
/// > _ReplyType_ :\
/// > &nbsp;&nbsp; `->` [_Type_]
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [_Visibility_]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
/// > [IDENTIFIER]: https://doc.rust-lang.org/reference/identifiers.html
/// > [_GenericParams_]: https://doc.rust-lang.org/reference/items/generics.html
/// > [_Expression_]: https://doc.rust-lang.org/reference/expressions.html
/// > [_Type_]: https://doc.rust-lang.org/reference/types.html
pub struct Request {
	/// Attributes associated with the request's struct, including doc comments.
	pub attributes: Vec<Attribute>,

	/// The visibility of the request's struct.
	pub visibility: Visibility,
	/// The struct token: `struct`.
	pub struct_token: Token![struct],
	/// The name of the request.
	pub ident: Ident,
	/// Generics (lifetimes and/or generic types) associated with the request's
	/// struct.
	pub generics: Generics,

	/// A colon token: `:`.
	pub colon_token: Token![:],
	/// Specifies that this is a request: `Request`.
	pub request_token: Ident,

	/// A pair of normal brackets surrounding the opcodes: `(` and `)`.
	pub paren_token: token::Paren,
	/// An expression that evaluates to the major opcode associated with the
	/// request.
	pub major_opcode: Expr,
	/// An optional comma then expression that evaluates to the minor opcode
	/// associated with the request.
	pub minor_opcode: Option<(Token![,], Expr)>,

	/// An optional arrow followed by a type representing replies generated by
	/// the request.
	pub reply: Option<(Token![->], Type)>,
}

/// Metadata for a reply struct.
///
/// > **<sup>Syntax</sup>**\
/// > _Reply_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> [_Visibility_]<sup>?</sup>
/// > `struct` [IDENTIFIER] [_GenericParams_]<sup>?</sup>\
/// > &nbsp;&nbsp; `:` `Reply` `for` [_Type_]
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [_Visibility_]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
/// > [IDENTIFIER]: https://doc.rust-lang.org/reference/identifiers.html
/// > [_GenericParams_]: https://doc.rust-lang.org/reference/items/generics.html
/// > [_Type_]: https://doc.rust-lang.org/reference/types.html
pub struct Reply {
	/// Attributes associated with the reply's struct.
	pub attributes: Vec<Attribute>,

	/// The visibility of the reply's struct.
	pub visibility: Visibility,
	/// The struct token: `struct`.
	pub struct_token: Token![struct],
	/// The name of the reply.
	pub ident: Ident,
	/// Generics (lifetimes and/or generic types) associated with the reply's
	/// struct.
	pub generics: Generics,

	/// A colon token: `:`.
	pub colon_token: Token![:],
	/// Specifies that this is a reply: `Reply`.
	pub reply_token: Ident,

	/// A for token: `for`.
	pub for_token: Token![for],
	/// The type of request that generates this reply.
	pub request: Type,
}

/// Metadata for an event struct.
///
/// > **<sup>Syntax</sup>**\
/// > _Event_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> [_Visibility_]<sup>?</sup>
/// > `struct` [IDENTIFIER] [_GenericParams_]<sup>?</sup>\
/// > &nbsp;&nbsp; `:` `Event` `(` [_Expression_] `)`
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [_Visibility_]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
/// > [IDENTIFIER]: https://doc.rust-lang.org/reference/identifiers.html
/// > [_GenericParams_]: https://doc.rust-lang.org/reference/items/generics.html
/// > [_Expression_]: https://doc.rust-lang.org/reference/expressions.html
pub struct Event {
	/// Attributes associated with the event's struct, including doc comments.
	pub attributes: Vec<Attribute>,

	/// The visibility of the event's struct.
	pub visibility: Visibility,
	/// The struct token: `struct`.
	pub struct_token: Token![struct],
	/// The name of the event.
	pub ident: Ident,
	/// Generics (lifetimes and/or generic types) associated with the event's
	/// struct.
	pub generics: Generics,

	/// A colon token: `:`.
	pub colon_token: Token![:],
	/// Specifies that this is an event: `Event`.
	pub event_token: Ident,

	/// A pair of normal brackets: `(` and `)`.
	pub paren_token: token::Paren,
	/// An expression that evaluates to the code associated with the event.
	pub code: Expr,
}

/// An enum definition that contains [`Variant`]s which may contain
/// [`Element`]s.
///
/// > **<sup>Syntax</sup>**\
/// > _Enum_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> [_Visibility_]<sup>?</sup>
/// > `enum` [IDENTIFIER] [_GenericParams_]<sup>?</sup>
/// > [_WhereClause_]<sup>?</sup>\
/// > &nbsp;&nbsp;_Variants_
/// >
/// > _Variants_ :\
/// > &nbsp;&nbsp; `{` [_Variant_] ([_Variant_] `,`)<sup>\*</sup>
/// > `,`<sup>?</sup> `}`
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [_Visibility_]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
/// > [IDENTIFIER]: https://doc.rust-lang.org/reference/identifiers.html
/// > [_GenericParams_]: https://doc.rust-lang.org/reference/items/generics.html
/// > [_WhereClause_]: https://doc.rust-lang.org/reference/items/generics.html
/// > [_Variant_]: Variant
///
/// [`Element`]: crate::element::Element
pub struct Enum {
	/// Attributes associated with the enum.
	pub attributes: Vec<Attribute>,

	/// The visibility of the enum.
	pub visibility: Visibility,
	/// The enum token: `enum`.
	pub enum_token: Token![enum],
	/// The name of the enum.
	pub ident: Ident,
	/// Generics (lifetimes and/or generic types) associated with the enum.
	pub generics: Generics,

	/// A pair of curly brackets (`{` and `}`) surrounding the enum `variants`.
	pub brace_token: token::Brace,
	/// A list of the enum's [`Variant`]s, punctuated by commas.
	pub variants: Punctuated<Variant, Token![,]>,
}

/// An [`Enum`] variant that may contain [`Element`]s.
///
/// > **<sup>Syntax</sup>**\
/// > _Variant_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> [IDENTIFIER] [_Content_]
/// > _Discriminant_<sup>?</sup>
/// >
/// > _Discriminant_ :\
/// > &nbsp;&nbsp; `=` [_Expression_]
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [IDENTIFIER]: https://doc.rust-lang.org/reference/identifiers.html
/// > [_Content_]: Content
/// > [_Expression_]: https://doc.rust-lang.org/reference/expressions.html
///
/// [`Element`]: crate::element::Element
pub struct Variant {
	/// Attributes associated with the enum variant.
	pub attributes: Vec<Attribute>,

	/// The name of the enum variant.
	pub ident: Ident,
	/// [`Content`] associated with the enum variant.
	pub content: Content,

	/// An optional discriminant expression for the enum variant.
	pub discriminant: Option<(Token![=], Expr)>,
}

#[derive(Clone, Copy)]
pub enum DefinitionType {
	Basic,

	Request,
	Reply,
	Event,
}

impl DefinitionType {
	pub fn min_length(&self) -> Option<usize> {
		match self {
			Self::Basic => None,

			Self::Request => None,
			Self::Reply => Some(32),
			Self::Event => Some(32),
		}
	}

	pub fn length_type(&self) -> Option<Type> {
		match self {
			Self::Request => Some(Type::Verbatim(quote!(u16))),
			Self::Reply => Some(Type::Verbatim(quote!(u32))),

			_ => None,
		}
	}

	pub fn length_syntax(&self) -> bool {
		matches!(self, Self::Request | Self::Reply)
	}
}
