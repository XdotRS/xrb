// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use cornflakes::{derive::DataSize, *};

use std::{
	io::Error,
	string::{FromUtf8Error, String},
};

/// A string of text with 1-byte characters.
///
/// This is different from the built-in [`String`] in that Rust's [`String`]
/// is encoded with 4 bytes per character.
#[derive(Clone, Eq, PartialEq, Hash, Debug, DataSize)]
pub struct String8(Vec<u8>);
/// A string of text with 2-byte characters.
///
/// This is different from the built-in [`String`] in that Rust's [`String`]
/// is encoded with 4 bytes per character.
#[derive(Clone, Eq, PartialEq, Hash, Debug, DataSize)]
pub struct String16(Vec<(u8, u8)>);

/// A string of text with 1-byte characters, encoded with its length.
///
/// This is different from the built-in [`String`] in that Rust's [`String`]
/// is encoded with 4 bytes per character, and from [`String8`] in that the
/// length of the string is included in (de)serialization.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LenString8(Vec<u8>);

impl DataSize for LenString8 {
	fn data_size(&self) -> usize {
		// 1 byte for the length
		1 + self.0.byte_size()
	}
}

impl Readable for LenString8 {
	fn read_from(reader: &mut impl Buf) -> ReadResult<Self> {
		// read the length of the list
		let len = reader.get_u8() as usize;
		// read `len` bytes, because the list is a list of bytes
		let out = Vec::new();
		for _ in 0..len {
			out.push(reader.get_u8());
		}
		Ok(Self(out))
	}
}

impl Writable for LenString8 {
	#[allow(
		clippy::cast_possible_truncation,
		reason = "`LenString8`'s length must fit in a `u8` value by definition"
	)]
	fn write_to(&self, writer: &mut impl BufMut) -> Result<(), Error>
	where
		Self: Sized,
	{
		// writer.write(self.0.len() as u8)?;
		writer.put_u8(self.0.len() as u8);
		writer.put_slice(self.0.as_slice());

		Ok(())
	}
}

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
		Self::from_utf8(
			text.0
				.iter()
				.flat_map(|&r#char| u32::from(r#char).to_ne_bytes())
				.collect(),
		)
	}
}

impl From<String> for LenString8 {
	fn from(text: String) -> Self {
		// Convert the character to one byte.
		Self(text.chars().map(|r#char| r#char as u8).collect())
	}
}

impl TryFrom<LenString8> for String {
	type Error = FromUtf8Error;

	fn try_from(text: LenString8) -> Result<Self, FromUtf8Error> {
		// Try to convert the byte to a character.
		Self::from_utf8(
			text.0
				.iter()
				.flat_map(|&r#char| u32::from(r#char).to_ne_bytes())
				.collect(),
		)
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
		Self::from_utf8(
			text.0
				.iter()
				// Since characters are four bytes, we must expand the pair of
				// bytes to four bytes. This is probably the easiest way of
				// doing that.
				.flat_map(|&(a, b)| u32::from(u16::from_ne_bytes([a, b])).to_ne_bytes())
				.collect(),
		)
	}
}
