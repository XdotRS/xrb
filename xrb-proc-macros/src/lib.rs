// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro::TokenStream;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, bracketed, parse_macro_input, token, Ident, LitInt, Result, Token, Type};

use quote::quote;

mod derive_parse;
mod parsing;

#[proc_macro]
pub fn requests(tokens: TokenStream) -> TokenStream {
	let input = parse_macro_input!(tokens as Request);

	let name = input.request.name();
	let length = input.request.length();
	let major_opcode = input.major.opcode;
	let minor_opcode = input.request.minor_opcode();
	let minop = if minor_opcode.is_some() {
		let min = minor_opcode.unwrap();
		quote!(Some(min))
	} else {
		quote!(None)
	};

	let meta_ty = input.request.meta_byte_type();
	let meta_name = input.request.meta_byte_name();

	let fields = input.request.definition().fields();

	let names: Vec<Ident> = fields.iter().map(|f| f.0.clone()).collect();
	let lengths: Vec<u8> = fields.iter().map(|f| f.1.clone()).collect();
	let types: Vec<Type> = fields.iter().map(|f| f.2.clone()).collect();

	let reply = input.request.definition().reply();

	let meta = if meta_name.is_some() && meta_ty.is_some() {
		quote!(pub #meta_name: #meta_ty,)
	} else {
		quote!()
	};

	let expanded = quote! {
			pub struct #name {
				#meta
				#(pub #names: #types),*
			}

			impl crate::Request<#reply> for #name {
				fn opcode() -> u8 { #major_opcode }
				fn minor_opcode() -> Option<u8> { #minop }
				fn length(&self) -> u16 { #length }
			}

			impl crate::rw::Serialize for #name {
				fn serialize(self) -> crate::rw::WriteResult<Vec<u8>> {
					let mut bytes = vec![];

					// Header {{{
					<u8 as crate::rw::WriteValue>::write_1b_to(
						#major_opcode,
						&mut bytes
					)?;

					<u8 as crate::rw::WriteValue>::write_1b_to(
						0 #(+ #minor_opcode)? //#(
	//						+ <#meta_ty as crate::rw::WriteValue>::write_1b(self.#meta_name)
	//					)?,
	//					,
						&mut bytes
					)?;

					<u16 as crate::rw::WriteValue>::write_2b_to(#length, &mut bytes)?;
					// }}}

					#(
						match self.#lengths {
							1 => <#types as crate::rw::WriteValue>::write_1b_to(self.#names, &mut bytes)?,
							2 => <#types as crate::rw::WriteValue>::write_2b_to(self.#names, &mut bytes)?,
							4 => <#types as crate::rw::WriteValue>::write_4b_to(self.#names, &mut bytes)?,
							_ => (),
						}
					)*

					Ok(bytes)
				}
			}
		};

	TokenStream::from(expanded)
}

struct Request {
	major: Opcode,
	request: RequestType,
}

impl Parse for Request {
	fn parse(input: syn::parse::ParseStream) -> Result<Self> {
		Ok(Request {
			major: input.parse()?,
			request: input.parse()?,
		})
	}
}

enum RequestType {
	Major(MajorRequest),
	Minor(MinorRequest),
}

impl RequestType {
	fn name(&self) -> Ident {
		match self {
			Self::Major(req) => req.name.clone(),
			Self::Minor(req) => req.name.clone(),
		}
	}

	fn length(&self) -> u16 {
		match self {
			Self::Major(req) => req.length.length,
			Self::Minor(req) => req.length.length,
		}
	}

	fn definition(&self) -> &Definition {
		match self {
			Self::Major(req) => &req.definition,
			Self::Minor(req) => &req.definition,
		}
	}

	fn minor_opcode(&self) -> Option<u8> {
		match self {
			Self::Major(_) => None,
			Self::Minor(req) => Some(req.minor.opcode),
		}
	}

	fn meta_byte_name(&self) -> Option<Ident> {
		match self {
			Self::Major(req) => req.data_byte.clone().map(|d| d.field.name),
			Self::Minor(_) => None,
		}
	}

	fn meta_byte_type(&self) -> Option<Type> {
		match self {
			Self::Major(req) => req.data_byte.clone().map(|data| data.field.ty),
			_ => None,
		}
	}
}

impl Parse for RequestType {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.lookahead1().peek(LitInt) {
			input.parse().map(RequestType::Minor)
		} else {
			input.parse().map(RequestType::Major)
		}
	}
}

struct MajorRequest {
	length: Length,
	name: Ident,
	data_byte: Option<DataByte>,
	definition: Definition,
}

impl Parse for MajorRequest {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(MajorRequest {
			length: input.parse()?,
			name: input.parse()?,
			data_byte: input.parse().ok(),
			definition: input.parse()?,
		})
	}
}

struct MinorRequest {
	minor: Opcode,
	length: Length,
	name: Ident,
	definition: Definition,
}

impl Parse for MinorRequest {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(MinorRequest {
			minor: input.parse()?,
			length: input.parse()?,
			name: input.parse()?,
			definition: input.parse()?,
		})
	}
}

struct Opcode {
	opcode: u8,
	//	_dot_token: Token![.],
}

impl Parse for Opcode {
	fn parse(input: ParseStream) -> Result<Self> {
		let lit: LitInt = input.parse()?;

		Ok(Opcode {
			opcode: lit.base10_parse::<u8>()?,
			//		_dot_token: input.parse()?,
		})
	}
}

struct Length {
	_brace_token: token::Brace,
	length: u16,
}

impl Parse for Length {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		let _brace_token = braced!(content in input);
		let lit: LitInt = content.parse()?;

		Ok(Length {
			_brace_token,
			length: lit.base10_parse::<u16>()?,
		})
	}
}

#[derive(Clone)]
struct DataByte {
	_bracket_token: token::Bracket,
	field: FixedLenField,
}

impl Parse for DataByte {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		Ok(DataByte {
			_bracket_token: bracketed!(content in input),
			field: content.parse()?,
		})
	}
}

/// `name: Ident` `:` `ty: Type`
///
/// A field of a fixed-length; no length nor visibility can be specified.
///
/// # Example
/// ```rust
/// window: Window
/// ```
#[derive(Clone)]
struct FixedLenField {
	name: Ident,
	_colon_token: Token![:],
	ty: Type,
}

impl Parse for FixedLenField {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(FixedLenField {
			name: input.parse()?,
			_colon_token: input.parse()?,
			ty: input.parse()?,
		})
	}
}

/// `->` `reply_type: Type`
///
/// The type of reply associated with this request, if any. `()` represents no
/// reply.
///
/// # Example
/// ```rust
/// -> GetWindowAttributesReply
/// ```
#[derive(Clone)]
struct Reply {
	_arrow_token: Token![->],
	reply_type: Type,
}

impl Parse for Reply {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Reply {
			_arrow_token: input.parse()?,
			reply_type: input.parse()?,
		})
	}
}

/// The definition of the request's fields.
///
/// Can either be a full [`Body`], or the shorthand version that defines a
/// single 4-byte field.
enum Definition {
	Long(Body),
	Short(Shorthand),
}

impl Definition {
	fn fields(&self) -> Vec<(Ident, u8, Type)> {
		match self {
			Self::Long(body) => body
				.fields
				.first()
				.iter()
				.filter_map(|f| f.field().is_some().then(|| f.field().unwrap()))
				.collect(),
			Self::Short(short) => match short.field.clone() {
				Some(val) => vec![(val.field.name, 1, val.field.ty)],
				None => vec![],
			},
		}
	}

	fn reply(&self) -> Option<Type> {
		match self {
			Self::Long(body) => body.reply.clone().map(|reply| reply.reply_type),
			Self::Short(short) => short.reply.clone().map(|reply| reply.reply_type),
		}
	}
}

impl Parse for Definition {
	fn parse(input: ParseStream) -> Result<Self> {
		// TODO: This should be a fork, not a lookahead.
		let a: u8;

		if input.lookahead1().peek(Token![<]) {
			input.parse().map(Definition::Short)
		} else {
			input.parse().map(Definition::Long)
		}
	}
}

/// (`field: ShorthandField`) (`reply: Reply`) `;`
///
/// Shorthand for declaring a single 4-byte field for a request.
///
/// # Examples
/// ```rust
/// ;
/// <window: Window>;
/// <colormap: Colormap> -> GetWindowAttributesReply;
/// <cursor: Cursor>;
/// ```
struct Shorthand {
	field: Option<ShorthandField>, // <name: Type>

	reply: Option<Reply>,        // -> GetWindowAttributesReply
	_semicolon_token: Token![;], // ;
}

impl Parse for Shorthand {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Shorthand {
			field: input.parse().ok(),
			reply: input.parse().ok(),
			_semicolon_token: input.parse()?,
		})
	}
}

#[derive(Clone)]
struct ShorthandField {
	_open_token: Token![<],  // <
	field: FixedLenField,    // name: Type
	_close_token: Token![>], // >
}

impl Parse for ShorthandField {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(ShorthandField {
			_open_token: input.parse()?,
			field: input.parse()?,
			_close_token: input.parse()?,
		})
	}
}

/// (`reply: Reply`) `{` `fields: $(ReqField),*` `}`
///
/// The body of a request that defines its fields. See [ReqField].
///
/// # Examples
/// ```rust
/// {
///     window[4]: Window,
///     mode[1]: Mode,
///     width[2]: u16,
///     y[2]: i16,
/// }
///
/// -> GetWindowAttributesReply {}
///
/// -> GetWindowAttributesReply {
///     window: Window,
/// }
/// ```
struct Body {
	reply: Option<Reply>,
	_brace_token: token::Brace,
	fields: Punctuated<ReqField, Token![,]>,
}

impl Parse for Body {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		Ok(Body {
			reply: input.parse().ok(),
			_brace_token: braced!(content in input),
			fields: content.parse_terminated(ReqField::parse)?,
		})
	}
}

/// Either a named [`PresentField`] with a type, or an [`Unused`] field with
/// random data.
enum ReqField {
	Present(PresentField),
	Unused(UnusedField),
}

impl ReqField {
	fn field(&self) -> Option<(Ident, u8, Type)> {
		match self {
			Self::Present(field) => Some((
				field.field_name.clone(),
				field.length.clone().map_or(4, |f| f.field_length),
				field.field_type.clone(),
			)),
			Self::Unused(_) => None,
		}
	}
}

impl Parse for ReqField {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.lookahead1().peek(Token![?]) {
			input.parse().map(ReqField::Unused)
		} else {
			input.parse().map(ReqField::Present)
		}
	}
}

/// `field_name: Ident` (`field_length: FieldLength`) `:` `field_type: Type`
///
/// A field in a request with its size in bytes (1, 2, or 4). Defaults to four
/// bytes in length.
///
/// # Examples
/// ```rust
/// window[4]: Window // 4 bytes
/// mode[1]: Mode // 1 byte
/// width[2]: u16 // 2 bytes
/// y[2]: i16 // 2 bytes
/// colormap: Colormap // 4 bytes
/// ```
struct PresentField {
	field_name: Ident,
	length: Option<FieldLength>,
	_colon_token: Token![:],
	field_type: Type,
}

impl Parse for PresentField {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(PresentField {
			field_name: input.parse()?,
			length: input.parse().ok(),
			_colon_token: input.parse()?,
			field_type: input.parse()?,
		})
	}
}

/// `?` (`field_length: FieldLength`)
///
/// An unused field that will be ignored and filled with random data.
/// `field_length` specifies the number of unused bytes. Defaults to one byte in
/// length.
///
/// # Examples
/// ```rust
/// ? // 1 unused byte
/// ?[1] // 1 unused byte
/// ?[21] // 21 unused bytes
/// ```
struct UnusedField {
	_question_mark_token: Token![?],
	length: Option<FieldLength>,
}

impl Parse for UnusedField {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(UnusedField {
			_question_mark_token: input.parse()?,
			length: input.parse().ok(),
		})
	}
}

/// `[` `field_length: u8` `]`
///
/// The length of a request field in number of bytes.
///
/// # Examples
/// ```rust
/// [1] // 1 byte
/// [2] // 2 bytes
/// [4] // 4 bytes
/// [21] // 21 bytes
/// ```
#[derive(Clone)]
struct FieldLength {
	_bracket_token: token::Bracket,
	field_length: u8,
}

impl Parse for FieldLength {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;

		let _bracket_token = bracketed!(content in input);
		let lit: LitInt = content.parse()?;

		Ok(FieldLength {
			_bracket_token,
			field_length: lit.base10_parse::<u8>()?,
		})
	}
}
