// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

pub use attributes::*;
pub use field::*;
pub use items::*;
pub use r#let::*;
pub use source::*;
pub use unused::*;

use crate::{ItemDeserializeTokens, ItemSerializeTokens, TsExt};

mod attributes;
mod items;
mod source;

pub enum Item {
	Field(Box<Field>),
	Let(Box<Let>),
	Unused(Unused),
}

impl Item {
	pub fn is_metabyte(&self) -> bool {
		match self {
			Self::Field(field) => field.attributes.iter().any(|attribute| {
				matches!(
					attribute,
					Attribute {
						content: AttrContent::Metabyte(..),
						..
					}
				)
			}),

			Self::Let(r#let) => r#let.attributes.iter().any(|attribute| {
				matches!(
					attribute,
					Attribute {
						content: AttrContent::Metabyte(..),
						..
					}
				)
			}),

			Self::Unused(Unused::Array(array)) => array.attributes.iter().any(|attribute| {
				matches!(
					attribute,
					Attribute {
						content: AttrContent::Metabyte(..),
						..
					}
				)
			}),

			Self::Unused(Unused::Single { attribute, .. }) => matches!(
				attribute,
				Some(Attribute {
					content: AttrContent::Metabyte(..),
					..
				})
			),
		}
	}

	pub fn datasize_tokens(
		&self,
		tokens: &mut TokenStream2,
		id: &ItemId,
		min_length: Option<usize>,
	) {
		match self {
			Self::Unused(Unused::Single { .. }) => {
				if !self.is_metabyte() {
					tokens.append_tokens(|| {
						quote!(
							datasize += 1;
						)
					});
				}
			}

			Self::Unused(Unused::Array(array)) => {
				let ident = id.formatted();

				tokens.append_tokens(|| {
					match &array.content {
						ArrayContent::Source(source) => {
							let args = TokenStream2::with_tokens(|tokens| {
								source.args_to_tokens(tokens);
							});
							let formatted_args = TokenStream2::with_tokens(|tokens| {
								source.formatted_args_to_tokens(tokens);
							});

							let expr = &source.expr;

							quote!(
								fn #ident(#args) -> usize {
									#expr
								}

								datasize += #ident(#formatted_args);
							)
						}

						ArrayContent::Infer { last_item, .. } => {
							if *last_item && let Some(min_length) = min_length {
								quote!(
									datasize += if datasize < #min_length {
										#min_length - datasize
									} else {
										(4 - (datasize % 4)) % 4
									};
								)
							} else {
								quote!(
									datasize += (4 - (datasize % 4)) % 4;
								)
							}
						}
					}
				});
			}

			Self::Let(r#let) => {
				let ident = id.formatted();
				let r#type = &r#let.r#type;

				tokens.append_tokens(|| {
					quote!(
						datasize += <#r#type as cornflakes::DataSize>::data_size(&#ident);
					)
				});
			}

			Self::Field(field) => {
				let ident = id.formatted();
				let r#type = &field.r#type;

				tokens.append_tokens(|| {
					quote!(
						datasize += <#r#type as cornflakes::DataSize>::data_size(&#ident);
					)
				});
			}
		}
	}
}

// Expansion {{{

impl ToTokens for Item {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// If `self` is a `Field`, convert it to tokens, otherwise don't - the
		// other items are used for generating the serialization and
		// deserialization code.
		if let Self::Field(field) = self {
			field.to_tokens(tokens);
		}
	}
}

// }}}

// Implementations {{{

impl Item {
	pub(crate) fn serialize_tokens(
		&self,
		tokens: &mut TokenStream2,
		id: &ItemId,
		min_length: Option<usize>,
	) {
		match self {
			Item::Field(field) => field.serialize_tokens(tokens, id),

			Item::Let(r#let) => r#let.serialize_tokens(tokens, id),

			Item::Unused(unused) => unused.serialize_tokens(tokens, id, min_length),
		}
	}
}

impl Item {
	pub(crate) fn deserialize_tokens(
		&self,
		tokens: &mut TokenStream2,
		id: &ItemId,
		min_length: Option<usize>,
	) {
		match self {
			Item::Field(field) => field.deserialize_tokens(tokens, id),

			Item::Let(r#let) => r#let.deserialize_tokens(tokens, id),

			Item::Unused(unused) => unused.deserialize_tokens(tokens, id, min_length),
		}
	}
}

// }}}
