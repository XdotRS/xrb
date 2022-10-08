// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::{
	braced, bracketed, parenthesized,
	parse::{discouraged::Speculative, Parse, ParseStream, Result},
	punctuated::Punctuated,
	spanned::Spanned,
	token, Attribute, Error, Expr, Ident, Token, Type, Visibility,
};

use quote::{ToTokens, TokenStreamExt};

use proc_macro2::{Delimiter, Group, Span, TokenStream as TokenStream2};

pub struct Variant {
	pub attributes: Vec<Attribute>,
	pub name: Ident,
	pub items: Items,
}

impl ToTokens for Variant {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		self.name.to_tokens(tokens);
		self.items.to_tokens(tokens);
	}
}

pub enum Items {
	Named(Punctuated<Item, Token![,]>),
	Unnamed(Punctuated<Item, Token![,]>),
	Unit,
}

impl ToTokens for Items {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			// If these are named items, surround them with curly brackets.
			Self::Named(items) => {
				tokens.append(Group::new(Delimiter::Brace, items.to_token_stream()));
			}
			// If these are unnamed items, surround them with normal brackets.
			Self::Unnamed(items) => {
				tokens.append(Group::new(Delimiter::Parenthesis, items.to_token_stream()));
			}
			// If there are no items, append no tokens.
			Self::Unit => (),
		}
	}
}

pub enum Item {
	Field(Field),
	Unused(Unused),
	Let(LetItem),
}

impl ToTokens for Item {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// If `self` is a field, convert the field to tokens. Other items won't
		// be converted to tokens because they are not normal Rust syntax.
		match self {
			Self::Field(field) => field.to_tokens(tokens),
			_ => (),
		}
	}
}

pub struct Field {
	pub attributes: Vec<Attribute>,
	pub vis: Visibility,
	pub named: Option<(Ident, Token![:])>,
	pub r#type: Ty,
}

impl ToTokens for Field {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Convert each attribute to tokens:
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		self.vis.to_tokens(tokens);
		// If there is a name (and colon), convert them to tokens:
		self.named.as_ref().map(|(name, colon)| {
			name.to_tokens(tokens);
			colon.to_tokens(tokens);
		});
		self.r#type.to_tokens(tokens);
	}
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

impl Ty {
	fn span(&self) -> Span {
		match self {
			Self::Type(r#type) => r#type.span(),
			Self::ContextualType(r#type) => r#type.r#type.span(),
		}
	}
}

impl ToTokens for Ty {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Ty::Type(r#type) => {
				// If `self` is a normal type, just convert it to tokens.
				r#type.to_tokens(tokens);
			}
			Ty::ContextualType(ctx_type) => {
				// If `self is a contextual type (context for how to deserialize
				// it), just convert the actual type itself to tokens.
				ctx_type.r#type.to_tokens(tokens);
			}
		}
	}
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

impl Parse for Variant {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			attributes: input.call(Attribute::parse_outer)?,
			name: input.parse()?,
			items: input.parse()?,
		})
	}
}

impl Parse for Items {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(token::Brace) {
			// If the next token is a curly bracket, then these items are meant
			// to have named fields.
			Self::Named(input.parse_terminated(Item::parse_named)?)
		} else if input.peek(token::Paren) {
			// Otherwise, if the next token is a normal bracket, then these
			// items are meant to have unnamed fields.
			Self::Unnamed(input.parse_terminated(Item::parse_unnamed)?)
		} else {
			// Otherwise, if there is no curly bracket or normal bracket, then
			// there are no items expected; this state is known as _unit_ (like
			// `()`).
			Self::Unit
		})
	}
}

impl Parse for Item {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(Token![let]) {
			// If the next token is `let`, then this is a let item.
			Self::Let(input.parse()?)
		} else if input.peek(token::Bracket) || input.peek(token::Paren) {
			// If the next token is a square bracket or a normal bracket, then
			// this is an unused bytes item.
			Self::Unused(input.parse()?)
		} else {
			// Otherwise, this is a field.
			Self::Field(input.parse()?)
		})
	}
}

impl Item {
	/// Parses an item, but generates an error if it was a field without a name.
	#[allow(dead_code)]
	fn parse_named(input: ParseStream) -> Result<Self> {
		let this = Self::parse(input)?;

		match &this {
			// If `this` is a field...
			Self::Field(field) => {
				// Check if it has no name.
				if field.named.is_none() {
					// If so, generate this error with the span of the field
					// type (which is where the name is meant to go).
					return Err(Error::new(field.r#type.span(), "expected field name"));
				}
			}
			_ => {}
		}

		Ok(this)
	}

	/// Parses an item, but generates an error if it was a field with a name.
	#[allow(dead_code)]
	fn parse_unnamed(input: ParseStream) -> Result<Self> {
		let this = Self::parse(input)?;

		match &this {
			// If `this` is a field...
			Self::Field(field) => {
				// Check if it has a name.
				if field.named.is_some() {
					// If so, reference the name (the `Ident`)...
					let (name, _) = field.named.as_ref().expect("we already checked for this");

					// ...so we can use its span to generate this error:
					return Err(Error::new(
						name.span(),
						"no field name expected, this is a tuple body",
					));
				}
			}
			_ => {}
		};

		Ok(this)
	}

	/// Whether `self` is either not a field, or is a field with a name.
	#[allow(dead_code)]
	fn is_named(&self) -> bool {
		match self {
			Self::Field(field) => field.named.is_some(),
			_ => true,
		}
	}

	/// Whether `self` is a [`Field`].
	#[allow(dead_code)]
	fn is_field(&self) -> bool {
		match self {
			Self::Field(_) => true,
			_ => false,
		}
	}

	/// Whether `self` is an [`Unused`] item.
	#[allow(dead_code)]
	fn is_unused(&self) -> bool {
		match self {
			Self::Unused(_) => true,
			_ => false,
		}
	}

	/// Whether `self` is a [`LetItem`].
	#[allow(dead_code)]
	fn is_let(&self) -> bool {
		match self {
			Self::Let(_) => true,
			_ => false,
		}
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
		let _unit;

		if look.peek(token::Paren) {
			Ok(Self::Single(parenthesized!(_unit in input)))
		} else if look.peek(token::Bracket) {
			let content;

			Ok(Self::Full(FullUnused {
				bracket_token: bracketed!(content in input),
				unit_token: parenthesized!(_unit in content),
				semicolon_token: content.parse()?,
				count: content.parse()?,
			}))
		} else {
			Err(look.error())
		}
	}
}

impl Parse for FullUnused {
	fn parse(input: ParseStream) -> Result<Self> {
		let (content, _unit);

		Ok(Self {
			bracket_token: bracketed!(content in input),
			unit_token: parenthesized!(_unit in content),
			semicolon_token: content.parse()?,
			count: content.parse()?,
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
