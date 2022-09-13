// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{format_ident, quote, quote_spanned};

use syn::spanned::Spanned;
use syn::{parse_quote, Data, Fields, GenericParam, Generics, Ident, Index, Type};

use crate::content::*;
use crate::message::*;

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
		// If the minor opcode is declared, quote it, otherwise if the metabyte
		// is declared, quote it, otherwise quote `0`.
		.map_or_else(
			|| {
				content
					.metabyte()
					.map_or_else(|| quote!(0), |metabyte| quote!(#metabyte))
			},
			|(_, minor)| quote!(#minor),
		);

	let items = content.items_sans_metabyte();

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

	// If there is a metabyte declared, quote it, otherwise quote `0`.
	let metabyte = content
		.metabyte()
		.map_or_else(|| quote!(0), |item| quote!(#item));

	// Get a list of the non-metabyte items that are to be written in the body.
	let items = content.items_sans_metabyte();

	quote! {
		impl #impl_generics cornflakes::ToBytes for #name #ty_generics #where_clause {
			fn write_to(
				&self,
				writer: &mut impl cornflakes::ByteWriter
			) -> Result<(), std::io::Error> {
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
	content: Content,
) -> TokenStream2 {
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	// Punctuate the field names with commas, so that they can easily be
	// inserted into the generated constructor invocation.
	let fields = content.fields().into_iter().map(|field| {
		let name = &field.name;
		quote!(#name,)
	});

	// Deserialization for the metabyte.
	let metabyte = content.metabyte().map_or_else(
		|| quote!(__reader.skip(1);),
		|item| match item {
			// If the item is a field, read it into a variable with its own
			// name.
			Item::Field(field) => {
				let name = &field.name;
				let ty = &field.ty;

				quote!(let #name: #ty = __reader.limit(1).read()?;)
			}
			// If the item is a field length, then we know the length must be of
			// type `u8`, so we read that length as `u8` (and then cast to
			// `usize`, as lengths in Rust are usually `usize`).
			Item::FieldLength(field_len) => {
				let field = &field_len.field_name;
				let ident = format_ident!("__{}_len", field);

				quote!(let #ident = __reader.read::<u8>()? as usize;)
			}
			// Since the metabyte is a single byte, we know that if this is an
			// unused byte item, it is exactly one unused byte. So we just skip
			// that byte.
			Item::UnusedBytes(_) => quote!(__reader.skip(1);),
		},
	);

	// Deserialization for all other items.
	let items = content
		.items_sans_metabyte()
		.into_iter()
		.map(|item| match item {
			// If the item is a field, read it into a variable with its own name.
			//
			// TODO: What if it is a list? How do we use the field lengths to read
			// lists?
			Item::Field(field) => {
				let name = &field.name;
				let ty = &field.ty;

				quote!(let #name: #ty = __reader.read()?;)
			}
			// If the item is a field length, read it into a variable with the
			// name `__fieldname_len`, where `fieldname` is the name of the
			// field it is for, with the numerical type that it is for.
			Item::FieldLength(field_len) => {
				let field = &field_len.field_name;
				let ident = format_ident!("__{}_len", field);

				let ty = &field_len.ty;

				quote!(let #ident = __reader.read::<#ty>()?;)
			}
			// If the item is unused bytes, skip the correct number of unused
			// bytes.
			Item::UnusedBytes(unused) => match unused {
				// If it is a single unused byte, skip a single byte.
				UnusedBytes::Single(_) => quote!(__reader.skip(1);),

				UnusedBytes::FullySpecified(full) => match &full.definition {
					// If it is a numerical definition, simply skip the given
					// number of bytes.
					UnusedBytesDefinition::Numerical(val) => quote!(__reader.skip(#val);),

					// Otherwise, if it pads a field, calculate the correct
					// number of bytes requried to pad that field and skip that
					// many bytes.
					UnusedBytesDefinition::Padding((_, field)) => quote! {
						__reader.skip((4 - (cornflakes::ByteSize::byte_size(#field) % 4)) % 4);
					},
				},
			},
		});

	quote! {
		impl #impl_generics cornflakes::FromBytes for #name #ty_generics #where_clause {
			fn read_from(__reader: &mut impl cornflakes::ByteReader) -> Result<Self, std::io::Error> {
				// The first byte of a reply is always `1`: we already know this
				// is a reply, so we can skip it.
				__reader.skip(1);
				// Read the metabyte.
				#metabyte

				let __sequence_number = __reader.read_u16();

				// Read the length of the request (which is in 4-byte units),
				// convert it to bytes, and add 32 bytes (since the minimum
				// reply length is 32 bytes, which gets subtracted from the
				// length when it is recorded).
				let __length = ((reader.read_u32() * 4) + 32) as usize;
				// Limit the reader to that length (minus the 8 bytes already
				// consumed), so no types read too many bytes and potentially
				// mess things up. Of course, the reader provided should already
				// do this...
				let __reader = reader.limit(__length - 8);

				// Read any and all non-metabyte items.
				#(#items)*

				// Construct `Self`.
				Ok(Self {
					__sequence_number,
					// This is the names of all of the fields; variables with
					// these names are generated while reading each respective
					// field.
					#(#fields)*
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
