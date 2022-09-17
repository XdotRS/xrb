// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, token, Expr, Result, Token, Type};

/// Information specifically associated with requests, not replies.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct RequestMetadata {
	pub paren_token: token::Paren,
	/// The major opcode of this request.
	pub major_opcode: Expr,
	/// The minor opcode of this request, if any. Only used in extensions.
	pub minor_opcode: Option<(Token![,], Expr)>,
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
			major_opcode: content.parse()?,
			// Optional: `,` + minor opcode.
			minor_opcode: content
				.parse() // `,`
				.map(|comma| {
					// If there is no `Expr`, return an error: "expected minor
					// opcode expression", otherwise return that expression:
					let expr = content.parse::<Expr>().map_or_else(
						|_| Err(content.error("expected minor opcode expression")),
						|expr| Ok(expr),
					);

					// Pair the comma and the expression.
					expr.map(|expr| (comma, expr))
				})?
				.ok(),
			// Optional: `->` + reply type.
			reply: input.parse().ok().zip(input.parse().ok()),
		})
	}
}

// }}}
