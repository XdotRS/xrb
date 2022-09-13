// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{format_ident, quote, quote_spanned};

use syn::spanned::Spanned;
use syn::{parse_quote, Data, Fields, GenericParam, Generics, Ident, Index, Type};

use crate::content::Content;
use crate::message::*;

/// Returns the tokens that get the metabyte from [`Content`].
///
/// If there is a metabyte declared in the given `content`, this returns that
/// metabyte item [`to_tokenstream()`], otherwise it will return a `0` token.
///
/// [`to_tokenstream()`]: quote::ToTokens::to_tokenstream
pub fn metabyte(content: &Content) -> TokenStream2 {
	match &content {
		Content::Shorthand(shorthand) => shorthand
			.item
			.as_ref()
			// Filter out the item if it isn't a metabyte.
			.filter(|(_, item)| item.is_metabyte())
			// If there is an item and it is a metabyte, quote that item,
			// else quote `0`.
			.map_or_else(|| quote!(0), |(_, item)| quote!(#item)),

		Content::Longhand(longhand) => longhand
			// Get the metabyte item if one is declared.
			.metabyte()
			// If it is declared, quote it, else quote `0`.
			.map_or_else(|| quote!(0), |item| quote!(#item)),
	}
}

#[allow(dead_code)]
pub fn serialize_request(
	name: Ident,
	generics: Generics,
	metadata: RequestMetadata,
	content: Content,
) -> TokenStream2 {
	// Split the provided generics to be placed in the generated code.
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let major_opcode = metadata.major_opcode;
	// 'Metabyte' is our affectionate term for the second byte of a message
	// header.
	let metabyte = metadata
		// Get the minor opcode, if one is declared...
		.minor_opcode
		// If it is declared, quote it, else find the metabyte from the
		// `content`.
		.map_or_else(|| metabyte(&content), |(_, minor)| quote!(#minor));

	let items = match content {
		Content::Shorthand(shorthand) => shorthand
			.item
			.filter(|(_, item)| !item.is_metabyte())
			.map_or_else(std::vec::Vec::new, |(_, item)| vec![item]),

		Content::Longhand(longhand) => longhand.items_sans_metabyte(),
	};

	quote! {
		impl #impl_generics cornflakes::ToBytes for #name #ty_generics #where_clause {
			fn write_to(&self, writer: &mut impl cornflakes::ByteWriter) -> Result<(), std::io::Error> {
				// Header
				writer.write_u8(#major_opcode);
				writer.write_u8(#metabyte);
				writer.write_u16(crate::Request::length(self));

				// Everything else
				#(#items)*

				Ok(())
			}
		}
	}
}

#[allow(dead_code)]
pub fn deserialize_request(
	name: Ident,
	generics: Generics,
	_metadata: RequestMetadata,
	_content: Content,
) -> TokenStream2 {
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	quote! {
		impl #impl_generics cornflakes::FromBytes for #name #ty_generics #where_clause {
			fn read_from(__reader: &mut impl cornflakes::ByteReader) -> Result<Self, std::io::Error> {
				// Don't need to read the major opcode since we already know
				// which request we're deserializing.
				__reader.skip(1);
				// TODO: if metabyte item, read metabyte item, else skip

				// Read the length of the request (which is in units of 4
				// bytes), and multiply it by 4 to get the length of the request
				// in bytes.
				let __length = (__reader.read::<u16>() * 4) as usize;
				// Limit the `reader` to `length - 4`, as 4 bytes have already
				// been read.
				let __reader = __reader.limit(__length - 4);

				// TODO: Loop over all non-metabyte items:
				// - if unused, skip the unused bytes
				// - if length, store length as `fieldname_length` for use in
				//   deserializing `fieldname`
				// - if field, `let fieldname = FieldType::read_from(reader)?;`

				Ok(Self {
					// TODO: If metabyte field, put metabyte field
					// any other fields.
				})
			}
		}
	}
}

#[allow(dead_code)]
pub fn serialize_reply(
	name: Ident,
	generics: Generics,
	_metadata: ReplyMetadata,
	content: Content,
) -> TokenStream2 {
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	// Expand the serialization code for the metabyte item, if one is defined,
	// else to simply write a single `0` byte.
	let metabyte = metabyte(&content);

	// Get a list of the non-metabyte items that are to be written in the body.
	let items = match content {
		Content::Shorthand(shorthand) => shorthand
			.item
			// Filter out the item if it is a metabyte item.
			.filter(|(_, item)| !item.is_metabyte())
			// If there was no item declared, or the declared item was in the
			// metabyte position, create an empty `Vec` with `std::vec::Vec::new`,
			// else create a `Vec` with the non-metabyte item.
			.map_or_else(std::vec::Vec::new, |(_, item)| vec![item]),

		// `Longhand` has its own method to get a list of non-metabyte items:
		Content::Longhand(longhand) => longhand.items_sans_metabyte(),
	};

	quote! {
		impl #impl_generics cornflakes::ToBytes for #name #ty_generics #where_clause {
			fn write_to(&self, writer: &mut impl cornflakes::ByteWriter) -> Result<(), std::io::Error> {
				// Header {{{

				// A first byte of `1` indicates that this message is a reply.
				writer.write_u8(1);
				// Write the minor opcode if `Some`, else the metabyte.
				writer.write_u8(self.__minor_opcode.unwrap_or_else(|| #metabyte));
				// Write the sequence number associated with the request that
				// initiated the reply.
				writer.write_u16(self.__sequence_number);

				// Write the length of the reply.
				writer.write_u32(crate::Reply::length(self));

				// }}}

				// Write any and all non-metabyte items.
				#(#items)*

				Ok(())
			}
		}
	}
}

#[allow(dead_code)]
pub fn deserialize_reply(
	name: Ident,
	generics: Generics,
	_metadata: ReplyMetadata,
	_content: Content,
) -> TokenStream2 {
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	quote! {
		impl #impl_generics cornflakes::FromBytes for #name #ty_generics #where_clause {
			fn read_from(__reader: &mut impl cornflakes::ByteReader) -> Result<Self, std::io::Error> {
				// The first byte of a reply is always `1`: we already know this
				// is a reply, so we can skip it.
				__reader.skip(1);
				// TODO: if metabyte item, read metabyte item, else skip

				let __sequence_number = __reader.read_u16();

				let __length = ((reader.read_u32() * 4) + 32) as usize;
				let __reader = reader.limit(__length - 8);

				// TODO: read everything else

				Ok(Self {
					__sequence_number,
					// TODO: everything else
				})
			}
		}
	}
}

/// Adds the given `r#trait` bounds to the given [`Generics`].
pub fn add_trait_bounds(mut generics: Generics, r#trait: TokenStream2) -> Generics {
	for param in &mut generics.params {
		if let GenericParam::Type(ref mut type_param) = *param {
			type_param.bounds.push(parse_quote!(#r#trait));
		}
	}

	generics
}

/// Expands to the `cornflakes::StaticByteSize` of the given type.
pub fn static_byte_size_recurse(ty: &Type, span: Span) -> TokenStream2 {
	quote_spanned!(span=> <#ty as cornflakes::StaticByteSize>::static_byte_size())
}

/// Expands to the `cornflakes::ByteSize` of the given type.
pub fn byte_size_recurse(field: TokenStream2, span: Span) -> TokenStream2 {
	quote_spanned!(span=> cornflakes::ByteSize::byte_size(&#field))
}

/// Sums the `cornflakes::StaticByteSize` of all the variants and/or fields in
/// an enum or struct.
pub fn static_byte_size_sum(data: &Data) -> TokenStream2 {
	match *data {
		Data::Struct(ref data) => match data.fields {
			// Structs.
			Fields::Named(ref fields) => {
				// Named fields.

				let recurse = fields
					.named
					.iter()
					.map(|field| static_byte_size_recurse(&field.ty, field.span()));

				// For every named field, add its size.
				quote!(0 #(+ #recurse)*)
			}
			Fields::Unnamed(ref fields) => {
				// Unnamed fields.

				let recurse = fields
					.unnamed
					.iter()
					.map(|field| static_byte_size_recurse(&field.ty, field.span()));

				// For every unnamed field, add its size.
				quote!(0 #(+ #recurse)*)
			}
			// How can a unit struct have a size? What would we write it as?
			Fields::Unit => unimplemented!(),
		},
		Data::Enum(ref data) => {
			// Enums.

			let sum = data
				.variants
				.iter()
				.map(|variant| match variant.fields {
					// Enum variants.
					Fields::Named(ref fields) => {
						// Named fields in a variant.

						let recurse = fields
							.named
							.iter()
							.map(|field| static_byte_size_recurse(&field.ty, field.span()));

						// For every named field, add its size.
						quote!(0 #(+ #recurse)*)
					}
					Fields::Unnamed(ref fields) => {
						// Unnamed fields in a variant

						let recurse = fields
							.unnamed
							.iter()
							.map(|field| static_byte_size_recurse(&field.ty, field.span()));

						// For every unnamed field, add its size.
						quote!(0 #(+ #recurse)*)
					}
					// If there are no fields, don't add any size.
					Fields::Unit => quote!(0),
				})
				// Take the maximum size of any variant. The whole enum's size
				// has to be large enough for the largest possible variant.
				.reduce(|a, b| quote!(std::cmp::max(#a, #b)));

			// Add 1 to the sum, because the sum is of the fields for the
			// variants, not the variants themselves.
			quote!(#sum + 1)
		}
		Data::Union(_) => unimplemented!(),
	}
}

/// Sums the `cornflakes::ByteSize` of all of the variants and/or fields in an
/// enum or struct.
pub fn byte_size_sum(data: &Data) -> TokenStream2 {
	match *data {
		Data::Struct(ref data) => match data.fields {
			// Structs.
			Fields::Named(ref fields) => {
				// Named fields.

				let recurse = fields.named.iter().map(|field| {
					let name = &field.ident;
					byte_size_recurse(quote!(self.#name), field.span())
				});

				// For every named field, add its size.
				quote!(0 #(+ #recurse)*)
			}
			Fields::Unnamed(ref fields) => {
				// Unnamed fields.

				let recurse = fields.unnamed.iter().enumerate().map(|(i, field)| {
					let index = Index::from(i);
					byte_size_recurse(quote!(self.#index), field.span())
				});

				// For every unnamed field, add its size.
				quote!(0 #(+ #recurse)*)
			}
			// How can a unit struct have a size? What would we write it as?
			Fields::Unit => unimplemented!(),
		},
		Data::Enum(ref data) => {
			// Enums.

			let variants = data.variants.iter().map(|variant| {
				// Enum variants.
				let name = &variant.ident;

				match variant.fields {
					Fields::Named(ref fields) => {
						// Named fields in a variant.
						let recurse = fields.named.iter().map(|field| {
							let name = &field.ident;
							byte_size_recurse(quote!(self.#name), field.span())
						});

						// For every named field, add its size.
						quote!(Self::#name => 1 #(+ #recurse)*,)
					}

					Fields::Unnamed(ref fields) => {
						// Unnamed fields in a variant
						let fields = fields.unnamed.iter().enumerate();

						let recurse = fields
							.map(|(i, field)| {
								let index = Index::from(i);
								let ident = format_ident!("_{}", index);

								(
									ident.clone(),
									byte_size_recurse(quote!(#ident), field.span()),
								)
							})
							.collect::<Vec<_>>();

						let ident = recurse.iter().map(|(ident, _)| ident);
						let size = recurse.iter().map(|(_, size)| size);

						// For every unnamed field, add its size.
						quote!(Self::#name(#(#ident),*) => 1 #(+ #size)*,)
					}

					Fields::Unit => quote!(Self::#name => 1,),
				}
			});

			quote! {
				match self {
					#(#variants)*
				}
			}
		}
		Data::Union(_) => unimplemented!(),
	}
}
