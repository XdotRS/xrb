// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;

pub use attributes::*;
pub use field::*;
pub use items::*;
pub use r#let::*;
pub use source::*;
pub use unused::*;

use crate::{ItemDeserializeTokens, ItemSerializeTokens};

mod attributes;
mod items;
mod source;

pub enum Item {
	Field(Box<Field>),
	Let(Box<Let>),
	Unused(Unused),
}

impl Item {
	pub fn is_metabyte(&self) -> bool {
		match self {
			Self::Field(field) => field.attributes.iter().any(|attr| {
				matches!(
					attr,
					Attribute {
						content: AttrContent::Metabyte(_),
						..
					}
				)
			}),

			Self::Let(r#let) => {
				matches!(
					r#let.attribute,
					Some(Attribute {
						content: AttrContent::Metabyte(_),
						..
					})
				)
			}

			Self::Unused(unused) => {
				matches!(
					unused,
					Unused::Unit {
						attribute: Some(Attribute {
							content: AttrContent::Metabyte(_),
							..
						}),
						..
					}
				)
			}
		}
	}
}

// Expansion {{{

impl ToTokens for Item {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// If `self` is a `Field`, convert it to tokens, otherwise don't - the
		// other items are used for generating the serialization and
		// deserialization code.
		if let Self::Field(field) = self {
			field.to_tokens(tokens);
		}
	}
}

// }}}

// Implementations {{{

impl ItemSerializeTokens for Item {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		match self {
			Item::Field(field) => field.serialize_tokens(tokens, id),

			Item::Let(r#let) => r#let.serialize_tokens(tokens, id),

			Item::Unused(unused) => unused.serialize_tokens(tokens, id),
		}
	}
}

impl ItemDeserializeTokens for Item {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		match self {
			Item::Field(field) => field.deserialize_tokens(tokens, id),

			Item::Let(r#let) => r#let.deserialize_tokens(tokens, id),

			Item::Unused(unused) => unused.deserialize_tokens(tokens, id),
		}
	}
}

// }}}
