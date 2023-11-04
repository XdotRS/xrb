// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::element::Element;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};

use super::*;
use crate::TsExt;

impl Request {
	pub fn impl_trait(&self, tokens: &mut TokenStream2) {
		let name = &self.ident;

		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let reply = if let Some((_, r#type)) = &self.reply {
			quote!(#r#type)
		} else {
			quote!(())
		};

		let major_opcode = &self.major_opcode;
		let minor_opcode = if let Some(minor_opcode) = &self.minor_opcode {
			quote!(Some(#minor_opcode))
		} else {
			quote!(None)
		};
		let other_errors = if let Some(other_errors) = &self.other_errors {
			other_errors.to_token_stream()
		} else {
			quote!(::std::convert::Infallible)
		};

		let request_token = &self.request_token;

		tokens.append_tokens({
			quote_spanned!(self.request_token.span()=>
				#[automatically_derived]
				impl #impl_generics #request_token for #name #type_generics #where_clause {
					type Reply = #reply;
					type OtherErrors = #other_errors;

					const MAJOR_OPCODE: u8 = {
						#major_opcode
					};

					const MINOR_OPCODE: Option<u16> = {
						#minor_opcode
					};

					#[allow(clippy::cast_possible_truncation)]
					fn length(&self) -> u16 {
						(<Self as ::xrbk::X11Size>::x11_size(self) / 4) as u16
					}
				}
			)
		});
	}
}

impl Reply {
	pub fn impl_trait(&self, tokens: &mut TokenStream2) {
		let name = &self.ident;

		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let request = &self.request;
		let sequence = match &self.content {
			StructlikeContent::Regular { content, .. }
				if let Some(Element::Field(field)) = content.sequence_element() =>
			{
				&field.id
			},

			StructlikeContent::Tuple { content, .. }
				if let Some(Element::Field(field)) = content.sequence_element() =>
			{
				&field.id
			},

			_ => panic!("replies must have a sequence field of type `u32`"),
		};

		let reply_token = &self.reply_token;

		tokens.append_tokens({
			quote_spanned!(self.reply_token.span()=>
				#[automatically_derived]
				impl #impl_generics #reply_token for #name #type_generics #where_clause {
					type Request = #request;

					#[allow(clippy::cast_possible_truncation)]
					fn length(&self) -> u32 {
						((<Self as ::xrbk::X11Size>::x11_size(self) / 4) - 8) as u32
					}

					fn sequence(&self) -> u16 {
						self.#sequence
					}
				}
			)
		});
	}
}

impl Event {
	pub fn impl_trait(&self, tokens: &mut TokenStream2) {
		let name = &self.ident;

		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let code = &self.event_code;
		let sequence = match &self.content {
			StructlikeContent::Regular { content, .. }
				if let Some(Element::Field(field)) = content.sequence_element() =>
			{
				let id = &field.id;
				quote!(Some(self.#id))
			},

			StructlikeContent::Tuple { content, .. }
				if let Some(Element::Field(field)) = content.sequence_element() =>
			{
				let id = &field.id;
				quote!(Some(self.#id))
			},

			_ => quote!(None),
		};

		let event_token = &self.event_token;

		tokens.append_tokens({
			quote_spanned!(self.event_token.span()=>
				#[automatically_derived]
				impl #impl_generics #event_token for #name #type_generics #where_clause {
					const CODE: u8 = {
						#code
					};

					fn sequence(&self) -> Option<u16> {
						#sequence
					}
				}
			)
		});
	}
}

impl Error {
	pub fn impl_trait(&self, tokens: &mut TokenStream2) {
		let name = &self.ident;

		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let error_path = &self.error_token;
		let error_code = &self.error_code;

		let sequence = match &self.content {
			StructlikeContent::Regular { content, .. }
				if let Some(Element::Field(field)) = content.sequence_element() =>
			{
				let id = &field.id;
				quote!(self.#id)
			},

			StructlikeContent::Tuple { content, .. }
				if let Some(Element::Field(field)) = content.sequence_element() =>
			{
				let id = &field.id;
				quote!(self.#id)
			},

			_ => panic!("expected a sequence field"),
		};

		let minor_opcode = match &self.content {
			StructlikeContent::Regular { content, .. }
				if let Some(Element::Field(field)) = content.minor_opcode_element() =>
			{
				let id = &field.id;
				quote!(self.#id)
			},

			StructlikeContent::Tuple { content, .. }
				if let Some(Element::Field(field)) = content.minor_opcode_element() =>
			{
				let id = &field.id;
				quote!(self.#id)
			},

			_ => panic!("expected a minor opcode field"),
		};

		let major_opcode = match &self.content {
			StructlikeContent::Regular { content, .. }
				if let Some(Element::Field(field)) = content.major_opcode_element() =>
			{
				let id = &field.id;
				quote!(self.#id)
			},

			StructlikeContent::Tuple { content, .. }
				if let Some(Element::Field(field)) = content.major_opcode_element() =>
			{
				let id = &field.id;
				quote!(self.#id)
			},

			_ => panic!("expected a major opcode field"),
		};

		tokens.append_tokens({
			quote_spanned!(error_path.span()=>
				#[automatically_derived]
				impl #impl_generics #error_path for #name #type_generics #where_clause {
					const CODE: u8 = {
						#error_code
					};

					fn sequence(&self) -> u16 {
						#sequence
					}

					fn minor_opcode(&self) -> u16 {
						#minor_opcode
					}

					fn major_opcode(&self) -> u8 {
						#major_opcode
					}
				}
			)
		});
	}
}
