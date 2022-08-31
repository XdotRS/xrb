// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{Result, Token, Type};

use proc_macro2::{Punct, Spacing, TokenStream as TokenStream2};
use quote::{ToTokens, TokenStreamExt};

/// A reply's declaration of the type of its associated request.
///
/// # Examples
/// ```rust
/// for GetWindowAttributes
/// ```
#[derive(Clone)]
pub struct RequestDeclaration {
	pub request_ty: Type,
}

impl RequestDeclaration {
	#[allow(dead_code)]
	/// Construct a new [`RequestDeclaration`] with the given type of the
	/// associated request.
	pub fn new(request_ty: Type) -> Self {
		Self { request_ty }
	}
}

impl ToTokens for RequestDeclaration {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Write `<`: `Alone` means that it isn't _combined_ with the next token,
		// e.g. `<=`.
		tokens.append(Punct::new('<', Spacing::Alone));
		// Write the request type.
		self.request_ty.to_tokens(tokens);
		// Write `>`.
		tokens.append(Punct::new('>', Spacing::Alone));

		// This will end up looking like `<RequestType>`.
	}
}

// Parsing {{{

impl Parse for RequestDeclaration {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse the `for` token, but don't save it.
		input.parse::<Token![for]>()?;

		Ok(Self {
			// Parse the request's type.
			request_ty: input.parse()?,
		})
	}
}

// }}}
