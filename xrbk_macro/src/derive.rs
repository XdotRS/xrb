// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Data, Fields, Index};

use crate::TsExt;

pub fn derive_writes(data: &Data) -> TokenStream2 {
	fn derive_for_fields(tokens: &mut TokenStream2, fields: &Fields) {
		match &fields {
			Fields::Named(fields) => {
				for field in &fields.named {
					let ident = &field.ident;
					let r#type = &field.ty;

					tokens.append_tokens(|| {
						quote!(
							<#r#type as cornflakes::Writable>::write_to(&self.#ident, buf)?;
						)
					});
				}
			},

			Fields::Unnamed(fields) => {
				for (i, field) in fields.unnamed.iter().enumerate() {
					let index = Index::from(i);
					let r#type = &field.ty;

					tokens.append_tokens(|| {
						quote!(
							<#r#type as cornflakes::Writable>::write_to(&self.#index, buf)?;
						)
					});
				}
			},

			Fields::Unit => {},
		}
	}

	match data {
		Data::Struct(r#struct) => {
			let mut writes = TokenStream2::new();

			derive_for_fields(&mut writes, &r#struct.fields);

			writes
		},

		Data::Enum(r#enum) => {
			let mut arms = TokenStream2::new();
			let mut discrim = quote!(0);

			for variant in &r#enum.variants {
				let ident = &variant.ident;
				let pat = match &variant.fields {
					Fields::Named(_) => quote!({ .. }),
					Fields::Unnamed(_) => quote!((..)),
					Fields::Unit => TokenStream2::new(),
				};

				if let Some((_, expr)) = &variant.discriminant {
					discrim = expr.to_token_stream();
				}

				let mut writes = TokenStream2::new();

				derive_for_fields(&mut writes, &variant.fields);

				arms.append_tokens(|| {
					quote!(
						#ident #pat => {
							buf.put_u8((#discrim) as u8);

							#writes
						},
					)
				});

				quote!(/* discrim */ + 1).to_tokens(&mut discrim);
			}

			quote!(
				match &self {
					#arms
				}
			)
		},

		Data::Union(_) => unimplemented!(),
	}
}
