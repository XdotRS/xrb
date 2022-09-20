// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::{Delimiter, Group, TokenStream as TokenStream2};
use quote::{ToTokens, TokenStreamExt};

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
	braced, bracketed, parenthesized, token, Attribute, Error, Ident, Result, Token, Type,
	Visibility,
};

use crate::closure::*;

// Items {{{

/// Represents the definition of [`Named`] or [`Unnamed`] items, if any.
///
/// [`Named`]: Items::Named
/// [`Unnamed`]: Items::Unnamed
%[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
	/// An iterator over the defined [`Item`]s. May be empty.
	pub fn iter(&self) -> syn::punctuated::Iter<Item> {
		match self {
			Self::Named(items) => items.iter(),
			Self::Unnamed(items) => items.iter(),
			Self::Unit => Punctuated::<Item, Token![,]>::new().iter(),
		}
	}

	/// Returns a [`Vec`] of [`Field`]s defined within `self`.
	pub fn fields(&self) -> Vec<&Field> {
		self.iter()
			.filter_map(|item| match item {
				Item::Field(field) => Some(field),
				_ => None,
			})
			.collect()
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

	/// Gets the defined metabyte [`Item`], if there is any.
	pub fn get_metabyte(&self) -> Option<&Item> {
		match self {
			Self::Named(items) => items.get_metabyte(),
			Self::Unnamed(items) => items.get_metabyte(),
			Self::Unit => None,
		}
	}

	/// Returns the number of metabyte [`Item`]s defined within `self`.
	pub fn metabyte_items_count(&self) -> usize {
		self.iter().filter(|item| item.is_metabyte()).count()
	}

	/// Returns a [`Vec`] of [`Item`]s without any metabyte [`Item`]s.
	pub fn sans_metabyte(&self) -> Vec<&Item> {
		self.iter().filter(|item| !item.is_metabyte()).collect()
	}

	/// Returns whether any [`Item`] within `self` is defined for the 'metabyte
	/// position'.
	///
	/// The 'metabyte position' is the second byte of the header in a message.
	/// It does not exist for non-message structs and enums.
	pub fn has_metabyte(&self) -> bool {
		self.get_metabyte().is_some()
	}
}

/// A list of [`Item`] definitions punctuated by commas and surrounded by curly
/// brackets.
///
/// `NamedItems` can contain [`NamedField`]s, but not [`UnnamedField`]s.
%[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct NamedItems {
	/// Named items are surrounded by curly brackets (`{` and `}`).
	pub brace_token: token::Brace,
	/// A list of [`Item`] definitions, punctuated by commas (`,`).
	///
	/// The final comma is optional.
	pub items: Punctuated<Item, Token![,]>,
}

impl<'a> NamedItems {
	/// An iterator over the contained [`Item`]s.
	pub fn iter(&self) -> syn::punctuated::Iter<Item> {
		self.items.iter()
	}

	/// Returns a <code>[Vec]<&'a [Field]></code> of any [`Field`]s contained
	/// within `self`.
	pub fn fields(&'a self) -> Vec<&'a Field> {
		self.iter()
			.filter_map(|item| match item {
				Item::Field(field) => Some(field),
				_ => None,
			})
			.collect()
	}

	/// Gets the contained metabyte [`Item`], if any.
	pub fn get_metabyte(&self) -> Option<&Item> {
		self.iter().find(|item| item.is_metabyte())
	}

	/// Returns whether any [`Item`] within `self` is defined for the 'metabyte
	/// position'.
	///
	/// The 'metabyte position' is the second byte of the header in a message.
	/// It does not exist for non-message structs and enums.
	pub fn has_metabyte(&self) -> bool {
		self.get_metabyte().is_some()
	}
}

/// A list of unnamed [`Item`]s punctuated by commas and surrounded by normal
/// brackets.
///
/// `UnnamedItems` can contain unnamed [`Field`]s, but not named [`Field`]s. The
/// syntax for `UnnamedItems` may be familiar to you under names such as 'tuple
/// structs' and 'tuple variants'.
%[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct UnnamedItems {
	/// Unnamed items are surrounded by normal brackets (`(` and `)`).
	pub paren_token: token::Paren,
	/// A list of unnamed [`Item`]s, punctuated by commas (`,`).
	///
	/// The final comma is optional.
	pub items: Punctuated<Item, Token![,]>,
}

impl<'a> UnnamedItems {
	/// An iterator over the contained [`Item`]s.
	pub fn iter(&self) -> syn::punctuated::Iter<Item> {
		self.items.iter()
	}

	/// Returns a <code>[Vec]<&'a [UnnamedField]></code> of any
	/// [`UnnamedField`]s contained within `self`.
	pub fn fields(&'a self) -> Vec<&'a Field> {
		self.iter()
			.filter_map(|item| match item {
				Item::Field(field) => Some(field),
				_ => None,
			})
			.collect()
	}

	/// Gets the metabyte defined within `self`, if any.
	pub fn get_metabyte(&self) -> Option<&Item> {
		self.iter().find(|item| item.is_metabyte())
	}

	/// Returns whether any [`Item`] within `self` is defined for the 'metabyte
	/// position'.
	///
	/// The 'metabyte position' is the second byte of the header in a message.
	/// It does not exist for non-message structs and enums.
	pub fn has_metabyte(&self) -> bool {
		self.get_metabyte().is_some()
	}
}

/// Either an [`UnusedBytes`] item, a [`FieldLength`] item, or a [`NamedField`].
%[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Item {
	/// This is equivalent to a [`syn::Field`], but it can have a metabyte token
	/// (`%`).
	Field(Field),
	LetVar(LetVar),
	/// An [`UnusedBytes`] item, representing bytes that are unused in
	/// serialization and deserialization.
	UnusedBytes(UnusedBytes),
}

impl Item {
	/// Returns whether this `Item` is defined for the 'metabyte position'.
	///
	/// The 'metabyte position' is the second byte of the header in a message.
	/// It does not exist for non-message structs and enums.
	pub fn is_metabyte(&self) -> bool {
		match self {
			Self::UnusedBytes(unused_bytes) => unused_bytes.is_metabyte(),
			Self::LetVar(let_var) => let_var.is_metabyte(),
			Self::Field(field) => field.is_metabyte(),
		}
	}
}

// }}}

// Fields {{{

/// A field.
///
/// This is like a [`syn::Field`], but it can have a `metabyte_token`.
%[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Field {
	/// Attributes associated with the field.
	pub attributes: Vec<Attribute>,
	/// The visibility of the field.
	pub vis: Visibility,
	/// A metabyte token used in [`messages!`] to indicate the second byte of
	/// the message header.
	///
	/// If the `Field` is contained within the definition of a message, and an
	/// item in that message has already been defined for the metabyte, or a
	/// minor opcode is defined for that message, then the presence of this
	/// token will generate an error.
	///
	/// If the `Field` is not contained within the definition of a message (i.e.
	/// if the [`define!`] macro is used), then the presence of this token will
	/// also generate an error.
	///
	/// [`messages!`]: crate::messages
	/// [`define!`]: crate::define
	pub metabyte_token: Option<Token![%]>,
	/// The name of the field, if it is a named field.
	pub name: Option<Ident>,
	/// A colon token: `:`, if it is a named field.
	pub colon_token: Option<Token![:]>,
	/// The type of the field.
	pub ty: Type,
}

impl Field {
	/// Returns whether this `Field` is defined for the 'metabyte position'.
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
%[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
	pub metabyte_token: Option<Token![%]>,
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
%[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum UnusedBytesDefinition {
	/// A single unused byte with the unit syntax (`()` or `%()`).
	Unit(token::Paren),
	/// The full unused bytes syntax (`[(); count]`).
	Full(UnusedBytesFull),
}

/// A full definition of [`UnusedBytes`].
%[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
	pub count: IdentClosure,
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
		// Write any and all named `Field`s, surrounded by curly brackets (`{`
		// and `}`).
		tokens.append(Group::new(Delimiter::Brace, self.items.to_token_stream()))
	}
}

impl ToTokens for UnnamedItems {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Write any and all unnamed `Field`s, surrounded by normal brackets
		// (`(` and `)`).
		tokens.append(Group::new(
			Delimiter::Parenthesis,
			self.items.to_token_stream(),
		))
	}
}

impl ToTokens for Item {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			// If this item is a `Field`, write it.
			Self::Field(field) => field.to_tokens(tokens),
			// Otherwise, if it isn't a `Field`, don't write it.
			_ => (),
		}
	}
}

impl ToTokens for Field {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Attributes associated with the field.
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// The visibility of the field.
		self.vis.to_tokens(tokens);
		// The name of the field, if any.
		self.name.to_tokens(tokens);
		// The colon token: `:`, if any.
		self.colon_token.to_tokens(tokens);
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

		// Parse the `Item`s separated by commas and surrounded by curly
		// brackets (`{` and `}`).
		let this = Self {
			brace_token: braced!(content in input),
			items: input.parse_terminated(Item::parse)?,
		};

		// Make sure every field is named.
		for item in this.iter() {
			match item {
				// For each field:
				Item::Field(field) => {
					if field.name.is_none() {
						// If no field name was found, return an error.
						Err(Error::new(
							field.name.expect("we checked for this already").span(),
							"expected field name for named field",
						))
					} else {
						// If a field name was found, it's `Ok`.
						Ok(())
					}
				}
				// If this is an unused bytes item or a field length, it's `Ok`.
				_ => Ok(()),
			}?
		}

		Ok(this)
	}
}

impl Parse for UnnamedItems {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		// Parse `Self`:
		let this = Self {
			// Normal brackets surrounding the items (`(` and `)`).
			paren_token: parenthesized!(content in input),
			// The item definitions themselves, separated by commas.
			items: input.parse_terminated(Item::parse)?,
		};

		// Make sure that there are no named fields.
		for item in this.iter() {
			match item {
				// Check against every field:
				Item::Field(field) => {
					if field.name.is_some() {
						// If a field name was found, return an error.
						Err(Error::new(
							field.name.expect("we already checked for this").span(),
							"no name expected in unnamed field",
						))
					} else if field.colon_token.is_some() {
						// If a colon token (`:`) was found, return an error.
						Err(Error::new(
							field.colon_token.expect("we already checked for this").span,
							"no colon expected in unnamed field",
						))
					} else {
						// Otherwise, if there was no name nor colon found, it's
						// `Ok`.
						Ok(())
					}
				}
				// Unused bytes and field lengths are fine.
				_ => Ok(()),
			}?
		}

		Ok(this)
	}
}

impl Parse for Item {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(Token![%]) {
			// If the next token is `%`, then we need to look _two_ tokens in
			// advance to find out which type of item this is. We also know it
			// isn't a let-variable item: let-variable items always require the
			// keyword `let` to come before the metabyte token (`%`).
			if input.peek2(token::Bracket) || input.peek2(token::Paren) {
				// If the token after the `%` is a square bracket (`[`) or a
				// normal bracket (`(`), then it is an unused bytes item.
				Self::UnusedBytes(input.parse()?)
			} else {
				// Otherwise, we assume this item is a field item.
				Self::Field(input.parse()?)
			}
		} else {
			// Otherwise, if the next token is not `%`, then we only need to
			// look at that next token to find the type of item.
			if input.peek(Token![let]) {
				// If the token after the `%` is `let`, then it is a
				// let-variable item.
				Self::LetVar(input.parse()?)
			} else if input.peek(token::Bracket) || input.peek(token::Paren) {
				// If the next token is a square bracket (`[`) or a normal
				// bracket (`(`), then it is an unused bytes item.
				Self::UnusedBytes(input.parse()?)
			} else {
				// Otherwise, we assume this item is a field item.
				Self::Field(input.parse()?)
			}
		})
	}
}

//     }}}

//     Field {{{
impl Parse for Field {
	fn parse(input: ParseStream) -> Result<Self> {
		// Attributes associated with the field.
		let attributes = input.call(Attribute::parse_outer)?;
		// The visibility of the field.
		let vis: Visibility = input.parse()?;
		// A metabyte token (`%`) which, if present, declares this item as
		// being intended for the second byte in a message header. Only for
		// message definitions.
		let metabyte_token: Option<Token![%]> = input.parse().ok();

		// Attempt to parse the field name.
		let name: Option<Ident> = input.parse().ok();
		// Only attempt to read a colon if a name was found.
		let colon_token: Option<Token![:]> = name.map(|_| input.parse().ok()).flatten();

		// If a field name was found but a colon was not, return an error.
		if name.is_some() && !colon_token.is_some() {
			return Err(input.error("expected colon after field name"));
		}

		//  The type of the field.
		let ty = input.parse()?;

		Ok(Self {
			attributes,
			vis,
			metabyte_token,
			name,
			colon_token,
			ty,
		})
	}
}
//     }}}

//     Field length {{{
impl Parse for FieldLength {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// A metabyte token (`%`) which, if present, declares this item as
			// being intended for the second byte in a message header. Only for
			// message definitions.
			metabyte_token: input.parse().ok(),
			// A number sign token: `%`.
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
			// A metabyte token (`%`) which, if present, declares this item as
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
			count: input.parse()?,
		})
	}
}
//     }}}

// }}}
