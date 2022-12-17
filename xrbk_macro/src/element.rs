// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod expansion;
mod iter;
mod parsing;

use quote::format_ident;
use syn::{punctuated::Punctuated, token, Attribute, Ident, Index, Token, Type, Visibility};

use crate::{
	attribute::{ContextAttribute, MetabyteAttribute, SequenceAttribute},
	source::Source,
};

pub enum Content {
	/// Named elements surrounded by curly brackets (`{` and `}`).
	Struct {
		/// A pair of curly brackets (`{` and `}`) surrounding the `elements`.
		brace_token: token::Brace,
		/// The [`Elements`] contained in the `Content`.
		elements: Elements,
	},

	/// Elements, including unnamed fields, surrounding by normal brackets (`(`
	/// and `)`).
	Tuple {
		/// A pair of normal brackets (`(` and `)`) surrounding the `elements`.
		paren_token: token::Paren,
		/// The [`Elements`] contained in the `Content`.
		elements: Elements,
	},

	/// No [`Elements`] and no delimiters.
	Unit,
}

impl Content {
	/// Whether there is an [`ArrayUnused`] element with
	/// [`UnusedContent::Infer`] within this `Content`.
	///
	/// See [`Elements::contains_infer`] for more information.
	pub const fn contains_infer(&self) -> bool {
		match self {
			Self::Struct { elements, .. } | Self::Tuple { elements, .. } => elements.contains_infer,

			Self::Unit => false,
		}
	}
}

enum ElementsItem {
	Element(Element),
	Metabyte,
	Sequence,
}

pub struct Elements {
	/// The [`Punctuated`] (by commas) list of [`Element`]s, as parsed.
	elements: Punctuated<ElementsItem, Token![,]>,

	/// The [`Element`] which has a [`MetabyteAttribute`], if there is one.
	pub metabyte_element: Option<Element>,
	/// The [`Element`] which has a [`SequenceAttribute`], if there is one.
	pub sequence_element: Option<Element>,

	/// Whether there is an [`ArrayUnused`] element with
	/// [`UnusedContent::Infer`] within these `Elements`.
	///
	/// This is used because if there is no [`UnusedContent::Infer`]
	/// [`ArrayUnused`] element, the cumulative data size of previous elements
	/// does not need to be kept track of during serialization and
	/// deserialization.
	pub contains_infer: bool,
}

// Element {{{

/// An `Element` that takes the place of fields in struct and enum definitions.
pub enum Element {
	/// See [`Field`] for more information.
	Field(Box<Field>),

	/// See [`Let`] for more information.
	Let(Box<Let>),

	/// A single unused byte that will be skipped over when reading and writing.
	///
	/// See [`SingleUnused`] for more information.
	SingleUnused(SingleUnused),
	/// A number of unused bytes that will be skipped over when reading and
	/// writing.
	///
	/// See [`ArrayUnused`] for more information.
	ArrayUnused(Box<ArrayUnused>),
}

impl Element {
	/// Whether this `Element` has a [`MetabyteAttribute`].
	pub const fn is_metabyte(&self) -> bool {
		match self {
			Self::Field(field) => field.is_metabyte(),

			Self::Let(r#let) => r#let.is_metabyte(),

			Self::SingleUnused(unused) => unused.is_metabyte(),
			// Array-type unused bytes elements cannot have metabyte attributes.
			Self::ArrayUnused(_) => false,
		}
	}
}

// }}} Field {{{

pub struct Field {
	/// Attributes associated with the `Field`.
	pub attributes: Vec<Attribute>,
	/// An optional [`ContextAttribute`] to provide context for types
	/// implementing [`cornflakes::ContextualReadable`].
	///
	/// See [`ContextAttribute`] for more information.
	pub context_attribute: Option<ContextAttribute>,
	/// An optional [`MetabyteAttribute`] which places this `Field` in the
	/// metabyte position.
	///
	/// See [`MetabyteAttribute`] for more information.
	pub metabyte_attribute: Option<MetabyteAttribute>,
	/// An optional [`SequenceAttribute`] which indicates that this field
	/// represents the sequence number in a reply or event.
	///
	/// See [`SequenceAttribute`] for more information.
	pub sequence_attribute: Option<SequenceAttribute>,

	/// The visibility of the `Field`.
	pub visibility: Visibility,
	/// Either the field's name, if it is named, or its index, if it is unnamed.
	pub id: FieldId,
	/// A colon token (`:`) following the name of the field, if there is one.
	pub colon_token: Option<Token![:]>,
	/// The `Field`'s type.
	pub r#type: Type,

	/// The formatted identifier used to refer to this `Field` in generated
	/// code.
	pub formatted: Ident,
}

impl Field {
	/// Whether this `Field` has a [`MetabyteAttribute`].
	pub const fn is_metabyte(&self) -> bool {
		self.metabyte_attribute.is_some()
	}

	/// Whether this `Field` has a [`SequenceAttribute`].
	pub const fn is_sequence(&self) -> bool {
		self.sequence_attribute.is_some()
	}
}

pub enum FieldId {
	Ident(Ident),
	Index(Index),
}

impl ToString for FieldId {
	fn to_string(&self) -> String {
		match self {
			Self::Ident(ident) => ident.to_string(),
			Self::Index(Index { index, .. }) => index.to_string(),
		}
	}
}

// }}} Let {{{

pub struct Let {
	/// Attributes associated with the `Let` element.
	pub attributes: Vec<Attribute>,
	/// An optional [`ContextAttribute`] to provide context for reading `Let`
	/// element types which implement [`cornflakes::ContextualReadable`].
	///
	/// See [`ContextAttribute`] for more information.
	pub context_attribute: Option<ContextAttribute>,
	/// An optional [`MetabyteAttribute`] which places this `Let` element in the
	/// metabyte position.
	///
	/// See [`MetabyteAttribute`] for more information.
	pub metabyte_attribute: Option<MetabyteAttribute>,

	/// The let token: `let`.
	pub let_token: Token![let],
	/// The name of the `Let` element.
	pub ident: Ident,
	/// A colon token preceding the `Let` element's type: `:`.
	pub colon_token: Token![:],
	/// The type of the `Let` element.
	pub r#type: Type,

	/// An equals token: `=`.
	pub equals_token: Token![=],

	/// The [`Source`] which provides serialization for the `Let` element.
	pub source: Source,

	/// The formatted identifier used to refer to this `Let` element in
	/// generated code.
	pub formatted: Ident,
}

impl Let {
	/// Whether this `Let` element has a [`MetabyteAttribute`].
	pub const fn is_metabyte(&self) -> bool {
		self.metabyte_attribute.is_some()
	}
}

// }}} Single unused byte {{{

pub struct SingleUnused {
	/// An optional [`MetabyteAttribute`] which places this `SingleUnused` byte
	/// element in the metabyte position.
	///
	/// If this is present for a single unused byte, then it is the same
	/// behavior as if there is no metabyte element at all.
	///
	/// See [`MetabyteAttribute`] for more information.
	pub attribute: Option<MetabyteAttribute>,

	/// An underscore token: `_`.
	pub underscore_token: Token![_],
}

impl SingleUnused {
	/// Whether this `SingleUnused` byte element has a [`MetabyteAttribute`].
	pub const fn is_metabyte(&self) -> bool {
		self.attribute.is_some()
	}
}

// }}} Array-type unused bytes {{{

pub struct ArrayUnused {
	/// Attributes associated with the `ArrayUnused` bytes element's
	/// [`Source`] function, if there is one.
	pub attributes: Vec<Attribute>,

	/// A pair of square brackets (`[` and `]`) surrounding the element.
	pub bracket_token: token::Bracket,

	/// An underscore token: `_`.
	pub underscore_token: Token![_],
	/// A semicolon token: `;`.
	pub semicolon_token: Token![;],

	/// The content of the `ArrayUnused` element.
	///
	/// This determines how many unused bytes are skipped when reading and
	/// writing.
	///
	/// See [`UnusedContent`] for more information.
	pub content: UnusedContent,

	/// The formatted identifier used to refer to this `ArrayUnused` bytes
	/// element in generated code.
	pub formatted: Ident,
}

/// The content of an [`ArrayUnused`] element.
pub enum UnusedContent {
	/// Infer the number of unused bytes.
	///
	/// If this is the last element in a message which has a minimum length, and
	/// that minimum length is not yet reached, this will mean the number of
	/// bytes needed to exactly reach the minimum length of the message will be
	/// skipped.
	///
	/// Otherwise, if this is not the last element, or if the minimum length of
	/// the message has already been reached, or if the definition has no
	/// minimum length, the number of bytes needed to reach the next multiple of
	/// 4 bytes will be skipped. This is for purposes of alignment: the X11
	/// protocol requires messages to be a multiple of 4 bytes in length, for
	/// example.
	Infer {
		/// A double dot 'infer' token: `..`.
		double_dot_token: Token![..],
		/// Whether this is the last element in a definition.
		last_element: bool,
	},

	/// Determine the number of unused bytes by a [`Source`] which returns a
	/// `usize` quantity.
	Source(Box<Source>),
}

// }}}

#[derive(Clone, Copy)]
pub enum ElementType {
	Named,
	Unnamed,
}
