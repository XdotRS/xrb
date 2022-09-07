// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, token, LitInt, Result, Token, Type};

/// Information specifically associated with requests, not replies.
#[derive(Clone)]
pub struct RequestMetadata {
	pub paren_token: token::Paren,
	/// The major opcode of this request.
	pub major_opcode: u8,
	/// The minor opcode of this request, if any. Only used in extensions.
	pub minor_opcode: Option<(Token![,], u8)>,
	/// The type of reply that is returned by this request, if any.
	pub reply: Option<(Token![->], Type)>,
}

// Parsing {{{

impl Parse for RequestMetadata {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parnetheses (`(` and `)`) for the opcodes.
		let content;

		Ok(Self {
			// `(` and `)`.
			paren_token: parenthesized!(content in input),
			// Major opcode.
			major_opcode: content.parse::<LitInt>()?.base10_parse()?,
			// Optional: `,` + minor opcode.
			minor_opcode: content
				.parse() // ,
				.ok()
				.map(|comma| {
					(
						comma,
						content.parse::<LitInt>().unwrap().base10_parse().unwrap(),
					)
				}),
			// Optional: `->` + reply type.
			reply: input.parse().ok().zip(input.parse().ok()),
		})
	}
}

// }}}
