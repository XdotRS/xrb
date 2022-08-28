// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::rw::{
	Deserialize, ReadError, ReadResult, ReadValue, Serialize, WriteResult, WriteValue,
};

use bytes::{BufMut, BytesMut};

use crate::x11::common::values::{Char2b, HostFamily};

/// A UTF-16-encoded string of [two-byte characters](Char2b).
pub type String16<'a> = &'a [Char2b];

/// A rectangle represented by its coordinates and dimensions.
///
/// The coordinates are those of the upper-left corner of the rectangle. The
/// units for the coordinates and dimensions are not specified.
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
pub struct Arc {
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

pub struct Host {
	/// The protocol family of the host, e.g. [InternetV6](HostFamily::InternetV6).
	pub family: HostFamily,
	/// The address of the host in question.
	pub address: String,
}

impl Serialize for String16<'_> {
	fn serialize(self) -> WriteResult<Vec<u8>> {
		let mut bytes = vec![];

		for ch in self {
			ch.write_2b_to(&mut bytes)?;
		}

		Ok(bytes)
	}
}

// We can't implement [`Deserialize`] for [`String16`] because we don't know its
// length from just the [`String16`] itself.

impl Serialize for Rectangle {
	fn serialize(self) -> WriteResult<Vec<u8>> {
		let mut bytes = BytesMut::new();

		// Write each field as two bytes to `bytes`.
		self.x.write_2b_to(&mut bytes)?;
		self.y.write_2b_to(&mut bytes)?;
		self.width.write_2b_to(&mut bytes)?;
		self.height.write_2b_to(&mut bytes)?;

		Ok(bytes.to_vec())
	}
}

impl Deserialize for Rectangle {
	fn deserialize(buf: &mut impl bytes::Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		// Read each field as two bytes from `buf`.
		Ok(Self {
			x: i16::read_2b_from(buf)?,
			y: i16::read_2b_from(buf)?,
			width: u16::read_2b_from(buf)?,
			height: u16::read_2b_from(buf)?,
		})
	}
}

impl Serialize for Arc {
	fn serialize(self) -> WriteResult<Vec<u8>> {
		let mut bytes = BytesMut::new();

		// Write each field as two bytes to `bytes`.
		self.x.write_2b_to(&mut bytes)?;
		self.y.write_2b_to(&mut bytes)?;
		self.width.write_2b_to(&mut bytes)?;
		self.height.write_2b_to(&mut bytes)?;
		self.start.write_2b_to(&mut bytes)?;
		self.end.write_2b_to(&mut bytes)?;

		Ok(bytes.to_vec())
	}
}

impl Deserialize for Arc {
	fn deserialize(buf: &mut impl bytes::Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		// Read each field as two bytes from `buf`.
		Ok(Self {
			x: i16::read_2b_from(buf)?,
			y: i16::read_2b_from(buf)?,
			width: u16::read_2b_from(buf)?,
			height: u16::read_2b_from(buf)?,
			start: i16::read_2b_from(buf)?,
			end: i16::read_2b_from(buf)?,
		})
	}
}

impl Serialize for Host {
	fn serialize(self) -> WriteResult<Vec<u8>> {
		let mut bytes = BytesMut::new();

		self.family.write_1b_to(&mut bytes)?; // protocol family

		0u8.write_1b_to(&mut bytes)?; // empty byte; unused

		// While [`String`] has a `serialize` implementation, [`Host`]
		// specifically uses a 16-bit length field for the address, so we have
		// to write the [`String`] manually. Usually it would be just a single
		// byte for the length.
		let length = self.address.len();
		// Calculate the number of empty padding bytes required to bring the
		// total length to a multiple of 4. All message lengths must be a
		// multiple of 4 bytes long.
		let padding = 4 - length % 4;

		length.write_2b_to(&mut bytes)?; // length
		bytes.put(self.address.as_bytes()); // address itself
		bytes.put_bytes(0, padding); // any extra unused padding

		Ok(bytes.to_vec())
	}
}

impl Deserialize for Host {
	fn deserialize(buf: &mut impl bytes::Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		let family = HostFamily::read_1b_from(buf)?;
		buf.advance(1); // skip the unused byte

		// Read the address length.
		let length = usize::read_2b_from(buf)?;
		// Calculate the number of empty padding bytes that will be included at
		// the end of the message that ensure the message is a multiple of 4
		// bytes long. We'll need to skip this many bytes.
		let padding = 4 - length % 4;

		// Read `length` number of bytes from the buffer to a vec.
		let bytes = buf.copy_to_bytes(length).to_vec();
		// Create a [`String`] from `bytes`. Map any error to an [`InvalidData`]
		// error.
		let address =
			String::from_utf8(bytes).map_or(Err(ReadError::InvalidData), |addr| Ok(addr))?;

		buf.advance(padding); // skip the unused padding bytes at the end

		Ok(Self { family, address })
	}
}
