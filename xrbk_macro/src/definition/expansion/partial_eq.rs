// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use crate::ext::TsExt;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote_spanned;
use syn::Path;

impl Struct {
	pub fn impl_partial_eq(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		// Expand the tokens to read each element.
		let eqs = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				element.partial_eq_tokens(tokens);
			}
		});

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::std::cmp::PartialEq for #ident #type_generics #where_clause {
				fn eq(&self, other: &Self) -> bool {
					// Default value which will be compared to all checked fields
					true

					// All the fields checks
					#eqs
				}
			}
		));
	}
}

impl Enum {
	pub fn impl_partial_eq(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let arms = TokenStream2::with_tokens(|tokens| {
			for variant in &self.variants {
				let ident = &variant.ident;

				let pat = TokenStream2::with_tokens(|tokens| {
					variant.content.pat_cons_to_tokens(tokens);
				});

				let sizes = TokenStream2::with_tokens(|tokens| {
					for element in &variant.content {
						element.partial_eq_tokens(tokens);
					}
				});

				tokens.append_tokens(quote_spanned!(trait_path.span()=>
					Self::#ident #pat => {
						// Default value which will be compared to all checked fields
						true

						// All the fields checks
						#sizes
					},
				));
			}
		});

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::std::cmp::PartialEq for #ident #type_generics #where_clause {
				fn eq(&self, other: &Self) -> bool {
					match self {
						#arms
					}
				}
			}
		));
	}
}

impl Request {
	pub fn impl_partial_eq(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		// Expand the tokens to read each element.
		let eqs = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				element.partial_eq_tokens(tokens);
			}
		});

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::std::cmp::PartialEq for #ident #type_generics #where_clause {
				fn eq(&self, other: &Self) -> bool {
					// Default value which will be compared to all checked fields
					true

					// All the fields checks
					#eqs
				}
			}
		));
	}
}

impl Reply {
	pub fn impl_partial_eq(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		// Expand the tokens to read each element.
		let eqs = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				element.partial_eq_tokens(tokens);
			}
		});

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::std::cmp::PartialEq for #ident #type_generics #where_clause {
				fn eq(&self, other: &Self) -> bool {
					// Default value which will be compared to all checked fields
					true

					// All the fields checks
					#eqs
				}
			}
		));
	}
}

impl Event {
	pub fn impl_partial_eq(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		// Expand the tokens to read each element.
		let eqs = TokenStream2::with_tokens(|tokens| {
			for element in &self.content {
				element.partial_eq_tokens(tokens);
			}
		});

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::std::cmp::PartialEq for #ident #type_generics #where_clause {
				fn eq(&self, other: &Self) -> bool {
					// Default value which will be compared to all checked fields
					true

					// All the fields checks
					#eqs
				}
			}
		));
	}
}
