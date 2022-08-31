// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{LitInt, Result, Token};

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;

/// An opcode that can appear in `request!` macros.
///
/// [`Opcode`] is used both for major and minor opcodes in `request!` macros.
///
/// # Examples
/// ```rust
/// 4!  // opcode: 4
/// 57! // opcode: 57
/// 13! // opcode: 13
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Opcode {
	pub opcode: u8,
}

impl Opcode {
	#[allow(dead_code)]
	/// Construct a new [`Opcode`] from the given [`u8`] integer.
	pub fn new(opcode: u8) -> Self {
		Self { opcode }
	}
}

impl ToTokens for Opcode {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.opcode.to_tokens(tokens);
	}
}

/// Parsing {{{

impl Parse for Opcode {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse the opcode as an integer literal.
		let value: LitInt = input.parse()?;
		let opcode: u8 = value.base10_parse()?;

		// Parse the `!` token, but don't save it. The point of this is that it
		// returns an error if it wasn't present.
		input.parse::<Token![!]>()?;

		Ok(Self { opcode })
	}
}

// }}}
