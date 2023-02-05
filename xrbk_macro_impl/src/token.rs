// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::error::ExpectedButFound;
use chumsky::prelude::*;
use proc_macro2::{Ident, TokenTree};

pub mod delimiter;
pub mod punct;

pub fn ident() -> impl Parser<TokenTree, Ident, Error = ExpectedButFound<TokenTree>> {
	filter_map(|span, token| match token {
		TokenTree::Ident(ident) => Ok(ident),

		_ => Err(
			ExpectedButFound::expected_input_found(span, [], Some(token))
				.with_label("expected identifier".into()),
		),
	})
}
