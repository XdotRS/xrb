// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{
	parse::{Parse, ParseBuffer, ParseStream, Result},
	punctuated::Punctuated,
};

pub trait PsExt {
	/// Parses a syntax tree node of type `T`, given some additional `context`.
	fn parse_with<T: ParseWithContext>(&self, context: T::Context<'_>) -> Result<T>;

	/// Parses zero or more occurrences of `T` given some additional `context`,
	/// separated by punctuation of type `P`, with optional trailing
	/// punctuated.
	///
	/// Parsing continues until the end of this parse stream. The entire content
	/// of this parse stream must consist of `T` and `P`.
	fn parse_terminated_with<'a, T, F, P>(&self, context: F) -> Result<Punctuated<T, P>>
	where
		T: ParseWithContext,
		F: FnMut() -> T::Context<'a>,
		P: Parse;
}

impl<'a> PsExt for ParseBuffer<'a> {
	fn parse_with<T: ParseWithContext>(&self, context: T::Context<'_>) -> Result<T> {
		T::parse_with(self, context)
	}

	fn parse_terminated_with<'b, T, F, P>(&self, context: F) -> Result<Punctuated<T, P>>
	where
		T: ParseWithContext,
		F: FnMut() -> T::Context<'b>,
		P: Parse,
	{
		let mut punctuated = Punctuated::new();

		while !self.is_empty() {
			punctuated.push_value(self.parse_with(context())?);

			if self.is_empty() {
				break;
			} else {
				punctuated.push_punct(self.parse()?);
			}
		}

		Ok(punctuated)
	}
}

pub trait ParseWithContext {
	type Context<'a>;

	fn parse_with(input: ParseStream, context: Self::Context<'_>) -> Result<Self>
	where
		Self: Sized;
}

pub trait TsExt {
	/// Creates a new [`TokenStream`] by applying the given function `f` to an
	/// empty [`TokenStream`].
	///
	/// # Examples
	/// ```ignore
	/// # use proc_macro2::TokenStream;
	/// # use syn::Token;
	/// #
	/// use crate::TsExt;
	///
	/// let let_token = Token![let];
	///
	/// let tokens = TokenStream::with_tokens(|tokens| {
	///     let_token.to_tokens(tokens);
	/// });
	/// ```
	///
	/// [`TokenStream`]: proc_macro::TokenStream
	fn with_tokens<F>(f: F) -> Self
	where
		F: FnOnce(&mut Self);

	/// Appends a [`TokenStream`] given by the given function `f`.
	///
	/// # Examples
	/// ```ignore
	/// # use proc_macro2::TokenStream;
	/// # use quote::quote;
	/// #
	/// use crate::TsExt;
	///
	/// let mut tokens = TokenStream::new();
	///
	/// tokens.append_tokens(|| {
	///     quote!(
	///         println!("hello world!");
	///     )
	/// });
	/// ```
	///
	/// [`TokenStream`]: proc_macro::TokenStream
	fn append_tokens<F>(&mut self, f: F)
	where
		F: FnOnce() -> Self;
}

impl TsExt for TokenStream2 {
	fn with_tokens<F>(f: F) -> Self
	where
		F: FnOnce(&mut Self),
	{
		let mut tokens = Self::new();
		f(&mut tokens);

		tokens
	}

	fn append_tokens<F>(&mut self, f: F)
	where
		F: FnOnce() -> Self,
	{
		f().to_tokens(self);
	}
}
