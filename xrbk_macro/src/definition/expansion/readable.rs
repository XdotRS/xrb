// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote_spanned};
use syn::Path;

use crate::{element::Element, TsExt};

use super::*;

impl Struct {
	pub fn impl_readable(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		// Expand the tokens to call Self's constructor.
		let cons = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		// Expand the tokens to read each element.
		let reads = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				element.read_tokens(tokens, DefinitionType::Basic);

				// if self.content.contains_infer() {
				element.add_x11_size_tokens(tokens);
				// }
			}
		});

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::Readable for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
					unused_mut,
				)]
				fn read_from(
					buf: &mut impl ::xrbk::Buf,
				) -> Result<Self, ::xrbk::ReadError> {
					// Declare a x11_size variable if it is going to be
					// used in an infer unused bytes element.
					let mut size: usize = 0;

					// Read each element.
					#reads

					// Construct and return `Self`.
					Ok(Self #cons)
				}
			}
		));
	}
}

impl Request {
	pub fn impl_readable(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let cons = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let reads = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				if !element.is_metabyte() && !element.is_sequence() {
					element.read_tokens(tokens, DefinitionType::Request);

					// if self.content.contains_infer() {
					element.add_x11_size_tokens(tokens);
					// }
				}
			}
		});

		let metabyte = if self.minor_opcode.is_some() {
			// If there is a minor opcode, then it has already been read in order to
			// determine that this is the request to read.
			// TODO: can't be in metabyte, must check this in protocol!!
			None
		} else if let Some(element) = self.content.metabyte_element() {
			Some(TokenStream2::with_tokens(|tokens| {
				element.read_tokens(tokens, DefinitionType::Request);
			}))
		} else {
			Some(quote_spanned!(trait_path.span()=> <_ as ::xrbk::Buf>::advance(buf, 1);))
		};

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::Readable for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
					unused_mut,
				)]
				fn read_from(
					buf: &mut impl ::xrbk::Buf,
				) -> Result<Self, ::xrbk::ReadError> {
					let mut size: usize = 4;

					// If there is a metabyte element, read it, if not and
					// there is no minor opcode, skip one byte. If there
					// is a minor opcode, do nothing - it has already been
					// read.
					#metabyte
					// Read the request's length.
					#[cfg(not(feature = "big-requests"))]
					let length = <_ as ::xrbk::Buf>::get_u16(buf);
					#[cfg(feature = "big-requests")]
					let mut length = <_ as ::xrbk::Buf>::get_u16(buf) as u32;
					#[cfg(feature = "big-requests")]
					if length == 0 {
						length = <_ as ::xrbk::Buf>::get_u32(buf);
					}

					let buf = &mut <_ as ::xrbk::Buf>::take(
						buf,
						((length - 1) as usize) * 4,
					);

					// Read other elements.
					#reads

					// Construct and return Self.
					Ok(Self #cons)
				}
			}
		));
	}
}

impl Reply {
	pub fn impl_readable(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let cons = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let reads = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				if !element.is_metabyte() && !element.is_sequence() {
					element.read_tokens(tokens, DefinitionType::Reply);

					// if self.content.contains_infer() {
					element.add_x11_size_tokens(tokens);
					// }
				}
			}
		});

		let metabyte = if let Some(element) = self.content.metabyte_element() {
			TokenStream2::with_tokens(|tokens| {
				element.read_tokens(tokens, DefinitionType::Reply);
			})
		} else {
			quote_spanned!(trait_path.span()=> <_ as ::xrbk::Buf>::advance(buf, 1);)
		};

		let sequence = match self.content.sequence_element() {
			Some(Element::Field(field)) => &field.formatted,
			_ => panic!("replies must have a sequence field"),
		};

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::Readable for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
					unused_mut,
				)]
				fn read_from(
					buf: &mut impl ::xrbk::Buf,
				) -> Result<Self, ::xrbk::ReadError> {
					let mut size: usize = 8;

					// Metabyte position
					#metabyte
					// Sequence field
					let #sequence = <_ as ::xrbk::Buf>::get_u16(buf);
					// Length
					let length = <_ as ::xrbk::Buf>::get_u32(buf);
					let buf = &mut <_ as ::xrbk::Buf>::take(
						buf,
						(((length) as usize) * 4) + (32 - 8),
					);

					// Other elements
					#reads

					// Construct and return Self.
					Ok(Self #cons)
				}
			}
		));
	}
}

impl Event {
	pub fn impl_readable(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let x11_size: usize = if self.content.sequence_element().is_some() {
			4
		} else {
			1
		};

		let cons = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let reads = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				if !element.is_metabyte() && !element.is_sequence() {
					element.read_tokens(tokens, DefinitionType::Event);

					// if self.content.contains_infer() {
					element.add_x11_size_tokens(tokens);
					// }
				}
			}
		});

		let metabyte = if self.content.sequence_element().is_none() {
			None
		} else if let Some(element) = self.content.metabyte_element() {
			Some(TokenStream2::with_tokens(|tokens| {
				element.read_tokens(tokens, DefinitionType::Event);
			}))
		} else {
			Some(quote_spanned!(trait_path.span()=>
				<_ as ::xrbk::Buf>::advance(buf, 1);
			))
		};

		let sequence = if let Some(Element::Field(field)) = self.content.sequence_element() {
			let formatted = &field.formatted;

			Some(quote_spanned!(trait_path.span()=>
				let #formatted = <_ as ::xrbk::Buf>::get_u16(buf);
			))
		} else {
			None
		};

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::Readable for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
					unused_mut,
				)]
				fn read_from(
					buf: &mut impl ::xrbk::Buf,
				) -> Result<Self, ::xrbk::ReadError> {
					let mut size: usize = #x11_size;

					// Metabyte position
					#metabyte
					// Sequence field
					#sequence

					// Other elements
					#reads

					// Construct and return Self.
					Ok(Self #cons)
				}
			}
		));
	}
}

impl Error {
	pub fn impl_readable(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let cons = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let reads = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				if element.is_normal() {
					element.read_tokens(tokens, DefinitionType::Error);

					// if self.content.contains_infer() {
					element.add_x11_size_tokens(tokens);
					// }
				}
			}
		});

		let sequence = match self.content.sequence_element() {
			Some(Element::Field(field)) => {
				let formatted = &field.formatted;

				quote_spanned!(trait_path.span()=>
					let #formatted = <_ as ::xrbk::Buf>::get_u16(buf);
				)
			},

			_ => panic!("errors must have sequence fields"),
		};

		let minor_opcode = match self.content.minor_opcode_element() {
			Some(Element::Field(field)) => {
				let formatted = &field.formatted;

				quote_spanned!(trait_path.span()=>
					let #formatted = <_ as ::xrbk::Buf>::get_u16(buf);
				)
			},

			_ => panic!("errors must have minor opcode fields"),
		};

		let major_opcode = match self.content.major_opcode_element() {
			Some(Element::Field(field)) => {
				let formatted = &field.formatted;

				quote_spanned!(trait_path.span()=>
					let #formatted = <_ as ::xrbk::Buf>::get_u8(buf);
				)
			},

			_ => panic!("errors must have major opcode fields"),
		};

		let error_data = match self.content.error_data_element() {
			Some(Element::Field(field)) => {
				TokenStream2::with_tokens(|tokens| field.read_tokens(tokens))
			},

			_ => quote_spanned!(trait_path.span()=> <_ as ::xrbk::Buf>::advance(buf, 4);),
		};

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::Readable for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
					unused_mut,
				)]
				fn read_from(
					buf: &mut impl ::xrbk::Buf,
				) -> Result<Self, ::xrbk::ReadError> {
					// 11 bytes includes:
					// - 1 byte to say it's an error
					// - 1 byte for its code
					// - 2 bytes for its sequence number
					// - 4 bytes for its (optional) error data
					// - 2 bytes for the request's minor opcode
					// - 1 byte for the request's major opcode
					let mut size: usize = 11;

					#sequence
					#error_data
					#minor_opcode
					#major_opcode

					#reads

					Ok(Self #cons)
				}
			}
		));
	}
}

impl Enum {
	pub fn impl_readable(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;
		let discrim_type = self.discriminant_type.as_ref().map_or_else(
			|| quote_spanned!(trait_path.span()=> u8),
			|(_, r#type)| r#type.to_token_stream(),
		);

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = &self.where_clause;

		let discriminants = TokenStream2::with_tokens(|tokens| {
			for variant in &self.variants {
				if let Some((_, expr)) = &variant.discriminant {
					let ident = format_ident!("discrim_{}", variant.ident);

					tokens.append_tokens(quote_spanned!(trait_path.span()=>
						// Isolate the discriminant's expression in a
						// function so that it doesn't have access to
						// identifiers used in the surrounding generated
						// code.
						#[allow(non_snake_case)]
						fn #ident() -> #discrim_type {
							(#expr) as #discrim_type
						}

						// Call the discriminant's function just once and
						// store it in a variable for later use.
						#[allow(non_snake_case)]
						let #ident = #ident();
					));
				}
			}
		});

		let arms = TokenStream2::with_tokens(|tokens| {
			let mut discrim = quote_spanned!(trait_path.span()=> 0);

			for variant in &self.variants {
				let ident = &variant.ident;

				let declare_x11_size = {
					let discrim_type = quote_spanned!(discrim_type.span()=>
						<#discrim_type as ::xrbk::ConstantX11Size>
					);

					quote_spanned!(trait_path.span()=>
						let mut size: usize = #discrim_type::X11_SIZE;
					)
				};

				if variant.discriminant.is_some() {
					let discrim_ident = format_ident!("discrim_{}", ident);

					discrim = discrim_ident.into_token_stream();
				}

				let cons = TokenStream2::with_tokens(|tokens| {
					variant.content.pat_cons_to_tokens(tokens);
				});

				let reads = TokenStream2::with_tokens(|tokens| {
					for element in &variant.content {
						element.read_tokens(tokens, DefinitionType::Basic);

						// if variant.content.contains_infer() {
						element.add_x11_size_tokens(tokens);
						// }
					}
				});

				tokens.append_tokens(quote_spanned!(trait_path.span()=>
					discrim if discrim == #discrim => {
						#declare_x11_size

						#reads

						Ok(Self::#ident #cons)
					},
				));

				quote_spanned!(trait_path.span()=> /* discrim */ + 1).to_tokens(&mut discrim);
			}
		});

		let discrim_type = quote_spanned!(discrim_type.span()=>
			<#discrim_type as ::xrbk::Readable>
		);

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::Readable for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
					clippy::unnecessary_cast,
					unused_mut,
				)]
				fn read_from(
					buf: &mut impl ::xrbk::Buf,
				) -> Result<Self, ::xrbk::ReadError> {
					// Define functions and variables for variants which
					// have custom discriminant expressions.
					#discriminants

					match #discrim_type::read_from(buf)? {
						#arms

						other_discrim => Err(
							::xrbk::ReadError::UnrecognizedDiscriminant(other_discrim as usize),
						),
					}
				}
			}
		));
	}
}
