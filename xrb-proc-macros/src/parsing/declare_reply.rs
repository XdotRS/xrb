// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{Result, Token, Type};

use proc_macro2::{Punct, Spacing, TokenStream as TokenStream2};
use quote::{ToTokens, TokenStreamExt};

/// A request's declaration of the type of its associated reply.
///
/// # Examples
/// ```rust
/// -> GetWindowAttributesReply
/// ```
#[derive(Clone)]
pub struct ReplyDeclaration {
	pub reply_ty: Type,
}

impl ReplyDeclaration {
	#[allow(dead_code)]
	/// Construct a new [`ReplyDeclaration`] with the given type of the returned
	/// reply.
	pub fn new(reply_ty: Type) -> Self {
		Self { reply_ty }
	}
}

impl ToTokens for ReplyDeclaration {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Write `<`: `Alone` means that it isn't _combined_ with the next token,
		// e.g. `<=`.
		tokens.append(Punct::new('<', Spacing::Alone));
		// Write the reply type.
		self.reply_ty.to_tokens(tokens);
		// Write `>`.
		tokens.append(Punct::new('>', Spacing::Alone));

		// This will end up looking like `<ReplyType>`.
	}
}

// Parsing {{{

impl Parse for ReplyDeclaration {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse the `->` token, but don't save it.
		input.parse::<Token![->]>()?;

		Ok(Self {
			// Parse the reply's type.
			reply_ty: input.parse()?,
		})
	}
}

// }}}
