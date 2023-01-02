// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::element::Element;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use super::*;
use crate::TsExt;

impl Request {
	pub fn impl_trait(&self, tokens: &mut TokenStream2) {
		let name = &self.ident;
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let reply = if let Some((_, r#type)) = &self.reply {
			quote!(#r#type)
		} else {
			quote!(())
		};

		let major_opcode = &self.major_opcode;
		let minor_opcode = if let Some((_, minor_opcode)) = &self.minor_opcode {
			quote!(Some(#minor_opcode))
		} else {
			quote!(None)
		};

		let r#trait = &self.request_token;

		tokens.append_tokens(|| {
			quote!(
				#[automatically_derived]
				impl #impl_generics xrb::#r#trait for #name #type_generics #where_clause {
					type Reply = #reply;

					fn major_opcode() -> u8 {
						#major_opcode
					}

					fn minor_opcode() -> Option<u8> {
						#minor_opcode
					}

					#[allow(clippy::cast_possible_truncation)]
					fn length(&self) -> u16 {
						(<Self as ::xrbk::DataSize>::data_size(&self) / 4) as u16
					}
				}
			)
		});
	}
}

impl Reply {
	pub fn impl_trait(&self, tokens: &mut TokenStream2) {
		let name = &self.ident;
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let request = &self.request;
		let sequence = match &self.content {
			StructlikeContent::Regular {
				content,
				..
			} if let Some(Element::Field(field)) = content.sequence_element() => {
				let id = &field.id;
				quote!(Some(self.#id))
			},

			StructlikeContent::Tuple {
				content,
				..
			} if let Some(Element::Field(field)) = content.sequence_element() => {
				let id = &field.id;
				quote!(Some(self.#id))
			},

			_ => quote!(None),
		};

		let r#trait = &self.reply_token;

		tokens.append_tokens(|| {
			quote!(
				#[automatically_derived]
				impl #impl_generics xrb::#r#trait for #name #type_generics #where_clause {
					type Req = #request;

					#[allow(clippy::cast_possible_truncation)]
					fn length(&self) -> u32 {
						((<Self as ::xrbk::DataSize>::data_size(&self) / 4) - 8) as u32
					}

					fn sequence(&self) -> Option<u16> {
						#sequence
					}
				}
			)
		});
	}
}

impl Event {
	pub fn impl_trait(&self, tokens: &mut TokenStream2) {
		let name = &self.ident;
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let code = &self.code;
		let sequence = match &self.content {
			StructlikeContent::Regular {
				content,
				..
			} if let Some(Element::Field(field)) = content.sequence_element() => &field.id,

			StructlikeContent::Tuple {
				content,
				..
			} if let Some(Element::Field(field)) = content.sequence_element() => &field.id,

			_ => panic!("events must have a sequence field"),
		};

		let r#trait = &self.event_token;

		tokens.append_tokens(|| {
			quote!(
				#[automatically_derived]
				impl #impl_generics xrb::#r#trait for #name #type_generics #where_clause {
					fn code() -> u8 {
						#code
					}

					fn sequence(&self) -> u16 {
						self.#sequence
					}
				}
			)
		});
	}
}
