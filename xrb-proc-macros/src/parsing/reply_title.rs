// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{Visibility, Ident, LitInt, Token, Result};

/// The 'title' of a reply, including its visibility, name, and length.
///
/// The visibility refers to keywords like `pub` that may be present at the
/// start of the title. [`None`] means that no visibility modifier was given.
///
/// The minimum reply length in bytes is 32 bytes (i.e. 8 * 4 bytes). The
/// length given in the title is represented in units of 4 bytes, and starts
/// at the minimum reply length: that means that a length of `0` is 32 bytes,
/// a length of `1` is 36 bytes, and a length of `2` is 40 bytes.
///
/// # Examples
/// ```rust
/// pub struct GetWindowAttributesReply    // vis: Some(`pub`), length: `0`
/// pub struct GetWindowAttributesReply<0> // vis: Some(`pub`), length: `0`
/// pub struct GetWindowAttributesReply<3> // vis: Some(`pub`), length: `3`
/// struct OtherReply                      // vis: None, length: `0`
/// struct OtherReply<0>                   // vis: None, length: `0`
/// struct OtherReply<3>                   // vis: None, length: `3`
/// ```
#[derive(Clone)]
pub struct ReplyTitle {
	pub vis: Option<Visibility>,
	pub name: Ident,
	pub length: u32,
}

impl ReplyTitle {
	#[allow(dead_code)]
	/// Construct a new [`ReplyTitle`] with the given [`Visibility`], name, and length.
	pub fn new(vis: Option<Visibility>, name: Ident, length: u32) -> Self {
		Self {
			vis,
			name,
			length,
		}
	}
}

/// The extra length in a reply, in units of 4 bytes.
///
/// The minimum reply length is 32 bytes (8 units): this specifies how many
/// units of 4 bytes _more_ than that minimum reply length to use. That means
/// that a given reply length of `0` is an actual reply length of `8` units.
///
/// # Panics
/// Parsing the reply length panics if an opening `<` was found, but not
/// matching `>` to close the length: this is because, while a missing reply
/// length can simply be ignored, a missing `>` means that there was
/// specifically a mistake made when calling the macro. We don't normally have
/// to do this because [`syn`] automatically handles this for `(` and `)`, `[`
/// and `]`, and `{` and `}`. `<` and `>` are an exception because they are
/// also used as less than and greater than operators, so one of them missing
/// is not necessarily always a problem.
///
/// # Examples
/// ```rust
/// <0> // total length: (8 + 0) * 4 bytes [32]
/// <1> // total length: (8 + 1) * 4 bytes [36]
/// <5> // total length: (8 + 5) * 4 bytes [52]
/// ```
#[derive(Copy, Clone, Hash, Debug, Default)]
pub struct ReplyLength {
	pub length: u32,
}

impl ReplyLength {
	#[allow(dead_code)]
	/// Construct a new [`ReplyLength`] node with the default length of `0`.
	pub fn new() -> Self {
		Self { length: 0 }
	}

	#[allow(dead_code)]
	/// Construct a new [`ReplyLength`] node with the given length.
	pub fn with_length(length: u32) -> Self {
		Self { length }
	}
}

// Parsing {{{

impl Parse for ReplyTitle {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse the visibility (e.g. `pub`). `None` if it is missing.
		let vis: Option<Visibility> = input.parse().ok();
		// Parse the `struct` token, but don't save it.
		input.parse::<Token![struct]>()?;

		// Parse the reply's name.
		let name: Ident = input.parse()?;
		// Parse the reply's length, but default to `0` if it was missing.
		let length = input.parse::<ReplyLength>().map_or(0, |len| len.length);

		Ok(Self {
			vis,
			name,
			length,
		})
	}
}

impl Parse for ReplyLength {
	fn parse(input: ParseStream) -> Result<Self> {
		// Parse the `<` token, but don't save it.
		input.parse::<Token![<]>()?;

		// Parse the length as a 32-bit integer.
		let value: LitInt = input.parse()?;
		let length: u32 = value.base10_parse()?;

		// Parse the `>` token. This panics if it isn't found, because we know
		// that if we have gotten to this point, reading `<` hasn't already
		// returned an error: we shouldn't simply ignore the length and use a
		// default, because the macro invocation itself has a mistake.
		input.parse::<Token![>]>()
			.expect("found opening `<` in reply length, so a closing `>` was also expected");

		Ok(Self { length })
	}
}

// }}}
