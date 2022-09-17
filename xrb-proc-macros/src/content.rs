// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::{Delimiter, Group, TokenStream as TokenStream2};
use quote::{ToTokens, TokenStreamExt};

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
	braced, bracketed, parenthesized, token, Attribute, Expr, Ident, Result, Token, Type,
	Visibility,
};

// Items {{{

/// Represents the definition of [`Named`] or [`Unnamed`] items, if any.
///
/// [`Named`]: Items::Named
/// [`Unnamed`]: Items::Unnamed
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Items {
	/// [`NamedItems`], surrounded by `{` and `}`.
	///
	/// [`NamedItems`] can contain [`NamedField`]s, but not [`UnnamedField`]s.
	Named(NamedItems),
	/// [`UnnamedItems`], surrounded by `(` and `)`.
	///
	/// [`UnnamedItems`] can contain [`UnnamedField`]s, but not [`NamedField`]s.
	/// The syntax for [`UnnamedItems`] may be familiar to you under names such
	/// as 'tuple structs' and 'tuple variants'.
	Unnamed(UnnamedItems),
	/// No items.
	Unit,
}

impl<'a> Items {
	/// If `self` is [`Items::Named`], returns
	/// <code>[Some](&[NamedItems])</code>.
	pub fn named_items(&self) -> Option<&NamedItems> {
		match self {
			Self::Named(items) => Some(items),
			_ => None,
		}
	}

	/// If `self` is [`Items::Unnamed`], returns
	/// <code>[Some](&[UnnamedItems])</code>.
	pub fn unnamed_items(&self) -> Option<&UnnamedItems> {
		match self {
			Self::Unnamed(items) => Some(items),
			_ => None,
		}
	}

	/// If `self` is [`Items::Named`], returns
	/// <code>[Some]([Vec]<&'a [NamedField]>)</code> with any [`NamedField`]s
	/// contained.
	pub fn named_fields(&'a self) -> Option<Vec<&'a NamedField>> {
		match self {
			Self::Named(items) => Some(items.fields()),
			_ => None,
		}
	}

	/// If `self` is [`Items::Unnamed`], returns
	/// <code>[Some]([Vec]<&'a [UnnamedField]>)</code> with any
	/// [`UnnamedField`]s contained.
	pub fn unnamed_fields(&'a self) -> Option<Vec<&'a UnnamedField>> {
		match self {
			Self::Unnamed(items) => Some(items.fields()),
			_ => None,
		}
	}

	/// Returns whether `self` is [`Items::Named`].
	pub fn is_named(&self) -> bool {
		match self {
			Self::Named(_) => true,
			_ => false,
		}
	}

	/// Returns whether `self` is [`Items::Unnamed`].
	pub fn is_unnamed(&self) -> bool {
		match self {
			Self::Unnamed(_) => true,
			_ => false,
		}
	}

	/// Returns whether `self` is [`Items::Unit`].
	pub fn is_unit(&self) -> bool {
		match self {
			Self::Unit => true,
			_ => false,
		}
	}

	/// Returns whether any item within `self` is defined for the 'metabyte
	/// position'.
	///
	/// The 'metabyte position' is the second byte of the header in a message.
	/// It does not exist for non-message structs and enums.
	pub fn has_metabyte(&self) -> bool {
		match self {
			Self::Named(items) => items.has_metabyte(),
			Self::Unnamed(items) => items.has_metabyte(),
			Self::Unit => false,
		}
	}
}

/// A list of [`NamedItem`]s punctuated by commas and surrounded by curly
/// brackets.
///
/// `NamedItems` can contain [`NamedField`]s, but not [`UnnamedField`]s.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct NamedItems {
	/// Named items are surrounded by curly brackets (`{` and `}`).
	pub brace_token: token::Brace,
	/// A list of [`NamedItem`]s, punctuated by commas (`,`).
	///
	/// The final comma is optional.
	pub items: Punctuated<NamedItem, Token![,]>,
}

impl<'a> NamedItems {
	/// An iterator over the [`NamedItem`]s.
	pub fn iter(&self) -> syn::punctuated::Iter<NamedItem> {
		self.items.iter()
	}

	/// Returns a <code>[Vec]<&'a [NamedField]></code> of any [`NamedField`]s
	/// contained within `self`.
	pub fn fields(&'a self) -> Vec<&'a NamedField> {
		self.iter()
			.filter_map(|item| match item {
				NamedItem::NamedField(field) => Some(field),
				_ => None,
			})
			.collect()
	}

	/// Returns whether any [`NamedItem`] within `self` is defined for the
	/// 'metabyte position'.
	///
	/// The 'metabyte position' is the second byte of the header in a message.
	/// It does not exist for non-message structs and enums.
	pub fn has_metabyte(&self) -> bool {
		self.iter().find(|item| item.is_metabyte()).is_some()
	}
}

/// A list of [`UnnamedItem`]s punctuated by commas and surrounded by normal
/// brackets.
///
/// `UnnamedItems` can contain [`UnnamedField`]s, but not [`NamedField`]s. The
/// syntax for `UnnamedItems` may be familiar to you under names such as 'tuple
/// structs' and 'tuple variants'.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct UnnamedItems {
	/// Unnamed items are surrounded by normal brackets (`(` and `)`).
	pub paren_token: token::Paren,
	/// A list of [`UnnamedItem`]s, punctuated by commas (`,`).
	///
	/// The final comma is optional.
	pub items: Punctuated<UnnamedItem, Token![,]>,
}

impl<'a> UnnamedItems {
	/// An iterator over the [`UnnamedItem`]s.
	pub fn iter(&self) -> syn::punctuated::Iter<UnnamedItem> {
		self.items.iter()
	}

	/// Returns a <code>[Vec]<&'a [UnnamedField]></code> of any
	/// [`UnnamedField`]s contained within `self`.
	pub fn fields(&'a self) -> Vec<&'a UnnamedField> {
		self.iter()
			.filter_map(|item| match item {
				UnnamedItem::UnnamedField(field) => Some(field),
				_ => None,
			})
			.collect()
	}

	/// Returns whether any [`UnnamedItem`] within `self` is defined for the
	/// 'metabyte position'.
	///
	/// The 'metabyte position' is the second byte of the header in a message.
	/// It does not exist for non-message structs and enums.
	pub fn has_metabyte(&self) -> bool {
		self.iter().find(|item| item.is_metabyte()).is_some()
	}
}

/// Either an [`UnusedBytes`] item, a [`FieldLength`] item, or a [`NamedField`].
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum NamedItem {
	/// An [`UnusedBytes`] item, representing bytes that are unused in
	/// serialization and deserialization.
	UnusedBytes(UnusedBytes),
	/// A [`FieldLength`] item, representing the length of a field that is some
	/// kind of list.
	FieldLength(FieldLength),
	/// A [`NamedField`].
	///
	/// This is equivalent to a [`syn::Field`] which has an `ident` and
	/// `colon_token`.
	NamedField(NamedField),
}

impl NamedItem {
	/// Returns whether this `NamedItem` is defined for the 'metabyte position'.
	///
	/// The 'metabyte position' is the second byte of the header in a message.
	/// It does not exist for non-message structs and enums.
	pub fn is_metabyte(&self) -> bool {
		match self {
			Self::UnusedBytes(unused_bytes) => unused_bytes.is_metabyte(),
			Self::FieldLength(field_length) => field_length.is_metabyte(),
			Self::NamedField(field) => field.is_metabyte(),
		}
	}
}

/// Either an [`UnusedBytes`] item, a [`FieldLength`] item, or an
/// [`UnnamedField`].
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum UnnamedItem {
	/// An [`UnusedBytes`] item, representing bytes that are unused in
	/// serialization and deserialization.
	UnusedBytes(UnusedBytes),
	/// A [`FieldLength`] item, representing the length of a field that is some
	/// kind of list.
	FieldLength(FieldLength),
	/// An [`UnnamedField`].
	///
	/// This is equivalent to a [`syn::Field`] which doesn't have an `ident` or
	/// `colon_token`.
	UnnamedField(UnnamedField),
}

impl UnnamedItem {
	/// Returns whether this `UnnamedItem` is defined for the 'metabyte
	/// position'.
	///
	/// The 'metabyte position' is the second byte of the header in a message.
	/// It does not exist for non-message structs and enums.
	pub fn is_metabyte(&self) -> bool {
		match self {
			Self::UnusedBytes(item) => item.is_metabyte(),
			Self::FieldLength(item) => item.is_metabyte(),
			Self::UnnamedField(item) => item.is_metabyte(),
		}
	}
}

// }}}

// Fields {{{

/// A field with a name.
///
/// This is equivalent to a [`syn::Field`] which has an `ident` and
/// `colon_token`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct NamedField {
	/// Attributes associated with the field.
	pub attributes: Vec<Attribute>,
	/// The visibility of the field.
	pub vis: Visibility,
	/// A metabyte token used in [`messages!`] to indicate the second byte of
	/// the message header.
	///
	/// If the `NamedField` is contained within the definition of a message, and
	/// an item in that message has already been defined for the metabyte, or a
	/// minor opcode is defined for that message, then the presence of this
	/// token will generate an error.
	///
	/// If the `NamedField` is not contained within the definition of a message
	/// (i.e. if the [`define!`] macro is used), then the presence of this token
	/// will also generate an error.
	///
	/// [`messages!`]: crate::messages
	/// [`define!`]: crate::define
	pub metabyte_token: Option<Token![$]>,
	/// The name of the field.
	pub name: Ident,
	/// A colon token: `:`.
	pub colon_token: Token![:],
	/// The type of the field.
	pub ty: Type,
}

impl NamedField {
	/// Returns whether this `NamedField` is defined for the 'metabyte
	/// position'.
	///
	/// The 'metabyte position' is the second byte of the header in a message.
	/// It does not exisit for non-message structs and enums.
	pub fn is_metabyte(&self) -> bool {
		self.metabyte_token.is_some()
	}
}

/// A field without a name.
///
/// This is equivalent to a [`syn::Field`] which doesn't have an `ident` or
/// `colon_token`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct UnnamedField {
	/// Attributes associated with the field.
	pub attributes: Vec<Attribute>,
	/// The visibility of the field.
	pub vis: Visibility,
	/// A metabyte token used in [`messages!`] to indicate the second byte of
	/// the message header.
	///
	/// If the `UnamedField` is contained within the definition of a message,
	/// and an item in that message has already been defined for the metabyte,
	/// or a minor opcode is defined for that message, then the presence of this
	/// token will generate an error.
	///
	/// If the `UnamedField` is not contained within the definition of a message
	/// (i.e. if the [`define!`] macro is used), then the presence of this token
	/// will also generate an error.
	///
	/// [`messages!`]: crate::messages
	/// [`define!`]: crate::define
	pub metabyte_token: Option<Token![$]>,
	/// The type of the field.
	pub ty: Type,
}

impl UnnamedField {
	/// Returns whether this `UnnamedField` is defined for the 'metabyte
	/// position'.
	///
	/// The 'metabyte position' is the second byte of the header in a message.
	/// It does not exisit for non-message structs and enums.
	pub fn is_metabyte(&self) -> bool {
		self.metabyte_token.is_some()
	}
}

// }}}

// Field length {{{

/// Encodes the length of some kind of list field in serialization and
/// deserialization.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FieldLength {
	/// A metabyte token used in [`messages!`] to indicate the second byte of
	/// the message header.
	///
	/// If the `FieldLength` is contained within the definition of a message,
	/// and an item in that message has already been defined for the metabyte,
	/// or a minor opcode is defined for that message, then the presence of this
	/// token will generate an error.
	///
	/// If the `FieldLength` is not contained within the definition of a message
	/// (i.e. if the [`define!`] macro is used), then the presence of this token
	/// will also generate an error.
	///
	/// [`messages!`]: crate::messages
	/// [`define!`]: crate::define
	pub metabyte_token: Option<Token![$]>,
	/// A number sign token: `#`.
	pub number_sign_token: Token![#],
	/// An identifier referring to the field which this `FieldLength` describes
	/// the length of.
	///
	/// If the field is a [`NamedField`], this will be its name, otherwise, if
	/// the field is an [`UnnamedField`], this will be its index.
	///
	/// # Examples
	/// ```
	/// use xrb_proc_macros::define;
	///
	/// define! {
	///     pub struct NamedExample<'a> {
	///         #bytes: u16, // the length of `bytes`
	///         pub bytes: &'a [u8],
	///         pub name: &'a str,
	///         #name: u8, // the length of `name`
	///     }
	///
	///     // Though the list of bytes here is the second position, it is the
	///     // first _field_, so it has an index of `0`. `&'a str` field is the
	///     // second field, so it has an index of `1`.
	///     pub struct UnnamedExample<'a>(#0: u16, &'a [u8], &'a str, #1: u8);
	/// }
	/// ```
	pub field_ident: Ident,
	/// A colon token; `:`.
	pub colon_token: Token![:],
	/// The type used to represent the length of the field.
	///
	/// This type determines how many bytes the `FieldLength` is written as. It
	/// must be an integer. For example, a `FieldLength` of type `u8` will be
	/// written as one byte, and a `FieldLength` of type `u32` will be written
	/// as four bytes.
	pub ty: Type,
}

impl FieldLength {
	/// Returns whether this `FieldLength` is defined for the 'metabyte
	/// position'.
	///
	/// The 'metabyte position' is the second byte of the header in a message.
	/// It does not exisit for non-message structs and enums.
	pub fn is_metabyte(&self) -> bool {
		self.metabyte_token.is_some()
	}
}

// }}}

// Unused bytes {{{

/// Defines unused bytes for serializing or deserializing a message, struct, or
/// enum.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct UnusedBytes {
	/// A metabyte token used in [`messages!`] to indicate the second byte of
	/// the message header.
	///
	/// If the `UnusedBytes` item is contained within the definition of a
	/// message, and an item in that message has already been defined for the
	/// metabyte, or a minor opcode is defined for that message, then the
	/// presence of this token will generate an error.
	///
	/// If the `UnusedBytes` item is not contained within the definition of a
	/// message (i.e. if the [`define!`] macro is used), then the presence of
	/// this token will also generate an error.
	///
	/// [`messages!`]: crate::messages
	/// [`define!`]: crate::define
	pub metabyte_token: Option<Token![$]>,
	/// The actual [`UnusedBytesDefinition`].
	pub unused_bytes: UnusedBytesDefinition,
}

impl UnusedBytes {
	/// Returns whether this `UnusedBytes` item is defined for the 'metabyte
	/// position'.
	///
	/// The 'metabyte position' is the second byte of the header in a message.
	/// It does not exisit for non-message structs and enums.
	pub fn is_metabyte(&self) -> bool {
		self.metabyte_token.is_some()
	}
}

/// The definition of [`UnusedBytes`].
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum UnusedBytesDefinition {
	/// A single unused byte with the unit syntax (`()` or `$()`).
	Unit(token::Paren),
	/// The full unused bytes syntax (`[(); count]`).
	Full(UnusedBytesFull),
}

/// A full definition of [`UnusedBytes`].
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct UnusedBytesFull {
	/// Square bracket tokens: `[` and `]`.
	///
	/// These square brackets surround an [`UnusedBytesFull`] definition.
	pub bracket_token: token::Bracket,
	/// A pair of normal brackets, representing a unit type: `()`.
	pub paren_token: token::Paren,
	/// A semicolon token: `;`.
	pub semicolon_token: Token![;],
	/// Determines the number of unused bytes.
	pub count: UnusedBytesCount,
}

/// Determines the number of [`UnusedBytes`]. For use in serialization and
/// deserialization.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum UnusedBytesCount {
	/// An expression representing a number of unused bytes.
	Expression(Expr),
	/// Indicates that zero to three unused bytes should be written to pad the
	/// size of the given field to a multiple of four bytes.
	///
	/// Padding a field is indicated by surrounding the _field identifier_ with
	/// curly brackets (`{` and `}`). If the field is a [`NamedField`], that
	/// means its name, otherwise if the field is an [`UnnamedField`], that
	/// means its index.
	///
	/// Unused padding bytes may need to be added for a particular field as the
	/// total byte length of messages in X must be a multiple of four bytes.
	/// This padding ensures that, for fields which have a dynamic byte size,
	/// the length of the message is correct.
	///
	/// # Examples
	/// ```
	/// use xrb_proc_macros::define;
	///
	/// define! {
	///     // Pad the bytes field:
	///     pub struct NamedExample<'a> {
	///         pub bytes: &'a [u8],
	///         [(); {bytes}],
	///     }
	///
	///     // Pad the first field:
	///     pub struct UnnamedExample<'a>(&'a [u8], [(); {0}]);
	/// }
	/// ```
	Padding((token::Brace, Ident)),
}

// }}}

// Expansion {{{

impl ToTokens for Items {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			// If there are `NamedItems`, write the named fields, if any.
			Self::Named(items) => items.to_tokens(tokens),
			// If there are `UnnamedItems`, write the unnamed fields, if any.
			Self::Unnamed(items) => items.to_tokens(tokens),
			// If there are no items, write nothing.
			Self::Unit => (),
		}
	}
}

impl ToTokens for NamedItems {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Write any and all `NamedField`s, surrounded by curly brackets (`{`
		// and `}`).
		tokens.append(Group::new(Delimiter::Brace, self.items.to_token_stream()))
	}
}

impl ToTokens for UnnamedItems {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Write any and all `UnnamedField`s, surrounded by normal brackets (`(`
		// and `)`).
		tokens.append(Group::new(
			Delimiter::Parenthesis,
			self.items.to_token_stream(),
		))
	}
}

impl ToTokens for NamedItem {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			// If this item is a `NamedField`, write it.
			Self::NamedField(field) => field.to_tokens(tokens),
			// Otherwise, if it isn't a `NamedField`, don't write it.
			_ => (),
		}
	}
}

impl ToTokens for UnnamedItem {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			// If this item is an `UnnamedField`, write it.
			Self::UnnamedField(field) => field.to_tokens(tokens),
			// Otherwise, if it isn't an `UnnamedField`, don't write it.
			_ => (),
		}
	}
}

impl ToTokens for NamedField {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Attributes associated with the field.
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// The visibility of the field.
		self.vis.to_tokens(tokens);
		// The name of the field.
		self.name.to_tokens(tokens);
		// The colon token: `:`.
		self.colon_token.to_tokens(tokens);
		// The type of the field.
		self.ty.to_tokens(tokens);
	}
}

impl ToTokens for UnnamedField {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Attributes associated with the field.
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// The visibility of the field.
		self.vis.to_tokens(tokens);
		// The type of the field.
		self.ty.to_tokens(tokens);
	}
}

// }}}

// Parsing {{{

//     Items {{{
impl Parse for Items {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(token::Brace) {
			// If the next token is a curly bracket (`{`), then parse as
			// `Named`.
			Self::Named(input.parse()?)
		} else if input.peek(token::Paren) {
			// Otherwise, if the next token is a normal bracket (`(`), then
			// parse as `Unnamed`.
			Self::Unnamed(input.parse()?)
		} else {
			// Otherwise, if the next token is neither a curly bracket, nor a
			// normal bracket, parse as `Unit` (no items).
			Self::Unit
		})
	}
}

impl Parse for NamedItems {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		Ok(Self {
			brace_token: braced!(content in input),
			items: input.parse_terminated(NamedItem::parse)?,
		})
	}
}

impl Parse for UnnamedItems {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		Ok(Self {
			paren_token: parenthesized!(content in input),
			items: input.parse_terminated(UnnamedItem::parse)?,
		})
	}
}

impl Parse for NamedItem {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(Token![$]) {
			// If the next token is `$`, then we need to look _two_ tokens in
			// advance to find out which type of item this is.
			if input.peek2(Token![#]) && !input.peek3(token::Bracket) {
				// If the token after the `$` is `#`, but the token after the
				// `#` is not a square bracket (`[`), then this is a field
				// length. We have to check that there is no square bracket
				// because attributes, which we allow for fields, are started
				// with `#[`.
				Self::FieldLength(input.parse()?)
			} else if input.peek2(token::Bracket) || input.peek2(token::Paren) {
				// If the token after the `$` is a square bracket (`[`) or a
				// normal bracket (`(`), then it is an unused bytes item.
				Self::UnusedBytes(input.parse()?)
			} else {
				// Otherwise, we assume this item is a field item.
				Self::NamedField(input.parse()?)
			}
		} else {
			// Otherwise, if the next token is not `$`, then we only need to
			// look at that next token to find the type of item.
			if input.peek(Token![#]) && !input.peek2(token::Bracket) {
				// If the next token is `#`, but the token after it is not a
				// square bracket (`[`), then this is a field length. We have to
				// check that there is no square bracket because attributes,
				// which we allow for fields, are started with `#[`.
				Self::FieldLength(input.parse()?)
			} else if input.peek(token::Bracket) || input.peek(token::Paren) {
				// If the next token is a square bracket (`[`) or a normal
				// bracket (`(`), then it is an unused bytes item.
				Self::UnusedBytes(input.parse()?)
			} else {
				// Otherwise, we assume this item is a field item.
				Self::NamedField(input.parse()?)
			}
		})
	}
}

impl Parse for UnnamedItem {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(Token![$]) {
			// If the next token is `$`, then we need to look _two_ tokens in
			// advance to find out which type of item this is.
			if input.peek2(Token![#]) && !input.peek3(token::Bracket) {
				// If the token after the `$` is `#`, but the token after the
				// `#` is not a square bracket (`[`), then this is a field
				// length. We have to check that there is no square bracket
				// because attributes, which we allow for fields, are started
				// with `#[`.
				Self::FieldLength(input.parse()?)
			} else if input.peek2(token::Bracket) || input.peek2(token::Paren) {
				// If the token after the `$` is a square bracket (`[`) or a
				// normal bracket (`(`), then it is an unused bytes item.
				Self::UnusedBytes(input.parse()?)
			} else {
				// Otherwise, we assume this item is a field item.
				Self::UnnamedField(input.parse()?)
			}
		} else {
			// Otherwise, if the next token is not `$`, then we only need to
			// look at that next token to find the type of item.
			if input.peek(Token![#]) && !input.peek2(token::Bracket) {
				// If the next token is `#`, but the token after it is not a
				// square bracket (`[`), then this is a field length. We have to
				// check that there is no square bracket because attributes,
				// which we allow for fields, are started with `#[`.
				Self::FieldLength(input.parse()?)
			} else if input.peek(token::Bracket) || input.peek(token::Paren) {
				// If the next token is a square bracket (`[`) or a normal
				// bracket (`(`), then it is an unused bytes item.
				Self::UnusedBytes(input.parse()?)
			} else {
				// Otherwise, we assume this item is a field item.
				Self::UnnamedField(input.parse()?)
			}
		})
	}
}
//     }}}

//     Fields {{{
impl Parse for NamedField {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// Attributes associated with the field.
			attributes: input.call(Attribute::parse_outer)?,
			// The visibility of the field.
			vis: input.parse()?,
			// A metabyte token (`$`) which, if present, declares this item as
			// being intended for the second byte in a message header. Only for
			// message definitions.
			metabyte_token: input.parse().ok(),
			// The name of the field.
			name: input.parse()?,
			// A colon token: `:`.
			colon_token: input.parse()?,
			// The type of the field.
			ty: input.parse()?,
		})
	}
}

impl Parse for UnnamedField {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// Attributes associated with the field.
			attributes: input.call(Attribute::parse_outer)?,
			// The visibility of the field.
			vis: input.parse()?,
			// A metabyte token (`$`) which, if present, declares this item as
			// being intended for the second byte in a message header. Only for
			// message definitions.
			metabyte_token: input.parse().ok(),
			// The type of the field.
			ty: input.parse()?,
		})
	}
}
//     }}}

//     Field length {{{
impl Parse for FieldLength {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// A metabyte token (`$`) which, if present, declares this item as
			// being intended for the second byte in a message header. Only for
			// message definitions.
			metabyte_token: input.parse().ok(),
			// A number sign token: `#`.
			number_sign_token: input.parse()?,
			// An identifier for a field. If it refers to a named field, that
			// means the field's name, otherwise, if it refers to an unnamed
			// field, that means the field's index.
			field_ident: input.parse()?,
			// A colon token: `:`.
			colon_token: input.parse()?,
			// The type used for the `FieldLength`. This should be an unsigned
			// integer type: `u8`, `u16`, `u32`, `u64`, or `u128`.
			ty: input.parse()?,
		})
	}
}
//     }}}

//     Unused bytes {{{
impl Parse for UnusedBytes {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// A metabyte token (`$`) which, if present, declares this item as
			// being intended for the second byte in a message header. Only for
			// message definitions.
			metabyte_token: input.parse().ok(),
			// An actual [`UnusedBytesDefinition`].
			unused_bytes: input.parse()?,
		})
	}
}

impl Parse for UnusedBytesDefinition {
	fn parse(input: ParseStream) -> Result<Self> {
		// Creates a utility for generating error messages if no matching token
		// was found.
		let look = input.lookahead1();

		if look.peek(token::Bracket) {
			// If the next token is a square bracket (`[`), parse as
			// `Full`.
			Ok(Self::Full(input.parse()?))
		} else if look.peek(token::Paren) {
			let paren;

			// Otherwise, if the next token is a bracket (`(`), parse the
			// (empty) pair of brackets. This is a unit: `()`.
			Ok(Self::Unit(parenthesized!(paren in input)))
		} else {
			// If the next token is neither a bracket nor a square bracket,
			// generate an error message.
			Err(look.error())
		}
	}
}

impl Parse for UnusedBytesFull {
	fn parse(input: ParseStream) -> Result<Self> {
		let (content, paren);

		Ok(Self {
			// A pair of square brackets (`[` and `]`).
			bracket_token: bracketed!(content in input),
			// A pair of brackets, in the form of a unit: `()`.
			paren_token: parenthesized!(paren in input),
			// A semicolon: `;`.
			semicolon_token: input.parse()?,
			// An [`UnusedBytesCount`].
			count: input.parse()?,
		})
	}
}

impl Parse for UnusedBytesCount {
	fn parse(input: ParseStream) -> Result<Self> {
		// If the next token is a curly bracket (`{`), parse as `Padding`.
		if input.peek(token::Brace) {
			let content;

			Ok(Self::Padding((braced!(content in input), content.parse()?)))
		} else {
			// Otherwise, if the next token is not a curly bracket, parse as an
			// `Expression`.
			Ok(Self::Expression(input.parse()?))
		}
	}
}
//     }}}

// }}}
