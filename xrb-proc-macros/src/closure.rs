// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::{
	parse::{discouraged::Speculative, Parse, ParseStream, Result},
	punctuated::Punctuated,
	Expr, Ident, ReturnType, Token,
};

/// # Examples
/// ```
/// let data_len: u8 = || self.data.len() as u8
/// ```
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct LetVar {
	/// A let token: `let`.
	pub let_token: Token![let],
	/// The name of the `LetVar`.
	pub name: Ident,
	/// An optional colon token to specify the type: `:`.
	pub colon_token: Option<Token![:]>,
	/// The type of the `LetVar`; optional.
	pub ty: Option<Type>,
	/// An equals token: `=`.
	pub eq_token: Token![=],
	/// The [`IdentClosure`] that defines how this `LetVar` is written.
	pub closure: IdentClosure,
}

/// A closure that takes identifiers as input, rather than patterns.
///
/// # Examples
/// ```
/// define! {
///     pub struct ChangeProperty<'a, T>: Request<18> {
///         pub %mode: ChangePropertyMode,
///         pub window: Window,
///         pub property: Atom,
///         pub property_type: Atom,
///         // `IdentClosure`
///         let format: u8 = || T::data_size() as u8,
///         // `IdentClosure`
///         [(); || 3],
///         // `IdentClosure`
///         let data_len: u32 = || self.data.len() as u32,
///         // `IdentClosure`
///         pub data: <&'a [T]; |data_len| data_len>,
///         // `IdentClosure`
///         [(); |data| pad(data)],
///     }
/// }
/// ```
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct IdentClosure {
	/// The start of the closure; an or token: `|`.
	pub or_token1: Token![|],
	/// A list of already-defined identifiers (field names or data variables)
	/// for use in the `body`.
	pub ident_params: Punctuated<Ident, Token![,]>,
	/// The end of a closure's parameters; an or token: `|`.
	pub or_token2: Token![|],
	/// The return type of the closure.
	pub output: ReturnType,
	/// The closure's body expression.
	pub body: Box<Expr>,
}

/// # Examples
/// ```
/// define! {
///     pub struct ChangeProperty<'a, T>: Request<18> {
///         pub #mode: ChangePropertyMode,
///         pub window: Window,
///         pub property: Atom,
///         pub property_type: Atom,
///         let format: u8 = || T::data_size() as u8,
///         [(); || 3],
///         let data_len: u32 = || self.data.len() as u32,
///         // `TypeClosure`
///         pub data: <&'a [T]; |data_len| data_len>,
///         [(); |data| pad(data)],
///     }
/// }
/// ```
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct TypeClosure {
	/// The start of the `TypeClosure` expression; a left arrow bracket token: `<`.
	pub lt_token: Token![<],
	/// The type half of the `TypeClosure` expression.
	pub ty: syn::Type,
	/// A semicolon token: `;`.
	pub semicolon_token: Token![;],
	/// The closure half of the `TypeClosure` expression.
	///
	/// This gives additional context to the type, such as information used when
	/// reading a value.
	pub closure: Box<IdentClosure>,
	/// The end of the `TypeClosure` expression; a right arrow bracket token: `>`.
	pub gt_token: Token![>],
}

/// A wrapper that can be a [`Type`] or a [`TypeClosure`].
///
/// [`Type`]: syn::Type
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Type {
	/// A [`Type`].
	///
	/// [`Type`]: syn::Type
	Type(syn::Type),
	/// A [`TypeClosure`] that can provide additional context for reading the
	/// type.
	TypeClosure(TypeClosure),
}

// Parsing {{{

impl Parse for LetVar {
	fn parse(input: ParseStream) -> Result<Self> {
		// Let token: `let`.
		let let_token: Token![let] = input.parse()?;
		// The name of the `LetVar`.
		let name: Ident = input.parse()?;
		// A colon token, if provided: `:`.
		let colon_token: Option<Token![:]> = input.parse().ok();
		// A type, if a colon was found.
		let ty: Option<Type> = colon_token.and_then(|_| input.parse().ok());

		// If a colon token is provided but a type is not, return an error.
		if colon_token.is_some() && !ty.is_some() {
			return Err(input.error("expected type after colon"));
		}

		let eq_token: Token![=] = input.parse()?;
		let closure: IdentClosure = input.parse()?;

		Ok(Self {
			let_token,
			name,
			colon_token,
			ty,
			eq_token,
			closure,
		})
	}
}

impl Parse for IdentClosure {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// Or token: `|`.
			or_token1: input.parse()?,
			// Identifer parameters to the closure, if any.
			ident_params: input.parse_terminated(Ident::parse)?,
			// Or token: `|`.
			or_token2: input.parse()?,
			// The `ReturnType` of the closure.
			output: input.parse()?,
			// The closure's body.
			body: input.parse()?,
		})
	}
}

impl Parse for TypeClosure {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			// Left arrow bracket token: `<`.
			lt_token: input.parse()?,
			// The `TypeClosure`'s type.
			ty: input.parse()?,
			// The semicolon token: `;`.
			semicolon_token: input.parse()?,
			// The `TypeClosure`'s closure.
			closure: input.parse()?,
			// Right arrow bracket token: `>`.
			gt_token: input.parse()?,
		})
	}
}

impl Parse for Type {
	fn parse(input: ParseStream) -> Result<Self> {
		// If the next token is `<`, then it could be either a `Type` _or_ a
		// `TypeClosure`, so some more in-depth checking is required.
		Ok(if input.peek(Token![<]) {
			// Fork the `input` stream so we can return back to this point if we
			// find that this is a `Type`.
			let ahead = input.fork();

			// Parse the `<` token that was found.
			let lt_token: Token![<] = ahead.parse()?;
			// Parse a `Type` that would be contained within a `TypeClosure`.
			// Technically I think this should parse correctly if this is a
			// `Type` too, but it will be checked again when we parse a `Type`,
			// so it can be returned later if this is a `Type`.
			let ty: Result<syn::Type> = ahead.parse();

			if ahead.peek(Token![;]) {
				// If the next token is a semicolon, then this is definitely a
				// `TypeClosure`. We advance the `input` stream to the current
				// position of the `ahead` fork, since we can use the tokens
				// we've already parsed.
				//
				// This is marked as a `discouraged` trait, but that's only when
				// using it for certain uses. This is a valid use case of
				// `Speculative::advance_to`.
				input.advance_to(&ahead);

				Self::TypeClosure(TypeClosure {
					// The `<` token already parsed.
					lt_token,
					// The `Type` already parsed.
					ty: ty?,
					// Parse the semicolon token: `;`.
					semicolon_token: input.parse()?,
					// Parse the closure.
					closure: input.parse()?,
					// Parse the `>` token.
					gt_token: input.parse()?,
				})
			} else {
				// Otherwise, if the next token is not a semicolon, then this is
				// a `Type`. `Type` will be parsed starting at the position when
				// we forked, not after reading the `<` and contained `Type`.
				Self::Type(input.parse()?)
			}
		} else {
			// If the next token is not `<`, then this has to be a `Type`.
			Self::Type(input.parse()?)
		})
	}
}

// }}}
