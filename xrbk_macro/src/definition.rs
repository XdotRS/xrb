// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use quote::quote;
use syn::{
	braced,
	parenthesized,
	punctuated::Punctuated,
	token,
	Attribute,
	Expr,
	Generics,
	Ident,
	Path,
	Result,
	Token,
	Type,
	Visibility,
	WhereClause,
};

use crate::{
	attribute::parsing::ParsedItemAttributes,
	element::{Content, StructlikeContent},
};

mod expansion;
mod parsing;

/// Multiple [`Definition`]s.
///
/// > **<sup>Syntax</sup>**\
/// > _Definitions_ :\
/// > &nbsp;&nbsp; [_Definition_]<sup>\*</sup>
/// >
/// > [_Definition_]: Definition
pub struct Definitions(Vec<Definition>);

/// A definition within the [`derive_xrb!`] macro.
///
/// > **<sup>Syntax</sup>**\
/// > _Definition_ :\
/// > &nbsp;&nbsp; &nbsp;&nbsp; [_Struct_](Struct)\
/// > &nbsp;&nbsp; | [_Enum_]\
/// > &nbsp;&nbsp; | [_Request_]\
/// > &nbsp;&nbsp; | [_Reply_]\
/// > &nbsp;&nbsp; | [_Event_]\
/// > &nbsp;&nbsp; | [_Item_][^other]
/// >
/// > [_Enum_]: Enum
/// > [_Request_]: Request
/// > [_Reply_]: Reply
/// > [_Event_]: Event
/// >
/// > [_Item_]: https://doc.rust-lang.org/reference/items.html
/// > [^other]: Except [_Struct_]s and [_Enumeration_]s.
/// >
/// > [_Metadata_]: Metadata
/// > [_Content_]: Content
/// >
/// > [_Struct_]: https://doc.rust-lang.org/reference/items/structs.html
/// > [_Enumeration_]: https://doc.rust-lang.org/reference/items/enumerations.html
///
/// [`derive_xrb!`]: crate::derive_xrb!
pub enum Definition {
	Struct(Struct),
	Enum(Enum),

	Request(Request),
	Reply(Reply),
	Event(Event),
	Error(Error),

	/// Any other item allowed in Rust that isn't a struct nor an enum.
	Other(syn::Item),
}

/// A struct with support for [`Element`]s.
///
/// > **<sup>Syntax</sup>**\
/// > _Struct_ :\
/// > &nbsp;&nbsp;
/// > [_OuterAttribute_]<sup>\*</sup>&nbsp;[_Visibility_]<sup>?</sup>
/// > _StructMetadata_\
/// > &nbsp;&nbsp; [_StructlikeContent_]
/// >
/// > _StructMetadata_ :\
/// > &nbsp;&nbsp; `struct`&nbsp;[IDENTIFIER]&nbsp;[_GenericParams_]<sup>?</sup>
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [_Visibility_]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
/// > [_GenericParams_]: https://doc.rust-lang.org/reference/items/generics.html
/// > [IDENTIFIER]: https://doc.rust-lang.org/reference/identifiers.html
/// > [_StructlikeContent_]: StructlikeContent
///
/// [`Element`]: crate::element::Element
pub struct Struct {
	/// Attributes associated with the struct, including doc comments.
	pub item_attributes: ParsedItemAttributes,

	/// The visibility of the struct.
	pub visibility: Visibility,
	/// The struct token: `struct`.
	pub struct_token: Token![struct],
	/// The name of the struct.
	pub ident: Ident,
	/// Generics (lifetimes and/or generic types) associated with the struct.
	pub generics: Generics,

	/// The content of the `Struct`, containing its elements.
	pub content: StructlikeContent,
}

/// A struct with metadata for request messages and support for [`Element`]s.
///
/// > **<sup>Syntax</sup>**\
/// > _Request_ :\
/// > &nbsp;&nbsp;
/// > [_OuterAttribute_]<sup>\*</sup>&nbsp;[_Visibility_]<sup>?</sup>
/// > [_StructMetadata_]\
/// > &nbsp;&nbsp;
/// > `:`&nbsp;`Request`&nbsp;`(`&nbsp;_Meta_&nbsp;`)`&nbsp;_ReplyType_<sup>?
/// > </sup>\
/// > &nbsp;&nbsp; [_StructlikeContent_]
/// >
/// > _Meta_ :\
/// > &nbsp;&nbsp; _MajorOpcode_\
/// > &nbsp;&nbsp; ( `,` _MinorOpcode_ )<sup>?</sup>\
/// > &nbsp;&nbsp; ( `,` _OtherErrors_ )<sup>?</sup>
/// >
/// > _MajorOpcode_, _MinorOpcode_ :\
/// > &nbsp;&nbsp; [_Expression_]
/// >
/// > _OtherErrors_ :\
/// > &nbsp;&nbsp; [_Type_]
/// >
/// > _ReplyType_ :\
/// > &nbsp;&nbsp; `->` [_Type_]
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [_Visibility_]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
/// > [_StructMetadata_]: Struct
/// > [_Expression_]: https://doc.rust-lang.org/reference/expressions.html
/// > [_Type_]: https://doc.rust-lang.org/reference/types.html
/// > [_StructlikeContent_]: StructlikeContent
///
/// [`Element`]: crate::element::Element
pub struct Request {
	/// Attributes associated with the request's struct, including doc comments.
	pub item_attributes: ParsedItemAttributes,

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
	/// A path representing the `Request` trait.
	pub request_token: Path,

	/// A pair of normal brackets surrounding the opcodes: `(` and `)`.
	pub paren_token: token::Paren,
	pub opcodes: RequestOpcodes,
	/// An optional type representing the `OtherErrors` generated by the
	/// request.
	pub other_errors: Option<Type>,
	/// A comma token: `,`.
	pub comma_end: Option<Token![,]>,

	/// An optional arrow followed by a type representing replies generated by
	/// the request.
	pub reply: Option<(Token![->], Type)>,

	/// The content of the `Request`, containing its elements.
	pub content: StructlikeContent,
}

pub enum RequestOpcodes {
	CoreRequest {
		/// An expression representing the major opcode associated with the
		/// request.
		major_opcode: Expr,
		/// A comma token: `,`. This is required before the `minor_opcode`.
		comma1: Option<Token![,]>,
	},
	ExtensionRequest {
		/// An ident that references the static variable containing the
		/// extensions' major opcode.
		major_opcode: Ident,
		/// A comma token: `,`. This is required before the `minor_opcode`.
		comma1: Option<Token![,]>,
		/// An expression representing the minor opcode associated with the
		/// request.
		minor_opcode: Expr,
		/// A comma token: `,`. This is required before `other_errors`.
		comma2: Option<Token![,]>,
	},
}

/// A struct with metadata for reply messages and support for [`Element`]s.
///
/// > **<sup>Syntax</sup>**\
/// > _Reply_ :\
/// > &nbsp;&nbsp;
/// > [_OuterAttribute_]<sup>\*</sup>&nbsp;[_Visibility_]<sup>?</sup>
/// > [_StructMetadata_]\
/// > &nbsp;&nbsp; `:`&nbsp;`Reply`&nbsp;`for`&nbsp;[_Type_]\
/// > &nbsp;&nbsp; [_StructlikeContent_]
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [_Visibility_]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
/// > [_StructMetadata_]: Struct
/// > [_Type_]: https://doc.rust-lang.org/reference/types.html
/// > [_StructlikeContent_]: StructlikeContent
///
/// [`Element`]: crate::element::Element
pub struct Reply {
	/// Attributes associated with the reply's struct.
	pub item_attributes: ParsedItemAttributes,

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
	/// A path representing the `Reply` trait.
	pub reply_token: Path,

	/// A for token: `for`.
	pub for_token: Token![for],
	/// The type of request that generates this reply.
	pub request: Type,

	/// The content of the `Reply`, containing its elements.
	pub content: StructlikeContent,
}

/// A struct with metadata for event messages and support for [`Element`]s.
///
/// > **<sup>Syntax</sup>**\
/// > _Event_ :\
/// > &nbsp;&nbsp;
/// > [_OuterAttribute_]<sup>\*</sup>&nbsp;[_Visibility_]<sup>?</sup>
/// > [_StructMetadata_]\
/// > &nbsp;&nbsp; `:`&nbsp;`Event`&nbsp;`(`&nbsp;[_Expression_]&nbsp;`)`\
/// > &nbsp;&nbsp; [_StructlikeContent_]
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [_Visibility_]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
/// > [_StructMetadata_]: Struct
/// > [_Expression_]: https://doc.rust-lang.org/reference/expressions.html
/// > [_StructlikeContent_]: StructlikeContent
///
/// [`Element`]: crate::element::Element
pub struct Event {
	/// Attributes associated with the event's struct, including doc comments.
	pub item_attributes: ParsedItemAttributes,

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
	/// A path representing the `Event` trait.
	pub event_token: Path,

	/// A pair of normal brackets surrounding the `event_code`: `(` and `)`.
	pub paren_token: token::Paren,
	/// An expression representing the unique event code associated with the
	/// event.
	pub event_code: Expr,
	/// A comma token: `,`.
	pub comma: Option<Token![,]>,

	/// The content of the `Event`, containing its elements.
	pub content: StructlikeContent,
}

/// A struct with metadata  for error messages and support for [`Element`]s.
///
/// > **<sup>Syntax</sup>**\
/// > _Error_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> [_Visibility_]<sup>?</sup>
/// > [_StructlikeMetadata_]\
/// > &nbsp;&nbsp; `:` `Error` `(` [_Expression_] `)`\
/// > &nbsp;&nbsp; [_StructlikeContent_]
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [_Visibility_]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
/// > [_StructMetadata_]: Struct
/// > [_Expression_]: https://doc.rust-lang.org/reference/expressions.html
/// > [_StructlikeContent_]: StructlikeContent
///
/// [`Element`]: crate::element::Element
pub struct Error {
	/// Attributes associated with the error's struct, including doc comments.
	pub item_attributes: ParsedItemAttributes,

	/// The visibility of the error's struct.
	pub visibility: Visibility,
	/// The struct token: `struct.
	pub struct_token: Token![struct],
	/// The name of the error.
	pub ident: Ident,
	/// Generics (lifetimes and/or generic types) associated with the error
	/// struct.
	pub generics: Generics,

	/// A colon token: `:`.
	pub colon_token: Token![:],
	/// A path representing the `Error` trait.
	pub error_token: Path,

	/// A pair of normal brackets surrounding the `error_code`: `(` and `)`.
	pub paren_token: token::Paren,
	/// An expression representing the unique error code associated with the
	/// error.
	pub error_code: Expr,
	/// A comma token: `,`.
	pub comma: Option<Token![,]>,

	/// The content of the `Error`, containing its elements.
	pub content: StructlikeContent,
}

/// An enum with support for [`Element`]s.
///
/// > **<sup>Syntax</sup>**\
/// > _Enum_ :\
/// > &nbsp;&nbsp;
/// > [_OuterAttribute_]<sup>\*</sup>&nbsp;[_Visibility_]<sup>?</sup>
/// > `enum`&nbsp;[IDENTIFIER]&nbsp;[_GenericParams_]<sup>?</sup>
/// > [_WhereClause_]<sup>?</sup>\
/// > &nbsp;&nbsp; `{` _Variants_ `}`
/// >
/// > _Variants_ :\
/// > &nbsp;&nbsp;
/// > [_Variant_]&nbsp;(&nbsp;`,`&nbsp;[_Variant_]&nbsp;)<sup>\*</sup>&nbsp;`,`
/// > <sup>?</sup>
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [_Visibility_]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
/// > [IDENTIFIER]: https://doc.rust-lang.org/reference/identifiers.html
/// > [_GenericParams_]: https://doc.rust-lang.org/reference/items/generics.html
/// > [_WhereClause_]: https://doc.rust-lang.org/reference/items/generics.html#where-clauses
/// > [_Variant_]: Variant
///
/// [`Element`]: crate::element::Element
pub struct Enum {
	/// Attributes associated with the enum.
	pub item_attributes: ParsedItemAttributes,

	/// The visibility of the enum.
	pub visibility: Visibility,
	/// The enum token: `enum`.
	pub enum_token: Token![enum],
	/// The name of the enum.
	pub ident: Ident,
	/// Generics (lifetimes and/or generic types) associated with the enum.
	pub generics: Generics,
	/// The numerical primitive type used for this enum's discriminants.
	///
	/// This defaults to `u8`.
	pub discriminant_type: Option<(Token![:], Type)>,
	pub where_clause: Option<WhereClause>,

	/// A pair of curly brackets (`{` and `}`) surrounding the enum `variants`.
	pub brace_token: token::Brace,
	/// A list of the enum's [`Variant`]s, punctuated by commas.
	pub variants: Punctuated<Variant, Token![,]>,
}

/// An [`Enum`] variant that may contain [`Element`]s.
///
/// > **<sup>Syntax</sup>**\
/// > _Variant_ :\
/// > &nbsp;&nbsp;
/// > [_OuterAttribute_]<sup>\*</sup>&nbsp;[IDENTIFIER]&nbsp;[_Content_]
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
	Error,
}

impl DefinitionType {
	pub fn min_length(&self) -> Option<usize> {
		match self {
			Self::Basic => None,

			Self::Request => None,
			Self::Reply => Some(32),
			Self::Event => Some(32),
			Self::Error => Some(32),
		}
	}

	pub fn length_type(&self) -> Option<Type> {
		match self {
			Self::Request => Some(Type::Verbatim(quote!(u16))),
			Self::Reply => Some(Type::Verbatim(quote!(u32))),

			_ => None,
		}
	}

	pub fn remaining_syntax(&self) -> bool {
		matches!(self, Self::Request | Self::Reply)
	}
}
