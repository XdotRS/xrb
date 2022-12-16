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

pub enum Content<'a> {
	/// Named elements surrounded by curly brackets (`{` and `}`).
	Struct {
		/// A pair of curly brackets (`{` and `}`) surrounding the `elements`.
		brace_token: token::Brace,
		/// The [`Elements`] contained in the `Content`.
		elements: Elements<'a>,
	},

	/// Elements, including unnamed fields, surrounding by normal brackets (`(`
	/// and `)`).
	Tuple {
		/// A pair of normal brackets (`(` and `)`) surrounding the `elements`.
		paren_token: token::Paren,
		/// The [`Elements`] contained in the `Content`.
		elements: Elements<'a>,
	},

	/// No [`Elements`] and no delimiters.
	Unit,
}

impl Content<'_> {
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

pub struct Elements<'a> {
	/// The [`Punctuated`] (by commas) list of [`Element`]s, as parsed.
	pub elements: Punctuated<Element<'a>, Token![,]>,

	/// A [`Punctuated`] list referencing only the [`Field`]s contained within
	/// [`elements`].
	///
	/// [`elements`]: Self::elements
	pub fields: Punctuated<&'a Field<'a>, &'a Token![,]>,
	/// A reference to the [`Element`] which has a [`MetabyteAttribute`], if
	/// there is one.
	pub metabyte_element: Option<&'a Element<'a>>,
	/// A reference to the [`Field`] which has a [`SequenceAttribute`], if there
	/// is one.
	pub sequence_field: Option<&'a Field<'a>>,

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
pub enum Element<'a> {
	/// See [`Field`] for more information.
	Field(Box<Field<'a>>),

	/// See [`Let`] for more information.
	Let(Box<Let<'a>>),

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

impl Element<'_> {
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

pub struct Field<'a> {
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
	/// The name of the `Field`, followed by a colon token (`:`), for named
	/// fields.
	pub ident: Option<(Ident, Token![:])>,
	/// The `Field`'s type.
	pub r#type: Type,

	pub id: FieldId<'a>,
}

impl Field<'_> {
	/// Whether this `Field` has a [`MetabyteAttribute`].
	pub const fn is_metabyte(&self) -> bool {
		self.metabyte_attribute.is_some()
	}

	/// Whether this `Field` has a [`SequenceAttribute`].
	pub const fn is_sequence(&self) -> bool {
		self.sequence_attribute.is_some()
	}
}

pub enum FieldId<'a> {
	Ident {
		/// A reference to the [`Field`]'s [`ident`].
		///
		/// [`ident`]: Field::ident
		ident: &'a Ident,
		/// The [`Field`]'s formatted [`struct@Ident`] for use in generated
		/// code.
		formatted: Ident,
	},

	Index {
		/// The [`Field`]'s index.
		///
		/// This counts from `0` for the first field, and increments for each
		/// field. The presence of other elements does not increment the field
		/// index.
		index: Index,
		/// The [`Field`]'s formatted [`struct@Ident`] for use in generated
		/// code.
		formatted: Ident,
	},
}

impl<'a> FieldId<'a> {
	/// The [`FieldId`]'s formatted [`struct@Ident`].
	pub const fn formatted(&self) -> &Ident {
		match self {
			Self::Index { formatted, .. } => formatted,
			Self::Ident { formatted, .. } => formatted,
		}
	}

	/// Creates a new [`FieldId::Ident`] with the given `ident`.
	pub fn new_ident(ident: &'a Ident) -> Self {
		Self::Ident {
			ident,

			formatted: format_ident!("field_{}", ident),
		}
	}

	/// Creates a new [`FieldId::Index`] with the given `index`.
	pub fn new_index(index: usize) -> Self {
		Self::Index {
			index: Index::from(index),
			formatted: format_ident!("field_{}", index),
		}
	}
}

impl ToString for FieldId<'_> {
	fn to_string(&self) -> String {
		match self {
			Self::Ident { ident, .. } => ident.to_string(),
			Self::Index { index, .. } => index.index.to_string(),
		}
	}
}

// }}} Let {{{

pub struct Let<'a> {
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

	pub id: LetId<'a>,
}

impl Let<'_> {
	/// Whether this `Let` element has a [`MetabyteAttribute`].
	pub const fn is_metabyte(&self) -> bool {
		self.metabyte_attribute.is_some()
	}
}

pub struct LetId<'a> {
	/// A reference to the [`Let`] element's [`ident`].
	///
	/// [`ident`]: Let::ident
	ident: &'a Ident,
	/// The [`Let`] element's formatted [`struct@Ident`] for use in generated
	/// code.
	formatted: Ident,
}

impl<'a> LetId<'a> {
	/// Creates a new `LetId` with the given `ident`.
	pub fn new(ident: &'a Ident) -> Self {
		Self {
			ident,

			// Create a formatted `Ident` by prepending `let_` to the let element's identifier.
			formatted: format_ident!("let_{}", ident),
		}
	}
}

impl ToString for LetId<'_> {
	fn to_string(&self) -> String {
		self.ident.to_string()
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

	pub id: UnusedId,
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

pub struct UnusedId {
	/// The [`ArrayUnused`] bytes element's index.
	///
	/// This counts from `0` for the first array-type unused bytes element, and
	/// increments for each array-type unused bytes element. The presence of
	/// other types of element does not increment the array-type unused bytes
	/// element index.
	index: Index,
	/// The [`ArrayUnused`] bytes element's formatted [`struct@Ident`] for use
	/// in generated code.
	formatted: Ident,
}

impl UnusedId {
	/// Creates a new `UnusedId` with the given `index`.
	pub fn new(index: usize) -> Self {
		Self {
			index: Index::from(index),
			// Create a formatted `Ident` by prepending `unused_` to the index.
			formatted: format_ident!("unused_{}", index),
		}
	}
}

impl ToString for UnusedId {
	fn to_string(&self) -> String {
		self.index.index.to_string()
	}
}

// }}}

#[derive(Clone, Copy)]
pub enum ElementType {
	Named,
	Unnamed,
}
