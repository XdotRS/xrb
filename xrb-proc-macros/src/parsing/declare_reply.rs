// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{Result, Token, Type};

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
