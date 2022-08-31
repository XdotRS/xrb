// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Type, Visibility};

use super::databyte::Databyte;
use super::declare_reply::ReplyDeclaration;
use super::definition::Definition;
use super::opcodes::Opcode;
use super::request_title::RequestTitle;

/// A fully parsed request, for use in a `requests!` macro.
///
/// # Examples
/// _All_ of the following are equivalent:
/// ```rust
/// 4! pub struct DeleteWindow<2> window: Window[4];
/// 4! pub struct DeleteWindow<2>(?) window: Window[4];
/// 4! pub struct DeleteWindow<2>(?[1]) window: Window[4];
/// 4! pub struct DeleteWindow<2> -> () window: Window[4];
/// 4! pub struct DeleteWindow<2>(?[1]) -> () window: Window[4];
///
/// 4! pub struct DeleteWindow<2> {
///	    window: Window[4],
/// }
///
/// 4! pub struct DeleteWindow<2>(?) {
///     window: Window[4],
/// }
///
/// 4! pub struct DeleteWindow<2>(?[1]) {
///     window: Window[4],
/// }
///
/// 4! pub struct DeleteWindow<2> -> () {
///     window: Window[4],
/// }
///
/// 4! pub struct DeleteWindow<2>(?[1]) -> () {
///     window: Window[4],
/// }
/// ```
/// In the above example, this struct looks like so:
/// ```rust
/// major_opcode: 4,
/// meta_byte: Databyte { field: ?[1] }, // ? = unused/empty
/// vis: Some(`pub`),
/// name: `DeleteWindow`,
/// length: 2,
/// // 1 field `window` with a type of `Window` and a byte-length of `4`
/// ```
#[derive(Clone)]
pub struct Request {
	/// The request's major opcode.
	pub major_opcode: u8,
	/// The second byte of the request: either the minor opcode or a [`Databyte`].
	pub meta_byte: Metabyte,
	/// The request's visibility (e.g. `pub`).
	pub vis: Option<Visibility>,
	/// The name of the request.
	pub name: Ident,
	/// The length of the request in units of 4-bytes; at least one unit, for
	/// the header.
	pub length: u16,
	pub reply_ty: Option<Type>,
	/// The definition of the request with zero or more fields.
	pub definition: Definition,
}

/// Represents the second byte of the request header.
///
/// Either a minor opcode, a 1-byte field, or an unused byte.
#[derive(Clone)]
pub enum Metabyte {
	Normal(Databyte),
	Minor { minor_opcode: u8 },
}

impl Metabyte {
	/// Constructs a [`Metabyte`] with the given minor opcode.
	pub fn with_minor_opcode(minor_opcode: u8) -> Self {
		Self::Minor { minor_opcode }
	}

	#[allow(dead_code)]
	/// Gets the wrapped [`Databyte`] if this is [`Metabyte::Normal`], else [`None`].
	pub fn databyte(&self) -> Option<Databyte> {
		match self {
			Self::Normal(databyte) => Some(databyte.clone()),
			_ => None,
		}
	}

	#[allow(dead_code)]
	/// Gets the wrapped minor opcode if this is [`Metabyte::Minor`], else [`None`].
	pub fn minor_opcode(&self) -> Option<u8> {
		match self {
			Self::Minor { minor_opcode } => Some(*minor_opcode),
			_ => None,
		}
	}
}

impl From<Databyte> for Metabyte {
	fn from(databyte: Databyte) -> Self {
		Self::Normal(databyte)
	}
}

// Parsing {{{

impl Parse for Request {
	fn parse(input: ParseStream) -> Result<Self> {
		let major_opcode: u8 = input.parse::<Opcode>()?.opcode;

		match input.lookahead1().peek(LitInt) {
			false => parse_normal(major_opcode, input),
			true => parse_minor(major_opcode, input),
		}
	}
}

/// Parses a request that does not have a minor opcode.
fn parse_normal(major_opcode: u8, input: ParseStream) -> Result<Request> {
	// Parse the title (visibility, name, length).
	let title: RequestTitle = input.parse()?;
	// Attempt to parse a databyte definition.
	let databyte: Result<Databyte> = input.parse();
	// Attempt to parse a reply type declaration.
	let reply_declaration: Option<ReplyDeclaration> = input.parse().ok();
	// Parse the definition of zero or more fields.
	let definition: Definition = input.parse()?;

	// Convert either a read databyte or the default of a 1-byte unused field
	// to a [`Metabyte`].
	let meta_byte: Metabyte = databyte.unwrap_or_default().into();
	// Get the type from the reply declaration.
	let reply_ty = reply_declaration.map(|rep| rep.reply_ty);

	Ok(Request {
		major_opcode,
		meta_byte,
		vis: title.vis,
		name: title.name,
		length: title.length,
		reply_ty,
		definition,
	})
}

/// Parses a request that has a minor opcode instead of a request header data
/// byte field.
fn parse_minor(major_opcode: u8, input: ParseStream) -> Result<Request> {
	// Parse the minor opcode as an 8-bit integer.
	let minor_opcode: u8 = input.parse::<Opcode>()?.opcode;
	// Parse the title (visibility, name, length).
	let title: RequestTitle = input.parse()?;
	let reply_declaration: Option<ReplyDeclaration> = input.parse().ok();
	// Parse the definition or zero or more fields.
	let definition: Definition = input.parse()?;

	// Convert the minor opcode to a [`Metabyte`].
	let meta_byte = Metabyte::with_minor_opcode(minor_opcode);
	let reply_ty = reply_declaration.map(|rep| rep.reply_ty);

	Ok(Request {
		major_opcode,
		meta_byte,
		vis: title.vis,
		name: title.name,
		length: title.length,
		reply_ty,
		definition,
	})
}

// }}}
