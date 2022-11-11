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

impl SerializeTokens for Field {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		let name = id.formatted().expect("field identifier must exist");
		tokens.append_tokens(|| quote!(#name.write_to(writer)?;));
	}
}

impl SerializeTokens for Let {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		let name = id.formatted().expect("let item identifier must exist");
		let args = self.source.fmt_args();

		quote!(
			#name(
				#(#args,)*
			).write_to(writer)?;
		)
		.to_tokens(tokens);
	}
}

impl SerializeTokens for Unused {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		match self {
			Self::Unit(_) => {
				tokens.append_tokens(|| quote!(0u8.write_to(writer)?;));
			}

			Self::Array(array) => {
				let name = id
					.formatted()
					.expect("array-type unused bytes item must have identifier");
				let args = array.source.fmt_args();

				tokens.append_tokens(|| {
					quote!(
						writer.put_many(
							0u8,
							#name( #(#args,)* )
						);
					)
				});
			}
		}
	}
}

impl SerializeTokens for Item {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		match self {
			Item::Field(field) => field.serialize_tokens(tokens, id),

			Item::Let(r#let) => r#let.serialize_tokens(tokens, id),

			Item::Unused(unused) => unused.serialize_tokens(tokens, id),
		}
	}
}

impl Definition {
	pub fn serialize_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Enum(r#enum) => r#enum.serialize_tokens(tokens),
			Self::Struct(r#struct) => r#struct.serialize_tokens(tokens),
		}
	}

	pub fn deserialize_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Enum(r#enum) => r#enum.deserialize_tokens(tokens),
			Self::Struct(r#struct) => r#struct.deserialize_tokens(tokens),
		}
	}
}

impl Enum {
	pub fn serialize_tokens(&self, tokens: &mut TokenStream2) {
		let name = &self.ident;

		let arms = TokenStream2::with_tokens(|tokens| {
			for variant in &self.variants {
				let name = &variant.ident;
				let pat = TokenStream2::with_tokens(|tokens| {
					variant.items.pattern_to_tokens(tokens);
				});

				let inner = TokenStream2::with_tokens(|tokens| {
					for (id, item) in variant.items.pairs() {
						item.serialize_tokens(tokens, id);
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

	pub fn deserialize_tokens(&self, _tokens: &mut TokenStream2) {}
}

impl Struct {
	pub fn serialize_tokens(&self, tokens: &mut TokenStream2) {
		let name = self.metadata.name();

		let pat = TokenStream2::with_tokens(|tokens| {
			self.items.pattern_to_tokens(tokens);
		});

		let inner = TokenStream2::with_tokens(|tokens| {
			for (id, item) in self.items.pairs() {
				item.serialize_tokens(tokens, id);
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

	pub fn deserialize_tokens(&self, _tokens: &mut TokenStream2) {
		// TODO
	}
}
