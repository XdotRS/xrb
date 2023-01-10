// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{punctuated::Pair, Attribute, Data, Fields, FieldsNamed, FieldsUnnamed, Index};

use crate::TsExt;

pub fn pat_cons(fields: &Fields) -> TokenStream2 {
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

pub fn args(fields: &Fields) -> TokenStream2 {
	TokenStream2::with_tokens(|tokens| {
		match fields {
			Fields::Named(FieldsNamed { named, ..}) => {
				for pair in named.pairs() {
					let (field, comma) = match pair {
						Pair::Punctuated(field, comma) => (field, Some(comma)),
						Pair::End(field) => (field, None),
					};
					
					field.ident.to_tokens(tokens);
					field.colon_token.to_tokens(tokens);
					field.ty.to_tokens(tokens);
					
					comma.to_tokens(tokens);
				}
			},
			
			Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
				for (i, pair) in unnamed.pairs().enumerate() {
					let (field, comma) = match pair {
						Pair::Punctuated(field, comma) => (field, Some(comma)),
						Pair::End(field) => (field, None),
					};
					
					let formatted = format_ident!("field{}", i);
					let r#type = &field.ty;
					
					quote!(#formatted: #r#type).to_tokens(tokens);
					comma.to_tokens(tokens);
				}
			},
			
			Fields::Unit => {},
		}
	})
}

/// This is used in [`derive_unwrap`] for the tuple return.
/// 
/// This does not construct a tuple, however, it is just the names of the
/// fields that need to be surrounded with `(` and  `)`.
pub fn names(fields: &Fields) -> TokenStream2 {
	TokenStream2::with_tokens(|tokens| {
		match fields {
			Fields::Named(FieldsNamed { named, .. }) => {
				for pair in named.pairs() {
					let (field, comma) = match pair {
						Pair::Punctuated(field, comma) => (field, Some(comma)),
						Pair::End(field) => (field, None),
					};
					
					field.ident.to_tokens(tokens);
					comma.to_tokens(tokens);
				}
			},
			
			Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
				for (i, pair) in unnamed.pairs().enumerate() {
					let comma = match pair {
						Pair::Punctuated(_, comma) => Some(comma),
						Pair::End(_) => None,
					};
					
					format_ident!("field{}", i).to_tokens(tokens);
					comma.to_tokens(tokens);
				}
			},
			
			Fields::Unit => {},
		}
	})
}

pub fn unwrap_return(fields: &Fields) -> TokenStream2 {
	TokenStream2::with_tokens(|tokens| {
		match fields {
			Fields::Named(FieldsNamed { named: fields, .. }) | Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. }) => {
				for pair in fields.pairs() {
					let (field, comma) = match pair {
						Pair::Punctuated(field, comma) => (field, Some(comma)),
						Pair::End(field) => (field, None),
					};
					
					field.ty.to_tokens(tokens);
					comma.to_tokens(tokens);
				}
			},
			
			Fields::Unit => {},
		}
	})
}

pub fn derive_writes(attributes: &[Attribute], data: &Data) -> TokenStream2 {
	fn derive_for_fields(fields: &Fields) -> TokenStream2 {
		TokenStream2::with_tokens(|tokens| match &fields {
			Fields::Named(fields) => {
				for field in &fields.named {
					if !field.attrs.iter().any(|attr| attr.path.is_ident("hide")) {
						let ident = &field.ident;
						let r#type = &field.ty;

						tokens.append_tokens(|| {
							quote!(
								<#r#type as ::xrbk::Writable>::write_to(#ident, buf)?;
							)
						});
					}
				}
			},

			Fields::Unnamed(fields) => {
				for (i, field) in fields.unnamed.iter().enumerate() {
					if !field.attrs.iter().any(|attr| attr.path.is_ident("hide")) {
						let formatted = format_ident!("field{}", Index::from(i));
						let r#type = &field.ty;

						tokens.append_tokens(|| {
							quote!(
								<#r#type as ::xrbk::Writable>::write_to(#formatted, buf)?;
							)
						});
					}
				}
			},

			Fields::Unit => {},
		})
	}

	let no_discrim = {
		let mut no_discrim = false;

		for attribute in attributes {
			if attribute.path.is_ident("no_discrim") {
				no_discrim = true;
				break;
			}
		}

		no_discrim
	};

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

				if !no_discrim && let Some((_, expr)) = &variant.discriminant {
					discrim = quote!((#expr));
				}

				let pat = pat_cons(&variant.fields);
				let writes = derive_for_fields(&variant.fields);

				let write_discrim = if no_discrim {
					None
				} else {
					Some(quote!(
						buf.put_u8((#discrim) as u8);
					))
				};

				let arm = quote!(
					Self::#ident #pat => {
						#write_discrim

						#writes
					},
				);

				if !no_discrim {
					quote!(/* discrim */ + 1).to_tokens(&mut discrim);
				}

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

pub fn derive_reads(attributes: &[Attribute], data: &Data) -> TokenStream2 {
	for attribute in attributes {
		if attribute.path.is_ident("no_discrim") {
			panic!("found #[no_discrim]: cannot derive Readable without discriminants");
		}
	}

	fn derive_for_fields(fields: &Fields) -> TokenStream2 {
		TokenStream2::with_tokens(|tokens| match &fields {
			Fields::Named(fields) => {
				for field in &fields.named {
					if field.attrs.iter().any(|attr| attr.path.is_ident("hide"))
						&& !field.attrs.iter().any(|attr| attr.path.is_ident("context"))
					{
						panic!(
							"cannot derive Readable unless all fields with #[hide] have a \
							 #[context(...)] attribute"
						);
					}

					let ident = &field.ident;
					let r#type = &field.ty;

					tokens.append_tokens(|| {
						quote!(
							let #ident = <#r#type as ::xrbk::Readable>::read_from(buf)?;
						)
					});
				}
			},

			Fields::Unnamed(fields) => {
				for (i, field) in fields.unnamed.iter().enumerate() {
					if field.attrs.iter().any(|attr| attr.path.is_ident("hide"))
						&& !field.attrs.iter().any(|attr| attr.path.is_ident("context"))
					{
						panic!(
							"cannot derive Readable unless all fields with #[hide] have a \
							 #[context(...)] attribute"
						);
					}

					let formatted = format_ident!("field{}", Index::from(i));
					let r#type = &field.ty;

					tokens.append_tokens(|| {
						quote!(
							let #formatted = <#r#type as ::xrbk::Readable>::read_from(buf)?;
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
						::xrbk::ReadError::UnrecognizedDiscriminant(other_discrim),
					),
				}
			)
		},

		Data::Union(_) => unimplemented!(),
	}
}

pub fn derive_x11_sizes(attributes: &[Attribute], data: &Data) -> TokenStream2 {
	fn derive_for_fields(fields: &Fields) -> TokenStream2 {
		TokenStream2::with_tokens(|tokens| match &fields {
			Fields::Named(fields) => {
				for field in &fields.named {
					if !field.attrs.iter().any(|attr| attr.path.is_ident("hide")) {
						let ident = &field.ident;
						let r#type = &field.ty;

						tokens.append_tokens(|| {
							quote!(
								size += <#r#type as ::xrbk::X11Size>::x11_size(#ident);
							)
						});
					}
				}
			},

			Fields::Unnamed(fields) => {
				for (i, field) in fields.unnamed.iter().enumerate() {
					if !field.attrs.iter().any(|attr| attr.path.is_ident("hide")) {
						let formatted = format_ident!("field{}", i);
						let r#type = &field.ty;

						tokens.append_tokens(|| {
							quote!(
								size += <#r#type as ::xrbk::X11Size>::x11_size(#formatted);
							)
						});
					}
				}
			},

			Fields::Unit => {},
		})
	}

	let no_discrim = {
		let mut no_discrim = false;

		for attribute in attributes {
			if attribute.path.is_ident("no_discrim") {
				no_discrim = true;
				break;
			}
		}

		no_discrim
	};

	match data {
		Data::Struct(r#struct) => {
			let pat = pat_cons(&r#struct.fields);
			let sizes = derive_for_fields(&r#struct.fields);

			quote!(
				let Self #pat = &self;
				let mut size = 0;

				#sizes

				size
			)
		},

		Data::Enum(r#enum) => {
			let arms = r#enum.variants.iter().map(|variant| {
				let ident = &variant.ident;

				let pat = pat_cons(&variant.fields);
				let sizes = derive_for_fields(&variant.fields);

				let size = if no_discrim {
					quote! {
						let mut size = 0;
					}
				} else {
					quote! {
						let mut size = 1;
					}
				};

				quote!(
					Self::#ident #pat => {
						#size
						#sizes

						size
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

pub fn derive_constant_x11_sizes(_attributes: &[Attribute], data: &Data) -> TokenStream2 {
	fn derive_for_fields(fields: &Fields) -> TokenStream2 {
		TokenStream2::with_tokens(|tokens| match fields {
			Fields::Named(FieldsNamed { named: fields, .. })
			| Fields::Unnamed(FieldsUnnamed {
				unnamed: fields, ..
			}) => {
				for field in fields {
					if !field.attrs.iter().any(|attr| attr.path.is_ident("hide")) {
						let r#type = &field.ty;

						tokens.append_tokens(|| {
							quote!(
								size += <#r#type as ::xrbk::ConstantX11Size>::X11_SIZE;
							)
						});
					}
				}
			},

			Fields::Unit => {},
		})
	}

	match data {
		Data::Struct(r#struct) => {
			let sizes = derive_for_fields(&r#struct.fields);

			quote!(
				let mut size = 0;

				#sizes

				size
			)
		},

		// TODO: derive for enums if all variants are the same constant size
		Data::Enum(_) | Data::Union(_) => unimplemented!(),
	}
}
