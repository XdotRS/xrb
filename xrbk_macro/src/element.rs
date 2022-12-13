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

pub enum Elements {
	Struct {
		brace_token: token::Brace,
		elements: Punctuated<Element, Token![,]>,
	},

	Tuple {
		paren_token: token::Paren,
		elements: Punctuated<Element, Token![,]>,
	},

	Unit,
}

pub enum Element {
	Field(Box<Field>),

	Let(Box<Let>),

	SingleUnused(SingleUnused),
	ArrayUnused(Box<ArrayUnused>),
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

pub struct SingleUnused {
	pub attribute: Option<MetabyteAttribute>,
	pub underscore_token: Token![_],
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
