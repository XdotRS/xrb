// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{format_ident, quote, quote_spanned};

use syn::spanned::Spanned;
use syn::{parse_quote, Data, Fields, GenericParam, Generics, Ident, Index};

use crate::content::*;
use crate::message::*;

/// Deserialization for a single <code>[Option]<&[Item]></code> read from one
/// byte.
///
/// This is used to generate the deserialization for metabyte items.
pub fn deserialize_metabyte(metabyte: Option<&Item>) -> TokenStream2 {
	metabyte.map_or_else(
		|| quote!(reader.skip(1);),
		|item| match item {
			// If the item is a field, read it into a variable with its own
			// name.
			Item::Field(field) => {
				let name = &field.name;
				// Add `__` on either side of the field's name, so that we know
				// it isn't going to conflict with any of the variable names
				// we use in the deserialization.
				let ident = format_ident!("__{}__", name);

				let ty = &field.ty;

				quote!(let #ident: #ty = reader.limit(1).read()?;)
			}
			// If the item is a field length, then we know the length must be of
			// type `u8`, so we read that length as `u8` (and then cast to
			// `usize`, as lengths in Rust are usually `usize`).
			Item::FieldLength(field_len) => {
				let field = &field_len.field_name;
				// Add `__` to the start, but not the end, of the field's name,
				// so that we know it isn't going to conflict with any of the
				// variable names we use in the deserialization, nor with the
				// name of any fields (which we add `__` to _both_ sides of).
				let ident = format_ident!("__{}_len", field);

				quote!(let #ident = reader.read_u8()? as usize;)
			}
			// Since the metabyte is a single byte, we know that if this is an
			// unused byte item, it is exactly one unused byte. So we just skip
			// that byte.
			Item::UnusedBytes(_) => quote!(reader.skip(1);),
		},
	)
}

/// Deserialization for a <code>[Vec]<&[Item]></code>.
///
/// This is used to generate the deserialization code for non-metabyte items.
pub fn deserialize_items(items: Vec<&Item>) -> Vec<TokenStream2> {
	items
		.into_iter()
		.map(|item| match item {
			// If the item is a field, read it into a variable with its own name.
			Item::Field(field) => {
				let name = &field.name;
				// Add `__` on either side of the field's name, so that we know
				// it isn't going to conflict with any of the variable names
				// we use in the deserialization.
				let ident = format_ident!("__{}__", name);

				let ty = &field.ty;

				// TODO: We need to recognise field types that implement
				// `cornflakes::ReadableWithLength` here so that we can read
				// them separarately. We will also need to do the same with
				// `cornflakes::ReadableWithSize`, once we have syntax for that.

				quote!(let #ident: #ty = reader.read()?;)
			}
			// If the item is a field length, read it into a variable with the
			// name `__fieldname_len`, where `fieldname` is the name of the
			// field it is for, with the numerical type that it is for.
			Item::FieldLength(field_len) => {
				let field = &field_len.field_name;
				let ident = format_ident!("__{}_len", field);

				let ty = &field_len.ty;

				quote!(let #ident: #ty = reader.read()?;)
			}
			// If the item is unused bytes, skip the correct number of unused
			// bytes.
			Item::UnusedBytes(unused) => match unused {
				// If it is a single unused byte, skip a single byte.
				UnusedBytes::Single(_) => quote!(__reader.skip(1);),

				UnusedBytes::FullySpecified(full) => match &full.definition {
					// If it is a numerical definition, simply skip the given
					// number of bytes.
					UnusedBytesDefinition::Numerical(val) => quote!(reader.advance(#val);),

					// Otherwise, if it pads a field, calculate the correct
					// number of bytes requried to pad that field and skip that
					// many bytes.
					UnusedBytesDefinition::Padding((_, field)) => quote! {
						reader.advance((4 - (cornflakes::ByteSize::byte_size(#field) % 4)) % 4);
					},
				},
			},
		})
		.collect()
}

/// Serialization for a `Request`.
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
		impl #impl_generics cornflakes::Writable for #name #ty_generics #where_clause {
			fn write_to(&self, writer: &mut impl cornflakes::Writer) -> Result<(), std::io::Error> {
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

/// Deserialization for a `Request`.
#[allow(dead_code)]
pub fn deserialize_request(
	name: Ident,
	generics: Generics,
	_metadata: RequestMetadata,
	content: Content,
) -> TokenStream2 {
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let fields = content.fields().into_iter().map(|field| {
		let name = &field.name;
		let ident = format_ident!("__{}__", name);

		quote!(#name: #ident,)
	});

	// Deserialization for the metabyte.
	let metabyte = deserialize_metabyte(content.metabyte());

	// Deserialization for all other items.
	let items = deserialize_items(content.items_sans_metabyte());

	quote! {
		impl #impl_generics cornflakes::Readable for #name #ty_generics #where_clause {
			fn read_from(reader: &mut impl cornflakes::Reader) -> Result<Self, cornflakes::ReadError> {
				// Don't need to read the major opcode since we already know
				// which request we're deserializing.
				reader.skip(1);
				// Read the metabyte.
				#metabyte

				// Read the length of the request (which is in units of 4
				// bytes), and multiply it by 4 to get the length of the request
				// in bytes.
				let length = (reader.read_u16() * 4) as usize;
				// Limit the `reader` to `length - 4`, as 4 bytes have already
				// been read.
				let reader = reader.limit(length - 4);

				// Read any and all non-metabyte items.
				#(#items)*

				Ok(Self {
					#(#fields)*
				})
			}
		}
	}
}

/// Serialization for a `Reply`.
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
		impl #impl_generics cornflakes::Writable for #name #ty_generics #where_clause {
			fn write_to(
				&self,
				writer: &mut impl cornflakes::Writer
			) -> Result<(), cornflakes::WriteError> {
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

/// Deserialization for a `Reply`.
#[allow(dead_code)]
pub fn deserialize_reply(
	name: Ident,
	generics: Generics,
	_metadata: ReplyMetadata,
	content: Content,
) -> TokenStream2 {
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let fields = content.fields().into_iter().map(|field| {
		let name = &field.name;
		let ident = format_ident!("__{}__", name);

		quote!(#name: #ident,)
	});

	// Deserialization for the metabyte.
	let metabyte = deserialize_metabyte(content.metabyte());

	// Deserialization for all other items.
	let items = deserialize_items(content.items_sans_metabyte());

	quote! {
		impl #impl_generics cornflakes::Readable for #name #ty_generics #where_clause {
			fn read_from(
				reader: &mut impl cornflakes::Reader,
			) -> Result<Self, cornflakes::ReadError> {
				// The first byte of a reply is always `1`: we already know this
				// is a reply, so we can skip it.
				reader.skip(1);
				// Read the metabyte.
				#metabyte

				let sequence_number = reader.read_u16();

				// Read the length of the request (which is in 4-byte units),
				// convert it to bytes, and add 32 bytes (since the minimum
				// reply length is 32 bytes, which gets subtracted from the
				// length when it is recorded).
				let length = ((reader.read_u32() * 4) + 32) as usize;
				// Limit the reader to that length (minus the 8 bytes already
				// consumed), so no types read too many bytes and potentially
				// mess things up. Of course, the reader provided should already
				// do this...
				let reader = reader.limit(length - 8);

				// Read any and all non-metabyte items.
				#(#items)*

				// Construct `Self`.
				Ok(Self {
					sequence_number,
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

/// Expands to the `cornflakes::ByteSize` of the given type.
pub fn byte_size_recurse(field: TokenStream2, span: Span) -> TokenStream2 {
	quote_spanned!(span=> cornflakes::ByteSize::byte_size(&#field))
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
