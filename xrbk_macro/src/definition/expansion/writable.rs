// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use crate::{element::Element, TsExt};

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote_spanned, ToTokens};
use syn::Path;

impl Struct {
	pub fn impl_writable(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let declare_x11_size = if self.content.contains_infer() {
			Some(quote_spanned!(trait_path.span()=> let mut size: usize = 0;))
		} else {
			None
		};

		let pat = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let writes = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				element.write_tokens(tokens, DefinitionType::Basic);

				if self.content.contains_infer() {
					element.add_x11_size_tokens(tokens);
				}
			}
		});

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::Writable for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
				)]
				fn write_to(
					&self,
					buf: &mut impl ::xrbk::BufMut,
				) -> Result<(), ::xrbk::WriteError> {
					#declare_x11_size
					// Destructure the struct's fields, if any.
					let Self #pat = self;

					#writes

					Ok(())
				}
			}
		));
	}
}

impl Request {
	pub fn impl_writable(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let declare_x11_size = if self.content.contains_infer() {
			// The x11_size starts at `4` to account for the size of a request's header
			// being 4 bytes.
			Some(quote_spanned!(trait_path.span()=> let mut size: usize = 4;))
		} else {
			None
		};

		let pat = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let writes = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				if !element.is_metabyte() && !element.is_sequence() {
					element.write_tokens(tokens, DefinitionType::Request);

					if self.content.contains_infer() {
						element.add_x11_size_tokens(tokens);
					}
				}
			}
		});

		let metabyte = if self.minor_opcode.is_some() {
			// TODO: can't be in metabyte, must check this in protocol!!
			quote_spanned!(trait_path.span()=>
				buf.put_u16(<Self as xrb::message::Request>::MINOR_OPCODE.unwrap());
			)
		} else if let Some(element) = self.content.metabyte_element() {
			TokenStream2::with_tokens(|tokens| {
				element.write_tokens(tokens, DefinitionType::Request);
			})
		} else {
			quote_spanned!(trait_path.span()=>
				buf.put_u8(0);
			)
		};

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::Writable for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
				)]
				fn write_to(
					&self,
					buf: &mut impl ::xrbk::BufMut,
				) -> Result<(), ::xrbk::WriteError> {
					#declare_x11_size
					// Destructure the request struct's fields, if any.
					let Self #pat = self;

					// Major opcode
					buf.put_u8(<Self as xrb::message::Request>::MAJOR_OPCODE);
					// Metabyte position
					#metabyte
					// Length
					buf.put_u16(<Self as xrb::message::Request>::length(&self));

					// Other elements
					#writes

					Ok(())
				}
			}
		));
	}
}

impl Reply {
	pub fn impl_writable(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let declare_x11_size = if self.content.contains_infer() {
			// The x11_size starts at `8` to account for the size of a reply's\
			// header being 8 bytes.
			Some(quote_spanned!(trait_path.span()=> let mut size: usize = 8;))
		} else {
			None
		};

		let pat = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let writes = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				if !element.is_metabyte() && !element.is_sequence() {
					element.write_tokens(tokens, DefinitionType::Reply);

					if self.content.contains_infer() {
						element.add_x11_size_tokens(tokens);
					}
				}
			}
		});

		let metabyte = if let Some(element) = self.content.metabyte_element() {
			TokenStream2::with_tokens(|tokens| {
				element.write_tokens(tokens, DefinitionType::Reply);
			})
		} else {
			quote_spanned!(trait_path.span()=>
				buf.put_u8(0);
			)
		};

		let sequence = match self.content.sequence_element() {
			Some(Element::Field(field)) => &field.formatted,
			_ => panic!("replies must have a sequence field"),
		};

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::Writable for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
				)]
				fn write_to(
					&self,
					buf: &mut impl ::xrbk::BufMut,
				) -> Result<(), ::xrbk::WriteError> {
					#declare_x11_size
					// Destructure the reply struct's fields, if any.
					let Self #pat = self;

					// `1` - indicates this is a reply
					buf.put_u8(1);
					// Metabyte position
					#metabyte
					// Sequence field
					buf.put_u16(*#sequence);
					// Length
					buf.put_u32(<Self as xrb::message::Reply>::length(&self));

					// Other elements
					#writes

					Ok(())
				}
			}
		));
	}
}

impl Event {
	pub fn impl_writable(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let declare_x11_size = if self.content.contains_infer() {
			let x11_size: usize = if self.content.sequence_element().is_some() {
				4
			} else {
				1
			};

			Some(quote_spanned!(trait_path.span()=> let mut size: usize = #x11_size;))
		} else {
			None
		};

		let pat = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let writes = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				if element.is_normal() {
					element.write_tokens(tokens, DefinitionType::Event);

					if self.content.contains_infer() {
						element.add_x11_size_tokens(tokens);
					}
				}
			}
		});

		let metabyte = if self.content.sequence_element().is_none() {
			None
		} else if let Some(element) = self.content.metabyte_element() {
			Some(TokenStream2::with_tokens(|tokens| {
				element.write_tokens(tokens, DefinitionType::Event);
			}))
		} else {
			Some(quote_spanned!(trait_path.span()=>
				buf.put_u8(0);
			))
		};

		let sequence = if let Some(Element::Field(field)) = self.content.sequence_element() {
			let formatted = &field.formatted;

			Some(quote_spanned!(trait_path.span()=> buf.put_u16(*#formatted);))
		} else {
			None
		};

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::Writable for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
				)]
				fn write_to(
					&self,
					buf: &mut impl ::xrbk::BufMut,
				) -> Result<(), ::xrbk::WriteError> {
					#declare_x11_size
					// Destructure the event struct's fields, if any.
					let Self #pat = self;

					// Event code
					buf.put_u8(<Self as xrb::message::Event>::CODE);
					// Metabyte position
					#metabyte
					// Sequence field
					#sequence

					// Other elements
					#writes

					Ok(())
				}
			}
		));
	}
}

impl Error {
	pub fn impl_writable(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let declare_x11_size = if self.content.contains_infer() {
			// 11 bytes includes:
			// - 1 byte to say it's an error
			// - 1 byte for its code
			// - 2 bytes for its sequence number
			// - 4 bytes for its (optional) error data
			// - 2 bytes for the request's minor opcode
			// - 1 byte for the request's major opcode
			Some(quote_spanned!(trait_path.span()=> let mut size: usize = 11;))
		} else {
			None
		};

		let pat = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let writes = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				if element.is_normal() {
					element.write_tokens(tokens, DefinitionType::Error);

					if self.content.contains_infer() {
						element.add_x11_size_tokens(tokens);
					}
				}
			}
		});

		let sequence = match self.content.sequence_element() {
			Some(Element::Field(field)) => {
				let formatted = &field.formatted;

				quote_spanned!(trait_path.span()=> buf.put_u16(*#formatted);)
			},

			_ => panic!("errors must have sequence fields"),
		};

		let minor_opcode = match self.content.minor_opcode_element() {
			Some(Element::Field(field)) => {
				let formatted = &field.formatted;

				quote_spanned!(trait_path.span()=> buf.put_u16(*#formatted);)
			},

			_ => panic!("errors must have minor opcode fields"),
		};

		let major_opcode = match self.content.major_opcode_element() {
			Some(Element::Field(field)) => {
				let formatted = &field.formatted;

				quote_spanned!(trait_path.span()=> buf.put_u8(*#formatted);)
			},

			_ => panic!("errors must have major opcode fields"),
		};

		let error_data = match self.content.error_data_element() {
			Some(Element::Field(field)) => {
				TokenStream2::with_tokens(|tokens| field.write_tokens(tokens))
			},

			_ => quote_spanned!(trait_path.span()=> buf.put_bytes(0, 4);),
		};

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::Writable for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
				)]
				fn write_to(
					&self,
					buf: &mut impl ::xrbk::BufMut,
				) -> Result<(), ::xrbk::WriteError> {
					#declare_x11_size
					// Destructure the error struct's fields, if any.
					let Self #pat = self;

					// A first byte of `0` means that this is an error.
					buf.put_u8(0);
					// Error code, uniquely identifying the error.
					buf.put_u8(<Self as xrb::message::Error>::CODE);
					// Sequence number.
					#sequence
					// An optional 4-byte data field.
					#error_data
					// The minor opcode of the request generating the error.
					#minor_opcode
					// The major opcode of the request generating the error.
					#major_opcode

					// Other elements.
					#writes

					Ok(())
				}
			}
		));
	}
}

impl Enum {
	pub fn impl_writable(&self, tokens: &mut TokenStream2, trait_path: &Path) {
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

				let declare_x11_size = if variant.content.contains_infer() {
					let discrim_type = quote_spanned!(discrim_type.span()=>
						<#discrim_type as ::xrbk::ConstantX11Size>
					);

					Some(quote_spanned!(trait_path.span()=>
						let mut size: usize = #discrim_type::X11_SIZE;
					))
				} else {
					None
				};

				if variant.discriminant.is_some() {
					let discrim_ident = format_ident!("discrim_{}", ident);

					discrim = discrim_ident.into_token_stream();
				}

				let pat = TokenStream2::with_tokens(|tokens| {
					variant.content.pat_cons_to_tokens(tokens);
				});

				let writes = TokenStream2::with_tokens(|tokens| {
					for element in &variant.content {
						element.write_tokens(tokens, DefinitionType::Basic);

						if variant.content.contains_infer() {
							element.add_x11_size_tokens(tokens);
						}
					}
				});

				let discrim_writable = quote_spanned!(discrim_type.span()=>
					<#discrim_type as ::xrbk::Writable>
				);

				tokens.append_tokens(quote_spanned!(trait_path.span()=>
					Self::#ident #pat => {
						#declare_x11_size
						#discrim_writable::write_to(&((#discrim) as #discrim_type), buf)?;

						#writes
					},
				));

				quote_spanned!(trait_path.span()=> /* discrim */ + 1).to_tokens(&mut discrim);
			}
		});

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::Writable for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
					clippy::cast_possible_truncation,
					clippy::unnecessary_cast,
				)]
				fn write_to(
					&self,
					buf: &mut impl ::xrbk::BufMut,
				) -> Result<(), ::xrbk::WriteError> {
					// Define functions and variables for variants which
					// have custom discriminant expressions.
					#discriminants

					match self {
						#arms
					}

					Ok(())
				}
			}
		));
	}
}
