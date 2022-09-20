// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, token, Attribute, Expr, Generics, Ident, Result, Token, Visibility};

use proc_macro2::{Delimiter, Group, TokenStream as TokenStream2};
use quote::{ToTokens, TokenStreamExt};

use crate::content::*;

/// A definition for either a [`Struct`] or an [`Enum`] that can contain field
/// lengths and unused bytes.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Definition {
	/// Attributes associated with the [`Struct`] or [`Enum`] being defined.
	pub attributes: Vec<Attribute>,
	/// The visibility associated with the [`Struct`] or [`Enum`] being defined.
	///
	/// For example, this might be `pub`.
	pub vis: Visibility,
	/// The rest of the definition: either a [`Struct`] or an [`Enum`].
	pub definition: DefinitionType,
	/// The semicolon token, if the definition's items are not [`Items::Named`]:
	/// `;`.
	pub semicolon_token: Option<Token![;]>,
}

/// Either a [`Struct`] [`Definition`] or an [`Enum`] [`Definition`].
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum DefinitionType {
	/// A struct [`Definition`] that can contain field lengths and unused bytes.
	Struct(Struct),
	/// An enum [`Definition`] that can contain field lengths and unused bytes.
	Enum(Enum),
}

/// A struct [`Definition`] that can contain field lengths and unused bytes.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Struct {
	/// A `struct` token.
	pub struct_token: Token![struct],
	/// The name of the struct.
	pub name: Ident,
	/// Generics associated with the struct.
	///
	/// This includes generic types and lifetimes.
	pub generics: Generics,
	/// Items defined for this struct.
	///
	/// This includes fields that you may usually define within a struct, but
	/// also _field lengths_, to encode the lengths of collection fields (e.g.
	/// a field with a `Vec` type), an _unused bytes_, to define bytes that will
	/// be unused when serializing or deserializing the struct.
	pub items: Items,
}

/// An enum [`Definition`] that can contain field lengths and unused bytes.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Enum {
	/// An `enum` token.
	pub enum_token: Token![enum],
	/// The name of the enum.
	pub name: Ident,
	/// Generics associated with the enum.
	///
	/// This includes generic types and lifetimes.
	pub generics: Generics,
	/// Curly bracket tokens (`{` and `}`).
	pub brace_token: token::Brace,
	/// A list of [`Variant`]s, punctuated by commas (`,`).
	pub variants: Punctuated<Variant, Token![,]>,
}

/// A variant within an [`Enum`] definition.
///
/// The difference between this and a [`syn::Variant`] is that it can use
/// [`Content`] to define field lengths and unused bytes for the serialization
/// and deserialization of the variant.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Variant {
	/// Attributes associated with the enum variant.
	///
	/// This includes doc comments.
	pub attributes: Vec<Attribute>,
	/// The name of the enum variant.
	pub name: Ident,
	/// Items defined for this variant, if any.
	pub items: Items,
	/// The discriminant associated with the enum variant.
	///
	/// If specified, this is what the enum variant is encoded as, followed by
	/// its [`Content`], if any. Otherwise, if not specified, the enum variant
	/// will be encoded as the previous variant's discriminant plus one.
	///
	/// # Example
	/// This `InternetV6` variant will be encoded as `6`.
	/// ```
	/// InternetV6 = 6
	/// ```
	pub discriminant: Option<(Token![=], Expr)>,
}

// Expansion {{{

impl ToTokens for Definition {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Attributes associated with the struct or enum being defined,
		// including doc comments.
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// The visibility of the struct or enum being defined (e.g. `pub`).
		self.vis.to_tokens(tokens);
		// The rest of the definition of the struct or enum being defined.
		self.definition.to_tokens(tokens);
		// The semicolon token, if present.
		self.semicolon_token.to_tokens(tokens);
	}
}

impl ToTokens for DefinitionType {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			// Struct definitions.
			Self::Struct(def) => def.to_tokens(tokens),
			// Enum definitions.
			Self::Enum(def) => def.to_tokens(tokens),
		}
	}
}

impl ToTokens for Struct {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// `struct`.
		self.struct_token.to_tokens(tokens);
		// The name of the struct.
		self.name.to_tokens(tokens);
		// Generics associated with the struct.
		self.generics.to_tokens(tokens);
		// Items defined within the struct.
		self.items.to_tokens(tokens);
	}
}

impl ToTokens for Enum {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// `enum`.
		self.enum_token.to_tokens(tokens);
		// The name of the enum.
		self.name.to_tokens(tokens);
		// Generics associated with the enum.
		self.generics.to_tokens(tokens);

		tokens.append(Group::new(
			// Curly brackets (`{` and `}`).
			Delimiter::Brace,
			// The enum's variants punctuated with commas.
			self.variants.to_token_stream(),
		));
	}
}

impl ToTokens for Variant {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// The variant's attributes, if any.
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// The name of the variant.
		self.name.to_tokens(tokens);

		self.items.to_tokens(tokens);

		// The variant's discriminant, if any.
		self.discriminant.as_ref().map(|(eq, discrim)| {
			// `=`.
			eq.to_tokens(tokens);
			// The discriminant itself; an expression.
			discrim.to_tokens(tokens);
		});
	}
}

// }}}

// Parsing {{{

impl Parse for Definition {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			attributes: input.call(Attribute::parse_outer)?,
			vis: input.parse()?,
			definition: input.parse()?,
			semicolon_token: input.parse().ok(),
		})
	}
}

impl Parse for DefinitionType {
	fn parse(input: ParseStream) -> Result<Self> {
		// This is a utility that lets us peek at the next token for conditional
		// parsing, and generates errors if the none of the desired tokens were
		// found.
		let look = input.lookahead1();

		if look.peek(Token![struct]) {
			// If the next token is a `struct` token, parse as a `Struct`.
			Ok(Self::Struct(input.parse()?))
		} else if look.peek(Token![enum]) {
			// If the next token is an `enum` token, parse as an `Enum`.
			Ok(Self::Enum(input.parse()?))
		} else {
			// Otherwise, construct an error that says something along the lines
			// of "expected `struct` or `enum`, found ...".
			Err(look.error())
		}
	}
}

impl Parse for Struct {
	fn parse(input: ParseStream) -> Result<Self> {
		// `struct`.
		let struct_token: Token![struct] = input.parse()?;
		// The name of the struct.
		let name: Ident = input.parse()?;
		// Generics associated with the struct, including generic types and
		// lifetimes.
		let generics: Generics = input.parse()?;
		// Items defined within the struct.
		let items: Items = input.parse()?;

		match items {
			Items::Named(_) => {
				if input.peek(Token![;]) {
					return Err(input.error("did not expect semicolon after named items"));
				}
			}
			Items::Unnamed(_) => {
				if !input.peek(Token![;]) {
					return Err(input.error("expected semicolon after unnamed items"));
				}
			}
			Items::Unit => {
				if !input.peek(Token![;]) {
					return Err(input.error("expected semicolon after unit struct"));
				}
			}
		}

		Ok(Self {
			struct_token,
			name,
			generics,
			items,
		})
	}
}

impl Parse for Enum {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		Ok(Self {
			// `enum`.
			enum_token: input.parse()?,
			// The name of the enum.
			name: input.parse()?,
			// Generics associated with the enum, including generic types and
			// lifetimes.
			generics: input.parse()?,
			// Curly brackets (`{` and `}`).
			brace_token: braced!(content in input),
			// The enum's variants, separated by commas (`,`).
			variants: content.parse_terminated(Variant::parse)?,
		})
	}
}

impl Parse for Variant {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// Attributes, including doc comments.
			attributes: input.call(Attribute::parse_outer)?,
			// Name of the enum variant.
			name: input.parse()?,
			// Items defined within the enum variant, if any.
			items: input.parse()?,
			// Optional: `(Token![=], Expr)`.
			//
			// The enum variant's discrimination, if any (e.g.
			// `InternetV6 = 6`).
			discriminant: input.parse().ok().zip(input.parse().ok()),
		})
	}
}

// }}}
