// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

// This crate is based on `syn`'s `heapsize` example:
// https://github.com/dtolnay/syn/tree/master/examples/heapsize

use proc_macro2::{self, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput, Fields, GenericParam,
    Generics, Index,
};

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let generics = add_serialize_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let write = write(&input.data);

    let expanded = quote! {
        impl #impl_generics crate::serialization::Serialize for #name #ty_generics #where_clause {
            fn write(self, buf: &mut impl bytes::BufMut) {
                #write
            }
        }
    };

    expanded.into()
}

fn add_serialize_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(crate::serialization::Serialize));
        }
    }

    generics
}

fn write(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|field| {
                    let name = &field.ident;

                    quote_spanned! { field.span()=>
                        self.#name.write(buf);
                    }
                });

                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(index, field)| {
                    let index = Index::from(index);

                    quote_spanned! { field.span()=>
                        self.#index.write(buf);
                    }
                });

                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => {
                quote! {}
            }
        },
        Data::Enum(ref data) => {
            let recurse =
                data.variants.iter().enumerate().map(|(index, variant)| {
                    let index = if variant.discriminant.as_ref().is_some() {
                        let discriminant = &variant.discriminant.as_ref().unwrap().1;

                        quote! {
                            (#discriminant as u8)
                        }
                    } else {
                        quote! {
                            (#index as u8)
                        }
                    };

                    let name = &variant.ident;

                    let fields =
                        match variant.fields {
                            Fields::Named(ref fields) => {
                                let recurse_fields = fields.named.iter().map(|field| {
                                    let field_name = &field.ident;

                                    quote_spanned! { field.span()=>
                                        self.#name.#field_name.write(buf);
                                    }
                                });

                                quote! {
                                    #(#recurse_fields)*
                                }
                            }
                            Fields::Unnamed(ref fields) => {
                                let recurse_fields = fields.unnamed.iter().enumerate().map(
                                    |(field_index, field)| {
                                        let field_index = Index::from(field_index);

                                        quote_spanned! { field.span()=>
                                            self.#name.#field_index.write(buf);
                                        }
                                    },
                                );

                                quote! {
                                    #(#recurse_fields)*
                                }
                            }
                            Fields::Unit => {
                                quote! {}
                            }
                        };

                    quote! {
                        Self::#name => {
                            #index.write(buf);
                            #fields
                        }
                    }
                });

            quote! {
                match self {
                    #(#recurse)*
                }
            }
        }
        _ => {
            quote! {}
        }
    }
}

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let generics = add_deserialize_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let read = read(&input.data);

    let expanded = quote! {
        impl #impl_generics crate::serialization::Deserialize for #name #ty_generics #where_clause {
            fn read(buf: &mut impl bytes::Buf) -> Self {
                #read
            }
        }
    };

    expanded.into()
}

fn add_deserialize_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(crate::serialization::Deserialize));
        }
    }

    generics
}

fn read(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|field| {
                    let name = &field.ident;
                    let ty = &field.ty;

                    quote_spanned! { field.span()=>
                        #name: #ty::read(buf),
                    }
                });

                quote! {
                    Self {
                        #(#recurse)*
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().map(|field| {
                    let ty = &field.ty;

                    quote_spanned! { field.span()=>
                        #ty::read(buf),
                    }
                });

                quote! {
                    Self {
                        #(#recurse)*
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    Self {}
                }
            }
        },
        Data::Enum(ref data) => {
            let recurse = data.variants.iter().enumerate().map(|(index, variant)| {
                let index = index as u8;

                let index = if variant.discriminant.as_ref().is_some() {
                    let discriminant = &variant.discriminant.as_ref().unwrap().1;

                    quote! {
                        (#discriminant as u8)
                    }
                } else {
                    quote! {
                        #index
                    }
                };

                let name = &variant.ident;

                let fields = match variant.fields {
                    Fields::Named(ref fields) => {
                        let recurse_fields = fields.named.iter().map(|field| {
                            let field_name = &field.ident;
                            let field_ty = &field.ty;

                            quote_spanned! { field.span()=>
                                #field_name: #field_ty::read(buf),
                            }
                        });

                        quote! {
                            {#(#recurse_fields)*}
                        }
                    }
                    Fields::Unnamed(ref fields) => {
                        let recurse_fields = fields.unnamed.iter().map(|field| {
                            let field_ty = &field.ty;

                            quote_spanned! { field.span()=>
                                #field_ty::read(buf),
                            }
                        });

                        quote! {
                            {#(#recurse_fields)*}
                        }
                    }
                    Fields::Unit => {
                        quote! {}
                    }
                };

                quote! {
                    #index => Self::#name #fields,
                }
            });

            quote! {
                match u8::read(buf) {
                    #(#recurse)*
                    _ => panic!("tried to read variant but an invalid discriminant was found"),
                }
            }
        }
        _ => {
            quote! {
                Self {}
            }
        }
    }
}
