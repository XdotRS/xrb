// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, LitInt, Token};

/// `name: Ident` (`<` `length: Length` `>`)
///
/// ```rust
/// Title    // length: 1
/// Title<>  // length: 1
/// Title<1> // length: 1
/// Title<4> // length: 4
/// ```
pub(crate) struct RequestTitle {
	name: Ident,
	length: u16,
}

impl RequestTitle {
	/// The name of this request.
	pub fn name(self) -> Ident {
		self.name
	}

	/// The extra length of this request. Optional: defaults to 1.
	pub fn length(self) -> u16 {
		self.length
	}
}

/// `<` `value: u32` `>`
///
/// ```rust
/// <>
/// <1>
/// ```
pub(crate) struct RequestLength {
	value: Option<u16>,
}

impl Parse for RequestTitle {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			name: input.parse()?,
			length: input
				.parse::<RequestLength>()
				.map_or(1, |len| len.value.unwrap_or(1)),
		})
	}
}

impl Parse for RequestLength {
	fn parse(input: ParseStream) -> Result<Self> {
		input.parse::<Token![<]>()?; // <
		let value: Result<LitInt> = input.parse(); // 0
		input.parse::<Token![>]>()?; // >

		Ok(Self {
			value: value.map_or(None, |_| value.unwrap().base10_parse::<u16>().ok()),
		})
	}
}
