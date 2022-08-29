// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, LitInt, Token};

/// `name: Ident` (`<` `length: Length` `>`)
///
/// ```rust
/// Title<0>
/// ```
pub(crate) struct ReplyTitle {
	name: Ident,
	length: u32,
}

/// `<` `value: u32` `>`
///
/// ```rust
/// <0>
/// ```
pub(crate) struct ReplyLength {
	value: Option<u32>,
}

impl Parse for ReplyTitle {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			name: input.parse()?,
			length: input
				.parse::<ReplyLength>()
				.map_or(1, |len| len.value.unwrap_or(1)),
		})
	}
}

impl Parse for ReplyLength {
	fn parse(input: ParseStream) -> Result<Self> {
		input.parse::<Token![<]>()?; // <
		let value: Result<LitInt> = input.parse(); // 0
		input.parse::<Token![>]>()?; // >

		Ok(Self {
			value: value.map_or(None, |_| value.unwrap().base10_parse::<u32>().ok()),
		})
	}
}
