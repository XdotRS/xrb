// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::x11::common::values::{Char1b, Char2b, HostFamily};

use cornflakes::{ByteReader, ByteSize, ByteWriter, FromBytes, ToBytes};
use xrb_proc_macros::{ByteSize, StaticByteSize};

use std::io::Error;

pub type String8 = Vec<Char1b>;
pub type String16 = Vec<Char2b>;

pub struct Xstring(String8);

impl ByteSize for Xstring {
	fn byte_size(&self) -> usize {
		self.0.byte_size() + 1
	}
}

impl FromBytes for Xstring {
	fn read_from(reader: &mut impl ByteReader) -> Result<Self, Error> {
		let len = reader.read_u8() as usize;
		Ok(Self(reader.read_with_size(len)?))
	}
}

impl ToBytes for Xstring {
	fn write_to(&self, writer: &mut impl ByteWriter) -> Result<(), Error> {
		writer.write(self.0.len() as u8)?;
		writer.write_all(&self.0)?;

		Ok(())
	}
}

/// A rectangle represented by its coordinates and dimensions.
///
/// The coordinates are those of the upper-left corner of the rectangle. The
/// units for the coordinates and dimensions are not specified.
#[derive(StaticByteSize)]
pub struct Rectangle {
	/// X-coordinate of the upper-left corner of the rectangle.
	pub x: i16,
	/// Y-coordinate of the upper-left corner of the rectangle.
	pub y: i16,
	/// Width of the rectangle.
	pub width: u16,
	/// Height of the rectangle.
	pub height: u16,
}

/// A geometric arc, represented by its coordinates, dimensions, and start and end angles.
#[derive(StaticByteSize)]
pub struct GeomArc {
	/// X-coordinate of the arc.
	pub x: i16,
	/// Y-coordinate of the arc.
	pub y: i16,
	/// Width of the arc.
	pub width: u16,
	/// Height of the arc.
	pub height: u16,
	/// The start angle of the arc.
	pub start: i16,
	/// The end angle of the arc.
	pub end: i16,
}

#[derive(ByteSize)]
pub struct Host {
	/// The protocol family of the host, e.g. [InternetV6](HostFamily::InternetV6).
	pub family: HostFamily,
	/// The address of the host in question.
	pub address: String,
}
