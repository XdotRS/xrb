// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
	parenthesized,
	parse::{ParseStream, Parser, Result},
	punctuated::{Pair, Punctuated},
	spanned::Spanned,
	token,
	Attribute,
	DataEnum,
	DeriveInput,
	Fields,
	Ident,
	ImplGenerics,
	Token,
	Type,
	TypeGenerics,
	Variant,
	WhereClause,
};

use crate::TsExt;

pub enum WrapperMeta {
	DiscriminantType {
		paren_token: token::Paren,
		r#type: Type,
	},

	Fallback {
		paren_token: token::Paren,
		fallback_token: Ident,
	},
}

impl WrapperMeta {
	fn parse_discriminant_type(input: ParseStream) -> Result<Self> {
		let content;

		Ok(Self::DiscriminantType {
			paren_token: parenthesized!(content in input),
			r#type: content.parse()?,
		})
	}

	fn parse_fallback(input: ParseStream) -> Result<Self> {
		let content;

		Ok(Self::Fallback {
			paren_token: parenthesized!(content in input),
			fallback_token: content.parse()?,
		})
	}
}

pub fn derive(tokens: &mut TokenStream2, attrs: &[Attribute], item: &DeriveInput, data: &DataEnum) {
	let WrapperMeta::DiscriminantType {
		r#type: wrapped_type,
		..
	} = attrs
		.iter()
		.find(|attr| attr.path.is_ident("wrapper"))
		.map_or_else(
			|| panic!("missing #[wrapper(/* i* or u* */)] wrapper type attribute"),
			|Attribute { tokens, .. }| {
				WrapperMeta::parse_discriminant_type
					.parse2(tokens.clone())
					.unwrap()
			},
		) else { unreachable!() };

	let generics = item.generics.split_for_impl();

	derive_size(tokens, &item.ident, &wrapped_type, &generics);
	derive_writable(tokens, &item.ident, &wrapped_type, &generics, data);
	derive_readable(tokens, &item.ident, &wrapped_type, &generics, data);
}

fn derive_size(
	tokens: &mut TokenStream2, ident: &Ident, wrapped_type: &Type,
	(impl_generics, type_generics, where_clause): &(
		ImplGenerics,
		TypeGenerics,
		Option<&WhereClause>,
	),
) {
	let wrapped_type = quote_spanned!(wrapped_type.span()=>
		<#wrapped_type as ::xrbk::ConstantX11Size>
	);

	tokens.append_tokens(quote!(
		#[automatically_derived]
		impl #impl_generics ::xrbk::ConstantX11Size for #ident #type_generics #where_clause {
			const X11_SIZE: usize = #wrapped_type::X11_SIZE;
		}

		#[automatically_derived]
		impl #impl_generics ::xrbk::X11Size for #ident #type_generics #where_clause {
			fn x11_size(&self) -> usize {
				<Self as ::xrbk::ConstantX11Size>::X11_SIZE
			}
		}
	))
}

fn derive_writable(
	tokens: &mut TokenStream2, ident: &Ident, wrapped_type: &Type,
	(impl_generics, type_generics, where_clause): &(
		ImplGenerics,
		TypeGenerics,
		Option<&WhereClause>,
	),
	data: &DataEnum,
) {
	let wrapped_writable = quote_spanned!(wrapped_type.span()=>
		<#wrapped_type as ::xrbk::Writable>
	);

	let discrim_functions =
		TokenStream2::with_tokens(|tokens| discrim_functions(tokens, &data.variants));

	let mut fallback = None;
	let match_arms = TokenStream2::with_tokens(|tokens| {
		fallback = Some(remove_fallback(&data.variants, |(variant, _), discrim| {
			let variant_ident = &variant.ident;

			tokens.append_tokens(quote!(
				Self::#variant_ident => {
					#wrapped_writable::write_to(&#wrapped_type::from(#discrim), buf)?;
				},
			))
		}));
	});
	let fallback = fallback.unwrap();

	tokens.append_tokens(quote!(
		#[automatically_derived]
		impl #impl_generics ::xrbk::Writable for #ident #type_generics #where_clause {
			fn write_to(&self, buf: &mut impl ::xrbk::BufMut) -> Result<(), ::xrbk::WriteError> {
				#discrim_functions

				match self {
					#match_arms

					Self::#fallback(val) => {
						#wrapped_writable::write_to(&#wrapped_type::from(val.clone()), buf)?;
					},
				}

				Ok(())
			}
		}
	))
}

fn derive_readable(
	tokens: &mut TokenStream2, ident: &Ident, wrapped_type: &Type,
	(impl_generics, type_generics, where_clause): &(
		ImplGenerics,
		TypeGenerics,
		Option<&WhereClause>,
	),
	data: &DataEnum,
) {
	let wrapped_type = quote_spanned!(wrapped_type.span()=>
		<#wrapped_type as ::xrbk::Readable>
	);

	let discrim_functions =
		TokenStream2::with_tokens(|tokens| discrim_functions(tokens, &data.variants));

	let mut fallback = None;
	let match_arms = TokenStream2::with_tokens(|tokens| {
		fallback = Some(remove_fallback(&data.variants, |(variant, _), discrim| {
			let variant_ident = &variant.ident;

			match variant.fields {
				Fields::Unit => {},

				Fields::Named(_) | Fields::Unnamed(_) => {
					panic!("only the #[wrapper(fallback)] variant may have fields");
				},
			}

			tokens.append_tokens(quote!(
				discrim if isize::from(discrim) == #discrim => Self::#variant_ident,
			));
		}));
	});
	let fallback = fallback.unwrap();

	tokens.append_tokens(quote!(
		#[automatically_derived]
		impl #impl_generics ::xrbk::Readable for #ident #type_generics #where_clause {
			fn read_from(buf: &mut impl ::xrbk::Buf) -> Result<Self, ::xrbk::ReadError> {
				#discrim_functions

				Ok(match #wrapped_type::read_from(buf)? {
					#match_arms

					other_discrim => Self::#fallback(other_discrim.into()),
				})
			}
		}
	))
}

fn discrim_functions(tokens: &mut TokenStream2, variants: &Punctuated<Variant, Token![,]>) {
	for variant in variants {
		if let Some((_, expr)) = &variant.discriminant {
			let ident = format_ident!("__DISCRIM_{}", variant.ident);
			let fn_ident = format_ident!("__DISCRIM_{}_FN", variant.ident);

			tokens.append_tokens(quote!(
				#[allow(non_snake_case)]
				const fn #fn_ident() -> isize {
					#expr
				}

				#[allow(non_upper_case_globals)]
				const #ident: isize = #fn_ident();
			));
		}
	}
}

fn with_discrim<F>(variants: &Punctuated<Variant, Token![,]>, mut expand_function: F)
where
	F: FnMut((&Variant, Option<&Token![,]>), &TokenStream2),
{
	let mut discrim = quote!(0);

	for pair in variants.pairs() {
		let (variant, comma) = match pair {
			Pair::Punctuated(variant, comma) => (variant, Some(comma)),
			Pair::End(variant) => (variant, None),
		};

		if variant.discriminant.is_some() {
			let ident = format_ident!("__DISCRIM_{}", variant.ident);

			discrim = ident.into_token_stream();
		}

		expand_function((variant, comma), &discrim);

		discrim.append_tokens(quote!(+ 1));
	}
}

fn remove_fallback<F>(variants: &Punctuated<Variant, Token![,]>, mut expand_function: F) -> Ident
where
	F: FnMut((&Variant, Option<&Token![,]>), &TokenStream2),
{
	let mut fallback = None;

	with_discrim(variants, |(variant, comma), discrim| {
		for attribute in &variant.attrs {
			if attribute.path.is_ident("wrapper") {
				let _ = WrapperMeta::parse_fallback.parse2(attribute.tokens.clone());

				if fallback.is_some() {
					panic!("can only have one #[wrapper(fallback)] variant");
				}

				fallback = Some(variant.ident.clone());

				return;
			}
		}

		expand_function((variant, comma), discrim);
	});

	fallback.unwrap()
}
