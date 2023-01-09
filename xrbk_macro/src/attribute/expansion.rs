// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream;
use quote::ToTokens;

use super::*;

impl ToTokens for MetabyteAttribute {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		// `#`
		self.hash_token.to_tokens(tokens);
		// Square brackets surrounding `metabyte`.
		self.bracket_token.surround(tokens, |tokens| {
			self.path.to_tokens(tokens);
		});
	}
}

impl ToTokens for SequenceAttribute {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		// `#`.
		self.hash_token.to_tokens(tokens);
		// Square brackets surrounding `sequence`.
		self.bracket_token.surround(tokens, |tokens| {
			self.path.to_tokens(tokens);
		});
	}
}

impl ToTokens for HideAttribute {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		// `#`.
		self.hash_token.to_tokens(tokens);
		// Square brackets surrounding `hide`.
		self.bracket_token.surround(tokens, |tokens| {
			self.path.to_tokens(tokens);
		});
	}
}

impl ToTokens for ContextAttribute {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		// `#`.
		self.hash_token.to_tokens(tokens);
		// Square brackets surrounding `context` and the context delimiters.
		self.bracket_token.surround(tokens, |tokens| {
			self.path.to_tokens(tokens);
			self.context.to_tokens(tokens);
		});
	}
}

impl ToTokens for Context {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Self::Paren { paren_token, .. } => {
				// The normal brackets (but not the source, unfortunately).
				paren_token.surround(tokens, |_| {});
			},

			Self::Brace { brace_token, .. } => {
				// The curly brackets (but not the source, unfortunately).
				brace_token.surround(tokens, |_| {});
			},

			Self::Bracket { bracket_token, .. } => {
				// The square brackets (but not the source, unfortunately).
				bracket_token.surround(tokens, |_| {});
			},

			Self::Equals { equals_token, .. } => {
				// The equals token (but not the source, unfortunately).
				equals_token.to_tokens(tokens);
			},
		}
	}
}
