// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::quote_spanned;
use syn::Path;

use crate::{element::Element, TsExt};

use super::*;

impl Struct {
	pub fn impl_x11_size(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let pat = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let sizes = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				element.x11_size_tokens(tokens, DefinitionType::Basic);
			}
		});

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::X11Size for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
					unused_mut,
				)]
				fn x11_size(&self) -> usize {
					let mut size: usize = 0;
					// Destructure the struct's fields, if any.
					let Self #pat = self;

					// Add the size of each element.
					#sizes

					// Return the cumulative size.
					size
				}
			}
		));
	}
}

impl Request {
	pub fn impl_x11_size(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let pat = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let sizes = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				if element.is_normal() {
					element.x11_size_tokens(tokens, DefinitionType::Request);
				} else if let Element::Let(r#let) = element && element.is_metabyte() {
					r#let.function_call_tokens(tokens);
				}
			}
		});

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::X11Size for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
					unused_mut,
				)]
				fn x11_size(&self) -> usize {
					// The size starts at `4` to account for the size
					// of a request's header being 4 bytes.
					let mut size: usize = 4;
					// Destructure the request's fields, if any.
					let Self #pat = self;

					// Add the size of each element.
					#sizes

					// Return the cumulative size.
					size
				}
			}
		));
	}
}

impl Reply {
	pub fn impl_x11_size(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let pat = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let sizes = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				if element.is_normal() {
					element.x11_size_tokens(tokens, DefinitionType::Reply);
				} else if let Element::Let(r#let) = element && element.is_metabyte() {
					r#let.function_call_tokens(tokens);
				}
			}
		});

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::X11Size for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
					unused_mut,
				)]
				fn x11_size(&self) -> usize {
					// The size starts at `8` to account for the size
					// of a reply's header being 8 bytes.
					let mut size: usize = 8;
					// Destructure the reply's fields, if any.
					let Self #pat = self;

					// Add the size of each element.
					#sizes

					// Return the cumulative size.
					size
				}
			}
		));
	}
}

impl Event {
	pub fn impl_x11_size(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let size: usize = if self.content.sequence_element().is_some() {
			4
		} else {
			1
		};

		let pat = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let sizes = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				if element.is_normal() {
					element.x11_size_tokens(tokens, DefinitionType::Event);
				} else if let Element::Let(r#let) = element && element.is_metabyte() {
					r#let.function_call_tokens(tokens);
				}
			}
		});

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::X11Size for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
					unused_mut,
				)]
				fn x11_size(&self) -> usize {
					// The size starts at either `4` or `1`, depending
					// on whether there is a sequence field and metabyte
					// position, to account for the size of the event's
					// header.
					let mut size: usize = #size;
					// Destructure the event's fields, if any.
					let Self #pat = self;

					// Add the size of each element.
					#sizes

					// Return the cumulative size.
					size
				}
			}
		));
	}
}

impl Error {
	pub fn impl_x11_size(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = match &self.content {
			StructlikeContent::Regular { where_clause, .. } => where_clause,
			StructlikeContent::Tuple { where_clause, .. } => where_clause,
			StructlikeContent::Unit { where_clause, .. } => where_clause,
		};

		let pat = TokenStream2::with_tokens(|tokens| {
			self.content.pat_cons_to_tokens(tokens);
		});

		let sizes = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				if element.is_normal() {
					element.x11_size_tokens(tokens, DefinitionType::Error);
				} else if let Element::Let(r#let) = element && element.is_metabyte() {
					r#let.function_call_tokens(tokens);
				}
			}
		});

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::X11Size for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
					unused_mut,
				)]
				fn x11_size(&self) -> usize {
					// At least 11 bytes including all the required fields.
					let mut size: usize = 11;
					// Destructure the error's fields, if any.
					let Self #pat = self;

					#sizes

					size
				}
			}
		));
	}
}

impl Enum {
	pub fn impl_x11_size(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;
		let discrim_type = self.discriminant_type.as_ref().map_or_else(
			|| quote_spanned!(trait_path.span()=> u8),
			|(_, r#type)| r#type.to_token_stream(),
		);

		// TODO: add generic bounds
		let (impl_generics, type_generics, _) = self.generics.split_for_impl();
		let where_clause = &self.where_clause;

		let arms = TokenStream2::with_tokens(|tokens| {
			for variant in &self.variants {
				let ident = &variant.ident;

				let pat = TokenStream2::with_tokens(|tokens| {
					variant.content.pat_cons_to_tokens(tokens);
				});

				let sizes = TokenStream2::with_tokens(|tokens| {
					for element in &variant.content {
						element.x11_size_tokens(tokens, DefinitionType::Basic);
					}
				});

				tokens.append_tokens(quote_spanned!(trait_path.span()=>
					Self::#ident #pat => {
						// Add the size of each element.
						#sizes
					},
				));
			}
		});

		let discrim_type = quote_spanned!(discrim_type.span() =>
			<#discrim_type as ::xrbk::ConstantX11Size>
		);

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::xrbk::X11Size for #ident #type_generics #where_clause {
				#[allow(
					clippy::items_after_statements,
					clippy::trivially_copy_pass_by_ref,
					clippy::needless_borrow,
					clippy::identity_op,
					unused_mut,
				)]
				fn x11_size(&self) -> usize {
					let mut size: usize = #discrim_type::X11_SIZE;

					match self {
						#arms
					}

					// Return the cumulative size.
					size
				}
			}
		));
	}
}
