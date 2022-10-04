// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod closure;
mod content;
mod metadata;
mod util;

mod definition;
mod message;

use definition::*;
use message::*;
use util::*;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, DeriveInput, Result};

/// A wrapper around <code>[Vec]<T></code> that implements [`Parse`] and
/// [`ToTokens`].
///
/// This exists because we can't implement foreign traits on foreign types.
struct Many<T>
where
	T: Parse + ToTokens,
{
	pub things: Vec<T>,
}

impl<T> Parse for Many<T>
where
	T: Parse + ToTokens,
{
	fn parse(input: ParseStream) -> Result<Self> {
		let mut things = vec![];

		// Parse `T` until the end of `input`.
		while !input.is_empty() {
			things.push(input.parse()?);
		}

		Ok(Self { things })
	}
}

impl<T> ToTokens for Many<T>
where
	T: Parse + ToTokens,
{
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Write each `thing` in `&self.things` as tokens.
		for thing in &self.things {
			thing.to_tokens(tokens);
		}
	}
}

/// Defines `struct`s for X11 protocol messages and automatically generates
/// trait implementations.
///
/// Specifically, those trait implementations include the trait relevant for
/// that particular message (`crate::Request`, `crate::Reply`, or
/// `crate::Event`), as well as for serialization and deserialization with
/// `cornflakes::ToBytes` and `cornflakes::FromBytes`, respectively.
#[proc_macro]
pub fn messages(input: TokenStream) -> TokenStream {
	// Parse the input as [Many] [`Message`]s.
	let input = parse_macro_input!(input as Many<Message>);

	// The list of messages.
	let messages = input.things;

	// The trait implementations, not including serialization and deserialization.
	let trait_impls: Vec<TokenStream2> = messages
		.iter()
		.map(|message| message.message_trait_impl())
		.collect();

	// TODO: generate serialization and deserialization for messages.

	// The actual code generated:
	let expanded = quote! {
		#(#messages)*
		#(#trait_impls)*
	};

	expanded.into()
}

/// Defines enums and structs with special syntax to generate their
/// (de)serialization.
///
/// This uses the same syntax as [`messages!`].
///
/// [`messages!`]: messages
#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
	// Parse the input as [Many] [`Definition`]s.
	let input = parse_macro_input!(input as Many<Definition>);

	// The list of definitions.
	let definitions = input.things;

	// The actual code generated:
	let expanded = quote! {
		#(#definitions)*
		// TODO: serialization & deserialization
	};

	expanded.into()
}

/// Derives an implementation of `cornflakes::ByteSize` for an enum or struct.
#[proc_macro_derive(ByteSize)]
pub fn derive_byte_size(input: TokenStream) -> TokenStream {
	// Parse the input as derive macro input (`DeriveInput`).
	let input = parse_macro_input!(input as DeriveInput);

	// The name of the struct or enum deriving `ByteSize`.
	let name = input.ident;

	// The generics associated with the struct or enum deriving `ByteSize`.
	let generics = add_trait_bounds(input.generics, quote!(cornflakes::ByteSize));
	// Split the generics into forms that can be used after `impl` (e.g.
	// `impl<T>`), after the name (e.g. `Name<T>`), and in the where clause
	// (e.g. `where T: ByteSize`).
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	// If this is a
	// - struct: sum the fields.
	// - enum: take the highest byte size of the variants.
	let sum = byte_size_sum(&input.data);

	// The actual code generated:
	let expanded = quote! {
		impl #impl_generics cornflakes::ByteSize for #name #ty_generics #where_clause {
			fn byte_size(&self) -> usize {
				#sum
			}
		}
	};

	expanded.into()
}

/// Derives an implementation of `cornflakes::Writable` for an enum or struct.
#[proc_macro_derive(Writable)]
pub fn derive_writable(input: TokenStream) -> TokenStream {
	// Parse the input as derive macro input (`DeriveInput`).
	let input = parse_macro_input!(input as DeriveInput);

	// The name of the struct or enum deriving `Writable`.
	let name = input.ident;

	// The generics associated with the struct or enum deriving `Writable`.
	let generics = add_trait_bounds(input.generics, quote!(cornflakes::Writable));
	// Split the generics into forms that can be used after `impl` (e.g.
	// `impl<T>`), after the name (e.g. `Name<T>`), and in the where clause
	// (e.g. `where T: Writable`).
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	// TODO: Implementation of `write_to`.
	// let write = writable_write(&input.data);

	let expanded = quote! {
		impl #impl_generics cornflakes::Writable for #name #ty_generics #where_clause {
			fn write_to(
				&self,
				writer: &mut impl cornflakes::Writer,
			) -> Result<(), cornflakes::WriteError> {
				// #write

				Ok(())
			}
		}
	};

	expanded.into()
}

/// Derives an implementation of `cornflakes::Readable` for an enum or struct.
#[proc_macro_derive(Readable)]
pub fn derive_readable(input: TokenStream) -> TokenStream {
	// Parse the input as derive macro input (`DeriveInput`).
	let input = parse_macro_input!(input as DeriveInput);

	// The name of the struct or enum deriving `Writable`.
	let name = input.ident;

	// The generics associated with the struct or enum deriving `Readable`.
	let generics = add_trait_bounds(input.generics, quote!(cornflakes::Readable));
	// Split the generics into forms that can be used after `impl` (e.g.
	// `impl<T>`), after the name (e.g. `Name<T>`), and in the where clause
	// (e.g. `where T: Readable`).
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	// TODO: Implementation of `read_from`.
	// let read = readable_impl(&input.data);

	let expanded = quote! {
		impl #impl_generics cornflakes::Readable for #name #ty_generics #where_clause {
			fn read_from(reader: &mut impl cornflakes::Reader) -> Result<Self, cornflakes::ReadError> {
				// #read
			}
		}
	};

	expanded.into()
}
