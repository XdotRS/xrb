// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::{
	braced, bracketed, parenthesized,
	parse::{discouraged::Speculative, Parse, ParseStream, Result},
	punctuated::{Pair, Punctuated},
	spanned::Spanned,
	token, Error, Expr, Ident, Path, Token, Type, Visibility,
};

use quote::{ToTokens, TokenStreamExt};

use proc_macro2::{Delimiter, Group, Span, TokenStream as TokenStream2};

enum Params {
	None(Token![_]),
	Some(Punctuated<Ident, Token![,]>),
}

pub struct Attribute {
	pub hash_token: Token![#],
	pub bracket_token: token::Bracket,
	pub path: Path,
	pub tokens: TokenStream2,
}

impl Attribute {
	pub fn is_context(&self) -> bool {
		self.path.is_ident("context")
	}
}

impl ToTokens for Attribute {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Since the context attribute isn't actually going to replace the item,
		// it isn't a real attribute, so we don't want to actually write it as
		// one.
		if !self.is_context() {
			// `#`.
			self.hash_token.to_tokens(tokens);

			// e.g. `[derive(Debug)]`.
			self.bracket_token.surround(tokens, |tokens| {
				// e.g. `derive`.
				self.path.to_tokens(tokens);
				// e.g. `(Debug)`.
				self.tokens.to_tokens(tokens);
			});
		}
	}
}

/// An enum variant that uses [`Items`].
pub struct Variant {
	/// Attributes associated with the enum variant.
	pub attributes: Vec<Attribute>,

	/// The name of the enum variant.
	pub name: Ident,

	/// Items associated with the variant, if any (i.e. a variant struct/struct
	/// tuple).
	pub items: Items,
}

impl ToTokens for Variant {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Convert the attributes to tokens.
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// Convert the name to tokens.
		self.name.to_tokens(tokens);
		// Convert the items (and surrounding delimiters) to tokens.
		self.items.to_tokens(tokens);
	}
}

pub enum Items {
	/// [`Item`]s, but no unnamed fields are allowed.
	///
	/// Surrounded by curly brackets (`{` and `}`).
	Named((token::Brace, Punctuated<Item, Token![,]>)),
	/// [`Item`]s, but no named fields are allowed.
	///
	/// Surrounded by normal brackets (`(` and `)`).
	Unnamed((token::Paren, Punctuated<Item, Token![,]>)),
	/// Neither items nor delimiters (`{}` nor `()`).
	Unit,
}

impl ToTokens for Items {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			// If these are named items, surround them with curly brackets.
			Self::Named((brace, items)) => {
				brace.surround(tokens, |tokens| items.to_tokens(tokens));
			}

			// If these are unnamed items, surround them with normal brackets.
			Self::Unnamed((paren, items)) => {
				paren.surround(tokens, |tokens| items.to_tokens(tokens));
			}

			// If there are no items, append no tokens.
			Self::Unit => (),
		}
	}
}

/// A content item for the [`derive!`] macro syntax.
///
/// [`derive!`]: crate::derive
pub enum Item {
	/// A field item with similar syntax to normal struct fields.
	Field(Field),
	/// An item representing a number of unused bytes that will be skipped.
	Unused(Unused),
	/// An item representing data to be read and written that is not a field.
	Let(LetItem),
}

impl ToTokens for Item {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// If `self` is a field, convert the field to tokens. Other items won't
		// be converted to tokens because they are not normal Rust syntax.
		if let Self::Field(field) = self {
			field.to_tokens(tokens);
		}
	}
}

/// A field item with similar syntax to normal struct fields.
pub struct Field {
	/// Attributes associated with the field.
	pub attributes: Vec<Attribute>,
	/// The visibility of the field.
	pub vis: Visibility,

	/// The name of the field, followed by a colon before the type.
	///
	/// This will be present if this is a named field, but absent if it is an
	/// unnamed field.
	pub named: Option<(Ident, Token![:])>,

	/// The field's type, or a 'contextual type' that can provide extra context
	/// for its deserialization.
	pub r#type: Ty,
}

impl ToTokens for Field {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Convert each attribute to tokens:
		for attribute in &self.attributes {
			attribute.to_tokens(tokens);
		}

		// Convert the field's visibility to tokens:
		self.vis.to_tokens(tokens);
		// If there is a name (and colon), convert them to tokens:
		self.named.as_ref().map(|(name, colon)| {
			name.to_tokens(tokens);
			colon.to_tokens(tokens);
		});
		// Convert the field's type to tokens:
		self.r#type.to_tokens(tokens);
	}
}

/// Represents unused bytes that will be skipped when reading and writing.
pub enum Unused {
	/// A single unused byte, represented by `()`.
	Single(token::Paren),
	/// A full unused bytes definition that can provide a count.
	Full(FullUnused),
}

/// A full unused bytes definition that can provide a count.
pub struct FullUnused {
	/// A pair of square bracket tokens: `[` and `]`.
	pub bracket_token: token::Bracket,

	/// A 'unit' token (a pair of normal brackets): `()`.
	pub unit_token: token::Paren,
	/// A semicolon token: `;`.
	pub semicolon_token: Token![;],

	/// The number of unused bytes; can be inferred, or can be a [`Source`].
	pub count: UnusedCount,
}

/// The number of unused bytes; can be inferred, or can be a [`Source`].
pub enum UnusedCount {
	/// Use a specific [`Source`] to get the number of unused bytes.
	Source(Source),

	/// Infer the number of unused bytes.
	///
	/// This will calculate an appropriate
	/// number of bytes to pad until the minimum length if this is the last
	/// item, otherwise the number of bytes to meet the next 4-byte boundary, if
	/// one is not already met.
	Infer(Token![..]),
}

/// An [`Item`] for data that is read and written but isn't a standalone field.
///
/// For example, this is commonly used for the length of a list.
pub struct LetItem {
	/// The let token: `let`.
	pub let_token: Token![let],

	/// The name of the variable when reading it.
	pub name: Ident,
	/// An optional explicit [`Type`], preceded by a colon.
	pub r#type: Option<(Token![:], Type)>,

	/// The equals token: `=`.
	pub eq_token: Token![=],

	/// The [`Source`] used to write the data.
	pub source: Source,
}

/// Either a normal [`Type`] or a [`ContextualType`] that gives context to read
/// the type.
pub enum Ty {
	Type(Type),
	ContextualType(ContextualType),
}

impl Ty {
	/// Returns the span of the type.
	///
	/// If this is a normal [`Type`], it will simply be that [`Type`]'s span. If
	/// it is a [`ContextualType`], it will simply be the span of the [`Type`]
	/// contained within it.
	// TODO: The span of the [`ContextualType`] should cover the whole thing.
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
			// If `self` is a normal type, just convert it to tokens.
			Ty::Type(r#type) => {
				r#type.to_tokens(tokens);
			}

			// If `self is a contextual type (context for how to deserialize
			// it), just convert the actual type itself to tokens.
			Ty::ContextualType(ctx_type) => {
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
	/// A pair of normal brackets surrounding the contextual type: `(` and `)`.
	pub paren_token: token::Paren,

	/// The type itself.
	pub r#type: Type,

	/// The comma token: `,`.
	pub comma_token: Token![,],

	/// The [`Source`] used to write the type.
	pub source: Source,
}

// Ex.
// ```
// fn(list_len) list_len
// ```
/// A function that can take zero or more [`Field`] or [`LetItem`] identifiers
/// and have a full expression body.
pub struct Function {
	/// The function token: `fn`.
	pub fn_token: Token![fn],

	/// A pair of normal brackets (`(` and `)`).
	pub paren_token: token::Paren,
	/// [`Field`] and/or [`LetItem`] identifiers, separated by commas, to use in
	/// the function.
	pub params: Punctuated<(Ident, Token![:], Type), Token![,]>,

	/// The content of the function.
	pub content: Expr,
}

impl Function {
	#[allow(dead_code)]
	fn to_tokens(&self, name: Ident, tokens: &mut TokenStream2) {
		// `fn`.
		self.fn_token.to_tokens(tokens);
		// The name of the function.
		name.to_tokens(tokens);

		// Apparently `ToTokens` is not implemented for tuples with elements
		// that implement `ToTokens`...
		let mut param_tokens = TokenStream2::new();
		for pair in self.params.pairs() {
			match pair {
				Pair::Punctuated((ident, colon, r#type), comma) => {
					ident.to_tokens(&mut param_tokens);
					colon.to_tokens(&mut param_tokens);
					r#type.to_tokens(&mut param_tokens);

					comma.to_tokens(&mut param_tokens);
				}

				Pair::End((ident, colon, r#type)) => {
					ident.to_tokens(&mut param_tokens);
					colon.to_tokens(&mut param_tokens);
					r#type.to_tokens(&mut param_tokens);
				}
			}
		}

		// The function parameters.
		tokens.append(Group::new(Delimiter::Parenthesis, param_tokens));
		// The function's 'content' expression.
		tokens.append(Group::new(Delimiter::Brace, self.content.to_token_stream()));
	}
}

// Ex.
// ```
// fn(list_len) list_len
// ```
// or
// ```
// 3
// ```
/// Either a [`Function`] or an [expression].
///
/// [expression]: Expr
pub enum Source {
	Function(Function),
	Expr(Expr),
}

// Parsing {{{

impl Parse for Params {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(Token![_]) {
			Ok(Self::None(input.parse()?))
		} else {
			Ok(Self::Some(input.parse_terminated(Ident::parse)?))
		}
	}
}

impl Attribute {
	#[allow(dead_code)]
	fn parse_context(self) -> Result<Context> {
		if !self.is_context() {
			return Err(Error::new(self.path.span(), "expected `context` attribute"));
		}

		Ok(syn::parse2(self.tokens)?)
	}
}

struct Context(Params, Token![->], Expr);

impl Parse for Context {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Context(input.parse()?, input.parse()?, input.parse()?))
	}
}

impl Parse for Attribute {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		Ok(Self {
			hash_token: input.parse()?,
			bracket_token: bracketed!(content in input),
			path: input.parse()?,
			tokens: content.parse()?,
		})
	}
}

impl Attribute {
	fn parse_outer(input: ParseStream) -> Result<Vec<Self>> {
		let mut attributes = vec![];

		while input.peek(Token![#]) && input.peek2(token::Bracket) {
			attributes.push(input.parse()?);
		}

		Ok(attributes)
	}
}

impl Parse for Variant {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// Attributes associated with the variant.
			attributes: input.call(Attribute::parse_outer)?,
			// The name of the variant.
			name: input.parse()?,
			// Items associated with the variant, if any.
			items: input.parse()?,
		})
	}
}

impl Parse for Items {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		Ok(if input.peek(token::Brace) {
			// If the next token is a curly bracket, then these items are meant
			// to have named fields.
			Self::Named((
				braced!(content in input),
				content.parse_terminated(Item::parse_named)?,
			))
		} else if input.peek(token::Paren) {
			// Otherwise, if the next token is a normal bracket, then these
			// items are meant to have unnamed fields.
			Self::Unnamed((
				parenthesized!(content in input),
				content.parse_terminated(Item::parse_unnamed)?,
			))
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

		// If `this` is a field...
		if let Self::Field(field) = &this {
			// Check if it has no name.
			if field.named.is_none() {
				// If so, generate this error with the span of the field
				// type (which is where the name is meant to go).
				return Err(Error::new(field.r#type.span(), "expected field name"));
			}
		}

		Ok(this)
	}

	/// Parses an item, but generates an error if it was a field with a name.
	#[allow(dead_code)]
	fn parse_unnamed(input: ParseStream) -> Result<Self> {
		let this = Self::parse(input)?;

		// If `this` is a field...
		if let Self::Field(field) = &this {
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

		Ok(this)
	}

	/// Whether `self` is either not a field, or is a field with a name.
	#[allow(dead_code)]
	fn is_named(&self) -> bool {
		if let Self::Field(field) = self {
			field.named.is_some()
		} else {
			true
		}
	}

	/// Whether `self` is a [`Field`].
	#[allow(dead_code)]
	fn is_field(&self) -> bool {
		matches!(self, Self::Field(_))
	}

	/// Whether `self` is an [`Unused`] item.
	#[allow(dead_code)]
	fn is_unused(&self) -> bool {
		matches!(self, Self::Unused(_))
	}

	/// Whether `self` is a [`LetItem`].
	#[allow(dead_code)]
	fn is_let(&self) -> bool {
		matches!(self, Self::Let(_))
	}
}

impl Parse for Field {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// Attributes associated with the field.
			attributes: input.call(Attribute::parse_outer)?,
			// The visibility of the field.
			vis: input.parse()?,
			// The name of the field, followd by a colon, if this is a named
			// field.
			named: if input.peek(Ident) && input.peek2(Token![:]) {
				Some((input.parse()?, input.parse()?))
			} else {
				None
			},
			// The field's type.
			r#type: input.parse()?,
		})
	}
}

impl Parse for Unused {
	fn parse(input: ParseStream) -> Result<Self> {
		let look = input.lookahead1();
		// We have to use a variable in [`parenthesized!`], but this will
		// contain no tokens, because we are looking for an empty pair of normal
		// brackets.
		let _unit;

		if look.peek(token::Paren) {
			// If the next token is a unit token (`()`), then this is\
			// [`Self::Single`].
			Ok(Self::Single(parenthesized!(_unit in input)))
		} else if look.peek(token::Bracket) {
			// Otherwise, this is full definition.
			Ok(Self::Full(input.parse()?))
		} else {
			// Otherwise, if neither a normal bracket nor a square bracket is
			// found, then we generate an error with an error message supplied
			// by `look`.
			Err(look.error())
		}
	}
}

impl Parse for FullUnused {
	fn parse(input: ParseStream) -> Result<Self> {
		let (content, _unit);

		Ok(Self {
			// `[` and `]`.
			bracket_token: bracketed!(content in input),
			// `()`.
			unit_token: parenthesized!(_unit in content),
			// `;`.
			semicolon_token: content.parse()?,
			// The number of unused bytes.
			count: content.parse()?,
		})
	}
}

impl Parse for UnusedCount {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(Token![..]) {
			// If the next token is `..`, that indicates the number of unused
			// bytes should be inferred.
			Self::Infer(input.parse()?)
		} else {
			// Otherwise, we parse a [`Source`] for the number of unused bytes.
			Self::Source(input.parse()?)
		})
	}
}

impl Parse for LetItem {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// The let token: `let`.
			let_token: input.parse()?,
			// The name of the variable when reading.
			name: input.parse()?,
			// The type, if specified.
			r#type: input
				// This uses `.call` with a closure so that the `?` operator
				// doesn't return from the `parse` function.
				//
				// TODO: actually, this shouldn't ignore the type if there was
				// an error parsing it: it should only ignore it if no type was
				// given.
				.call(|input| Ok((input.parse()?, input.parse()?)))
				.ok(),
			// The equals token: `=`.
			eq_token: input.parse()?,
			// The source used to write the let item.
			source: input.parse()?,
		})
	}
}

impl Parse for Ty {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if !input.peek(token::Paren) {
			// If the next token is not a normal bracket, then this cannot be a
			// [`ContextualType`]: we can simply parse a normal [`Type`].
			Self::Type(input.parse()?)
		} else {
			// Otherwise, it is a bit more complicated: this could still be a
			// normal type, such as a tuple.

			// We fork the `ParseStream` so that we can revert to the current
			// position if we find out that this isn't a [`ContextualType`].
			let ahead = input.fork();

			let content;
			// Parse a normal bracket token in the forked stream.
			let paren_token = parenthesized!(content in ahead);
			// Parse a type in the forked stream, within the brackets. We only
			// store the result because if this was a unit type (`()`), then
			// it would not contain valid [`Type`].
			let r#type: Result<Type> = content.parse();

			if r#type.is_err() || !content.peek(Token![,]) {
				// If the type did not parse, or there is not a token following
				// the type, then we know it cannot be a [`ContextualType`].
				Self::Type(input.parse()?)
			} else {
				// Otherwise, if the type did parse and there is a comma
				// following it, then even if it is a [`Type`] it must have
				// successfully parsed, so we 'unwrap' the type from the
				// [`Result`].
				let r#type = r#type?;
				// Since we know there is a comma token next, we can parse that
				// too.
				let comma_token: Token![,] = content.parse()?;

				let type2: Result<Type> = content.parse();
				// Next, we attempt to parse an expression. We know that if this
				// parses correctly, it is a [`ContextualType`]*.
				//
				// *actually, unit structs or the unit type would also be valid
				// expressions... TODO
				let expr: Result<Expr> = content.parse();

				if !type2.is_ok() && expr.is_ok() || content.peek(Token![fn]) {
					// If the expression was parsed correctly, or the next token
					// is `fn`, then this is a [`ContextualType`] with a
					// [`Source`].

					// Since we know this is a [`ContextualType`] now, we don't
					// need to be working in a fork anymore. We can advance
					// `input` to the position of the fork.
					input.advance_to(&ahead);

					Self::ContextualType(ContextualType {
						paren_token,
						r#type,
						comma_token,
						// If the expression was parsed correctly, then this is
						// a [`Source::Expr`], otherwise this is a
						// [`Source::Function`].
						source: if expr.is_ok() {
							Source::Expr(expr?)
						} else {
							Source::Function(content.parse()?)
						},
					})
				} else {
					// Otherwise, if the expression was not parsed correctly and
					// there is not a `fn` token next, then this is just a
					// normal [`Type`].
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
		let content;

		Ok(Self {
			// `fn`.
			fn_token: input.parse()?,
			// A pair of normal brackets: `(` and `)`.
			paren_token: parenthesized!(content in input),
			// Function parameters (Ident, Token![:], Type).
			params: content
				.parse_terminated(|input| Ok((input.parse()?, input.parse()?, input.parse()?)))?,
			// The function's expression.
			content: content.parse()?,
		})
	}
}

impl Parse for Source {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(Token![fn]) {
			// If the next token is `fn`, then this is a [`Function`].
			Self::Function(input.parse()?)
		} else {
			// Otherwise, this is an expression.
			Self::Expr(input.parse()?)
		})
	}
}

// }}}
