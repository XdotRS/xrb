// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod expansion;
mod parsing;

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

pub struct Elements<'a> {
	pub elements: Punctuated<Element, Token![,]>,

	pub metabyte_element: Option<&'a Element>,
	pub sequence_field: Option<&'a Field>,
}

pub enum Element {
	Field(Box<Field>),

	Let(Box<Let>),

	SingleUnused(SingleUnused),
	ArrayUnused(Box<ArrayUnused>),
}

impl Element {
	pub const fn is_metabyte(&self) -> bool {
		match self {
			Self::Field(field) => field.is_metabyte(),

			Self::Let(r#let) => r#let.is_metabyte(),

			Self::SingleUnused(unused) => unused.is_metabyte(),
			Self::ArrayUnused(_) => false,
		}
	}
}

pub struct Field {
	pub attributes: Vec<Attribute>,
	pub context_attribute: Option<ContextAttribute>,
	pub metabyte_attribute: Option<MetabyteAttribute>,
	pub sequence_attribute: Option<SequenceAttribute>,

	pub visibility: Visibility,
	pub ident: Option<(Ident, Token![:])>,
	pub r#type: Type,
}

impl Field {
	pub const fn is_metabyte(&self) -> bool {
		self.metabyte_attribute.is_some()
	}

	pub const fn is_sequence(&self) -> bool {
		self.sequence_attribute.is_some()
	}
}

pub struct Let {
	pub attributes: Vec<Attribute>,
	pub context_attribute: Option<ContextAttribute>,
	pub metabyte_attribute: Option<MetabyteAttribute>,

	pub let_token: Token![let],
	pub ident: Ident,
	pub colon_token: Token![:],
	pub r#type: Type,

	pub equals_token: Token![=],

	pub source: Source,
}

impl Let {
	pub const fn is_metabyte(&self) -> bool {
		self.metabyte_attribute.is_some()
	}
}

pub struct SingleUnused {
	pub attribute: Option<MetabyteAttribute>,
	pub underscore_token: Token![_],
}

impl SingleUnused {
	pub const fn is_metabyte(&self) -> bool {
		self.attribute.is_some()
	}
}

pub struct ArrayUnused {
	pub attributes: Vec<Attribute>,

	pub bracket_token: token::Bracket,

	pub underscore_token: Token![_],
	pub semicolon_token: Token![_],

	pub content: UnusedContent,
}

pub enum UnusedContent {
	Infer(Token![..]),
	Source(Box<Source>),
}
