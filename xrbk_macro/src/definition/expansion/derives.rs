// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use crate::ext::TsExt;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote_spanned;
use syn::Path;

macro_rules! structlike_impl_partial_eq {
    ($def:path) => {
        impl $def {
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

            pub fn impl_hash(&self, tokens: &mut TokenStream2, trait_path: &Path) {
                let ident = &self.ident;

                // TODO: add generic bounds
                let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

                // Expand the tokens to read each element.
                let hashes = TokenStream2::with_tokens(|tokens| {
                    for element in &self.content {
                        element.hash_tokens(tokens);
                    }
                });

                tokens.append_tokens(quote_spanned!(trait_path.span()=>
                    #[automatically_derived]
                    impl #impl_generics ::core::hash::Hash for #ident #type_generics #where_clause {
                        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                            // All the hashes for all fiels
                            #hashes
                        }
                    }
                ));
            }
        }
    };
}

structlike_impl_partial_eq!(Struct);
structlike_impl_partial_eq!(Request);
structlike_impl_partial_eq!(Reply);
structlike_impl_partial_eq!(Event);

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

				let eqs = TokenStream2::with_tokens(|tokens| {
					for element in &variant.content {
						element.partial_eq_tokens(tokens);
					}
				});

				tokens.append_tokens(quote_spanned!(trait_path.span()=>
					Self::#ident #pat => {
						// Default value which will be compared to all checked fields
						true

						// All the fields checks
						#eqs
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

	pub fn impl_hash(&self, tokens: &mut TokenStream2, trait_path: &Path) {
		let ident = &self.ident;

		// TODO: add generic bounds
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let arms = TokenStream2::with_tokens(|tokens| {
			for variant in &self.variants {
				let ident = &variant.ident;

				let pat = TokenStream2::with_tokens(|tokens| {
					variant.content.pat_cons_to_tokens(tokens);
				});

				let hashes = TokenStream2::with_tokens(|tokens| {
					for element in &variant.content {
						element.hash_tokens(tokens);
					}
				});

				tokens.append_tokens(quote_spanned!(trait_path.span()=>
					Self::#ident #pat => {
						// All the hashes for all fiels
						#hashes
					},
				));
			}
		});

		tokens.append_tokens(quote_spanned!(trait_path.span()=>
			#[automatically_derived]
			impl #impl_generics ::core::hash::Hash for #ident #type_generics #where_clause {
				fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
					match self {
						#arms
					}
				}
			}
		));
	}
}
