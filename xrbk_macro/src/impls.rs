// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Type, Ident};

use crate::{*, ts_ext::TsExt};

impl Definitions {
    pub fn impl_tokens(&self, tokens: &mut TokenStream2) {
        let Self(definitions) = self;

        for definition in definitions {
            definition.serialize_tokens(tokens);
            definition.deserialize_tokens(tokens);
        }
    }
}

impl Definition {
    pub fn serialize_tokens(&self, tokens: &mut TokenStream2) {
        let mut inner = TokenStream2::new();

        fn format_ident(i: usize) -> Ident {
            format_ident!("item{}", i)
        }

        fn functions(tokens: &mut TokenStream2, items: impl Iterator<Item = &Item>) {
            items.enumerate().filter_map(|(i, item)| {
                match &item {
                    // If this is an Item::Let, generate a function with the
                    // let item's name and source.
                    Item::Let(r#let) => {
                        Some((r#let.ident.clone(), r#let.r#type.clone(), &r#let.source))
                    }

                    // If this is an Item::Field, and it has a context
                    // attribute, generate a function with the name of the
                    // field if any (otherwise item#), and the context's source.
                    Item::Field(field) => {
                        if let Some(context) = field.context() {
                            if let Some(ident) = &field.ident {
                                Some((ident.clone(), field.r#type.clone(), context.source()))
                            } else {
                                Some((format_ident(i), field.r#type.clone(), context.source()))
                            }
                        } else {
                            None
                        }
                    }

                    // If this is an Item::Unused...
                    Item::Unused(unused) => {
                        match unused {
                            Unused::Unit(_) => None,

                            // If it is Unused::Array, generate a function
                            // with the name `item#` and the unused array's
                            // source.
                            Unused::Array(array) => {
                                Some(
                                    (
                                        format_ident(i),
                                        Type::Verbatim(quote!(usize)),
                                        &array.source
                                    )
                                )
                            }
                        }
                    }
                }
            }).for_each(|(ident, r#type, source)| {
                // For each of the pairs of identifiers and sources, generate
                // the function tokens.
                source.fn_to_tokens(tokens, &ident, &r#type);
            })
        }

        fn field_ident(i: usize, ident: &Option<Ident>) -> Ident {
            if let Some(ident) = ident {
                ident.clone()
            } else {
                format_ident(i)
            }
        }

        let mut content = TokenStream2::new();

        match self {
            Self::Enum(r#enum) => {
                for variant in &r#enum.variants {
                    let name = &variant.name;
                    let pat = TokenStream2::with_tokens(|tokens| {
                        variant.items.pattern_to_tokens(tokens)
                    });

                    let mut writes = TokenStream2::new();

                    for (i, item) in variant.items.iter().enumerate() {
                        match item {
                            Item::Let(r#let) => {
                                let name = &r#let.ident;
                                let arg = r#let.source.format_args();

                                quote!(#name(#(#arg,)*).write_to(writer)?;).to_tokens(&mut writes);
                            }

                            Item::Field(field) => {
                                let name = field_ident(i, &field.ident);

                                quote!(#name.write_to(writer)?;).to_tokens(&mut writes);
                            }

                            Item::Unused(unused) => {
                                if let Unused::Array(array) = unused {
                                    let name = format_ident(i);
                                    let arg = array.source.format_args();

                                    quote!(writer.put_many(0u8, #name(#(#arg,)*));)
                                        .to_tokens(&mut writes);
                                } else {
                                    quote!(0u8.write_to(writer)?;).to_tokens(&mut writes);
                                }
                            }
                        }
                    }

                    quote!(Self::#name #pat => {
                        #writes
                    }).to_tokens(&mut content);
                }
            }

            Self::Struct(r#struct) => {
                let writes: Vec<TokenStream2> = match &r#struct.items {
                    Items::Named(_, items) => {
                        items.iter().filter_map(|item| if let Item::Field(field) = item {
                            Some(field)
                        } else {
                            None
                        }).map(|field| {
                            let ident = &field.ident;
                            quote!(self.#ident.write_to(writer)?;)
                        }).collect()
                    }

                    Items::Unnamed(_, items) => {
                        (0..items.len()).map(|i| quote!(self.#i.write_to(writer)?;)).collect()
                    }

                    Items::Unit => vec![],
                };

                quote!(#(#writes)*).to_tokens(&mut inner);
            },
        }

        let name = self.name();

        quote!(
            impl cornflakes::Writable for #name {
                fn write_to(
                    &self,
                    writer: &mut impl bytes::BufMut,
                ) -> Result<(), Box<dyn std::error::Error>> {
                    #inner
                }
            }
        ).to_tokens(tokens);
    }

    pub fn deserialize_tokens(&self, tokens: &mut TokenStream2) {
        let name = self.name();

        quote!(
        	impl cornflakes::Readable for #name {
            	fn read_from(
                    reader: &mut impl bytes::Buf,
                ) -> Result<Self, Box<dyn std::error::Error>> {
                	// ...
            	}
        	}
    	).to_tokens(tokens);
    }
}