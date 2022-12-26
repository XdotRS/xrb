// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use crate::TsExt;

use proc_macro2::TokenStream as TokenStream2;

impl Struct {
	pub fn impl_datasize(&self, tokens: &mut TokenStream2) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let pat = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let datasizes = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				element.datasize_tokens(tokens, DefinitionType::Basic);
			}
		});

		tokens.append_tokens(|| {
			quote!(
				#[automatically_derived]
				impl #impl_generics cornflakes::DataSize for #ident #type_generics #where_clause {
					fn data_size(&self) -> usize {
						let mut datasize: usize = 0;
						// Destructure the struct's fields, if any.
						let Self #pat = self;

						// Add the datasize of each element.
						#datasizes

						// Return the cumulative datasize.
						datasize
					}
				}
			)
		});
	}
}

impl Request {
	pub fn impl_datasize(&self, tokens: &mut TokenStream2) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let pat = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let datasizes = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				if !element.is_metabyte() && !element.is_sequence() {
					element.datasize_tokens(tokens, DefinitionType::Request);
				}
			}
		});

		tokens.append_tokens(|| {
			quote!(
				#[automatically_derived]
				impl #impl_generics cornflakes::DataSize for #ident #type_generics #where_clause {
					fn data_size(&self) -> usize {
						// The datasize starts at `4` to account for the size
						// of a request's header being 4 bytes.
						let mut datasize: usize = 4;
						// Destructure the request's fields, if any.
						let Self #pat = self;

						// Add the datasize of each element.
						#datasizes

						// Return the cumulative datasize.
						datasize
					}
				}
			)
		});
	}
}

impl Reply {
	pub fn impl_datasize(&self, tokens: &mut TokenStream2) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let pat = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let datasizes = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				if !element.is_metabyte() && !element.is_sequence() {
					element.datasize_tokens(tokens, DefinitionType::Reply);
				}
			}
		});

		tokens.append_tokens(|| {
			quote!(
				#[automatically_derived]
				impl #impl_generics cornflakes::DataSize for #ident #type_generics #where_clause {
					fn data_size(&self) -> usize {
						// The datasize starts at `8` to account for the size
						// of a reply's header being 8 bytes.
						let mut datasize: usize = 8;
						// Destructure the reply's fields, if any.
						let Self #pat = self;

						// Add the datasize of each element.
						#datasizes

						// Return the cumulative datasize.
						datasize
					}
				}
			)
		});
	}
}

impl Event {
	pub fn impl_datasize(&self, tokens: &mut TokenStream2) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let datasize: usize = if self.content.sequence_element().is_some() {
			4
		} else {
			1
		};

		let pat = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let datasizes = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				if !element.is_metabyte() && !element.is_sequence() {
					element.datasize_tokens(tokens, DefinitionType::Event);
				}
			}
		});

		tokens.append_tokens(|| {
			quote!(
				#[automatically_derived]
				impl #impl_generics cornflakes::DataSize for #ident #type_generics #where_clause {
					fn data_size(&self) -> usize {
						// The datasize starts at either `4` or `1`, depending
						// on whether there is a sequence field and metabyte
						// position, to account for the size of the event's
						// header.
						let mut datasize: usize = #datasize;
						// Destructure the event's fields, if any.
						let Self #pat = self;

						// Add the datasize of each element.
						#datasizes

						// Return the cumulative datasize.
						datasize
					}
				}
			)
		});
	}
}

impl Enum {
	pub fn impl_datasize(&self, tokens: &mut TokenStream2) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let arms = TokenStream2::with_tokens(|tokens| {
			for variant in &self.variants {
				let ident = &variant.ident;

				let pat = TokenStream2::with_tokens(|tokens| {
					variant.content.pat_cons_to_tokens(tokens);
				});

				let datasizes = TokenStream2::with_tokens(|tokens| {
					for element in &variant.content {
						element.datasize_tokens(tokens, DefinitionType::Basic);
					}
				});

				tokens.append_tokens(|| {
					quote!(
						Self::#ident #pat => {
							// Add the datasize of each element.
							#datasizes
						},
					)
				});
			}
		});

		tokens.append_tokens(|| {
			quote!(
				#[automatically_derived]
				impl #impl_generics cornflakes::DataSize for #ident #type_generics #where_clause {
					fn data_size(&self) -> usize {
						// The datasize starts at `1` to account for the
						// discriminant.
						let mut datasize: usize = 1;

						match self {
							#arms
						}

						// Return the cumulative datasize.
						datasize
					}
				}
			)
		});
	}
}
