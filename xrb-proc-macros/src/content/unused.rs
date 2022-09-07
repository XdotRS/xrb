// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{braced, bracketed, parenthesized, token, Ident, LitInt, Result, Token};

/// `$()` or `()` or `[(); definition]`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum UnusedBytes {
	/// `$()` or `()`.
	Single((Option<Token![$]>, token::Paren)),
	/// `[(); definition]`.
	FullySpecified(FullUnusedBytes),
}

/// `[(); definition]`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FullUnusedBytes {
	/// `[` and `]`.
	pub bracket_token: token::Bracket,
	/// `()`.
	pub unit_token: token::Paren,
	/// `;`.
	pub semicolon_token: Token![;],
	/// The definition of how many unused bytes there is.
	pub definition: UnusedBytesDefinition,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum UnusedBytesDefinition {
	/// A specific, unvarying number of bytes.
	///
	/// `22`.
	Numerical(usize),
	/// The number of bytes required to pad the size of a field to a multiple
	/// of 4 bytes.
	///
	/// For example, this might be used with a list of one-byte values, like a
	/// `String8`, to ensure that the message is a multiple of 4 bytes.
	///
	/// `{fieldname}`.
	Padding((token::Brace, Ident)),
}

// Expansion {{{

impl ToTokens for UnusedBytes {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// For reference, unused bytes are not necessarily zero. We could have
		// picked any number to write here instead of 0.

		match self {
			// If it is a single unused byte, it is just a single [`u8`] value
			// that we need to write.
			Self::Single(_) => quote!(writer.write_u8(0);),

			Self::FullySpecified(unused) => match &unused.definition {
				UnusedBytesDefinition::Numerical(val) => quote! {
					writer.write_many(0, #val);
				},
				UnusedBytesDefinition::Padding(padding) => {
					let name = &padding.1;

					quote! {
						// Equivalent to:
						// ```
						// writer.write_many(0, 4 - (self.#name.byte_size() % 4) % 4)?;
						// ```
						writer.write_many(0,
							// `4 - (size % 4) % 4` calculates the number of
							// bytes requried to pad any given `size` up to the
							// nearest multiple of 4.
							4 - (cornflakes::ByteSize::byte_size(self.#name) % 4) % 4
						)?;
					}
				}
			},
		}
		.to_tokens(tokens);
	}
}

// }}}

// Parsing {{{

impl Parse for UnusedBytes {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(token::Bracket) {
			// `[(); definition]`
			Self::FullySpecified(input.parse()?)
		} else {
			// `()`
			let _paren;
			Self::Single((input.parse().ok(), parenthesized!(_paren in input)))
		})
	}
}

impl Parse for FullUnusedBytes {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;
		let _paren;

		Ok(Self {
			// `[` and `]`.
			bracket_token: bracketed!(content in input),
			// `()`.
			unit_token: parenthesized!(_paren in content),
			// `;`.
			semicolon_token: content.parse()?,
			definition: content.parse()?,
		})
	}
}

impl Parse for UnusedBytesDefinition {
	fn parse(input: ParseStream) -> Result<Self> {
		let look = input.lookahead1();

		if look.peek(token::Brace) {
			// `{` and `}`
			let content;

			Ok(Self::Padding((braced!(content in input), content.parse()?)))
		} else if look.peek(LitInt) {
			// `4`, `22`, etc.
			Ok(Self::Numerical(input.parse::<LitInt>()?.base10_parse()?))
		} else {
			// Otherwise, construct the error:
			Err(look.error())
		}
	}
}

// }}}
