// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{definition::Struct, element::Content, ext::TsExt};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl Struct {
	pub fn impl_writable(&self, tokens: &mut TokenStream2, content: &Content) {
		let ident = &self.ident;

		let declare_datasize = if content.contains_infer() {
			// TODO: start at an appropriate number based on the size of the
			//       header. take into account e.g. whether there's a sequence
			//       field  too...
			Some(quote!(let mut datasize: usize = 0;))
		} else {
			None
		};

		let pat = TokenStream2::with_tokens(|tokens| {
			content.fields_to_tokens(tokens);
		});

		tokens.append_tokens(|| {
			quote!(
				impl #impl_generics cornflakes::Writable for #ident #type_generics #where_clause {
					#[allow(clippy::non_snake_case)]
					fn write_to(
						&self,
						// TODO: re-export `Buf` and `BufMut` in `cornflakes`
						buf: &mut impl bytes::BufMut,
					) -> Result<(), cornflakes::WriteError> {
						#declare_datasize
						// Destructure the struct's fields, if any.
						let Self #pat = self;

						#writes

						Ok(())
					}
				}
			)
		});
	}
}
