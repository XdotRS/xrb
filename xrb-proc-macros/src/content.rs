// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
	braced, bracketed, parenthesized, token, Attribute, Generics, Ident, LitInt, Result, Token,
	Type, Variant, Visibility,
};

use proc_macro2::{Delimiter, Group, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Content {
	Shorthand(Shorthand),
	Longhand(Longhand),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Shorthand {
	/// An optional item declaration preceded by a colon (`:`).
	///
	/// # Examples
	/// ```
	/// requests! {
	///     // With the optional item declaration:
	///     pub struct DestroyWindow(4): pub target: Window;
	///
	///     // Without:
	///     pub struct GrabServer(36);
	/// }
	/// ```
	pub item: Option<(Token![:], Item)>,
	/// The trailing semicolon (`;`) that is requried for a shorthand definition.
	pub semicolon_token: Token![;],
}

impl Shorthand {
	/// If a [`Field`] is declared, returns `Some` of that [`Field`].
	pub fn field(&self) -> Option<Field> {
		self.item
			.clone()
			.and_then(|declaration| match declaration.1 {
				Item::Field(field) => Some(field),
				_ => None,
			})
	}
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Longhand {
	/// The braces (`{` and `}`) that surround the declared `items`.
	pub brace_token: token::Brace,
	/// A list of [`Item`]s punctuated with commas. The final comma is optional.
	pub items: Punctuated<Item, Token![,]>,
}

impl Longhand {
	/// Returns a list of the [`Field`] items contained within `self.items`.
	pub fn fields(&self) -> Vec<&Field> {
		self.items
			.iter()
			.filter_map(|item| match item {
				Item::Field(field) => Some(field),
				_ => None,
			})
			.collect()
	}

	/// Returns the item within `self.items` that defines the metabyte position,
	/// if any.
	#[allow(dead_code)]
	pub fn metabyte(&self) -> Option<&Item> {
		self.items.iter().find(|item| item.is_metabyte())
	}

	/// Returns `self.items` with metabyte items removed.
	#[allow(dead_code)]
	pub fn items_sans_metabyte(&self) -> Vec<&Item> {
		self.items
			.iter()
			.filter(|item| !item.is_metabyte())
			.collect()
	}
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Item {
	UnusedBytes(UnusedBytes),
	FieldLength(FieldLength),
	Field(Field),
}

impl Item {
	/// Returns whether this [`Item`] is defined for the metabyte position.
	pub fn is_metabyte(&self) -> bool {
		match self {
			// If a `field` item has a metabyte token, it is a metabyte item.
			Self::Field(field) => field.metabyte_token.is_some(),

			// If a `field_len` item has a metabyte token, it is a metabyte item.
			Self::FieldLength(field_len) => field_len.metabyte_token.is_some(),

			Self::UnusedBytes(unused) => match unused {
				// If a single `unused` byte item has a metabyte token, it is a
				// metabyte item.
				UnusedBytes::Single((metabyte_token, _)) => metabyte_token.is_some(),
				// A fully specified unused bytes item can't have a metabyte
				// token.
				UnusedBytes::FullySpecified(_) => false,
			},
		}
	}

	pub fn size(&self) -> TokenStream2 {
		match self {
			// Unused bytes
			Self::UnusedBytes(unused) => match unused {
				UnusedBytes::Single(_) => quote!(1),

				UnusedBytes::FullySpecified(def) => match &def.definition {
					UnusedBytesDefinition::Numerical(val) => quote!(#val),

					UnusedBytesDefinition::Padding((_, name)) => quote! {
						4 - (cornflakes::ByteSize::byte_size(&self.#name) % 4) % 4
					},
				},
			},

			// Fields
			Self::Field(field) => {
				let name = &field.name;

				quote! {
					cornflakes::ByteSize::byte_size(&self.#name)
				}
			}

			// Field lengths
			Self::FieldLength(field_len) => field_len.size.map_or(quote!(1), |size| {
				let size = size.1;
				quote!(#size)
			}),
		}
	}
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Field {
	/// Attributes, including doc comments. Ex: `#[error("unsupported feature")]`
	pub attributes: Vec<Attribute>,
	/// Visibility. Ex: `pub`.
	pub vis: Visibility,
	/// `$`. Indicates that this field should be placed in the metabyte.
	pub metabyte_token: Option<Token![$]>,
	/// Name. Ex: `mode`.
	pub name: Ident,
	/// `:`.
	pub colon_token: Token![:],
	/// An optional 'inline' enum definition.
	pub enum_definition: Option<Enum>,
	/// Type. Ex: `u32`.
	pub ty: Type,
}

/// `#fieldname` or `#$fieldname` or `#fieldname[2]`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FieldLength {
	/// `#`.
	pub number_sign_token: Token![#],
	/// `$`. Indicates that this data should be placed in the metabyte.
	pub metabyte_token: Option<Token![$]>,
	/// The name of the field which this reads or writes the length of.
	pub field_name: Ident,
	/// The byte size of the field length.
	pub size: Option<(token::Bracket, usize)>,
}

/// `$()` or `()` or `[(); definition]`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum UnusedBytes {
	/// `$()` or `()`.
	Single((Option<Token![$]>, token::Paren)),
	/// `[(); definition]`.
	FullySpecified(FullUnusedBytes),
}

/// `[(); definition]`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FullUnusedBytes {
	/// `[` and `]`.
	pub bracket_token: token::Bracket,
	/// `()`.
	pub unit_token: token::Paren,
	/// `;`.
	pub semicolon_token: Token![;],
	/// The definition of how many unused bytes there is.
	pub definition: UnusedBytesDefinition,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum UnusedBytesDefinition {
	/// A specific, unvarying number of bytes.
	///
	/// `22`.
	Numerical(usize),
	/// The number of bytes required to pad the size of a field to a multiple
	/// of 4 bytes.
	///
	/// For example, this might be used with a list of one-byte values, like a
	/// `String8`, to ensure that the message is a multiple of 4 bytes.
	///
	/// `{fieldname}`.
	Padding((token::Brace, Ident)),
}

/// Allows an enum to be defined within the type of a field in a message for
/// convenience.
///
/// A lot of requests define enums that are only used for the purpose of that
/// request only, so it is convenient to be able to define those enums directly
/// within the request.
///
/// This is different from the [`ItemEnum`] in [`syn`] in that it doesn't allow
/// attributes. This is because it is an 'inline' definition, and the attributes
/// apply to the field, not the `enum`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Enum {
	/// Visibility. Ex: `pub`.
	pub vis: Visibility,
	/// `enum`.
	pub enum_token: Token![enum],
	/// Name. Ex: `Mode`.
	pub name: Ident,
	/// Generics. Ex: `<'a, T>`.
	pub generics: Generics,
	/// `{` and `}`.
	pub brace_token: token::Brace,
	/// A comma-separated list of variants.
	pub variants: Punctuated<Variant, Token![,]>,
}

// Expansion {{{
// This is the conversion of some of these structures 'back into' tokens, for
// use when the code is generated by macros. For example, the `ToTokens`
// definition for `Enum` generates an actual enum definition.

impl ToTokens for Enum {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		let name = &self.name;

		let generics = &self.generics;
		let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

		// `pub`
		self.vis.to_tokens(tokens);
		// `enum`
		self.enum_token.to_tokens(tokens);
		// `Mode`
		name.to_tokens(tokens);
		// <'a, T>
		generics.to_tokens(tokens);
		// { Variant1, Variant2 }
		tokens.append(Group::new(
			Delimiter::Brace,
			self.variants.to_token_stream(),
		));

		quote! {
			impl #impl_generics cornflakes::StaticByteSize for #name #ty_generics #where_clause {
				fn static_byte_size() -> usize {
					1
				}
			}
		}
		.to_tokens(tokens);
	}
}

impl ToTokens for Item {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Field(field) => field.to_tokens(tokens),
			Self::FieldLength(field_len) => field_len.to_tokens(tokens),
			Self::UnusedBytes(unused) => unused.to_tokens(tokens),
		}
	}
}

impl ToTokens for Field {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Attributes, including doc comments.
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// Visibility. Ex: `pub`.
		self.vis.to_tokens(tokens);
		// Name. Ex: `mode`.
		self.name.to_tokens(tokens);
		// `:`.
		self.colon_token.to_tokens(tokens);
		// Type. Ex: `u32`.
		self.ty.to_tokens(tokens);
	}
}

impl Field {
	#[allow(dead_code)]
	pub fn write_to(&self, tokens: &mut TokenStream2) {
		// Field name.
		let name = &self.name;
		// Field type.
		let ty = &self.ty;

		quote! {
			// Equivalent to:
			// ```
			// writer.write(self.#name)?;
			// ```
			<#ty as cornflakes::ToBytes>::write_to(self.#name, writer)?;
		}
		.to_tokens(tokens);
	}
}

impl ToTokens for FieldLength {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Field name.
		let name = &self.field_name;
		// The numerical type to represent the length with.
		let ty = self.size.map_or_else(
			|| quote!(u8),
			|size| match size.1 {
				1 => quote!(u8),
				2 => quote!(u16),
				4 => quote!(u32),
				8 => quote!(u64),
				16 => quote!(u128),
				_ => panic!("expected field length byte size of 1, 2, 4, 8, or 16"),
			},
		);

		quote! {
			// Equivalent to:
			// ```
			// writer.write(self.#name.len() as #ty)?;
			// ```
			<#ty as cornflakes::ToBytes>::write_to(self.#name.len() as #ty, writer)?;
		}
		.to_tokens(tokens);
	}
}

impl ToTokens for UnusedBytes {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// For reference, unused bytes are not necessarily zero. We could have
		// picked any number to write here instead of 0.

		match self {
			// If it is a single unused byte, it is just a single [`u8`] value
			// that we need to write.
			Self::Single(_) => quote!(writer.write_u8(0);),

			Self::FullySpecified(unused) => match &unused.definition {
				UnusedBytesDefinition::Numerical(val) => quote! {
					writer.write_many(0, #val);
				},
				UnusedBytesDefinition::Padding(padding) => {
					let name = &padding.1;

					quote! {
						// Equivalent to:
						// ```
						// writer.write_many(0, 4 - (self.#name.byte_size() % 4) % 4)?;
						// ```
						writer.write_many(0,
							// `4 - (size % 4) % 4` calculates the number of
							// bytes requried to pad any given `size` up to the
							// nearest multiple of 4.
							4 - (cornflakes::ByteSize::byte_size(self.#name) % 4) % 4
						)?;
					}
				}
			},
		}
		.to_tokens(tokens);
	}
}

// }}}

// Parsing {{{

impl Parse for Content {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(token::Brace) {
			Self::Longhand(input.parse()?)
		} else {
			Self::Shorthand(input.parse()?)
		})
	}
}

impl Parse for Shorthand {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// Optional: `(Token![:], Item)`:
			item: input.parse().ok().zip(input.parse().ok()),
			// `;`.
			semicolon_token: input.parse()?,
		})
	}
}

impl Parse for Longhand {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		Ok(Self {
			brace_token: braced!(content in input),
			items: content.parse_terminated(Item::parse)?,
		})
	}
}

impl Parse for Item {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(Token![#]) && !input.peek2(token::Bracket) {
			Self::FieldLength(input.parse()?)
		} else if input.peek(Token![$]) || input.peek(token::Paren) || input.peek(token::Bracket) {
			Self::UnusedBytes(input.parse()?)
		} else {
			Self::Field(input.parse()?)
		})
	}
}

impl Parse for Field {
	fn parse(input: ParseStream) -> Result<Self> {
		// Attributes, including doc comments.
		let attributes = input.call(Attribute::parse_outer)?;
		// Visibility. Ex: `pub`.
		let vis: Visibility = input.parse()?;
		// Optional: `$`.
		let metabyte_token: Option<Token![$]> = input.parse().ok();
		let name: Ident = input.parse()?;
		let colon_token: Token![:] = input.parse()?;

		let enum_definition: Option<Enum> = input.parse().ok();
		let ty: Type = if enum_definition.is_some() {
			Type::Verbatim(enum_definition.as_ref().unwrap().name.to_token_stream())
		} else {
			input.parse()?
		};

		Ok(Self {
			attributes,
			vis,
			metabyte_token,
			name,
			colon_token,
			enum_definition,
			ty,
		})
	}
}

impl Parse for FieldLength {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		Ok(Self {
			// `#`.
			number_sign_token: input.parse()?,
			// Optional: `$`.
			metabyte_token: input.parse().ok(),
			// Name of the field which `self` represents the length of.
			field_name: input.parse()?,
			// Optional: `[size]`.
			size: if input.peek(token::Bracket) {
				Some((
					// `[` and `]`.
					bracketed!(content in input),
					// Parse [`usize`].
					content.parse::<LitInt>()?.base10_parse()?,
				))
			} else {
				None
			},
		})
	}
}

impl Parse for UnusedBytes {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(token::Bracket) {
			// `[(); definition]`
			Self::FullySpecified(input.parse()?)
		} else {
			// `()`
			let _paren;
			Self::Single((input.parse().ok(), parenthesized!(_paren in input)))
		})
	}
}

impl Parse for FullUnusedBytes {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;
		let _paren;

		Ok(Self {
			// `[` and `]`.
			bracket_token: bracketed!(content in input),
			// `()`.
			unit_token: parenthesized!(_paren in content),
			// `;`.
			semicolon_token: content.parse()?,
			definition: content.parse()?,
		})
	}
}

impl Parse for UnusedBytesDefinition {
	fn parse(input: ParseStream) -> Result<Self> {
		let look = input.lookahead1();

		if look.peek(token::Brace) {
			// `{` and `}`
			let content;

			Ok(Self::Padding((braced!(content in input), content.parse()?)))
		} else if look.peek(LitInt) {
			// `4`, `22`, etc.
			Ok(Self::Numerical(input.parse::<LitInt>()?.base10_parse()?))
		} else {
			// Otherwise, construct the error:
			Err(look.error())
		}
	}
}

impl Parse for Enum {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		Ok(Self {
			// Visibility. Ex: `pub`.
			vis: input.parse()?,
			// `enum`.
			enum_token: input.parse()?,
			// Name.
			name: input.parse()?,
			// Generics.
			generics: input.parse()?,
			// `{` and `}`.
			brace_token: braced!(content in input),
			// Variants.
			variants: content.parse_terminated(Variant::parse)?,
		})
	}
}

// }}}
