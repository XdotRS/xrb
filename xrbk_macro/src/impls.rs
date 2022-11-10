// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::{ts_ext::TsExt, *};

impl Definitions {
	pub fn impl_tokens(&self, tokens: &mut TokenStream2) {
		let Self(definitions) = self;

		for definition in definitions {
			definition.serialize_tokens(tokens);
			definition.deserialize_tokens(tokens);
		}
	}
}

impl Field {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, i: usize) {
		let name = self.fmt_indexed_ident(i);
		quote!(#name.write_to(writer)?;).to_tokens(tokens);
	}
}

impl SerializeTokens for Let {
	fn serialize_tokens(&self, tokens: &mut TokenStream2) {
		let name = self.fmt_ident();
		let args = self.source.fmt_args();

		quote!(
			#name(
				#(#args,)*
			).write_to(writer)?;
		)
		.to_tokens(tokens);
	}
}

impl Unused {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, i: usize) {
		match self {
			Self::Unit(_) => {
				tokens.append_tokens(|| quote!(0u8.write_to(writer)?;));
			}

			Self::Array(array) => {
				let name = array.fmt_indexed_ident(i);
				let args = array.source.fmt_args();

				tokens.append_tokens(|| {
					quote!(
						writer.put_many(
							0u8,
							#name(
								#(#args,)*
							)
						);
					)
				});
			}
		}
	}
}

impl Item {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, i: usize) {
		match self {
			Item::Field(field) => field.serialize_tokens(tokens, i),

			Item::Let(r#let) => r#let.serialize_tokens(tokens),

			Item::Unused(unused) => unused.serialize_tokens(tokens, i),
		}
	}
}

impl SerializeTokens for Definition {
	fn serialize_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Enum(r#enum) => r#enum.serialize_tokens(tokens),
			Self::Struct(r#struct) => r#struct.serialize_tokens(tokens),
		}
	}
}

impl DeserializeTokens for Definition {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Enum(r#enum) => r#enum.deserialize_tokens(tokens),
			Self::Struct(r#struct) => r#struct.deserialize_tokens(tokens),
		}
	}
}

impl SerializeTokens for Enum {
	fn serialize_tokens(&self, tokens: &mut TokenStream2) {
		let name = &self.ident;

		let arms = TokenStream2::with_tokens(|tokens| {
			for variant in &self.variants {
				let name = &variant.ident;
				let pat = TokenStream2::with_tokens(|tokens| {
					variant.items.pattern_to_tokens(tokens);
				});

				let inner = TokenStream2::with_tokens(|tokens| {
					for (i, item) in variant.items.iter() {
						item.serialize_tokens(tokens, i);
					}
				});

				tokens.append_tokens(|| {
					quote!(
						Self::#name #pat => {
							#inner
						}
					)
				});
			}
		});

		tokens.append_tokens(|| {
			quote!(
				impl cornflakes::Writable for #name {
					fn write_to(
						&self,
						writer: &mut impl bytes::BufMut,
					) -> Result<(), Box<dyn std::error::Error>> {
						match self {
							#arms
						}
					}
				}
			)
		});
	}
}

impl SerializeTokens for Struct {
	fn serialize_tokens(&self, tokens: &mut TokenStream2) {
		let name = self.metadata.name();

		let pat = TokenStream2::with_tokens(|tokens| {
			self.items.pattern_to_tokens(tokens);
		});

		let inner = TokenStream2::with_tokens(|tokens| {
			for (i, item) in self.items.iter() {
				item.serialize_tokens(tokens, *i);
			}
		});

		tokens.append_tokens(|| {
			quote!(
				impl cornflakes::Writable for #name {
					fn write_to(
						&self,
						writer: &mut impl bytes::BufMut,
					) -> Result<(), Box<dyn std::error::Error>> {
						let Self #pat = self;

						#inner
					}
				}
			)
		});
	}
}

impl DeserializeTokens for Struct {
	fn deserialize_tokens(&self, _tokens: &mut TokenStream2) {
		// TODO
	}
}
