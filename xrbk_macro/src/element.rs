// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod expansion;
mod iter;
mod parsing;

use quote::format_ident;
use syn::{
	punctuated::Punctuated,
	token,
	Attribute,
	Ident,
	Index,
	Token,
	Type,
	Visibility,
	WhereClause,
};

use crate::{
	attribute::{
		ContextAttribute,
		ErrorDataAttribute,
		HideAttribute,
		MajorOpcodeAttribute,
		MetabyteAttribute,
		MinorOpcodeAttribute,
		SequenceAttribute,
	},
	source::Source,
};

/// > **<sup>Syntax</sup>**\
/// > _RegularContent_ :\
/// > &nbsp;&nbsp; `{`&nbsp;[_NamedElement_]<sup>\*</sup>&nbsp;`}`
/// >
/// > [_NamedElement_]: Element
pub struct RegularContent {
	brace_token: token::Brace,
	elements: Elements,
}

impl RegularContent {
	/// Whether there is an [`ArrayUnused`] element with
	/// [`UnusedContent::Infer`] within this `RegularContent`.
	///
	/// See [`Elements::contains_infer`] for more information.
	pub const fn contains_infer(&self) -> bool {
		self.elements.contains_infer
	}

	/// The [`Element`] contained within this `RegularContent` which has a
	/// [`MetabyteAttribute`], if there is one.
	pub const fn metabyte_element(&self) -> &Option<Element> {
		&self.elements.metabyte_element
	}

	/// The [`Element`] contained within this `RegularContent` which has a
	/// [`SequenceAttribute`], if there is one.
	pub const fn sequence_element(&self) -> &Option<Element> {
		&self.elements.sequence_element
	}

	/// The [`Element`] contained within this `RegularContent` which has a
	/// [`MinorOpcodeAttribute`], if there is one.
	pub const fn minor_opcode_element(&self) -> &Option<Element> {
		&self.elements.minor_opcode_element
	}

	/// The [`Element`] contained within this `RegularContent` which has a
	/// [`MajorOpcodeAttribute`], if there is one.
	pub const fn major_opcode_element(&self) -> &Option<Element> {
		&self.elements.major_opcode_element
	}

	/// The [`Element`] contained within this `RegularContent` which has an
	/// [`ErrorDataAttribute`], if there is one.
	pub const fn error_data_element(&self) -> &Option<Element> {
		&self.elements.error_data_element
	}
}

/// > **<sup>Syntax</sup>**\
/// > _TupleContent_ :\
/// > &nbsp;&nbsp; `(`&nbsp;[_UnnamedElement_]<sup>\*</sup>&nbsp;`)`
/// >
/// > [_UnnamedElement_]: Element
pub struct TupleContent {
	paren_token: token::Paren,
	elements: Elements,
}

impl TupleContent {
	/// Whether there is an [`ArrayUnused`] element with
	/// [`UnusedContent::Infer`] within this `TupleContent`.
	///
	/// See [`Elements::contains_infer`] for more information.
	pub const fn contains_infer(&self) -> bool {
		self.elements.contains_infer
	}

	/// The [`Element`] contained within this `TupleContent` which has a
	/// [`MetabyteAttribute`], if there is one.
	pub const fn metabyte_element(&self) -> &Option<Element> {
		&self.elements.metabyte_element
	}

	/// The [`Element`] contained within this `TupleContent` which has a
	/// [`SequenceAttribute`], if there is one.
	pub const fn sequence_element(&self) -> &Option<Element> {
		&self.elements.sequence_element
	}

	/// The [`Element`] contained within this `TupleContent` which has a
	/// [`MinorOpcodeAttribute`], if there is one.
	pub const fn minor_opcode_element(&self) -> &Option<Element> {
		&self.elements.minor_opcode_element
	}

	/// The [`Element`] contained within this `TupleContent` which has a
	/// [`MajorOpcodeAttribute`], if there is one.
	pub const fn major_opcode_element(&self) -> &Option<Element> {
		&self.elements.major_opcode_element
	}

	/// The [`Element`] contained within this `TupleContent` which has an
	/// [`ErrorDataAttribute`], if there is one.
	pub const fn error_data_element(&self) -> &Option<Element> {
		&self.elements.error_data_element
	}
}

/// Content (possibly) containing [`Elements`].
///
/// > **<sup>Syntax</sup>**\
/// > _Content_ :\
/// > &nbsp;&nbsp;
/// > (&nbsp;[_RegularContent_]&nbsp;|&nbsp;[_TupleContent_]&nbsp;)<sup>?</sup>
/// >
/// > [_RegularContent_]: RegularContent
/// > [_TupleContent_]: TupleContent
pub enum Content {
	/// Named elements surrounded by curly brackets (`{` and `}`).
	Regular(RegularContent),

	/// Elements, including unnamed fields, surrounding by normal brackets (`(`
	/// and `)`).
	Tuple(TupleContent),

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
			Self::Regular(content) => content.contains_infer(),
			Self::Tuple(content) => content.contains_infer(),

			Self::Unit => false,
		}
	}

	/// The [`Element`] contained within this `Content` which has a
	/// [`MetabyteAttribute`], if there is one.
	pub const fn metabyte_element(&self) -> &Option<Element> {
		match self {
			Self::Regular(content) => content.metabyte_element(),
			Self::Tuple(content) => content.metabyte_element(),

			Self::Unit => &None,
		}
	}

	/// The [`Element`] contained within this `Content` which has a
	/// [`SequenceAttribute`], if there is one.
	pub const fn sequence_element(&self) -> &Option<Element> {
		match self {
			Self::Regular(content) => content.sequence_element(),
			Self::Tuple(content) => content.sequence_element(),

			Self::Unit => &None,
		}
	}

	/// The [`Element`] contained within this `Content` which has a
	/// [`MinorOpcodeAttribute`], if there is one.
	pub const fn minor_opcode_element(&self) -> &Option<Element> {
		match self {
			Self::Regular(content) => content.minor_opcode_element(),
			Self::Tuple(content) => content.minor_opcode_element(),

			Self::Unit => &None,
		}
	}

	/// The [`Element`] contained within this `Content` which has a
	/// [`MajorOpcodeAttribute`], if there is one.
	pub const fn major_opcode_element(&self) -> &Option<Element> {
		match self {
			Self::Regular(content) => content.major_opcode_element(),
			Self::Tuple(content) => content.major_opcode_element(),

			Self::Unit => &None,
		}
	}
}

/// Content used in a structlike definition (possibly) containing [`Elements`].
///
/// > **<sup>Syntax</sup>**\
/// > _StructlikeContent_ :\
/// > &nbsp;&nbsp; &nbsp;&nbsp; _RegularStructlikeContent_\
/// > &nbsp;&nbsp; | _TupleStructlikeContent_\
/// > &nbsp;&nbsp; | _UnitStructlikeContent_
/// >
/// > _RegularStructlikeContent_ :\
/// > &nbsp;&nbsp; [_WhereClause_]<sup>?</sup>&nbsp;[_RegularContent_]
/// >
/// > _TupleStructlikeContent_ :\
/// > &nbsp;&nbsp; [_TupleContent_]&nbsp;[_WhereClause_]<sup>?</sup>&nbsp;`;`
/// >
/// > _UnitStructlikeContent_ :\
/// > &nbsp;&nbsp; [_WhereClause_]<sup>?</sup>&nbsp;`;`
/// >
/// > [_WhereClause_]: https://doc.rust-lang.org/reference/items/generics.html#where-clauses
/// > [_RegularContent_]: RegularContent
/// > [_TupleContent_]: TupleContent
pub enum StructlikeContent {
	Regular {
		where_clause: Option<WhereClause>,
		content: RegularContent,
	},

	Tuple {
		content: TupleContent,
		where_clause: Option<WhereClause>,
		semicolon: Token![;],
	},

	Unit {
		where_clause: Option<WhereClause>,
		semicolon: Token![;],
	},
}

impl StructlikeContent {
	/// Whether there is an [`ArrayUnused`] element with
	/// [`UnusedContent::Infer`] within this `StructlikeContent`.
	///
	/// See [`Elements::contains_infer`] for more information.
	pub const fn contains_infer(&self) -> bool {
		match self {
			Self::Regular { content, .. } => content.contains_infer(),
			Self::Tuple { content, .. } => content.contains_infer(),

			Self::Unit { .. } => false,
		}
	}

	/// The [`Element`] contained within this `StructlikeContent` which has a
	/// [`MetabyteAttribute`], if there is one.
	pub const fn metabyte_element(&self) -> &Option<Element> {
		match self {
			Self::Regular { content, .. } => content.metabyte_element(),
			Self::Tuple { content, .. } => content.metabyte_element(),

			Self::Unit { .. } => &None,
		}
	}

	/// The [`Element`] contained within this `StructlikeContent` which has a
	/// [`SequenceAttribute`], if there is one.
	pub const fn sequence_element(&self) -> &Option<Element> {
		match self {
			Self::Regular { content, .. } => content.sequence_element(),
			Self::Tuple { content, .. } => content.sequence_element(),

			Self::Unit { .. } => &None,
		}
	}

	/// The [`Element`] contained within this `StructlikeContent` which has a
	/// [`MinorOpcodeAttribute`], if there is one.
	pub const fn minor_opcode_element(&self) -> &Option<Element> {
		match self {
			Self::Regular { content, .. } => content.minor_opcode_element(),
			Self::Tuple { content, .. } => content.minor_opcode_element(),

			Self::Unit { .. } => &None,
		}
	}

	/// The [`Element`] contained within this `StructlikeContent` which has a
	/// [`MajorOpcodeAttribute`], if there is one.
	pub const fn major_opcode_element(&self) -> &Option<Element> {
		match self {
			Self::Regular { content, .. } => content.major_opcode_element(),
			Self::Tuple { content, .. } => content.major_opcode_element(),

			Self::Unit { .. } => &None,
		}
	}

	/// The [`Element`] contained within this `StructlikeContent` which has an
	/// [`ErrorDataAttribute`], if there is one.
	pub const fn error_data_element(&self) -> &Option<Element> {
		match self {
			Self::Regular { content, .. } => content.error_data_element(),
			Self::Tuple { content, .. } => content.error_data_element(),

			Self::Unit { .. } => &None,
		}
	}
}

enum ElementsItem {
	Element(Element),

	Metabyte,
	Sequence,

	MinorOpcode,
	MajorOpcode,
	ErrorData,
}

/// Multiple [`Element`]s.
///
/// > **<sup>Syntax</sup>**\
/// > _Elements_ :\
/// > &nbsp;&nbsp; [_NamedElement_]<sup>\*</sup> |
/// > [_UnnamedElement_]<sup>\*</sup>
/// >
/// > [_NamedElement_]: Element
/// > [_UnnamedElement_]: Element
pub struct Elements {
	/// The [`Punctuated`] (by commas) list of [`Element`]s, as parsed.
	elements: Punctuated<ElementsItem, Token![,]>,

	/// The [`Element`] which has a [`MetabyteAttribute`], if there is one.
	pub metabyte_element: Option<Element>,
	/// The [`Element`] which has a [`SequenceAttribute`], if there is one.
	pub sequence_element: Option<Element>,

	/// The [`Element`] which has a [`MinorOpcodeAttribute`], if there is one.
	pub minor_opcode_element: Option<Element>,
	/// The [`Element`] which has a [`MajorOpcodeAttribute`], if there is one.
	pub major_opcode_element: Option<Element>,
	/// The [`Element`] which has an [`ErrorDataAttribute`], if t here is one.
	pub error_data_element: Option<Element>,

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
///
/// > **<sup>Syntax</sup>**\
/// > _Element_ :\
/// > &nbsp;&nbsp; _NamedElement_ | _UnnamedElement_
/// >
/// > _NamedElement_ :\
/// > &nbsp;&nbsp; [_NamedField_] | _XrbkElement_
/// >
/// > _UnnamedElement_ :\
/// > &nbsp;&nbsp; [_UnnamedField_] | _XrbkElement_
/// >
/// > _XrbkElement_ :\
/// > &nbsp;&nbsp; &nbsp;&nbsp; [_LetElement_]\
/// > &nbsp;&nbsp; | [_SingleUnusedElement_]\
/// > &nbsp;&nbsp; | [_ArrayUnusedElement_]
/// >
/// > [_NamedField_]: Field
/// > [_UnnamedField_]: Field
/// > [_LetElement_]: Let
/// > [_SingleUnusedElement_]: SingleUnused
/// > [_ArrayUnusedElement_]: ArrayUnused
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
	/// Whether this `Element` does not have any custom XRBK macro attributes.
	///
	/// Custom XRBK macro attributes include:
	/// - [`MetabyteAttribute`]
	/// - [`SequenceAttribute`]
	/// - [`MinorOpcodeAttribute`]
	/// - [`MajorOpcodeAttribute`]
	/// - [`ErrorDataAttribute`]
	pub const fn is_normal(&self) -> bool {
		!self.is_metabyte()
			&& !self.is_sequence()
			&& !self.is_minor_opcode()
			&& !self.is_major_opcode()
			&& !self.is_error_data()
	}

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

	/// Whether this `Element` has a [`SequenceAttribute`].
	pub const fn is_sequence(&self) -> bool {
		if let Element::Field(field) = self {
			field.is_sequence()
		} else {
			false
		}
	}

	/// Whether this `Element` has a [`MinorOpcodeAttribute`].
	pub const fn is_minor_opcode(&self) -> bool {
		if let Element::Field(field) = self {
			field.is_minor_opcode()
		} else {
			false
		}
	}

	/// Whether this `Element` has a [`MajorOpcodeAttribute`].
	pub const fn is_major_opcode(&self) -> bool {
		if let Element::Field(field) = self {
			field.is_major_opcode()
		} else {
			false
		}
	}

	/// Whether this `Element` has an [`ErrorDataAttribute`].
	pub const fn is_error_data(&self) -> bool {
		if let Element::Field(field) = self {
			field.is_error_data()
		} else {
			false
		}
	}

	/// Whether this `Element` has an `ignore` attribute that contains the trait specified by
	/// `ident`.
	pub fn is_ignoring_trait(&self, ident: Ident) -> bool {
		if let Element::Field(field) = self {
			return field.is_ignoring_trait(ident);
		}
		false
	}
}

// }}} Field {{{

/// A field.
///
/// > **<sup>Syntax</sup>**\
/// > _Field_ :\
/// > &nbsp;&nbsp; _NamedField_ | _UnnamedField_
/// >
/// > _NamedField_ :\
/// > &nbsp;&nbsp; _FieldAttribute_<sup>\*</sup> [_Visibility_]<sup>?</sup>
/// > [IDENTIFIER] `:` [_Type_]\
/// >
/// > _UnnamedField_ :\
/// > &nbsp;&nbsp; _FieldAttribute_<sup>\*</sup> [_Visibility_]<sup>?</sup>
/// > [_Type_]\
/// >
/// > _FieldAttribute_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_] | [_ContextAttribute_] |
/// > [_MetabyteAttribute_] | [_SequenceAttribute_] | [_HideAttribute_]
/// >
/// > [_Visibility_]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
/// > [IDENTIFIER]: https://doc.rust-lang.org/reference/identifiers.html
/// > [_Type_]: https://doc.rust-lang.org/reference/types.html
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [_ContextAttribute_]: ContextAttribute
/// > [_MetabyteAttribute_]: MetabyteAttribute
/// > [_SequenceAttribute_]: SequenceAttribute
/// > [_HideAttribute_]: HideAttribute
pub struct Field {
	/// Attributes associated with the `Field`.
	pub attributes: Vec<Attribute>,
	/// An optional [`ContextAttribute`] to provide context for types
	/// implementing [`xrbk::ContextualReadable`].
	///
	/// See [`ContextAttribute`] for more information.
	///
	/// [`xrbk::ContextualReadable`]: https://docs.rs/xrbk/latest/xrbk/trait.ContextualReadable.html
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
	/// A [`MinorOpcodeAttribute`], required for errors but forbidden elsewhere,
	/// which indicates that this field represents the request that generated an
	/// error for that error.
	///
	/// See [`MinorOpcodeAttribute`] for more information.
	pub minor_opcode_attribute: Option<MinorOpcodeAttribute>,
	/// A [`MajorOpcodeAttribute`], required for errors but forbidden elsewhere,
	/// which indicates that this field represents the request that generated an
	/// error for that error.
	///
	/// See [`MajorOpcodeAttribute`] for more information.
	pub major_opcode_attribute: Option<MajorOpcodeAttribute>,
	/// An [`ErrorDataAttribute`], required for errors but forbidden elsewhere,
	/// which indicates that this field represents the incorrect value that
	/// caused the error to be generated.
	///
	/// See [`ErrorDataAttribute`] for more information.
	pub error_data_attribute: Option<ErrorDataAttribute>,
	/// An optional [`HideAttribute`] which indicates that this field should
	/// not be taken into account when implementing XRBK traits.
	///
	/// See [`HideAttribute`] for more information.
	pub hide_attribute: Option<HideAttribute>,

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
	/// Whether this `Field` does not have any custom XRBK macro attributes.
	///
	/// Custom XRBK macro attributes include:
	/// - [`MetabyteAttribute`]
	/// - [`SequenceAttribute`]
	/// - [`MinorOpcodeAttribute`]
	/// - [`MajorOpcodeAttribute`]
	/// - [`ErrorDataAttribute`]
	pub const fn is_normal(&self) -> bool {
		!self.is_metabyte()
			&& !self.is_sequence()
			&& !self.is_minor_opcode()
			&& !self.is_major_opcode()
			&& !self.is_error_data()
	}

	/// Whether this `Field` has a [`MetabyteAttribute`].
	pub const fn is_metabyte(&self) -> bool {
		self.metabyte_attribute.is_some()
	}

	/// Whether this `Field` has a [`SequenceAttribute`].
	pub const fn is_sequence(&self) -> bool {
		self.sequence_attribute.is_some()
	}

	/// Whether this `Field` has a [`MinorOpcodeAttribute`].
	pub const fn is_minor_opcode(&self) -> bool {
		self.minor_opcode_attribute.is_some()
	}

	/// Whether this `Field` has a [`MajorOpcodeAttribute`].
	pub const fn is_major_opcode(&self) -> bool {
		self.major_opcode_attribute.is_some()
	}

	/// Whether this `Field` has an [`ErrorDataAttribute`].
	pub const fn is_error_data(&self) -> bool {
		self.error_data_attribute.is_some()
	}

	/// Whether this `Element` has an `ignore` attribute that contains the trait specified by
	/// `ident`.
	pub fn is_ignoring_trait(&self, ident: Ident) -> bool {
		let Some(hide) = &self.hide_attribute else {
			return false;
		};
		hide.hidden_traits
			.iter()
			.any(|r#trait| r#trait.is_ident(&ident))
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

/// Data only used during serialization/deserialization.
///
/// > **<sup>Syntax</sup>**\
/// > _LetElement_ :\
/// > &nbsp;&nbsp; _LetAttribute_<sup>\*</sup> `let` [IDENTIFIER] `:` [_Type_]
/// > `=` [_Source_]
/// >
/// > _LetAttribute_ :\
/// > &nbsp;&nbsp; &nbsp;&nbsp; [_OuterAttribute_]\
/// > &nbsp;&nbsp; | [_ContextAttribute_]\
/// > &nbsp;&nbsp; | [_MetabyteAttribute_]
/// >
/// > [IDENTIFIER]: https://doc.rust-lang.org/reference/identifiers.html
/// > [_Type_]: https://doc.rust-lang.org/reference/types.html
/// > [_Source_]: Source
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [_ContextAttribute_]: ContextAttribute
/// > [_MetabyteAttribute_]: MetabyteAttribute
///
/// `Let` elements represent data that exists in the raw byte representation of
/// a particular construct, but not as a field in the Rust representation.
///
/// ## Serialization
/// During serialization, a `Let` element's [`Source`] is used to determine the
/// value that is written by the `Let` element's [`Type`]'s
/// [`xrbk::Writable`] implementation.
///
/// ## Deserialization
/// During deserialization, a `Let` element with a [`ContextAttribute`] will be
/// read with the `Let` element's `type`'s [`xrbk::ContextualReadable`]
/// implementation using the context given by the [`ContextAttribute`]'s
/// [`Source`].  A `Let` element with no [`ContextAttribute`] will simply be
/// read with the `Let` element's `type`'s [`xrbk::Readable`]
/// implementation.
///
/// [`xrbk::Writable`]: https://docs.rs/xrbk/latest/xrbk/trait.Writable.html
/// [`xrbk::ContextualReadable`]: https://docs.rs/xrbk/latest/xrbk/trait.ContextualReadable.html
/// [`xrbk::Readable`]: https://docs.rs/xrbk/latest/xrbk/trait.Readable.html
pub struct Let {
	/// Attributes associated with the `Let` element.
	pub attributes: Vec<Attribute>,
	/// An optional [`ContextAttribute`] to provide context for reading `Let`
	/// element types which implement [`xrbk::ContextualReadable`].
	///
	/// See [`ContextAttribute`] for more information.
	///
	/// [`xrbk::ContextualReadable`]: https://docs.rs/xrbk/latest/xrbk/trait.ContextualReadable.html
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

/// A single unused byte.
///
/// > **<sup>Syntax</sup>**\
/// > _SingleUnusedElement_ :\
/// > &nbsp;&nbsp; [_MetabyteAttribute_]<sup>?</sup> `_`
/// >
/// > [_MetabyteAttribute_]: MetabyteAttribute
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

/// Multiple unused bytes.
///
/// > **<sup>Syntax</sup>**\
/// > _ArrayUnusedElement_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> `[` `_` `;` [_UnusedContent_]
/// > `]`
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [_UnusedContent_]: UnusedContent
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
///
/// > **<sup>Syntax</sup>**\
/// > _UnusedContent_ :\
/// > &nbsp;&nbsp; `..` | [_Source_]
/// >
/// > [_Source_]: Source
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
