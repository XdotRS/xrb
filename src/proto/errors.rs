// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{Atom, Deserialize, ResId, Serialize};

#[allow(dead_code)]
pub enum ErrorType {
	Request,
	Value(u32),
	Window(ResId),
	Pixmap(ResId),
	Atom(Atom),
	Cursor(ResId),
	Font(ResId),
	Match,
	Drawable(ResId),
	Access,
	Alloc,
	Colormap(ResId),
	GContext(ResId),
	IdChoice(ResId),
	Name,
	Length,
	Implementation,
}

impl ErrorType {
	fn error_code(&self) -> u8 {
		match self {
			Self::Request => 1u8,
			Self::Value(_) => 2u8,
			Self::Window(_) => 3u8,
			Self::Pixmap(_) => 4u8,
			Self::Atom(_) => 5u8,
			Self::Cursor(_) => 6u8,
			Self::Font(_) => 7u8,
			Self::Match => 8u8,
			Self::Drawable(_) => 9u8,
			Self::Access => 10u8,
			Self::Alloc => 11u8,
			Self::Colormap(_) => 12u8,
			Self::GContext(_) => 13u8,
			Self::IdChoice(_) => 14u8,
			Self::Name => 15u8,
			Self::Length => 16u8,
			Self::Implementation => 17u8,
		}
	}
}

impl Serialize for ErrorType {
	fn write(self, buf: &mut impl bytes::BufMut) {
		match self {
			Self::Value(val) => val.write(buf),
			Self::Window(id) => id.write(buf),
			Self::Pixmap(id) => id.write(buf),
			Self::Atom(atom) => atom.write(buf),
			Self::Cursor(id) => id.write(buf),
			Self::Font(id) => id.write(buf),
			Self::Drawable(id) => id.write(buf),
			Self::Colormap(id) => id.write(buf),
			Self::GContext(id) => id.write(buf),
			Self::IdChoice(id) => id.write(buf),
			_ => 0u32.write(buf),
		}
	}
}

pub struct Error {
	pub error_type: ErrorType,
	pub sequence_number: u16,
	pub minor_opcode: u16,
	pub major_opcode: u8,
}

impl Serialize for Error {
	fn write(self, buf: &mut impl bytes::BufMut) {
		0u8.write(buf); // A first byte of 0 denotes an error

		self.error_type.error_code().write(buf);
		self.sequence_number.write(buf);
		self.error_type.write(buf);
		self.minor_opcode.write(buf);
		self.major_opcode.write(buf);

		buf.put_bytes(0u8, 21); // We have to pad out the rest of the error to 32 bytes in length
	}
}

impl Deserialize for Error {
	fn read(buf: &mut impl bytes::Buf) -> Self {
		// We don't skip the first byte that denotes this as an error, because that byte would have
		// been read in order to know to call [Error]'s [read] function anyway.

		let _error_code = u8::read(buf);
		let sequence_number = u16::read(buf);
		let _data = u32::read(buf);
		let minor_opcode = u16::read(buf);
		let major_opcode = u8::read(buf);

		buf.advance(21);

		Self {
			error_type: ErrorType::Access, // TODO: Deserialize [ErrorType]
			sequence_number,
			minor_opcode,
			major_opcode,
		}
	}
}
