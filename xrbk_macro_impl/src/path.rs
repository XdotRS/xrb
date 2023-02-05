// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::Punct;

use crate::{error::ExpectedButFound, token::ident};
use chumsky::prelude::*;
use proc_macro2::{Ident, TokenTree};

pub struct SimplePath {
	pub double_colon: Option<Punct![::]>,
	pub ident: Ident,
	pub segments: Vec<(Punct![::], Ident)>,
}

impl SimplePath {
	pub fn parser() -> impl Parser<TokenTree, SimplePath, Error = ExpectedButFound<TokenTree>> {
		<Punct![::]>::parser()
			.or_not()
			.then(ident())
			.then(<Punct![::]>::parser().then(ident()).repeated())
			.map(|((double_colon, ident), segments)| SimplePath {
				double_colon,
				ident,
				segments,
			})
	}
}
