// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

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
				}
			}
		}
	}
}

impl ItemSerializeTokens for Field {
	// Tokens to serialize a field.
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		let name = id.formatted();
		tokens.append_tokens(|| quote!(#name.write_to(writer)?;));
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
				let args = context.source().fmt_args();

				quote!(
					// let __my_field__ = <Vec<u8>>::read_with(
					//     reader,
					//     __my_field__(__my_len__),
					// )?;
					let #name = <#r#type as cornflakes::ContextualReadable>
						::read_with(
							reader,
							#name( #(#args,)* ),
						)?;
				)
			} else {
				quote!(
					// let __my_field2__ = u8::read_from(reader)?;
					let #name = <r#type as cornflakes::Readable>::read_from(reader)?;
				)
			}
		});
	}
}

impl ItemSerializeTokens for Let {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		let name = id.formatted();
		let args = self.source.fmt_args();

		quote!(
			// __data_len__(&__data__).write_to(writer)?;
			#name( #( &#args, )* ).write_to(writer)?;
		)
		.to_tokens(tokens);
	}
}

impl ItemDeserializeTokens for Let {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		let name = id.formatted();
		let r#type = &self.r#type;

		tokens.append_tokens(|| {
			// let __data_len__: u32 = reader.read()?;
			quote!(let #name: #r#type = reader.read()?;)
		});
	}
}

impl ItemSerializeTokens for Unused {
	fn serialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		match self {
			Self::Unit { .. } => {
				// 0u8.write_to(writer)?;
				tokens.append_tokens(|| {
					quote!(
						writer.put_u8(0);
					)
				});
			}

			Self::Array(array) => {
				let name = id.formatted();
				let args = array.source.fmt_args();

				tokens.append_tokens(|| {
					quote!(
						// writer.put_many(0u8, _unused_1_(&__data__));
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

impl ItemDeserializeTokens for Unused {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, id: &ItemId) {
		tokens.append_tokens(|| {
			match self {
				Self::Array(array) => {
					let name = id.formatted();
					let args = array.source.fmt_args();

					quote!(
						// reader.advance(_unused_1_(&__data__) as usize);
						reader.advance(
							#name( #(#args,)* ) as usize,
						);
					)
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

impl Enum {
	fn serialize_tokens(&self, tokens: &mut TokenStream2) {
		let name = &self.ident;

		let arms = TokenStream2::with_tokens(|tokens| {
			// Start the variants' discriminant tokens at `0`. We add `1` each
			// iteration, unless a variant explicitly specifies its
			// discriminant.
			let mut discrim = quote!(0);

			for variant in &self.variants {
				let name = &variant.ident;

				// If the variant explicitly specifies its discriminant, reset
				// the `discrim` tokens to that discriminant expression.
				if let Some((_, expr)) = &variant.discriminant {
					discrim = expr.to_token_stream();
				}

				// Tokens to destructure the variant's fields.
				let pat = TokenStream2::with_tokens(|tokens| {
					variant.items.pattern_to_tokens(tokens, ExpandMode::Normal);
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
							((#discrim) as u8).write_to(writer)?;

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
				// impl Writable for MyEnum {
				//     fn write_to(
				//         &self,
				//         writer: &mut impl BufMut,
				//     ) -> Result<(), Box<dyn Error>> {
				//         match self {
				//             Self::Variant => {
				//                 (0 as u8).write_to(writer)?;
				//             }
				//         }
				//     }
				// }
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

impl Enum {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2) {
		let name = &self.ident;

		let arms = TokenStream2::with_tokens(|tokens| {
			// Start the variants' discriminant tokens at `0`. We add `1` each
			// iteration, unless a variant explicitly specifies its
			// discriminant.
			let mut discrim = quote!(0);

			for variant in &self.variants {
				let name = &variant.ident;

				// If the variant explicitly specifies its discriminant, reset
				// the `discrim` tokens to that discriminant expression.
				if let Some((_, expr)) = &variant.discriminant {
					discrim = expr.to_token_stream();
				}

				// Tokens to fill in the fields for the variant's constructor.
				let cons = TokenStream2::with_tokens(|tokens| {
					variant.items.constructor_to_tokens(tokens);
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
						#discrim => {
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
				// impl Readable for MyEnum {
				//     fn read_from(reader: &mut impl Buf) -> Result<Self, Box<dyn Error>> {
				//         match reader.read::<u8>() {
				//             (0 as u8) => {
				//                 Self::Variant
				//             }
				//             _ => panic!("unrecognized enum variant discriminant"),
				//         }
				//     }
				// }
				impl cornflakes::Readable for #name {
					fn read_from(
						reader: &mut impl bytes::Buf,
					) -> Result<Self, Box<dyn std::error::Error>> {
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

		// Tokens to destructure the struct's fields.
		let pat = TokenStream2::with_tokens(|tokens| {
			items.pattern_to_tokens(tokens, ExpandMode::Normal);
		});

		let inner = TokenStream2::with_tokens(|tokens| {
			for (id, item) in items.pairs() {
				item.serialize_tokens(tokens, id);
			}
		});

		tokens.append_tokens(|| {
			quote!(
				// impl Writable for MyStruct {
				//     fn write_to(
				//         &self,
				//         writer: &mut impl BufMut,
				//     ) -> Result<(), Box<dyn Error>> {
				//         let Self(__0__, __1__) = self;
				//
				//         __0__.write_to(writer)?;
				//         __1__.write_to(writer)?;
				//     }
				// }
				impl cornflakes::Writable for #name {
					fn write_to(
						&self,
						writer: &mut impl bytes::BufMut,
					) -> Result<(), Box<dyn std::error::Error>> {
						// Destructure the struct.
						let Self #pat = self;

						#inner
					}
				}
			)
		});
	}
}

impl DeserializeMessageTokens for BasicStructMetadata {
	fn deserialize_tokens(&self, tokens: &mut TokenStream2, items: &Items) {
		let name = &self.name;

		// Tokens to fill in the fields for the struct's constructor.
		let cons = TokenStream2::with_tokens(|tokens| {
			items.constructor_to_tokens(tokens);
		});

		// Generate the tokens to deserialize each of the struct's items.
		let inner = TokenStream2::with_tokens(|tokens| {
			for (id, item) in items.pairs() {
				item.deserialize_tokens(tokens, id);
			}
		});

		tokens.append_tokens(|| {
			quote!(
				// impl Readable for MyStruct {
				//     fn read_from(reader: &mut impl Buf) -> Result<Self, Box<dyn Error>> {
				//         let __0__: i32 = reader.read();
				//         let __1__: i32 = reader.read();
				//
				//         Self(__0__, __1__)
				//     }
				// }
				impl cornflakes::Readable for #name {
					fn read_from(
						reader: &mut impl bytes::Buf,
					) -> Result<Self, Box<dyn std::error::Error>> {
						#inner

						Self #cons
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

		// Tokens required to destructure the request's fields.
		let pat = TokenStream2::with_tokens(|tokens| {
			items.pattern_to_tokens(tokens, ExpandMode::Request);
		});

		// If there is a metabyte item, generate its serialization tokens first.
		let metabyte = TokenStream2::with_tokens(|tokens| {
			if self.minor_opcode.is_some() {
				// If this request has a minor opcode, then that is to be
				// written in the metabyte position.
				tokens.append_tokens(|| {
					quote!(
						writer.put_u16(<Self as crate::x11::traits::Request>::minor_opcode());
					)
				});
			} else {
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
				impl cornflakes::Writable for #name {
					fn write_to(
						&self,
						writer: &mut impl bytes::BufMut,
					) -> Result<(), Box<dyn std::error::Error>> {
						// Destructure the struct.
						let Self #pat = self;

						// Major opcode.
						writer.put_u8(<Self as crate::x11::traits::Request>::major_opcode());
						// Metabyte (minor opcode, metabyte item, or nothing).
						#metabyte
						// Request length.
						writer.put_u16(<Self as crate::x11::traits::Request>::length(&self));

						// Rest of the items.
						#inner
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

		let metabyte = TokenStream2::with_tokens(|tokens| {
			// If the request has a minor opcode, then it must have already
			// been read to know to deserialize this request, so we only write
			// tokens for the second byte if there is no minor opcode.
			if self.minor_opcode.is_none() {
				items.metabyte_deserialize_tokens(tokens);
			}
		});

		let inner = TokenStream2::with_tokens(|tokens| {
			for (id, item) in items.pairs().filter(|(_, item)| !item.is_metabyte()) {
				item.deserialize_tokens(tokens, id);
			}
		});

		// Tokens required to use the request's struct's constructor.
		let cons = TokenStream2::with_tokens(|tokens| {
			items.constructor_to_tokens(tokens);
		});

		tokens.append_tokens(|| {
			quote!(
				impl cornflakes::Readable for #name {
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
						Self #cons
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

		// Tokens required to destructure the reply's fields.
		let pat = TokenStream2::with_tokens(|tokens| {
			items.pattern_to_tokens(
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
			for (id, item) in items.pairs().filter(|(_, item)| !item.is_metabyte()) {
				item.serialize_tokens(tokens, id);
			}
		});

		tokens.append_tokens(|| {
			quote!(
				impl cornflakes::Writable for #name {
					fn write_to(
						&self,
						writer: &mut impl bytes::BufMut,
					) -> Result<(), cornflakes::WriteError> {
						let Self #pat = self;

						writer.put_u8(1);
						#metabyte
						#sequence
						writer.put_u16(<Self as crate::x11::traits::Reply>::length(&self));

						#inner
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

		let metabyte = TokenStream2::with_tokens(|tokens| {
			items.metabyte_deserialize_tokens(tokens);
		});

		let sequence = TokenStream2::with_tokens(|tokens| {
			if self.sequence_token.is_none() {
				tokens.append_tokens(|| {
					quote!(
						let _sequence_ = reader.get_u16();
					)
				});
			}
		});

		let inner = TokenStream2::with_tokens(|tokens| {
			for (id, item) in items.pairs().filter(|(_, item)| !item.is_metabyte()) {
				item.deserialize_tokens(tokens, id);
			}
		});

		let cons = TokenStream2::with_tokens(|tokens| {
			items.constructor_to_tokens(tokens);
		});

		tokens.append_tokens(|| {
			quote!(
				impl cornflakes::Readable for #name {
					fn read_from(
						reader: &mut impl bytes::Buf,
					) -> Result<Self, cornflakes::ReadError> {
						#metabyte
						#sequence
						let _length_ = reader.get_u32();

						#inner

						Self #cons
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

		let pat = TokenStream2::with_tokens(|tokens| {
			items.pattern_to_tokens(tokens, ExpandMode::Event);
		});

		let metabyte = TokenStream2::with_tokens(|tokens| {
			items.metabyte_serialize_tokens(tokens);
		});

		let inner = TokenStream2::with_tokens(|tokens| {
			for (id, item) in items.pairs().filter(|(_, item)| !item.is_metabyte()) {
				item.serialize_tokens(tokens, id);
			}
		});

		tokens.append_tokens(|| {
			quote!(
				impl cornflakes::Writable for #name {
					fn write_to(
						&self,
						writer: &mut impl bytes::BufMut,
					) -> Result<(), cornflakes::WriteError> {
						let Self #pat = self;

						writer.put_u8(<Self as crate::x11::traits::Event>::code());
						#metabyte
						writer.put_u16(_sequence_);

						#inner
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

		let metabyte = TokenStream2::with_tokens(|tokens| {
			items.metabyte_deserialize_tokens(tokens);
		});

		let inner = TokenStream2::with_tokens(|tokens| {
			for (id, item) in items.pairs().filter(|(_, item)| !item.is_metabyte()) {
				item.deserialize_tokens(tokens, id);
			}
		});

		let cons = TokenStream2::with_tokens(|tokens| {
			items.constructor_to_tokens(tokens);
		});

		tokens.append_tokens(|| {
			quote!(
				impl cornflakes::Readable for #name {
					fn read_from(
						reader: &mut impl bytes::Buf,
					) -> Result<Self, cornflakes::ReadError> {
						#metabyte
						let _sequence_ = reader.get_u16();

						#inner

						Self #cons
					}
				}
			)
		});
	}
}
