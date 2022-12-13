// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::source::LengthArg;
use proc_macro2::TokenStream;
use quote::ToTokens;

impl ToTokens for LengthArg {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.self_token.to_tokens(tokens);
		self.double_colon_token.to_tokens(tokens);
		self.length_token.to_tokens(tokens);
	}
}
