// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{Result, Token, Type};

/// Information specifically associated with replies, not requests.
#[derive(Clone)]
pub struct ReplyMetadata {
	/// The request which this reply is returned for.
	pub request: (Token![for], Type),
}

// Parsing {{{

impl Parse for ReplyMetadata {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// `for` + request type.
			request: (input.parse()?, input.parse()?),
		})
	}
}

// }}}
