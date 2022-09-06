// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod content;
mod message;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, quote_spanned};

use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Result, Data, DeriveInput, Fields, GenericParam, Generics, Index, parse_quote};
use syn::spanned::Spanned;

use crate::content::*;
use crate::message::*;

struct Messages {
	pub messages: Vec<Message>,
}

impl Parse for Messages {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut messages: Vec<Message> = vec![];

		while !input.is_empty() {
			messages.push(input.parse()?);
		}

		Ok(Self { messages })
	}
}

#[proc_macro]
pub fn requests(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as Messages);

	let messages = input.messages;

	let trait_impls: Vec<TokenStream2> = messages
		.iter()
		.map(|message| message.message_trait_impl())
		.collect();

	let enum_defs: Vec<Enum> = messages
		.iter()
		.flat_map(|message| match &message.content {
			Content::Longhand(longhand) => longhand
				.fields()
				.iter()
				.filter_map(|field| field.enum_definition.clone())
				.collect::<Vec<Enum>>(),

			Content::Shorthand(shorthand) => shorthand
				.field()
				.and_then(|field| field.enum_definition)
				.iter()
				.map(|enum_def| enum_def.clone())
				.collect::<Vec<Enum>>(),
		})
		.collect();

//	let to_bytes_impls: Vec<TokenStream2> = messages
//		.iter()
//		.map(|message| {
//			quote! {
//				impl #impl_generics cornflakes::ToBytes for #name #ty_generics #where_clause {
//					fn write_to(&self, writer: &mut impl cornflakes::ByteWriter) -> Result<(), std::io::Error> {
//						writer.write_u8(#major);
//						writer.write(#metabyte);
//						writer.write_u16(<Self as crate::Request>::size());
//
//						#(#sans_metabyte)*
//					}
//				}
//			}
//		})
//		.collect();

	let expanded = quote! {
		#(#messages)*
		#(#trait_impls)*
		#(#enum_defs)*
	};

	expanded.into()
}

#[proc_macro_derive(StaticByteSize)]
pub fn derive_static_byte_size(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = input.ident;

	let generics = add_trait_bounds(input.generics, quote!(cornflakes::StaticByteSize));
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let sum = static_byte_size_sum(&input.data);

	let expanded = quote! {
		impl #impl_generics cornflakes::StaticByteSize for #name #ty_generics #where_clause {
			fn static_byte_size() -> usize {
				#sum
			}
		}
	};

	expanded.into()
}

#[proc_macro_derive(ByteSize)]
pub fn derive_byte_size(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = input.ident;

	let generics = add_trait_bounds(input.generics, quote!(cornflakes::ByteSize));
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let sum = byte_size_sum(&input.data);

	let expanded = quote! {
		impl #impl_generics cornflakes::ByteSize for #name #ty_generics #where_clause {
			fn byte_size(&self) -> usize {
				#sum
			}
		}
	};

	expanded.into()
}

fn add_trait_bounds(mut generics: Generics, r#trait: TokenStream2) -> Generics {
	for param in &mut generics.params {
		if let GenericParam::Type(ref mut type_param) = *param {
			type_param.bounds.push(parse_quote!(#r#trait));
		}
	}

	generics
}

fn static_byte_size_sum(data: &Data) -> TokenStream2 {
	match *data {
		Data::Struct(ref data) => {
			match data.fields {
				Fields::Named(ref fields) => {
					let recurse = fields.named.iter().map(|field| {
						let ty = &field.ty;

						quote_spanned! {field.span()=>
							<#ty as cornflakes::StaticByteSize>::static_byte_size()
						}
					});

					quote!(0 #(+ #recurse)*)
				}
				Fields::Unnamed(ref fields) => {
					let recurse = fields.unnamed.iter().map(|field| {
						let ty = &field.ty;

						quote_spanned! {field.span()=>
							<#ty as cornflakes::StaticByteSize>::static_byte_size()
						}
					});

					quote!(0 #(+ #recurse)*)
				}
				Fields::Unit => quote!(1),
			}
		}
		Data::Enum(ref data) => {
			let variants = data.variants.iter().map(|variant| match variant.fields {
				Fields::Named(ref fields) => {
					let recurse = fields.named.iter().map(|field| {
						let ty = &field.ty;

						quote_spanned! {field.span()=>
							<#ty as cornflakes::StaticByteSize>::static_byte_size()
						}
					});

					quote!(0 #(+ #recurse)*)
				}
				Fields::Unnamed(ref fields) => {
					let recurse = fields.unnamed.iter().map(|field| {
						let ty = &field.ty;

						quote_spanned! {field.span()=>
							<#ty as cornflakes::StaticByteSize>::static_byte_size()
						}
					});

					quote!(0 #(+ #recurse)*)
				}
				Fields::Unit => quote!(0),
			});

			quote!(1 #(+ #variants)*)
		}
		Data::Union(_) => unimplemented!(),
	}
}

fn byte_size_sum(data: &Data) -> TokenStream2 {
	match *data {
		Data::Struct(ref data) => {
			match data.fields {
				Fields::Named(ref fields) => {
					let recurse = fields.named.iter().map(|field| {
						let name = &field.ident;

						quote_spanned! {field.span()=>
							cornflakes::ByteSize::byte_size(&self.#name)
						}
					});

					quote!(0 #(+ #recurse)*)
				}
				Fields::Unnamed(ref fields) => {
					let recurse = fields.unnamed.iter().enumerate().map(|(i, field)| {
						let index = Index::from(i);

						quote_spanned! {field.span()=>
							cornflakes::ByteSize::byte_size(&self.#index)
						}
					});

					quote!(0 #(+ #recurse)*)
				}
				Fields::Unit => quote!(1),
			}
		}
		Data::Enum(ref data) => {
			let variants = data.variants.iter().map(|variant| match variant.fields {
				Fields::Named(ref fields) => {
					let recurse = fields.named.iter().map(|field| {
						let name = &field.ident;

						quote_spanned! {field.span()=>
							cornflakes::ByteSize::byte_size(&self.#name)
						}
					});

					quote!(0 #(+ #recurse)*)
				}
				Fields::Unnamed(ref fields) => {
					let recurse = fields.unnamed.iter().enumerate().map(|(i, field)| {
						let index = Index::from(i);

						quote_spanned! {field.span()=>
							cornflakes::ByteSize::byte_size(&self.#index)
						}
					});

					quote!(0 #(+ #recurse)*)
				}
				Fields::Unit => quote!(0),
			});

			quote!(1 #(+ #variants)*)
		}
		Data::Union(_) => unimplemented!(),
	}
}
