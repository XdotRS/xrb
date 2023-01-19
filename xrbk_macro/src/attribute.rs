// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod expansion;
pub mod parsing;

use syn::{punctuated::Punctuated, token, Path, Token};

use crate::Source;

/// An attribute which places an [`Element`] in the metabyte position.
///
/// > **<sup>Syntax</sup>**\
/// > _MetabyteAttribute_ :\
/// > &nbsp;&nbsp; `#` `[` `metabyte` `]`
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
/// > **<sup>Syntax</sup>**\
/// > _SequenceAttribute_ :\
/// > &nbsp;&nbsp; `#` `[` `sequence` `]`
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

/// An attribute which indicates that a [`Field`] for an [`Error`] represents
/// the minor opcode of the request which generated an error.
///
/// > **<sup>Syntax</sup>**\
/// > _MinorOpcodeAttribute_ :\
/// > &nbsp;&nbsp; `#` `[` `minor_opcode` `]`
///
/// [`Field`]: crate::element::Field
/// [`Error`]: crate::definition::Error
pub struct MinorOpcodeAttribute {
	/// A hash token: `#`.
	pub hash_token: Token![#],
	/// A pair of square brackets (`[` and `]`) surrounding the `path`.
	pub bracket_token: token::Bracket,

	/// The attribute path: `minor_opcode` for a `MinorOpcodeAttribute`.
	pub path: Path,
}

/// An attribute which indicates that a [`Field`] for an [`Error`] represents
/// the major opcode of the request which generated an error.
///
/// > **<sup>Syntax</sup>**\
/// > _MajorOpcodeAttribute_ :\
/// > &nbsp;&nbsp; `#` `[` `major_opcode` `]`
///
/// [`Field`]: crate::element::Field
/// [`Error`]: crate::definition::Error
pub struct MajorOpcodeAttribute {
	/// A hash token: `#`.
	pub hash_token: Token![#],
	/// A pair of square brackets (`[` and `]`) surrounding the `path`.
	pub bracket_token: token::Bracket,

	/// The attribute path: `major_opcode` for a `MajorOpcodeAttribute`.
	pub path: Path,
}

/// An attribute which indicates that a [`Field`] for an [`Error`] represents
/// the incorrect value which caused the error to be generated.
///
/// > **<sup>Syntax</sup>**\
/// > _ErrorDataAttribute_ :\
/// > &nbsp;&nbsp; `#` `[` `error_data` `]`
///
/// [`Field`]: crate::element::Field
/// [`Error`]: crate::definition::Error
pub struct ErrorDataAttribute {
	/// A hash token: `#`.
	pub hash_token: Token![#],
	/// A pair of square brackets (`[` and `]`) surrounding the `path`.
	pub bracket_token: token::Bracket,

	/// The attribute path: `error_data` for an `ErrorDataAttribute`.
	pub path: Path,
}

/// An attribute which indicates that a [`Field`] should not be taken into
/// consideration when implementing traits.
///
/// > **<sup>Syntax</sup>**\
/// > _HideAttribute_ :\
/// > &nbsp;&nbsp; `#` `[` `hide` `(` _HiddenTraits_ `)` `]`
/// >
/// > _HiddenTraits_ :\
/// > &nbsp;&nbsp; _HiddenTrait_[^hidden-traits] ( `,`
/// > _HiddenTrait_[^hidden-traits] )<sup>\*</sup>
/// >
/// > _HiddenTrait_ :\
/// > &nbsp;&nbsp; &nbsp;&nbsp; `Readable` \
/// > &nbsp;&nbsp; | `Writable` \
/// > &nbsp;&nbsp; | `X11Size` \
/// >
/// > [^hidden-traits]: *HideAttribute*s may only specify traits listed in
/// > *HiddenTraits*, any
/// > other traits will have no effects.
///
/// [`Field`]: crate::element::Field
pub struct HideAttribute {
	/// A hash token: `#`.
	pub hash_token: Token![#],
	/// A pair of square brackets (`[` and `]`) surrounding the `path`.
	pub bracket_token: token::Bracket,

	/// The attribute path: `hide` for a `HideAttribute`.
	pub path: Path,

	/// A pair of square brackets (`(` and `)`) surrounding the `hidden_traits`.
	pub paren_token: token::Paren,

	/// A list of traits which will ignore this field in their derived
	/// implementations.
	///
	/// See the [`HideAttribute`] syntax section for which traits are allowed
	/// here.
	pub hidden_traits: Punctuated<Path, Token![,]>,
}

/// An attribute which provides the [`ContextualReadable::Context`] for a type
/// implementing [`xrbk::ContextualReadable`].
///
/// > **<sup>Syntax</sup>**\
/// > _ContextAttribute_ :\
/// > &nbsp;&nbsp; `#` `[` `context` [_Context_] `]`
/// >
/// > [_Context_]: Context
///
/// [`ContextualReadable::Context`]: https://docs.rs/xrbk/latest/xrbk/trait.ContextualReadable.html#associatedtype.Context
/// [`xrbk::ContextualReadable`]: https://docs.rs/xrbk/latest/xrbk/trait.ContextualReadable.html
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

/// The context provided within a [`ContextAttribute`].
///
/// > **<sup>Syntax</sup>**\
/// > _Context_ :\
/// > &nbsp;&nbsp; ( `=` [_Source_] ) | _DelimitedContext_
/// >
/// > _DelimitedContext_ :\
/// > &nbsp;&nbsp; &nbsp;&nbsp; ( `(` [_Source_] `)` )
/// > &nbsp;&nbsp; | ( `{` [_Source_] `}` )
/// > &nbsp;&nbsp; | ( `[` [_Source_] `]` )
/// >
/// > [_Source_]: Source
pub enum Context {
	Paren {
		/// A pair of normal brackets (`(` and `)`) surrounding the [`source`].
		///
		/// [`source`]: Context::Paren::source
		paren_token: token::Paren,
		/// The [`Source`] providing the `Context`.
		source: Source,
	},

	Brace {
		/// A pair of curly brackets (`{` and `}`) surrounding the [`source`].
		///
		/// [`source`]: Context::Brace::source
		brace_token: token::Brace,
		/// The [`Source`] providing the `Context`.
		source: Source,
	},

	Bracket {
		/// A pair of square brackets (`[` and `]`) surrounding the [`source`].
		///
		/// [`source`]: Context::Bracket::source
		bracket_token: token::Bracket,
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

impl Context {
	pub const fn source(&self) -> &Source {
		match self {
			Self::Equals { source, .. } => source,

			Self::Paren { source, .. } => source,
			Self::Brace { source, .. } => source,
			Self::Bracket { source, .. } => source,
		}
	}
}
