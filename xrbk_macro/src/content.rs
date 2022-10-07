// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::{
	braced, bracketed, parenthesized,
	parse::{discouraged::Speculative, Parse, ParseStream, Result},
	punctuated::Punctuated,
	token, Attribute, Expr, Ident, Token, Type, Visibility,
};

pub struct Variant {
	pub attributes: Vec<Attribute>,
	pub name: Ident,
	pub items: Option<(token::Paren, Punctuated<Item, Token![,]>)>,
}

pub enum Item {
	Field(Field),
	Unused(Unused),
	Let(LetItem),
}

pub struct Field {
	pub attributes: Vec<Attribute>,
	pub vis: Visibility,
	pub named: Option<(Ident, Token![:])>,
	pub r#type: Ty,
}

pub enum Unused {
	Single(token::Paren),
	Full(FullUnused),
}

pub struct FullUnused {
	pub bracket_token: token::Bracket,
	pub unit_token: token::Paren,
	pub semicolon_token: Token![;],
	pub count: UnusedCount,
}

pub enum UnusedCount {
	Source(Source),
	Infer(Token![..]),
}

pub struct LetItem {
	pub let_token: Token![let],
	pub name: Ident,
	pub r#type: Option<(Token![:], Type)>,
	pub eq_token: Token![=],
	pub source: Source,
}

pub enum Ty {
	Type(Type),
	ContextualType(ContextualType),
}

// Ex.
// ```
// (&'a [u8], fn list_len)
// ```
pub struct ContextualType {
	pub paren_token: token::Paren,
	pub r#type: Type,
	pub comma_token: Token![,],
	pub source: Source,
}

pub enum Function {
	ShortFunction(ShortFunction),
	LongFunction(LongFunction),
}

// Ex.
// ```
// fn list_len
// ```
pub struct ShortFunction {
	pub fn_token: Token![fn],
	pub ident: Ident,
}

// Ex.
// ```
// fn(list_len) { list_len }
// ```
pub struct LongFunction {
	pub fn_token: Token![fn],
	pub paren_token: token::Paren,
	pub idents: Punctuated<Ident, Token![,]>,
	pub brace_token: token::Brace,
	pub content: Expr,
}

// Ex.
// ```
// fn(list_len) { list_len }
// ```
// or
// ```
// 3
// ```
pub enum Source {
	Function(Function),
	Expr(Expr),
}

// Parsing {{{

impl Parse for Item {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(Token![let]) {
			Self::Let(input.parse()?)
		} else if input.peek(token::Bracket) || input.peek(token::Paren) {
			Self::Unused(input.parse()?)
		} else {
			Self::Field(input.parse()?)
		})
	}
}

impl Parse for Field {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			attributes: input.call(Attribute::parse_outer)?,
			vis: input.parse()?,
			// TODO: Do the `?` operators here return `None` for `named` or
			// `Err` for `parse`?
			named: { Some((input.parse()?, input.parse()?)) },
			r#type: input.parse()?,
		})
	}
}

impl Parse for Unused {
	fn parse(input: ParseStream) -> Result<Self> {
		let look = input.lookahead1();
		let unit;

		if look.peek(token::Paren) {
			Ok(Self::Single(parenthesized!(unit in input)))
		} else if look.peek(token::Bracket) {
			let content;

			Ok(Self::Full(FullUnused {
				bracket_token: bracketed!(content in input),
				unit_token: parenthesized!(unit in input),
				semicolon_token: input.parse()?,
				count: input.parse()?,
			}))
		} else {
			Err(look.error())
		}
	}
}

impl Parse for FullUnused {
	fn parse(input: ParseStream) -> Result<Self> {
		let (content, unit);

		Ok(Self {
			bracket_token: bracketed!(content in input),
			unit_token: parenthesized!(unit in input),
			semicolon_token: input.parse()?,
			count: input.parse()?,
		})
	}
}

impl Parse for UnusedCount {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(Token![..]) {
			Self::Infer(input.parse()?)
		} else {
			Self::Source(input.parse()?)
		})
	}
}

impl Parse for LetItem {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			let_token: input.parse()?,
			name: input.parse()?,
			// TODO: Do the `?` operators here return `None` for `r#type` or
			// `Err` for `parse`?
			r#type: { Some((input.parse()?, input.parse()?)) },
			eq_token: input.parse()?,
			source: input.parse()?,
		})
	}
}

impl Parse for Ty {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if !input.peek(token::Paren) {
			Self::Type(input.parse()?)
		} else {
			let ahead = input.fork();
			let content;

			let paren_token = parenthesized!(content in ahead);
			let r#type: Result<Type> = content.parse();

			if r#type.is_err() || !content.peek(Token![,]) {
				Self::Type(input.parse()?)
			} else {
				let r#type = r#type?;
				let comma_token: Token![,] = content.parse()?;
				let expr: Result<Expr> = content.parse();

				if expr.is_ok() || content.peek(Token![fn]) {
					input.advance_to(&ahead);

					Self::ContextualType(ContextualType {
						paren_token,
						r#type,
						comma_token,
						source: if expr.is_ok() {
							Source::Expr(expr?)
						} else {
							Source::Function(content.parse()?)
						},
					})
				} else {
					Self::Type(input.parse()?)
				}
			}
		})
	}
}

impl Parse for ContextualType {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		Ok(Self {
			paren_token: parenthesized!(content in input),
			r#type: content.parse()?,
			comma_token: content.parse()?,
			source: content.parse()?,
		})
	}
}

impl Parse for Function {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(token::Paren) {
			Self::LongFunction(input.parse()?)
		} else {
			Self::ShortFunction(input.parse()?)
		})
	}
}

impl Parse for ShortFunction {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			fn_token: input.parse()?,
			ident: input.parse()?,
		})
	}
}

impl Parse for LongFunction {
	fn parse(input: ParseStream) -> Result<Self> {
		let (content, body);

		Ok(Self {
			fn_token: input.parse()?,
			paren_token: parenthesized!(content in input),
			idents: content.parse_terminated(Ident::parse)?,
			brace_token: braced!(body in input),
			content: body.parse()?,
		})
	}
}

impl Parse for Source {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(Token![fn]) {
			Self::Function(input.parse()?)
		} else {
			Self::Expr(input.parse()?)
		})
	}
}

// }}}
