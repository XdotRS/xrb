// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod attributes;
mod field;
mod items;
mod r#let;
mod source;
mod unused;

pub use attributes::*;
pub use field::*;
pub use items::*;
pub use r#let::*;
pub use source::*;
pub use unused::*;

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;

pub enum Item {
	Field(Box<Field>),
	Let(Box<Let>),
	Unused(Unused),
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
