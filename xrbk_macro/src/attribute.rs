// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod expansion;
pub mod parsing;

use syn::{token, Path, Token};

use crate::Source;

pub struct MetabyteAttribute {
	pub hash_token: Token![#],
	pub bracket_token: token::Bracket,

	pub path: Path,
}

pub struct SequenceAttribute {
	pub hash_token: Token![#],
	pub bracket_token: token::Bracket,

	pub path: Path,
}

pub struct ContextAttribute {
	pub hash_token: Token![#],
	pub bracket_token: token::Bracket,

	pub path: Path,

	pub context: Context,
}

pub enum Context {
	Paren {
		paren_token: token::Paren,
		source: Source,
	},

	Equals {
		equals_token: Token![=],
		source: Source,
	},
}
