// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xrb_proc_macros::ByteSize;

use std::str;
use std::str::Utf8Error;
use std::string::{String, FromUtf8Error};

/// A string slice of text with 1-byte characters.
///
/// This is the borrowed counterpart to [`String8`].
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, ByteSize)]
pub struct Str8<'a>(&'a [u8]);
/// A string slice of text with 2-byte characters.
///
/// This is the borrowed counterpart to [`String16`].
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, ByteSize)]
pub struct Str16<'a>(&'a [(u8, u8)]);

/// A string of text with 1-byte characters.
///
/// This is the owned counterpart to [`Str8`].
#[derive(Clone, Eq, PartialEq, Hash, Debug, ByteSize)]
pub struct String8(Vec<u8>);
/// A string of text with 2-byte characters.
///
/// This is the owned counterpart to [`Str16`].
#[derive(Clone, Eq, PartialEq, Hash, Debug, ByteSize)]
pub struct String16(Vec<(u8, u8)>);

// Owned (`String8`, `String16`) {{{
impl From<String> for String8 {
	fn from(text: String) -> Self {
		// Convert the character to one byte.
		Self(text.chars().map(|r#char| r#char as u8).collect())
	}
}

impl TryFrom<String8> for String {
	type Error = FromUtf8Error;

	fn try_from(text: String8) -> Result<Self, FromUtf8Error> {
		// Try to convert the byte to a character.
		Ok(String::from_utf8(
			text.0
				.iter()
				.flat_map(|&r#char| (r#char as u32).to_ne_bytes())
				.collect(),
		)?)
	}
}

impl From<String> for String16 {
	fn from(text: String) -> Self {
		Self(
			text.chars()
				// Map every character to its two least significant bytes.
				.map(|r#char| {
					let bytes = (r#char as u16).to_ne_bytes();

					(bytes[0], bytes[1])
				})
				.collect(),
		)
	}
}

impl TryFrom<String16> for String {
	type Error = FromUtf8Error;

	// Try to convert the pair of bytes to a character.
	fn try_from(text: String16) -> Result<Self, Self::Error> {
		Ok(String::from_utf8(
			text.0
				.iter()
				// Since characters are four bytes, we must expand the pair of
				// bytes to four bytes. This is probably the easiest way of
				// doing that.
				.flat_map(|&(a, b)| (u16::from_ne_bytes([a, b]) as u32).to_ne_bytes())
				.collect(),
		)?)
	}
}
// }}}

// Borrowed (`Str8`, `Str16`) {{{
impl<'a> From<&str> for Str8<'a> {
	fn from(text: &str) -> Self {
		// Convert the character to one byte.
		Self(&text.chars().map(|r#char| r#char as u8).collect::<Vec<u8>>())
	}
}

impl<'a> TryFrom<Str8<'a>> for &str {
	type Error = Utf8Error;

	fn try_from(text: Str8) -> Result<Self, Self::Error> {
		// Try to convert the byte to a character.
		Ok(str::from_utf8(
			&text
				.0
				.iter()
				.flat_map(|&r#char| (r#char as u32).to_ne_bytes())
				.collect::<Vec<u8>>(),
		)?)
	}
}

impl<'a> From<&str> for Str16<'a> {
	fn from(text: &str) -> Self {
		Self(
			&text
				.chars()
				// Map every character to its two least significant bytes.
				.map(|r#char| {
					let bytes = (r#char as u16).to_ne_bytes();

					(bytes[0], bytes[1])
				})
				.collect::<Vec<(u8, u8)>>(),
		)
	}
}

impl<'a> TryFrom<Str16<'a>> for &str {
	type Error = Utf8Error;

	// Try to convert the pair of bytes to a character.
	fn try_from(text: Str16<'a>) -> Result<Self, Self::Error> {
		Ok(str::from_utf8(
			&text
				.0
				.iter()
				// Since characters are four bytes, we must expand the pair of
				// bytes to four bytes. This is probably the easiest way of
				// doing that.
				.flat_map(|&(a, b)| (u16::from_ne_bytes([a, b]) as u32).to_ne_bytes())
				.collect::<Vec<u8>>(),
		)?)
	}
}
// }}}
