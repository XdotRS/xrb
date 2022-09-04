// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
	braced, bracketed, parenthesized, token, Attribute, Expr, Ident, LitInt, Result, Token, Type,
	Visibility,
};

use proc_macro2::{Delimiter, Group, Punct, Spacing, Span, TokenStream as TokenStream2};
use quote::{ToTokens, TokenStreamExt};

/// The definition of the content of a message.
///
/// This is typically fields associated with the message, but also includes
/// other information about the encoding of the message: unused bytes and
/// lengths of list fields.
#[derive(Clone)]
pub struct Content {
	/// A metabyte item is an item that is stored in the 'metabyte'.
	///
	/// That metabyte is the second byte of the request or reply header. For a
	/// request, the metabyte will be unavailable if a minor opcode is defined.
	///
	/// This metabyte is special in that it is a single byte: that means that
	/// any items defined for the metabyte position must be _exactly_ one byte
	/// in size.
	///
	/// You can specify the metabyte item using the `$` token preceding a field
	/// name or field list length encoding.
	///
	/// # Examples
	/// ```
	/// requests! {
	///     pub struct SetPointerMapping(116) -> SetPointerMappingReply {
	///         $#map, // the length of the `map` list in metabyte position
	///         pub map: [u8],
	///         ()[padding(map)],
	///     }
	/// }
	/// ```
	pub metabyte: Option<Item>,
	/// All items defined in the message, other than the metabyte item.
	pub items: Vec<Item>,
}

/// An item definition in the content of a message.
///
/// While the content of a message is typically fields, it can also include the
/// length of other fields or unused bytes.
#[derive(Clone)]
pub enum Item {
	/// Unused bytes within a message.
	///
	/// These are written as empty bytes when serializing, and skipped when
	/// deserializing.
	UnusedBytes(UnusedByteSize),
	/// Encodes the length of a collection field.
	///
	/// This is used to read and write the length of a list in a message, as
	/// defined in the X11 protocol. The length of lists does not accompany
	/// the lists themselves directly in X11, so we must specify the location
	/// where that length is encoded.
	FieldLength {
		/// Whether this is the definition of a 'metabyte' list length encoding.
		///
		/// The metabyte is the second byte in the header of a request or reply.
		/// Only one [`Item`] can be contained in the metabyte position, and it
		/// must be exactly one byte in length; no more, no less.
		///
		/// For the field length, that means that it _must_ specify its
		/// `length_type` if it is declared as a metabyte length encoding, as it
		/// must be specified as a numerical type that is one byte: `u8`.
		metabyte: bool,
		/// The name of the field that this list length encoding is writing the
		/// length of.
		name: Ident,
		/// The numerical type used for this list length encoding.
		///
		/// If not provided, this will default to [`u16`]. It is the type used
		/// to encode the length of the list field. This type determines the
		/// byte size of the list length encoding (e.g. `u8` for 1 byte, `u16`
		/// for 2 bytes, `u32` for 4 bytes...).
		length_type: Option<Type>,
	},
	/// A field like in any other struct definition, but with a special ability...
	///
	/// That special ability being that enums can be defined 'inline' in one of
	/// these fields for convenience, as they are used in quite a few messages.
	Field {
		/// Whether this is the definition of a 'metabyte' field.
		///
		/// The metabyte is the second byte in the header of a request or reply.
		/// Only one [`Item`] can be contained in the metabyte position, and it
		/// must be exactly one byte in length; no more, no less.
		metabyte: bool,
		/// Attributes associated with the field.
		attributes: Vec<Attribute>,
		/// The visibility of the field (e.g. `pub`).
		vis: Option<Visibility>,
		/// The name of the field.
		name: Ident,
		/// The type of the field.
		///
		/// If an enum is defined, this will be set to the name of that defined
		/// enum.
		ty: Type,
		/// An optional enum definition in place of a type, for convenience.
		///
		/// For example:
		/// ```
		/// pub $ordering: enum Ordering {
		///     Unsorted = 0,
		///     Ysorted = 1,
		///     YxSorted = 2,
		///     YxBanded = 3,
		/// }
		/// ```
		enum_definition: Option<Enum>,
	},
}

/// The number of unused bytes. Can be an absolute value, or it can pad a field.
#[derive(Clone)]
pub enum UnusedByteSize {
	/// An absolute number of bytes.
	Number(u8),
	/// The dynamically calculated number of bytes requried to pad the byte size
	/// of a field to a multiple of 4.
	Padding(Ident),
}

/// Allows an enum to be defined within the type of a field in a message for
/// convenience.
///
/// A lot of requests define enums that are only used for the purpose of that
/// request only, so it is convenient to be able to define those enums directly
/// within the request.
#[derive(Clone)]
pub struct Enum {
	/// The name of this enum definition.
	///
	/// This is not unique to the message this enum definition is defined within.
	/// Make sure it does not conflict with any other names that are in scope.
	/// This will also be accessible in entire module's scope, and will be made
	/// public - that does mean that you can define an enum in one message and
	/// then use it in another.
	pub name: Ident,
	/// The variants defined by this enum definition, separated by commas.
	///
	/// Each variant definition is required to define its value to be written
	/// when it is serialized. For example:
	/// ```rust
	/// enum Ordering {
	///     Unsorted = 0, // ok
	///     Ysorted = 1,  // ok
	///     YxSorted = 2, // ok
	///     YxBanded,     // error
	/// }
	/// ```
	pub variants: Punctuated<Variant, Token![,]>,
}

#[derive(Clone)]
/// An enum variant with a name and value used to serialize it.
pub struct Variant {
	pub attributes: Vec<Attribute>,
	/// The name of this variant (e.g. `XySorted`).
	pub name: Ident,
	/// The number to write this variant as when serializing (e.g. `0` or `2`).
	pub value: Expr,
}

// Expansion {{{
// This is the conversion of some of these structures 'back into' tokens, for
// use when the code is generated by macros. For example, the `ToTokens`
// definition for `Enum` generates an actual enum definition.

impl ToTokens for Enum {
	// pub enum Name {
	//     Variant1 = 0,
	//     Variant2 = 1,
	//     // etc.
	// }
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// `pub`.
		tokens.append(Ident::new("pub", Span::call_site()));
		// `enum`.
		tokens.append(Ident::new("enum", Span::call_site()));
		// Name.
		self.name.to_tokens(tokens);

		// Content, wrapped in `{` and `}`.
		tokens.append(Group::new(
			// `{` and `}`.
			Delimiter::Brace,
			// Comma-separated variants.
			self.variants.to_token_stream(),
		));
	}
}

impl ToTokens for Variant {
	// Variant1 = 0
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Attributes, including doc comments.
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// Name.
		self.name.to_tokens(tokens);
		// Equals sign.
		tokens.append(Punct::new('=', Spacing::Alone));
		// Value.
		self.value.to_tokens(tokens);
	}
}

// }}}

// Parsing {{{

impl Parse for Content {
	fn parse(input: ParseStream) -> Result<Self> {
		// `{` and `}`.
		let content;
		braced!(content in input);

		// Initialise variables for the loop.
		let mut metabyte: Option<Item> = None;
		let mut items: Vec<Item> = vec![];

		while !content.is_empty() {
			if metabyte.is_none() && content.peek(Token![$]) {
				// If the metabyte is not already set and there is a `$` token,
				// read the metabyte item.
				content.parse::<Token![$]>()?;
				metabyte = content.parse().ok();
			} else {
				// Otherwise, read the item like normal...
				items.push(content.parse()?);
			}

			// Require a comma after every item, even the last one, for simplicity.
			input.parse::<Token![,]>()?;
		}

		Ok(Self { metabyte, items })
	}
}

impl Parse for Item {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(Token![#]) {
			// Length of a list field.
			// #fieldname
			// #fieldname[u16]

			// `#` token
			input.parse::<Token![#]>()?;
			// Name of the field in question.
			let name: Ident = input.parse()?;

			// If there are square brackets following the name, get the type.
			let ty = if input.peek(token::Bracket) {
				let content;
				bracketed!(content in input);

				Some(content.parse::<Type>()?)
			} else {
				None
			};

			Ok(Self::FieldLength(name, ty))
		} else if input.peek(token::Paren) {
			// Unused bytes.
			// ()[22]
			// ()[padding(fieldname)]

			// `(` and `)`
			let content;
			parenthesized!(content in input);

			// TODO: Is there a better way to do this maybe? Testing for a unit
			// `()`.
			assert!(content.is_empty());

			// `[` and `]`
			let content;
			bracketed!(content in input);

			// Allows an error to be constructed if neither of the wanted tokens
			// are found.
			let look = content.lookahead1();

			if look.peek(LitInt) {
				// If a number is given, parse the number.

				Ok(Self::UnusedBytes(UnusedByteSize::Number(
					content.parse::<LitInt>()?.base10_parse()?,
				)))
			} else if look.peek(Ident) && content.parse::<Ident>()?.to_string() == "padding" {
				// If an identifier is given and that identifier is `padding`
				// (we use this explicit `padding` name to clarify what it
				// is)...

				// `(` and `)`
				let param;
				parenthesized!(param in content);

				Ok(Self::UnusedBytes(UnusedByteSize::Padding(param.parse()?)))
			} else {
				// Construct an error.
				Err(look.error())
			}
		} else {
			// A field with a name and type.
			// pub hosts: [Host]

			// Attributes, including doc comments.
			let attributes = input.call(Attribute::parse_outer)?;
			// Visibility, e.g. `pub`.
			let vis: Option<Visibility> = input.parse::<Visibility>().ok();
			// Whether this is a metabyte field definition with `$`.
			let metabyte = input.parse::<Token![$]>().is_ok();
			// Name.
			let name: Ident = input.parse()?;
			// Colon.
			input.parse::<Token![:]>()?;

			// Enum definition...
			let mut enum_def: Option<Enum> = None;
			// Allows that error construction from earlier.
			let look = input.lookahead1();

			// If an `enum` token is found, parse the enum definition, store it
			// in `enum_def`, and set the type to that enum.
			let ty = if look.peek(Token![enum]) {
				enum_def = Some(input.parse::<Enum>()?);

				// Get a type from the enum definition...
				Type::Verbatim(
					enum_def
						.clone()
						.map(|en| en.name)
						.unwrap()
						.to_token_stream(),
				)
			} else {
				// Parse the type as just a type, no enum definition or anything.
				input.parse()?
			};

			Ok(Self::Field {
				metabyte,
				attributes,
				vis,
				name,
				ty,
				enum_definition: enum_def,
			})
		}
	}
}

impl Parse for Enum {
	fn parse(input: ParseStream) -> Result<Self> {
		// enum Name { Variant = 0, Variant2 = 1 }

		// `enum` token.
		input.parse::<Token![enum]>()?;
		// Name.
		let name: Ident = input.parse()?;

		// `{` and `}`.
		let content;
		braced!(content in input);

		// Parse the variants, punctuated by commas.
		let variants: Punctuated<Variant, Token![,]> = Punctuated::parse_terminated(&content)?;

		Ok(Self { name, variants })
	}
}

impl Parse for Variant {
	fn parse(input: ParseStream) -> Result<Self> {
		// Variant = 0
		// /// Here is a doc message for my variant. It is an attribute.
		// OtherVariant = 1
		// MyVariant = 2
		// etc.

		// Attributes, including doc comments.
		let attributes = input.call(Attribute::parse_outer)?;
		// Name.
		let name: Ident = input.parse()?;
		// Equals sign.
		input.parse::<Token![=]>()?;
		// Value used to write this variant (e.g. `0` or `2`).
		let value: Expr = input.parse()?;

		Ok(Self {
			attributes,
			name,
			value,
		})
	}
}

// }}}
