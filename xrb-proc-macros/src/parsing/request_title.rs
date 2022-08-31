// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token, Visibility};

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, TokenStreamExt};

/// The 'title' of a request, including its visibility, name, and length.
///
/// The visibility refers to keywords like `pub` that may be present at the
/// start of the title. [`None`] means that no visibility modifier was given.
///
/// A request's length is measured in units of 4 bytes, and a request always
/// contains an initial 4-byte header, so the minimum length of a request is 1
/// 4-byte unit. Therefore, if the length of the request is omitted, it will
/// default to `1`.
///
/// # Examples
/// ```rust
/// pub struct DeleteWindow<2> // vis: Some(`pub`), length: `2`
/// pub struct GrabServer      // vis: Some(`pub`), length: `1`
/// pub struct GrabServer<1>   // vis: Some(`pub`), length: `1`
/// struct DeleteWindow<2>     // vis: None, length: `2`
/// struct GrabServer          // vis: None, length: `1`
/// struct GrabServer<1>       // vis: None, length: `1`
/// ```
#[derive(Clone)]
pub struct RequestTitle {
	pub vis: Option<Visibility>,
	pub name: Ident,
	pub length: u16,
}

impl RequestTitle {
	#[allow(dead_code)]
	pub fn new(vis: Option<Visibility>, name: Ident, length: u16) -> Self {
		Self { vis, name, length }
	}
}

impl ToTokens for RequestTitle {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		// Write the visibility, if any.
		self.vis.to_tokens(tokens);
		// Write the `struct` keyword (which is 'just' a special identifier).
		tokens.append(Ident::new("struct", Span::call_site()));
		// Write the name of the request.
		self.name.to_tokens(tokens);

		// This writes the request title in the format `#vis struct #Name`,
		// where `#vis` is the visibility and `#Name` is the name; for example:
		//
		// ```rust
		// pub struct MyRequest
		// ```
	}
}

/// The length of a request in units of 4 bytes.
///
/// The length of a request is measured in units of 4 bytes. The request always
/// contains an initial header that is 4 bytes in length, so the minimum
/// request length is `1`.
///
/// # Panics
/// Parsing the request length panics if an opening `<` was found, but not
/// matching `>` to close the length: this is because, while a missing request
/// length c an simply be ignored, a missing `>` means that there was
/// specifically a mistake made when calling the macro. We don't normally have
/// to do this because [`syn`] automatically handles this for `(` and `)`, `[`
/// and `]`, and `{` and `}`. `<` and `>` are an exception because they are
/// also used as less than and greater than operators, so one of th em missing
/// is not necessarily always a problem.
///
/// # Examples
/// ```rust
/// <1> // length: 1 * 4 bytes [4]
/// <2> // length: 2 * 4 bytes [8]
/// <5> // length: 5 * 4 bytes [20]
/// ```
#[derive(Copy, Clone, Hash, Debug, Default)]
pub struct RequestLength {
	pub length: u16,
}

impl RequestLength {
	#[allow(dead_code)]
	/// Construct a new [`RequestLength`] node with the default length of `1`.
	pub fn new() -> Self {
		Self { length: 1 }
	}

	#[allow(dead_code)]
	/// Construct a new [`RequestLength`] node with the given length.
	pub fn with_length(length: u16) -> Self {
		Self { length }
	}
}

impl ToTokens for RequestLength {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.length.to_tokens(tokens);
	}
}

// Parsing {{{

impl Parse for RequestTitle {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse the visibility (e.g. `pub`). `None` if it is missing.
		let vis: Option<Visibility> = input.parse().ok();
		// Parse the `struct` token, but don't save it.
		input.parse::<Token![struct]>()?;

		// Parse the request's name.
		let name: Ident = input.parse()?;
		// Parse the requests's length, but default to `1` if it was missing.
		let length = input.parse::<RequestLength>().map_or(1, |len| len.length);

		Ok(Self { vis, name, length })
	}
}

impl Parse for RequestLength {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse the `<` token, but don't save it.
		input.parse::<Token![<]>()?;

		// Parse the length as a 16-bit integer.
		let value: LitInt = input.parse()?;
		let length: u16 = value.base10_parse()?;

		// Parse the `>`  token. This panics if it isn't found, because we know
		// that if we have gotten to this point, reading `<` hasn't already
		// returned an error: we shouldn't simply ignore the length and use a
		// default, because the macro invocation itself has a mistake.
		input
			.parse::<Token![>]>()
			.expect("found opening `<` in request length, so a closing `>` was also expected");

		Ok(Self { length })
	}
}

// }}}
