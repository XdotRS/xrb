// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_quote, Generics, TypeParamBound};

use crate::{ts_ext::TsExt, *};

pub trait ItemSerializeTokens {
	/// Generates the tokens to serialize a given item.
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId);
}

pub trait ItemDeserializeTokens {
	/// Generates the tokens to deserialize a given item.
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId);
}

pub trait SerializeMessageTokens {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, items: &Items);
}

pub trait DeserializeMessageTokens {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, items: &Items);
}

impl Definitions {
	/// Expands the trait implementations for the given definition.
	pub fn impl_tokens(&self, tokens: &mut TokenStream2) {
		let Self(definitions) = self;

		for definition in definitions {
			match definition {
				Definition::Enum(r#enum) => {
					r#enum.serialize_tokens(tokens);
					r#enum.deserialize_tokens(tokens);
				}

				Definition::Struct(r#struct) => {
					r#struct.serialize_tokens(tokens);
					r#struct.deserialize_tokens(tokens);

					match &r#struct.metadata {
						StructMetadata::Request(request) => {
							request.impl_request_tokens(tokens);
						}

						StructMetadata::Reply(reply) => {
							reply.impl_reply_tokens(tokens);
						}

						StructMetadata::Event(event) => {
							event.impl_event_tokens(tokens);
						}

						_ => {}
					}
				}
			}
		}
	}
}

impl ItemSerializeTokens for Field {
	// Tokens to serialize a field.
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		let name = id.formatted();
		let r#type = &self.r#type;

		tokens
			.append_tokens(|| quote!(<#r#type as cornflakes::Writable>::write_to(#name, writer)?;));
	}
}

impl ItemDeserializeTokens for Field {
	// Tokens to deserialize a field.
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		let name = id.formatted();
		let r#type = &self.r#type;

		tokens.append_tokens(|| {
			// If this is a contextual field, that context must be provided.
			if let Some(context) = self.context() {
				let args = TokenStream2::with_tokens(|tokens| {
					context.source().args_to_tokens(tokens);
				});
				let formatted_args = TokenStream2::with_tokens(|tokens| {
					context.source().formatted_args_to_tokens(tokens);
				});

				let expr = &context.source().expr;

				quote!(
					fn #name(#args) -> <#r#type as cornflakes::ContextualReadable>::Context {
						#expr
					}

					let #name = <#r#type as cornflakes::ContextualReadable>::read_with(
						reader,
						&#name(#formatted_args),
					)?;
				)
			} else {
				quote!(
					// let __my_field2__ = u8::read_from(reader)?;
					let #name = <#r#type as cornflakes::Readable>::read_from(reader)?;
				)
			}
		});
	}
}

impl ItemSerializeTokens for Let {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		let name = id.formatted();
		let r#type = &self.r#type;

		let args = TokenStream2::with_tokens(|tokens| {
			self.source.args_to_tokens(tokens);
		});
		let formatted_args = TokenStream2::with_tokens(|tokens| {
			self.source.formatted_args_to_tokens(tokens);
		});

		let expr = &self.source.expr;

		quote!(
			fn #name(#args) -> #r#type {
				#expr
			}

			<#r#type as cornflakes::Writable>::write_to(
				#name(#formatted_args),
				writer,
			)?;
		)
		.to_tokens(tokens);
	}
}

impl ItemDeserializeTokens for Let {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		let name = id.formatted();
		let r#type = &self.r#type;

		tokens.append_tokens(
			|| quote!(let #name = <#r#type as cornflakes::Readable>::read_from(reader)?;),
		);
	}
}

impl ItemSerializeTokens for Unused {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		tokens.append_tokens(|| {
			match self {
				Self::Unit { .. } => {
					// 0u8.write_to(writer)?;
					quote!(
						writer.put_u8(0);
					)
				}

				Self::Array(array) => {
					let name = id.formatted();

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
								fn #name(#args) -> usize {
									#expr
								}

								writer.put_many(
									0u8,
									#name(#formatted_args),
								);
							)
						}

						ArrayContent::Infer(_) => {
							quote!(
								writer.put_many(
									0u8,
									// TODO: use padding function
								);
							)
						}
					}
				}
			}
		});
	}
}

impl ItemDeserializeTokens for Unused {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		tokens.append_tokens(|| {
			match self {
				Self::Array(array) => {
					let name = id.formatted();

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
								fn #name(#args) -> usize {
									#expr
								}

								reader.advance(#name(#formatted_args));
							)
						}

						ArrayContent::Infer(_) => {
							quote!(
								reader.advance(
									// TODO: use padding function
								);
							)
						}
					}
				}

				Self::Unit { .. } => {
					// reader.advance(1);
					quote!(reader.advance(1);)
				}
			}
		});
	}
}

impl ItemSerializeTokens for Item {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		match self {
			Item::Field(field) => field.serialize_tokens(tokens, id),

			Item::Let(r#let) => r#let.serialize_tokens(tokens, id),

			Item::Unused(unused) => unused.serialize_tokens(tokens, id),
		}
	}
}

impl ItemDeserializeTokens for Item {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		match self {
			Item::Field(field) => field.deserialize_tokens(tokens, id),

			Item::Let(r#let) => r#let.deserialize_tokens(tokens, id),

			Item::Unused(unused) => unused.deserialize_tokens(tokens, id),
		}
	}
}

fn add_bounds(generics: &mut Generics, bound: TypeParamBound) {
	for type_param in generics.type_params_mut() {
		type_param.bounds.push(bound.to_owned());
	}
}

impl Enum {
	fn serialize_tokens(&self, tokens: &mut TokenStream2) {
		let name = &self.ident;

		let generics = {
			let mut generics = self.generics.to_owned();
			add_bounds(&mut generics, parse_quote!(cornflakes::Writable));

			generics
		};
		let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

		// For every variant discriminant expression, create a function to
		// isolate the expression, then store it in a variable for later use.
		let discriminants = TokenStream2::with_tokens(|tokens| {
			for variant in &self.variants {
				if let Some((_, expr)) = &variant.discriminant {
					let name = format_ident!("_{}_discrim_", variant.ident);

					tokens.append_tokens(|| {
						quote!(
							fn #name() -> usize {
								#expr
							}

							let #name = #name();
						)
					});
				}
			}
		});

		let arms = TokenStream2::with_tokens(|tokens| {
			// Start the variants' discriminant tokens at `0`. We add `1` each
			// iteration, unless a variant explicitly specifies its
			// discriminant.
			let mut discrim = quote!(0);

			for variant in &self.variants {
				let name = &variant.ident;

				// If the variant has a discriminant, use that discriminant
				// evaluated earlier.
				if variant.discriminant.is_some() {
					let name = format_ident!("_{}_discrim_", variant.ident);
					discrim = quote!(#name);
				}

				// Tokens to destructure the variant's fields.
				let pat = TokenStream2::with_tokens(|tokens| {
					variant.items.fields_to_tokens(tokens, ExpandMode::Normal);
				});

				// Generate the tokens to serialize each of the variant's items.
				let inner = TokenStream2::with_tokens(|tokens| {
					for (id, item) in variant.items.pairs() {
						item.serialize_tokens(tokens, id);
					}
				});

				// Append the variant's match arm.
				tokens.append_tokens(|| {
					quote!(
						Self::#name #pat => {
							// Write the variant's discriminant (as a single byte).
							writer.put_u8((#discrim) as u8);

							#inner
						}
					)
				});

				// Add `1` to the discriminant tokens so that the next variant
				// starts with a discriminant one more than the current
				// variant's discriminant (unless that variant's discriminant
				// is specified explicitly).
				discrim.append_tokens(|| quote!(/* discrim */ + 1));
			}
		});

		tokens.append_tokens(|| {
			quote!(
				impl #impl_generics cornflakes::Writable for #name #type_generics #where_clause{
					fn write_to(
						&self,
						writer: &mut impl bytes::BufMut,
					) -> Result<(), cornflakes::WriteError> {
						#discriminants

						match self {
							#arms
						}

						Ok(())
					}
				}
			)
		});
	}
}

impl Enum {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2) {
		let name = &self.ident;

		let generics = {
			let mut generics = self.generics.to_owned();
			add_bounds(&mut generics, parse_quote!(cornflakes::Readable));

			generics
		};
		let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

		// For every variant discriminant expression, create a function to
		// isolate the expression, then store it in a variable for later use.
		let discriminants = TokenStream2::with_tokens(|tokens| {
			for variant in &self.variants {
				if let Some((_, expr)) = &variant.discriminant {
					let name = format_ident!("_{}_discrim_", variant.ident);

					tokens.append_tokens(|| {
						quote!(
							fn #name() -> usize {
								#expr
							}

							let #name = #name();
						)
					});
				}
			}
		});

		let arms = TokenStream2::with_tokens(|tokens| {
			// Start the variants' discriminant tokens at `0`. We add `1` each
			// iteration, unless a variant explicitly specifies its
			// discriminant.
			let mut discrim = quote!(0);

			for variant in &self.variants {
				let name = &variant.ident;

				// If the variant has a discriminant, use that discriminant
				// evaluated earlier.
				if variant.discriminant.is_some() {
					let name = format_ident!("_{}_discrim_", variant.ident);
					discrim = quote!(#name);
				}

				// Tokens to fill in the fields for the variant's constructor.
				let cons = TokenStream2::with_tokens(|tokens| {
					variant.items.fields_to_tokens(tokens, ExpandMode::Normal);
				});

				// Generate the tokens to deserialize each of the variant's items.
				let inner = TokenStream2::with_tokens(|tokens| {
					for (id, item) in variant.items.pairs() {
						item.deserialize_tokens(tokens, id);
					}
				});

				// Append the variant's match arm.
				tokens.append_tokens(|| {
					quote!(
						// Match against the discriminant...
						discrim if discrim == (#discrim) as u8 => {
							// Deserialize the items.
							#inner

							// Construct the variant.
							Self::#name #cons
						}
					)
				});

				// Add `1` for the next variant's discriminant.
				discrim.append_tokens(|| quote!(/* discrim */ + 1));
			}
		});

		tokens.append_tokens(|| {
			quote!(
				impl #impl_generics cornflakes::Readable for #name #type_generics #where_clause {
					fn read_from(
						reader: &mut impl bytes::Buf,
					) -> Result<Self, cornflakes::ReadError> {
						#discriminants

						// Match against the discriminant...
						Ok(match reader.read::<u8>()? {
							#arms

							other_discrim => return Err(
								cornflakes::ReadError::UnrecognizedDiscriminant(other_discrim)
							),
						})
					}
				}
			)
		});
	}
}

impl Struct {
	fn serialize_tokens(&self, tokens: &mut TokenStream2) {
		match &self.metadata {
			StructMetadata::Struct(r#struct) => r#struct.serialize_tokens(tokens, &self.items),

			StructMetadata::Request(request) => request.serialize_tokens(tokens, &self.items),
			StructMetadata::Reply(reply) => reply.serialize_tokens(tokens, &self.items),

			StructMetadata::Event(event) => event.serialize_tokens(tokens, &self.items),
		}
	}
}

impl Struct {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2) {
		match &self.metadata {
			StructMetadata::Struct(r#struct) => r#struct.deserialize_tokens(tokens, &self.items),

			StructMetadata::Request(request) => request.deserialize_tokens(tokens, &self.items),
			StructMetadata::Reply(reply) => reply.deserialize_tokens(tokens, &self.items),

			StructMetadata::Event(event) => event.deserialize_tokens(tokens, &self.items),
		}
	}
}

impl SerializeMessageTokens for BasicStructMetadata {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, items: &Items) {
		let name = &self.name;

		let generics = {
			let mut generics = self.generics.to_owned();
			add_bounds(&mut generics, parse_quote!(cornflakes::Writable));

			generics
		};
		let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

		// Tokens to destructure the struct's fields.
		let pat = TokenStream2::with_tokens(|tokens| {
			items.fields_to_tokens(tokens, ExpandMode::Normal);
		});

		// Tokens to serialize each of the struct's items.
		let inner = TokenStream2::with_tokens(|tokens| {
			for (id, item) in items.pairs() {
				item.serialize_tokens(tokens, id);
			}
		});

		tokens.append_tokens(|| {
			quote!(
				impl #impl_generics cornflakes::Writable for #name #type_generics #where_clause {
					fn write_to(
						&self,
						writer: &mut impl bytes::BufMut,
					) -> Result<(), cornflakes::WriteError> {
						// Destructure the struct.
						let Self #pat = self;

						#inner

						Ok(())
					}
				}
			)
		});
	}
}

impl DeserializeMessageTokens for BasicStructMetadata {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, items: &Items) {
		let name = &self.name;

		let generics = {
			let mut generics = self.generics.to_owned();
			add_bounds(&mut generics, parse_quote!(cornflakes::Readable));

			generics
		};
		let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

		// Tokens to fill in the fields for the struct's constructor.
		let cons = TokenStream2::with_tokens(|tokens| {
			items.fields_to_tokens(tokens, ExpandMode::Normal);
		});

		// Generate the tokens to deserialize each of the struct's items.
		let inner = TokenStream2::with_tokens(|tokens| {
			for (id, item) in items.pairs() {
				item.deserialize_tokens(tokens, id);
			}
		});

		tokens.append_tokens(|| {
			quote!(
				impl #impl_generics cornflakes::Readable for #name #type_generics #where_clause {
					fn read_from(
						reader: &mut impl bytes::Buf,
					) -> Result<Self, cornflakes::ReadError> {
						#inner

						Ok(Self #cons)
					}
				}
			)
		});
	}
}

impl SerializeMessageTokens for Request {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, items: &Items) {
		// Request
		// =======
		// u8	opcode
		// u8	metabyte
		// u16	length
		// ...

		let name = &self.name;

		let generics = {
			let mut generics = self.generics.to_owned();
			add_bounds(&mut generics, parse_quote!(cornflakes::Writable));

			generics
		};
		let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

		// Tokens required to destructure the request's fields.
		let pat = TokenStream2::with_tokens(|tokens| {
			items.fields_to_tokens(tokens, ExpandMode::Request);
		});

		// If there is a metabyte item, generate its serialization tokens first.
		let metabyte = TokenStream2::with_tokens(|tokens| {
			if self.minor_opcode.is_some() {
				// If this request has a minor opcode, then that is to be
				// written in the metabyte position.
				tokens.append_tokens(|| {
					quote!(
						writer.put_u16(<Self as xrb::Request>::minor_opcode());
					)
				});
			} else {
				// Otherwise, if there is no minor opcode, serialize the
				// metabyte item (or a blank byte if there is none).
				items.metabyte_serialize_tokens(tokens);
			}
		});

		let inner = TokenStream2::with_tokens(|tokens| {
			// Generate the serialization tokens for all non-metabyte items.
			for (id, item) in items.pairs().filter(|(_, item)| !item.is_metabyte()) {
				item.serialize_tokens(tokens, id);
			}
		});

		tokens.append_tokens(|| {
			quote!(
				impl #impl_generics cornflakes::Writable for #name #type_generics #where_clause {
					fn write_to(
						&self,
						writer: &mut impl bytes::BufMut,
					) -> Result<(), cornflakes::WriteError> {
						// Destructure the struct.
						let Self #pat = self;

						// Major opcode.
						writer.put_u8(<Self as xrb::Request>::major_opcode());
						// Metabyte (minor opcode, metabyte item, or nothing).
						#metabyte
						// Request length.
						writer.put_u16(<Self as xrb::Request>::length(&self));

						// Rest of the items.
						#inner

						Ok(())
					}
				}
			)
		});
	}
}

impl DeserializeMessageTokens for Request {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, items: &Items) {
		// Request
		// =======
		// u8	opcode
		// u8	metabyte
		// u16	length
		// ...

		let name = &self.name;

		let generics = {
			let mut generics = self.generics.to_owned();
			add_bounds(&mut generics, parse_quote!(cornflakes::Readable));

			generics
		};
		let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

		let metabyte = TokenStream2::with_tokens(|tokens| {
			// If the request has a minor opcode, then it must have already
			// been read to know to deserialize this request, so we only write
			// tokens for the second byte if there is no minor opcode.
			if self.minor_opcode.is_none() {
				items.metabyte_deserialize_tokens(tokens);
			}
		});

		let inner = TokenStream2::with_tokens(|tokens| {
			// Deserialize every non-metabyte item.
			for (id, item) in items.pairs().filter(|(_, item)| !item.is_metabyte()) {
				item.deserialize_tokens(tokens, id);
			}
		});

		// Tokens required to use the request's struct's constructor.
		let cons = TokenStream2::with_tokens(|tokens| {
			items.fields_to_tokens(tokens, ExpandMode::Request);
		});

		tokens.append_tokens(|| {
			quote!(
				impl #impl_generics cornflakes::Readable for #name #type_generics #where_clause {
					fn read_from(
						reader: &mut impl bytes::Buf,
					) -> Result<Self, cornflakes::ReadError> {
						// Read the metabyte item, if any.
						#metabyte
						// Read the length of the request.
						let _length_ = reader.get_u16();

						// Read the rest of the items.
						#inner

						// Call the constructor.
						Ok(Self #cons)
					}
				}
			)
		});
	}
}

impl SerializeMessageTokens for Reply {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, items: &Items) {
		// Reply
		// =====
		// u8	1 (reply)
		// u8	metabyte
		// u16	sequence (optional...)
		// u32	length
		// ...

		let name = &self.name;

		let generics = {
			let mut generics = self.generics.to_owned();
			add_bounds(&mut generics, parse_quote!(cornflakes::Writable));

			generics
		};
		let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

		// Tokens required to destructure the reply's fields.
		let pat = TokenStream2::with_tokens(|tokens| {
			items.fields_to_tokens(
				tokens,
				ExpandMode::Reply {
					has_sequence: self.sequence_token.is_none(),
				},
			);
		});

		// Tokens required to serialize the metabyte position.
		let metabyte = TokenStream2::with_tokens(|tokens| {
			items.metabyte_serialize_tokens(tokens);
		});

		// Tokens required to serialize the sequence field, unless opted out.
		let sequence = TokenStream2::with_tokens(|tokens| {
			if self.sequence_token.is_none() {
				tokens.append_tokens(|| {
					quote!(
						writer.put_u16(_sequence_);
					)
				});
			}
		});

		let inner = TokenStream2::with_tokens(|tokens| {
			// Serialize every non-metabyte item.
			for (id, item) in items.pairs().filter(|(_, item)| !item.is_metabyte()) {
				item.serialize_tokens(tokens, id);
			}
		});

		tokens.append_tokens(|| {
			quote!(
				impl #impl_generics cornflakes::Writable for #name #type_generics #where_clause {
					fn write_to(
						&self,
						writer: &mut impl bytes::BufMut,
					) -> Result<(), cornflakes::WriteError> {
						let Self #pat = self;

						// `1` indicates this is a reply.
						writer.put_u8(1);
						// Metabyte item, or a blank byte if none.
						#metabyte
						// The sequence field, if there is one.
						#sequence
						// The length of the reply.
						writer.put_u16(<Self as xrb::Reply>::length(&self));

						#inner

						Ok(())
					}
				}
			)
		});
	}
}

impl DeserializeMessageTokens for Reply {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, items: &Items) {
		// Reply
		// =====
		// u8	1 (reply)
		// u8	metabyte
		// u16	sequence (optional...)
		// u32	length
		// ...

		let name = &self.name;

		let generics = {
			let mut generics = self.generics.to_owned();
			add_bounds(&mut generics, parse_quote!(cornflakes::Readable));

			generics
		};
		let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

		// Deserialization tokens for the metabyte item.
		let metabyte = TokenStream2::with_tokens(|tokens| {
			items.metabyte_deserialize_tokens(tokens);
		});

		let sequence = TokenStream2::with_tokens(|tokens| {
			// If the sequence field hasn't been opted out of...
			if self.sequence_token.is_none() {
				// Deserialize the sequence field.
				tokens.append_tokens(|| {
					quote!(
						let _sequence_ = reader.get_u16();
					)
				});
			}
		});

		let inner = TokenStream2::with_tokens(|tokens| {
			// Deserialization tokens for every non-metabyte item.
			for (id, item) in items.pairs().filter(|(_, item)| !item.is_metabyte()) {
				item.deserialize_tokens(tokens, id);
			}
		});

		// Tokens to use the constructor for the struct.
		let cons = TokenStream2::with_tokens(|tokens| {
			items.fields_to_tokens(
				tokens,
				ExpandMode::Reply {
					has_sequence: self.sequence_token.is_none(),
				},
			);
		});

		tokens.append_tokens(|| {
			quote!(
				impl #impl_generics cornflakes::Readable for #name #type_generics #where_clause {
					fn read_from(
						reader: &mut impl bytes::Buf,
					) -> Result<Self, cornflakes::ReadError> {
						// Deserialize the metabyte item.
						#metabyte
						// Deserialize the sequence field.
						#sequence
						// Deserialize the reply field.
						let _length_ = reader.get_u32();

						#inner

						Ok(Self #cons)
					}
				}
			)
		});
	}
}

impl SerializeMessageTokens for Event {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, items: &Items) {
		// Event
		// =====
		// u8	code
		// u8	metabyte
		// u16	sequence
		// ...

		let name = &self.name;

		let generics = {
			let mut generics = self.generics.to_owned();
			add_bounds(&mut generics, parse_quote!(cornflakes::Writable));

			generics
		};
		let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

		// Pattern to destructure the event struct.
		let pat = TokenStream2::with_tokens(|tokens| {
			items.fields_to_tokens(tokens, ExpandMode::Event);
		});

		// Tokens to serialize the metabyte item, if any.
		let metabyte = TokenStream2::with_tokens(|tokens| {
			items.metabyte_serialize_tokens(tokens);
		});

		let inner = TokenStream2::with_tokens(|tokens| {
			// Serialization tokens for every non-metabyte item.
			for (id, item) in items.pairs().filter(|(_, item)| !item.is_metabyte()) {
				item.serialize_tokens(tokens, id);
			}
		});

		tokens.append_tokens(|| {
			quote!(
				impl #impl_generics cornflakes::Writable for #name #type_generics #where_clause {
					fn write_to(
						&self,
						writer: &mut impl bytes::BufMut,
					) -> Result<(), cornflakes::WriteError> {
						let Self #pat = self;

						// Event code.
						writer.put_u8(<Self as xrb::Event>::code());
						// Serialize the metabyte item.
						#metabyte
						// Serialize the sequence field.
						writer.put_u16(_sequence_);

						#inner

						Ok(())
					}
				}
			)
		});
	}
}

impl DeserializeMessageTokens for Event {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, items: &Items) {
		// Event
		// =====
		// u8	code
		// u8	metabyte
		// u16	sequence
		// ...

		let name = &self.name;

		let generics = {
			let mut generics = self.generics.to_owned();
			add_bounds(&mut generics, parse_quote!(cornflakes::Readable));

			generics
		};
		let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

		// Deserialize the metabyte item, if any (otherwise skip the byte).
		let metabyte = TokenStream2::with_tokens(|tokens| {
			items.metabyte_deserialize_tokens(tokens);
		});

		let inner = TokenStream2::with_tokens(|tokens| {
			// Deserialize every non-metabyte item.
			for (id, item) in items.pairs().filter(|(_, item)| !item.is_metabyte()) {
				item.deserialize_tokens(tokens, id);
			}
		});

		// Tokens for the event struct constructor.
		let cons = TokenStream2::with_tokens(|tokens| {
			items.fields_to_tokens(tokens, ExpandMode::Event);
		});

		tokens.append_tokens(|| {
			quote!(
				impl #impl_generics cornflakes::Readable for #name #type_generics #where_clause {
					fn read_from(
						reader: &mut impl bytes::Buf,
					) -> Result<Self, cornflakes::ReadError> {
						// Deserialize the metabyte item.
						#metabyte
						// Deserialize the sequence field.
						let _sequence_ = reader.get_u16();

						#inner

						Ok(Self #cons)
					}
				}
			)
		});
	}
}

impl Request {
	pub fn impl_request_tokens(&self, tokens: &mut TokenStream2) {
		// Request name.
		let name = &self.name;
		// Generics.
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();
		// Type of reply generated, if any.
		let reply = TokenStream2::with_tokens(|tokens| {
			if let Some((_, r#type)) = &self.reply_ty {
				r#type.to_tokens(tokens);
			} else {
				quote!(()).to_tokens(tokens);
			}
		});

		// The expression evaluating to the request's major opcode.
		let major = &self.major_opcode_expr;

		// The expression evaluating to the request's major opcode, if any.
		let minor = if let Some((_, minor)) = &self.minor_opcode {
			quote!(Some((#minor) as u8))
		} else {
			quote!(None)
		};

		tokens.append_tokens(|| {
			quote!(
				// NOTE: in `xrb`, `extern crate self as xrb;` will have to be
				//       used so that the trait path works.
				impl #impl_generics xrb::Request for #name #type_generics #where_clause {
					type Reply = #reply;

					// The major opcode uniquely identifying the request.
					fn major_opcode() -> u8 {
						(#major) as u8
					}

					// The minor opcode uniquely identifying the request
					// within a particular extension (if this is a request from
					// an extension, that extension has multiple requests, and
					// that extension chooses to make use of the minor opcode
					// field).
					fn minor_opcode() -> Option<u8> {
						#minor
					}

					// The length of the request, measured in multiples of 4 bytes.
					fn length(&self) -> u16 {
						// TODO: calculate length by summing item lengths, plus
						//       minimum length from header etc.
						0
					}
				}
			)
		});
	}
}

impl Reply {
	pub fn impl_reply_tokens(&self, tokens: &mut TokenStream2) {
		//  The name of the reply.
		let name = &self.name;
		// Generics.
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();
		// The type of request associated with this reply.
		let request = &self.request_ty;

		// The sequence number associated with the request that generated this
		// reply, if any.
		let sequence = if self.sequence_token.is_none() {
			quote!(Some(self._sequence_))
		} else {
			quote!(None)
		};

		tokens.append_tokens(|| {
			quote!(
				// NOTE: in `xrb`, `extern crate self as xrb;` will have to be
				//       used so that the trait path works.
				impl #impl_generics xrb::Reply for #name #type_generics #where_clause {
					type Req = #request;

					// The sequence number associated with the request that
					// generated this reply, if any.
					fn sequence(&self) -> Option<u16> {
						#sequence
					}

					// The number of 4-byte units greater than the minimum
					// length of 32 bytes.
					fn length(&self) -> u32 {
						// TODO: implement length
						0
					}
				}
			)
		});
	}
}

impl Event {
	pub fn impl_event_tokens(&self, tokens: &mut TokenStream2) {
		// Name of the event.
		let name = &self.name;
		// Generics.
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();
		// The expression evaluating to the event's event code.
		let code = &self.event_code_expr;

		tokens.append_tokens(|| {
			quote!(
				// NOTE: in `xrb`, `extern crate self as xrb;` will have to be
				//       used so that the trait path works.
				impl #impl_generics xrb::Event for #name #type_generics #where_clause {
					// The code uniquely identifying this event.
					fn code() -> u8 {
						(#code) as u8
					}

					// The sequence number associated with the last relevant
					// request sent to the X server prior to this event.
					fn sequence(&self) -> u16 {
						self._sequence_
					}
				}
			)
		});
	}
}
