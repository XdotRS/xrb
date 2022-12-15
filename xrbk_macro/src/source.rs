// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod expansion;
mod parsing;

use std::collections::HashMap;
use syn::{punctuated::Punctuated, Expr, Ident, Token, Type};

pub type IdentMap<'a> = &'a HashMap<String, Type>;
pub type IdentMapMut<'a> = &'a mut HashMap<String, Type>;

pub struct Arg {
	pub ident: Ident,
	pub r#type: Option<Type>,

	pub formatted_ident: Ident,
}

pub struct LengthArg {
	pub self_token: Token![self],
	pub double_colon_token: Token![::],
	pub length_token: Ident,

	pub formatted_length_token: Ident,
}

pub struct Args {
	pub args: Punctuated<Arg, Token![,]>,
	pub length_arg: Option<(LengthArg, Type)>,
}

pub struct Source {
	pub args: Option<(Args, Token![=>])>,
	pub expr: Expr,
}
