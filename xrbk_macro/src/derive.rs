// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{punctuated::Pair, Data, Fields, FieldsNamed, FieldsUnnamed, Index};

use crate::TsExt;

fn pat_cons(fields: &Fields) -> TokenStream2 {
	let mut tokens = TokenStream2::new();

	match fields {
		Fields::Named(FieldsNamed { brace_token, named }) => {
			brace_token.surround(&mut tokens, |tokens| {
				for pair in named.pairs() {
					let (field, comma) = match pair {
						Pair::Punctuated(field, comma) => (field, Some(comma)),
						Pair::End(field) => (field, None),
					};

					let ident = &field.ident;

					quote!(#ident #comma).to_tokens(tokens);
				}
			});
		},

		Fields::Unnamed(FieldsUnnamed {
			paren_token,
			unnamed,
		}) => paren_token.surround(&mut tokens, |tokens| {
			for (i, pair) in unnamed.pairs().enumerate() {
				let comma = match pair {
					Pair::Punctuated(_, comma) => Some(comma),
					Pair::End(_) => None,
				};

				let formatted = format_ident!("field{}", Index::from(i));

				quote!(#formatted #comma).to_tokens(tokens);
			}
		}),

		Fields::Unit => {},
	}

	tokens
}

pub fn derive_writes(data: &Data) -> TokenStream2 {
	fn derive_for_fields(fields: &Fields) -> TokenStream2 {
		TokenStream2::with_tokens(|tokens| match &fields {
			Fields::Named(fields) => {
				for field in &fields.named {
					let ident = &field.ident;
					let r#type = &field.ty;

					tokens.append_tokens(|| {
						quote!(
							<#r#type as xrbk::Writable>::write_to(#ident, buf)?;
						)
					});
				}
			},

			Fields::Unnamed(fields) => {
				for (i, field) in fields.unnamed.iter().enumerate() {
					let formatted = format_ident!("field{}", Index::from(i));
					let r#type = &field.ty;

					tokens.append_tokens(|| {
						quote!(
							<#r#type as xrbk::Writable>::write_to(#formatted, buf)?;
						)
					});
				}
			},

			Fields::Unit => {},
		})
	}

	match data {
		Data::Struct(r#struct) => {
			let pat = pat_cons(&r#struct.fields);
			let writes = derive_for_fields(&r#struct.fields);

			quote!(
				let Self #pat = &self;

				#writes
			)
		},

		Data::Enum(r#enum) => {
			let mut discrim = quote!(0);

			let arms = r#enum.variants.iter().map(|variant| {
				let ident = &variant.ident;

				if let Some((_, expr)) = &variant.discriminant {
					discrim = quote!((#expr));
				}

				let pat = pat_cons(&variant.fields);
				let writes = derive_for_fields(&variant.fields);

				let arm = quote!(
					Self::#ident #pat => {
						buf.put_u8((#discrim) as u8);

						#writes
					},
				);

				quote!(/* discrim */ + 1).to_tokens(&mut discrim);

				arm
			});

			quote!(
				match &self {
					#(#arms)*
				}
			)
		},

		Data::Union(_) => unimplemented!(),
	}
}

pub fn derive_reads(data: &Data) -> TokenStream2 {
	fn derive_for_fields(fields: &Fields) -> TokenStream2 {
		TokenStream2::with_tokens(|tokens| match &fields {
			Fields::Named(fields) => {
				for field in &fields.named {
					let ident = &field.ident;
					let r#type = &field.ty;

					tokens.append_tokens(|| {
						quote!(
							let #ident = <#r#type as xrbk::Readable>::read_from(buf)?;
						)
					});
				}
			},

			Fields::Unnamed(fields) => {
				for (i, field) in fields.unnamed.iter().enumerate() {
					let formatted = format_ident!("field{}", Index::from(i));
					let r#type = &field.ty;

					tokens.append_tokens(|| {
						quote!(
							let #formatted = <#r#type as xrbk::Readable>::read_from(buf)?;
						)
					});
				}
			},

			Fields::Unit => {},
		})
	}

	match data {
		Data::Struct(r#struct) => {
			let cons = pat_cons(&r#struct.fields);
			let reads = derive_for_fields(&r#struct.fields);

			quote!(
				#reads

				Ok(Self #cons)
			)
		},

		Data::Enum(r#enum) => {
			let mut discrim = quote!(0);

			let arms = r#enum.variants.iter().map(|variant| {
				let ident = &variant.ident;

				let cons = pat_cons(&variant.fields);
				let reads = derive_for_fields(&variant.fields);

				if let Some((_, expr)) = &variant.discriminant {
					discrim = quote!((#expr));
				}

				let arm = quote!(
					discrim if discrim == (#discrim) as u8 => {
						#reads

						Ok(Self::#ident #cons)
					},
				);

				quote!(/* discrim */ + 1).to_tokens(&mut discrim);

				arm
			});

			quote!(
				match buf.get_u8() {
					#(#arms)*

					other_discrim => Err(
						xrbk::ReadError::UnrecognizedDiscriminant(other_discrim),
					),
				}
			)
		},

		Data::Union(_) => unimplemented!(),
	}
}

pub fn derive_datasizes(data: &Data) -> TokenStream2 {
	fn derive_for_fields(fields: &Fields) -> TokenStream2 {
		TokenStream2::with_tokens(|tokens| match &fields {
			Fields::Named(fields) => {
				for field in &fields.named {
					let ident = &field.ident;
					let r#type = &field.ty;

					tokens.append_tokens(|| {
						quote!(
							datasize += <#r#type as xrbk::DataSize>::data_size(#ident);
						)
					});
				}
			},

			Fields::Unnamed(fields) => {
				for (i, field) in fields.unnamed.iter().enumerate() {
					let formatted = format_ident!("field{}", i);
					let r#type = &field.ty;

					tokens.append_tokens(|| {
						quote!(
							datasize += <#r#type as xrbk::DataSize>::data_size(#formatted);
						)
					});
				}
			},

			Fields::Unit => {},
		})
	}

	match data {
		Data::Struct(r#struct) => {
			let pat = pat_cons(&r#struct.fields);
			let datasizes = derive_for_fields(&r#struct.fields);

			quote!(
				let Self #pat = &self;
				let mut datasize = 0;

				#datasizes

				datasize
			)
		},

		Data::Enum(r#enum) => {
			let arms = r#enum.variants.iter().map(|variant| {
				let ident = &variant.ident;

				let pat = pat_cons(&variant.fields);
				let datasizes = derive_for_fields(&variant.fields);

				quote!(
					Self::#ident #pat => {
						let mut datasize = 1;

						#datasizes

						datasize
					},
				)
			});

			quote!(
				match &self {
					#(#arms)*
				}
			)
		},

		Data::Union(_) => unimplemented!(),
	}
}

pub fn derive_static_datasizes(data: &Data) -> TokenStream2 {
	fn derive_for_fields(fields: &Fields) -> TokenStream2 {
		TokenStream2::with_tokens(|tokens| match fields {
			Fields::Named(FieldsNamed { named: fields, .. })
			| Fields::Unnamed(FieldsUnnamed {
				unnamed: fields, ..
			}) => {
				for field in fields {
					let r#type = &field.ty;

					tokens.append_tokens(|| {
						quote!(
							datasize += <#r#type as xrbk::StaticDataSize>::static_data_size();
						)
					});
				}
			},

			Fields::Unit => {},
		})
	}

	match data {
		Data::Struct(r#struct) => {
			let datasizes = derive_for_fields(&r#struct.fields);

			quote!(
				let mut datasize = 0;

				#datasizes

				datasize
			)
		},

		Data::Enum(r#enum) => {
			let variant = r#enum
				.variants
				.iter()
				.find(|syn::Variant { attrs, .. }| {
					attrs.iter().any(|attr| attr.path.is_ident("wrapper"))
				})
				.expect("expected #[wrapper] on an enum variant");

			let datasizes = derive_for_fields(&variant.fields);

			quote!(
				// Enum wrappers don't write the discriminant.
				let mut datasize = 0;

				#datasizes

				datasize
			)
		},

		Data::Union(_) => unimplemented!(),
	}
}
