// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use crate::{element::Element, TsExt};
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;

impl Metadata {
	pub fn impl_readable(&self, tokens: &mut TokenStream2, content: &Content) {
		match self {
			Self::Struct(r#struct) => r#struct.impl_readable(tokens, content),

			Self::Request(request) => request.impl_readable(tokens, content),
			Self::Reply(reply) => reply.impl_readable(tokens, content),
			Self::Event(event) => event.impl_readable(tokens, content),
		}
	}
}

impl Struct {
	pub fn impl_readable(&self, tokens: &mut TokenStream2, content: &Content) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		// Expand the tokens to declare the datasize variable if there is an
		// UnusedContent::Infer unused bytes element to use it.
		let declare_datasize = if content.contains_infer() {
			Some(quote!(let mut datasize: usize = 0;))
		} else {
			None
		};

		// Expand the tokens to call Self's constructor.
		let cons = TokenStream2::with_tokens(|tokens| {
			content.fields_to_tokens(tokens);
		});

		// Expand the tokens to read each element.
		let reads = TokenStream2::with_tokens(|tokens| {
			for element in content {
				element.read_tokens(tokens, DefinitionType::Basic);

				if content.contains_infer() {
					element.add_datasize_tokens(tokens);
				}
			}
		});

		tokens.append_tokens(|| {
			quote!(
				#[automatically_derived]
				impl #impl_generics cornflakes::Readable for #ident #type_generics #where_clause {
					fn read_from(
						buf: &mut impl cornflakes::Buf,
					) -> Result<Self, cornflakes::ReadError> {
						// Declare a datasize variable if it is going to be
						// used in an infer unused bytes element.
						#declare_datasize

						// Read each element.
						#reads

						// Construct and return `Self`.
						Ok(Self #cons)
					}
				}
			)
		});
	}
}

impl Request {
	pub fn impl_readable(&self, tokens: &mut TokenStream2, content: &Content) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let declare_datasize = if content.contains_infer() {
			// The datasize starts at `4` to account for the size of a request's header
			// being 4 bytes.
			Some(quote!(let mut datasize: usize = 4;))
		} else {
			None
		};

		let cons = TokenStream2::with_tokens(|tokens| {
			content.fields_to_tokens(tokens);
		});

		let reads = TokenStream2::with_tokens(|tokens| {
			for element in content {
				if !element.is_metabyte() && !element.is_sequence() {
					element.read_tokens(tokens, DefinitionType::Request);

					if content.contains_infer() {
						element.add_datasize_tokens(tokens);
					}
				}
			}
		});

		let metabyte = if self.minor_opcode.is_some() {
			// If there is a minor opcode, then it has already been read in order to
			// determine that this is the request to read.
			None
		} else if let Some(element) = content.metabyte_element() {
			Some(TokenStream2::with_tokens(|tokens| {
				element.read_tokens(tokens, DefinitionType::Request);
			}))
		} else {
			Some(quote!(buf.advance(1);))
		};

		tokens.append_tokens(|| {
			quote!(
				#[automatically_derived]
				impl #impl_generics cornflakes::Readable for #ident #type_generics #where_clause {
					fn read_from(
						buf: &mut impl cornflakes::Buf,
					) -> Result<Self, cornflakes::ReadError> {
						#declare_datasize

						// If there is a metabyte element, read it, if not and
						// there is no minor opcode, skip one byte. If there
						// is a minor opcode, do nothing - it has already been
						// read.
						#metabyte
						// Read the request's length.
						let length = buf.get_u16();

						// Read other elements.
						#reads

						// Construct and return Self.
						Ok(Self #cons)
					}
				}
			)
		});
	}
}

impl Reply {
	pub fn impl_readable(&self, tokens: &mut TokenStream2, content: &Content) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let declare_datasize = if content.contains_infer() {
			Some(quote!(let mut datasize: usize = 8;))
		} else {
			None
		};

		let cons = TokenStream2::with_tokens(|tokens| {
			content.fields_to_tokens(tokens);
		});

		let reads = TokenStream2::with_tokens(|tokens| {
			for element in content {
				if !element.is_metabyte() && !element.is_sequence() {
					element.read_tokens(tokens, DefinitionType::Reply);

					if content.contains_infer() {
						element.add_datasize_tokens(tokens);
					}
				}
			}
		});

		let metabyte = if let Some(element) = content.metabyte_element() {
			TokenStream2::with_tokens(|tokens| {
				element.read_tokens(tokens, DefinitionType::Reply);
			})
		} else {
			quote!(buf.advance(1);)
		};

		let sequence = match content.sequence_element() {
			Some(Element::Field(field)) => &field.formatted,
			_ => panic!("replies must have a sequence field"),
		};

		tokens.append_tokens(|| {
			quote!(
				#[automatically_derived]
				impl #impl_generics cornflakes::Readable for #ident #type_generics #where_clause {
					fn read_from(
						buf: &mut impl cornflakes::Buf,
					) -> Result<Self, cornflakes::ReadError> {
						#declare_datasize

						// Metabyte position
						#metabyte
						// Sequence field
						let #sequence = buf.get_u16();
						// Length
						let length = buf.get_u32();

						// Other elements
						#reads

						// Construct and return Self.
						Ok(Self #cons)
					}
				}
			)
		});
	}
}

impl Event {
	pub fn impl_readable(&self, tokens: &mut TokenStream2, content: &Content) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let declare_datasize = if content.contains_infer() {
			let datasize: usize = if content.sequence_element().is_some() {
				4
			} else {
				1
			};

			Some(quote!(let mut datasize: usize = #datasize;))
		} else {
			None
		};

		let cons = TokenStream2::with_tokens(|tokens| {
			content.fields_to_tokens(tokens);
		});

		let reads = TokenStream2::with_tokens(|tokens| {
			for element in content {
				if !element.is_metabyte() && !element.is_sequence() {
					element.read_tokens(tokens, DefinitionType::Event);

					if content.contains_infer() {
						element.add_datasize_tokens(tokens);
					}
				}
			}
		});

		let metabyte = if content.sequence_element().is_none() {
			None
		} else if let Some(element) = content.metabyte_element() {
			Some(TokenStream2::with_tokens(|tokens| {
				element.read_tokens(tokens, DefinitionType::Event);
			}))
		} else {
			Some(quote!(
				buf.advance(1);
			))
		};

		let sequence = if let Some(Element::Field(field)) = content.sequence_element() {
			let formatted = &field.formatted;

			Some(quote!(let #formatted = buf.get_u16();))
		} else {
			None
		};

		tokens.append_tokens(|| {
			quote!(
				#[automatically_derived]
				impl #impl_generics cornflakes::Readable for #ident #type_generics #where_clause {
					fn read_from(
						buf: &mut impl cornflakes::Buf,
					) -> Result<Self, cornflakes::ReadError> {
						#declare_datasize

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
			)
		});
	}
}

impl Enum {
	pub fn impl_readable(&self, tokens: &mut TokenStream2) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let discriminants = TokenStream2::with_tokens(|tokens| {
			for variant in &self.variants {
				if let Some((_, expr)) = &variant.discriminant {
					let ident = format_ident!("discrim_{}", variant.ident);

					tokens.append_tokens(|| {
						quote!(
							// Isolate the discriminant's expression in a
							// function so that it doesn't have access to
							// identifiers used in the surrounding generated
							// code.
							#[allow(non_snake_case)]
							fn #ident() -> u8 {
								#expr
							}

							// Call the discriminant's function just once and
							// store it in a variable for later use.
							#[allow(non_snake_case)]
							let #ident = #ident();
						)
					});
				}
			}
		});

		let arms = TokenStream2::with_tokens(|tokens| {
			let mut discrim = quote!(0);

			for variant in &self.variants {
				let ident = &variant.ident;

				let declare_datasize = if variant.content.contains_infer() {
					// The datasize starts at `1` to account for the
					// discriminant.
					Some(quote!(let mut datasize: usize = 1;))
				} else {
					None
				};

				if variant.discriminant.is_some() {
					let discrim_ident = format_ident!("discrim_{}", ident);

					discrim = discrim_ident.into_token_stream();
				}

				let cons = TokenStream2::with_tokens(|tokens| {
					variant.content.fields_to_tokens(tokens);
				});

				let reads = TokenStream2::with_tokens(|tokens| {
					for element in &variant.content {
						element.read_tokens(tokens, DefinitionType::Basic);

						if variant.content.contains_infer() {
							element.add_datasize_tokens(tokens);
						}
					}
				});

				tokens.append_tokens(|| {
					quote!(
						discrim if discrim == #discrim => {
							#declare_datasize

							#reads

							Ok(Self::#ident #cons)
						},
					)
				});

				quote!(/* discrim */ + 1).to_tokens(&mut discrim);
			}
		});

		tokens.append_tokens(|| {
			quote!(
				#[automatically_derived]
				impl #impl_generics cornflakes::Readable for #ident #type_generics #where_clause {
					fn read_from(
						buf: &mut impl cornflakes::Buf,
					) -> Result<Self, cornflakes::ReadError> {
						// Define functions and variables for variants which
						// have custom discriminant expressions.
						#discriminants

						match buf.get_u8() {
							#arms

							other_discrim => Err(
								cornflakes::ReadError::UnrecognizedDiscriminant(other_discrim),
							),
						}
					}
				}
			)
		});
	}
}
