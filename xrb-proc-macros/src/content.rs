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
///
/// # Examples
/// Full request definition with just normal fields:
/// ```rust
/// requests! {
///     pub struct CopyArea(62) {
///         pub source: Drawable,
///         pub destination: Drawable,
///         pub context: GraphicsContext,
///         pub src_x: i16,
///         pub src_y: i16,
///         pub dest_x: i16,
///         pub dest_y: i16,
///         pub width: u16,
///         pub height: u16,
///     }
/// }
/// ```
/// Shorthand request definition (only one item):
/// ```rust
/// requests! {
///     // A request with a single `Window` field.
///     pub struct DeleteWindow(4): pub target: Window;
/// }
/// ```
/// Shorthand request definition (only one item), using a metabyte item:
/// ```rust
/// requests! {
///     // A request with a single boolean field in the metabyte position.
///     pub struct SetAccessControl(111): pub $enabled: bool;
/// }
/// ```
/// Full reply definition, using a metabyte item, list length encoding, unused
/// byte encoding, unused padding byte, and a normal field:
/// ```rust
/// requests! {
///     pub struct GetPointerMappingReply for GetPointerMapping {
///         // `$` means this is a metabyte item, and `#map[u8]` means it is
///         // counting the length of the `map` field (as it is a list) and
///         // writing it as a `u8`-type value.
///         $#map[u8],
///         // This means there are 24 unused bytes between the header and the
///         // `map` field.
///         ()[24],
///         // A field, just like in a normal struct definition.
///         pub map: [u8],
///         // An unused bytes encoding that automatically adds the correct
///         // amount of padding bytes depending on the size of the `map` field
///         // to bring the size up to a multiple of 4, which is required by
///         // the X11 protocol.
///         ()[padding(map)],
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
	///         $#map[u8], // the length of the `map` list in metabyte position
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
///
/// # Examples
/// 20 unused bytes:
/// ```
/// ()[20]
/// ```
/// Automatically add the correct number of padding bytes for the `map` field
/// to bring it to a multiple of 4 bytes, as required by the X11 protocol:
/// ```
/// ()[padding(map)]
/// ```
/// Write the length of the `charinfos` list as a [`u32`] value:
/// ```
/// #charinfos[u32]
/// ```
/// Define a boolean field for the metabyte position:
/// ```
/// pub $exposures: bool
/// ```
/// Define a field with an inline enum definition for convenience:
/// ```
/// pub draw_direction: enum DrawDirection {
///     LeftToRight = 0,
///     RightToLeft = 1,
/// }
/// ```
/// Define a `Window`-type field:
/// ```
/// pub target: Window
/// ```
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Item {
	/// Unused bytes within a message.
	///
	/// These are written as empty bytes when serializing, and skipped when
	/// deserializing.
	///
	/// # Examples
	/// 20 unused bytes:
	/// ```
	/// ()[20]
	/// ```
	/// Automatically add padding bytes for the `map` field:
	/// ```
	/// ()[padding(map)]
	/// ```
	UnusedBytes(UnusedByteSize),
	/// Encodes the length of a collection field.
	///
	/// This is used to read and write the length of a list in a message, as
	/// defined in the X11 protocol. The length of lists does not accompany
	/// the lists themselves directly in X11, so we must specify the location
	/// where that length is encoded.
	///
	/// # Examples
	/// Write the length of the `map` list as the default field length type of
	/// [`u16`]:
	/// ```
	/// #map
	/// ```
	/// Write the length of the `map` list as a [`u8`] value in the metabyte
	/// position:
	/// ```
	/// $#map[u8]
	/// ```
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
	///
	/// # Examples
	/// Define a `time` field:
	/// ```
	/// pub time: Time
	/// ```
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
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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

/// An enum variant with a name and value used to serialize it.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
		// Keep track of the metabyte item separately, if any is given. This is
		// because there can only be zero or one metabyte items, i.e. `None` or
		// `Some`.
		let mut metabyte: Option<Item> = None;
		let mut items: Vec<Item> = vec![];

		// Lets us build an error with the tokens that were found.
		let look = input.lookahead1();

		if look.peek(Token![:]) {
			// This is a shorthand definition with just a single item.

			// Parse the colon.
			input
				.parse::<Token![:]>()
				.expect("we already checked for the presence of a colon");

			// Parse the item.
			let item: Item = input.parse()?;

			// If the item was a metabyte declaration, set it as the metabyte,
			// otherwise push it as the single element of the items vector.
			if match item {
				Item::Field { metabyte, .. } => metabyte,
				Item::FieldLength { metabyte, .. } => metabyte,
				Item::UnusedBytes(..) => false,
			} {
				metabyte = Some(item);
			} else {
				items.push(item);
			}

			// Require a semicolon following a shorthand item definition.
			input.parse::<Token![;]>()?;
		} else if look.peek(token::Brace) {
			// `{` and `}`.
			let content;
			braced!(content in input);

			// Keeps track of whether this is the first item to be parsed, so that
			// the check for a preceding comma is not done for the first item.
			//
			// While it logically makes sense that the comma comes after an item,
			// it is easier to check that the comma comes before any item other
			// than the first.
			let mut initial_loop = true;

			while !content.is_empty() {
				if initial_loop {
					// If this is the first item, then set `initial_loop` to false
					// so we make the comma check next time.
					initial_loop = false;
				} else {
					// Break from the loop if no comma was found, or no item was
					// found after the comma. This is only met by the final item.
					if content.parse::<Token![,]>().is_err() || content.is_empty() {
						break;
					}
				}

				// Parse an item.
				let item: Item = content.parse()?;

				// If this item is declared as a metabyte item declaration with
				// `$` syntax...
				if match item {
					Item::Field { metabyte, .. } => metabyte,
					Item::FieldLength { metabyte, .. } => metabyte,
					Item::UnusedBytes(..) => false,
				} {
					if metabyte.is_some() {
						// If the item was declared as a metabyte item but one has
						// already been declared as such for this message, then
						// we can't honor its declaration as a metabyte item.
						panic!("found too many metabyte item declarations");
					}

					// Store this as the metabyte item, rather than appending it to
					// the list of other items.
					metabyte = Some(item);
				} else {
					// If this is just an ordinary non-metabyte item declaration,
					// add it to the list.
					items.push(item);
				}
			}
		} else if !look.peek(Token![;]) {
			// If no shorthand item definition nor full brace definition was
			// found, nor was a semicolon to end the definition, then this is
			// a syntax error, so we return an error built by `look` that refers
			// to the expected tokens that were not found.
			return Err(look.error());
		}

		Ok(Self { metabyte, items })
	}
}

impl Parse for Item {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(Token![#]) || input.peek(Token![$]) && input.peek2(Token![#]) {
			// If the item starts with `#` it is a normal list length encoding,
			// and if it starts with `$#` it is a metabyte list length encoding.

			// Length of a list field.
			// #fieldname
			// #fieldname[u16]
			// $#fieldname[u8] // metabyte position

			// Whether this is a metabyte list length encoding with `$`.
			let metabyte = input.parse::<Token![$]>().is_ok();
			// `#` token that precedes a list length encoding.
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

			Ok(Self::FieldLength {
				metabyte,
				name,
				length_type: ty,
			})
		} else if input.peek(token::Paren) {
			// If the item starts with `(`, then it is an unused bytes encoding.

			// Unused bytes.
			// ()[22]
			// ()[padding(fieldname)]

			// `(` and `)`
			let content;
			parenthesized!(content in input);

			// If there was anything found within the parentheses, we have a
			// problem... the syntax we're looking for is just `()`, i.e. the
			// unit value.
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
			// If the item doesn't start with `#`, `$#`, or `()`, then it is a
			// field.

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
