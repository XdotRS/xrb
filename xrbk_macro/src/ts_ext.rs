// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;

pub trait TsExt {
	fn with_tokens<F>(f: F) -> Self
	where
		F: FnOnce(&mut Self);

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
