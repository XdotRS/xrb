// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod expansion;
pub mod parsing;

use proc_macro2::Span;
use quote::format_ident;
use syn::{punctuated::Punctuated, token, Attribute, Ident, Token, Type, Visibility};

use crate::{
	attribute::{ContextAttribute, MetabyteAttribute, SequenceAttribute},
	source::Source,
};

pub enum Content<'a> {
	Struct {
		brace_token: token::Brace,
		elements: Elements<'a>,
	},

	Tuple {
		paren_token: token::Paren,
		elements: Elements<'a>,
	},

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
	pub elements: Punctuated<Element<'a>, Token![,]>,

	pub fields: Punctuated<&'a Field<'a>, &'a Token![,]>,
	pub metabyte_element: Option<&'a Element<'a>>,
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

pub enum Element<'a> {
	Field(Box<Field<'a>>),

	Let(Box<Let<'a>>),

	SingleUnused(SingleUnused),
	ArrayUnused(Box<ArrayUnused>),
}

impl Element<'_> {
	pub const fn is_metabyte(&self) -> bool {
		match self {
			Self::Field(field) => field.is_metabyte(),

			Self::Let(r#let) => r#let.is_metabyte(),

			Self::SingleUnused(unused) => unused.is_metabyte(),
			Self::ArrayUnused(_) => false,
		}
	}
}

// }}} Field {{{

pub struct Field<'a> {
	pub attributes: Vec<Attribute>,
	pub context_attribute: Option<ContextAttribute>,
	pub metabyte_attribute: Option<MetabyteAttribute>,
	pub sequence_attribute: Option<SequenceAttribute>,

	pub visibility: Visibility,
	pub ident: Option<(Ident, Token![:])>,
	pub r#type: Type,

	pub id: FieldId<'a>,
}

impl Field<'_> {
	pub const fn is_metabyte(&self) -> bool {
		self.metabyte_attribute.is_some()
	}

	pub const fn is_sequence(&self) -> bool {
		self.sequence_attribute.is_some()
	}
}

pub enum FieldId<'a> {
	Ident {
		ident: &'a Ident,
		formatted: Ident,
	},

	Index {
		index: usize,
		ident: Ident,
		formatted: Ident,
	},
}

impl<'a> FieldId<'a> {
	pub const fn ident(&self) -> &Ident {
		match self {
			Self::Index { ident, .. } => ident,
			Self::Ident { ident, .. } => ident,
		}
	}

	pub const fn formatted(&self) -> &Ident {
		match self {
			Self::Index { formatted, .. } => formatted,
			Self::Ident { formatted, .. } => formatted,
		}
	}

	pub fn new_ident(ident: &Ident) -> Self {
		Self::Ident {
			ident,

			formatted: format_ident!("field_{}", ident),
		}
	}

	pub fn new_index(index: usize) -> Self {
		Self::Index {
			index,

			ident: Ident::new(&*index.to_string(), Span::call_site()),
			formatted: format_ident!("field_{}", index),
		}
	}
}

impl ToString for FieldId<'_> {
	fn to_string(&self) -> String {
		match self {
			Self::Ident { ident, .. } => ident.to_string(),
			Self::Index { index, .. } => index.to_string(),
		}
	}
}

// }}} Let {{{

pub struct Let<'a> {
	pub attributes: Vec<Attribute>,
	pub context_attribute: Option<ContextAttribute>,
	pub metabyte_attribute: Option<MetabyteAttribute>,

	pub let_token: Token![let],
	pub ident: Ident,
	pub colon_token: Token![:],
	pub r#type: Type,

	pub equals_token: Token![=],

	pub source: Source,

	pub id: LetId<'a>,
}

impl Let<'_> {
	pub const fn is_metabyte(&self) -> bool {
		self.metabyte_attribute.is_some()
	}
}

pub struct LetId<'a> {
	ident: &'a Ident,
	formatted: Ident,
}

impl<'a> LetId<'a> {
	pub fn new(ident: &'a Ident) -> Self {
		Self {
			ident,

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
	pub attribute: Option<MetabyteAttribute>,
	pub underscore_token: Token![_],
}

impl SingleUnused {
	pub const fn is_metabyte(&self) -> bool {
		self.attribute.is_some()
	}
}

// }}} Array-type unused bytes {{{

pub struct ArrayUnused {
	pub attributes: Vec<Attribute>,

	pub bracket_token: token::Bracket,

	pub underscore_token: Token![_],
	pub semicolon_token: Token![;],

	pub content: UnusedContent,

	pub id: UnusedId,
}

pub enum UnusedContent {
	Infer {
		double_colon_token: Token![::],
		last_element: bool,
	},

	Source(Box<Source>),
}

pub struct UnusedId {
	index: usize,

	ident: Ident,
	formatted: Ident,
}

impl UnusedId {
	pub fn new(index: usize) -> Self {
		Self {
			index,

			ident: Ident::new(&*index.to_string(), Span::call_site()),
			formatted: format_ident!("unused_{}", index),
		}
	}
}

impl ToString for UnusedId {
	fn to_string(&self) -> String {
		self.index.to_string()
	}
}

// }}}
