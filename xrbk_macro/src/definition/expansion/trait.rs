// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::element::{Elements, Field};
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

		tokens.append_tokens(|| {
			quote!(
				impl #impl_generics xrb::Request for #name #type_generics #where_clause {
					type Reply = #reply;

					fn major_opcode() -> u8 {
						#major_opcode
					}

					fn minor_opcode() -> Option<u8> {
						#minor_opcode
					}

					#[allow(clippy::cast_possible_truncation)]
					fn length(&self) -> u16 {
						(<Self as cornflakes::DataSize>::data_size(&self) / 4) as u16
					}
				}
			)
		});
	}
}

impl Reply {
	pub fn impl_trait(&self, tokens: &mut TokenStream2, content: &Content) {
		let name = &self.ident;
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let request = &self.request;
		let sequence = match content {
			Content::Struct { elements, .. } | Content::Tuple { elements, .. } => {
				match elements.sequence_field {
					Some(Field { id, .. }) => {
						quote!(Some(self.#id))
					},

					None => quote!(None),
				}
			},

			Content::Unit => quote!(None),
		};

		tokens.append_tokens(|| {
			quote!(
				impl #impl_generics xrb::Reply for #name #type_generics #where_clause {
					type Req = #request;

					#[allow(clippy::cast_possible_truncation)]
					fn length(&self) -> u32 {
						((<Self as cornflakes::DataSize>::data_size(&self) / 4) - 8) as u32
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
	pub fn impl_trait(&self, tokens: &mut TokenStream2, content: &Content) {
		let name = &self.ident;
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let code = &self.code;
		let sequence = match content {
			Content::Struct {
				elements: Elements {
					sequence_field: Some(sequence),
					..
				},
				..
			} => &sequence.id,

			Content::Tuple {
				elements: Elements {
					sequence_field: Some(sequence),
					..
				},
				..
			} => &sequence.id,

			_ => panic!("events must have a sequence field"),
		};

		tokens.append_tokens(|| {
			quote!(
				impl #impl_generics xrb::Event for #name #type_generics #where_clause {
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
