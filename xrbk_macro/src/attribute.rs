// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod expansion;
pub mod parsing;

use syn::{token, Path, Token};

use crate::Source;

/// An attribute which places an [`Element`] in the metabyte position.
///
/// [`Element`]: crate::element::Element
pub struct MetabyteAttribute {
	/// A hash token: `#`.
	pub hash_token: Token![#],
	/// A pair of square brackets (`[` and `]`) surrounding the `path`.
	pub bracket_token: token::Bracket,

	/// The attribute path: `metabyte` for a `MetabyteAttribute`.
	pub path: Path,
}

/// An attribute which indicates that a [`Field`] represents the sequence number
/// of a reply or event.
///
/// [`Field`]: crate::element::Field
pub struct SequenceAttribute {
	/// A hash token: `#`.
	pub hash_token: Token![#],
	/// A pair of square brackets (`[` and `]`) surrounding the `path`.
	pub bracket_token: token::Bracket,

	/// The attribute path: `sequence` for a `SequenceAttribute`.
	pub path: Path,
}

/// An attribute which provides the [`ContextualReadable::Context`] for a type
/// implementing [`cornflakes::ContextualReadable`].
///
/// [`ContextualReadable::Context`]: cornflakes::ContextualReadable::Context
pub struct ContextAttribute {
	/// A hash token: `#`.
	pub hash_token: Token![#],
	/// A pair of square brackets (`[` and `]`) surrounding the `path`.
	pub bracket_token: token::Bracket,

	/// The attribute path: `context` for a `ContextAttribute`.
	pub path: Path,

	/// The provided context.
	pub context: Context,
}

pub enum Context {
	Paren {
		/// A pair of normal brackets (`(` and `)`) surrounding the [`source`].
		///
		/// [`source`]: Context::Paren::source
		paren_token: token::Paren,
		/// The [`Source`] providing the `Context`.
		source: Source,
	},

	Equals {
		/// An equals token (`=`) preceding the [`source`].
		///
		/// [`source`]: Context::Equals::source
		equals_token: Token![=],
		/// The [`Source`] providing the `Context`.
		source: Source,
	},
}
