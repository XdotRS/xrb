// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;

pub trait TsExt {
	/// Creates a new [`TokenStream`] by applying the given function `f` to an
	/// empty [`TokenStream`].
	///
	/// # Examples
	/// ```
	/// use proc_macro2::TokenStream;
	/// use syn::Token;
	///
	/// use crate::ts_ext::TsExt;
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
	/// ```
	/// use proc_macro2::TokenStream;
	/// use quote::quote;
	///
	/// use crate::ts_ext::TsExt;
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
