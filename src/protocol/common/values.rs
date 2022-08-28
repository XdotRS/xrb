// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::errors::{ReadError, ReadResult, WriteError, WriteResult};
use crate::rw::{ReadValue, WriteValue};

/// A raw bitmask value that indicates the presence of certain fields.
pub type Mask = u32;
/// A _resource ID_ that can be used to specify a particular window.
pub type Window = u32;
/// A _resoruce ID_ that can be used to specify a particular pixmap (a.k.a. texture).
pub type Pixmap = u32;
/// A _resource ID_ that can be used to specify a particular cursor appearance.
///
/// For example, the 'arrow' appearance of the cursor may be represented by a
/// [Cursor] resource ID.
pub type Cursor = u32;
/// A _resource ID_ that can be used to specify a particular system font.
pub type Font = u32;
/// A _resource ID_ that can be used to specify a particular gcontext.
///
/// TODO: What's a gcontext?
pub type Gcontext = u32;
/// A _resource ID_ that can be used to specify a particular colormap.
///
/// A colormap can be thought of as a palette of colors - it allows a limited
/// number of colors to be represented with a lower color depth than they might
/// ordinarily use.
pub type Colormap = u32;
/// A _resource ID_ that can be used to specify either a [Window] or a [Pixmap].
pub type Drawable = u32;
/// A _resource ID_ that can be used to specify either a [Font] or a [Gcontext].
pub type Fontable = u32;
/// An ID representing a string of text that has been registered with the X server.
///
/// An [Atom] provides a fixed-length representation of what may be a longer
/// string of text. It allows messages, such as requests, to remain a fixed
/// length, even if the text that has been registered with the X server is longer
/// than four bytes.
pub type Atom = u32;
/// An ID representing a 'visual'.
///
/// TODO: What is a visual?
pub type VisualId = u32;
/// A timestamp expressed in milliseconds, typically since the last server reset.
pub type Timestamp = u32;

pub type Keysym = u32;
pub type Keycode = u8;
pub type Button = u8;

/// A UTF-16-encoded character.
pub type Char2b = (u8, u8);
/// A pair of two-dimensional coordinates; x and y.
pub type Point = (i16, i16);

pub enum BitGravity {
	Forget,
	NorthWest,
	North,
	NorthEast,
	West,
	Center,
	East,
	SouthWest,
	South,
	SouthEast,
	Static,
}

pub enum WinGravity {
	Unmap,
	NorthWest,
	North,
	NorthEast,
	West,
	Center,
	East,
	SouthWest,
	South,
	SouthEast,
	Static,
}

pub enum HostFamily {
	Internet,
	Decnet,
	Chaos,
	ServerInterpreted,
	InternetV6,
}

impl WriteValue for Char2b {
	fn write_1b(self) -> WriteResult<u8> {
		// A two-byte character obviously can't be contained within a single byte.
		Err(WriteError::CapacityTooLow)
	}

	fn write_2b(self) -> WriteResult<u16> {
		Ok(u16::from_ne_bytes([self.0, self.1]))
	}

	fn write_4b(self) -> WriteResult<u32> {
		// Cast the existing `write_2b` result as `u32` for simplicity.
		Ok(self.write_2b()? as u32)
	}
}

impl ReadValue for Char2b {
	fn read_1b(_byte: u8) -> ReadResult<Self>
	where
		Self: Sized,
	{
		// A two-byte character obviously can't be contained within a single byte.
		Err(ReadError::UnsupportedSize)
	}

	fn read_2b(bytes: u16) -> ReadResult<Self>
	where
		Self: Sized,
	{
		// Convert the [`u16`] value to a pair of bytes (with native endianness)
		let bytes = bytes.to_ne_bytes();

		Ok((bytes[0], bytes[1]))
	}

	fn read_4b(bytes: u32) -> ReadResult<Self>
	where
		Self: Sized,
	{
		// Cast `bytes` to a [`u16`] value and use the existing `read_2b` for
		// simplicity.
		Self::read_2b(bytes as u16)
	}
}

impl WriteValue for Point {
	fn write_1b(self) -> WriteResult<u8> {
		// A [`Point`] is a pair of two-byte coordinates, and can therefore
		// only be written to a `u32` value.
		Err(WriteError::CapacityTooLow)
	}

	fn write_2b(self) -> WriteResult<u16> {
		// A [`Point`] is a pair of two-byte coordinates, and can therefore
		// only be written to a `u32` value.
		Err(WriteError::CapacityTooLow)
	}

	fn write_4b(self) -> WriteResult<u32> {
		// Convert the `x` and `y` coordinates to pairs of native-endianness
		// bytes so they can be written (x first, then y) to a `u32` value.
		let x = self.0.to_ne_bytes();
		let y = self.1.to_ne_bytes();

		// Create a `u32` value from four bytes: x's and y's pairs of bytes.
		Ok(u32::from_ne_bytes([x[0], x[1], y[0], y[1]]))
	}
}

impl ReadValue for Point {
	fn read_1b(_byte: u8) -> ReadResult<Self>
	where
		Self: Sized,
	{
		// A point must be a pair of 16-bit values, i.e. 32 bits total.
		Err(ReadError::UnsupportedSize)
	}

	fn read_2b(_bytes: u16) -> ReadResult<Self>
	where
		Self: Sized,
	{
		// A point must be a pair of 16-bit values, i.e. 32 bits total.
		Err(ReadError::UnsupportedSize)
	}

	fn read_4b(bytes: u32) -> ReadResult<Self>
	where
		Self: Sized,
	{
		// Convert the `u32` value to a list of native-endianness bytes.
		let bytes = bytes.to_ne_bytes();

		Ok((
			i16::from_ne_bytes([bytes[0], bytes[1]]), // x
			i16::from_ne_bytes([bytes[2], bytes[3]]), // y
		))
	}
}

impl WriteValue for BitGravity {
	fn write_1b(self) -> WriteResult<u8> {
		Ok(match self {
			Self::Forget => 0,
			Self::NorthWest => 1,
			Self::North => 2,
			Self::NorthEast => 3,
			Self::West => 4,
			Self::Center => 5,
			Self::East => 6,
			Self::SouthWest => 7,
			Self::South => 8,
			Self::SouthEast => 9,
			Self::Static => 10,
		})
	}

	fn write_2b(self) -> WriteResult<u16> {
		Ok(self.write_1b()? as u16)
	}

	fn write_4b(self) -> WriteResult<u32> {
		Ok(self.write_1b()? as u32)
	}
}

impl ReadValue for BitGravity {
	fn read_1b(byte: u8) -> ReadResult<Self>
	where
		Self: Sized,
	{
		match byte {
			0 => Ok(Self::Forget),
			1 => Ok(Self::NorthWest),
			2 => Ok(Self::North),
			3 => Ok(Self::NorthEast),
			4 => Ok(Self::West),
			5 => Ok(Self::Center),
			6 => Ok(Self::East),
			7 => Ok(Self::SouthWest),
			8 => Ok(Self::South),
			9 => Ok(Self::SouthEast),
			10 => Ok(Self::Static),
			// If none of those values have matched, then we don't know how to
			// read this as a [`BitGravity`].
			_ => Err(ReadError::InvalidData),
		}
	}

	fn read_2b(bytes: u16) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::read_1b(bytes as u8)
	}

	fn read_4b(bytes: u32) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::read_1b(bytes as u8)
	}
}

impl WriteValue for WinGravity {
	fn write_1b(self) -> WriteResult<u8> {
		Ok(match self {
			Self::Unmap => 0,
			Self::NorthWest => 1,
			Self::North => 2,
			Self::NorthEast => 3,
			Self::West => 4,
			Self::Center => 5,
			Self::East => 6,
			Self::SouthWest => 7,
			Self::South => 8,
			Self::SouthEast => 9,
			Self::Static => 10,
		})
	}

	fn write_2b(self) -> WriteResult<u16> {
		Ok(self.write_1b()? as u16)
	}

	fn write_4b(self) -> WriteResult<u32> {
		Ok(self.write_1b()? as u32)
	}
}

impl ReadValue for WinGravity {
	fn read_1b(byte: u8) -> ReadResult<Self>
	where
		Self: Sized,
	{
		match byte {
			0 => Ok(Self::Unmap),
			1 => Ok(Self::NorthWest),
			2 => Ok(Self::North),
			3 => Ok(Self::NorthEast),
			4 => Ok(Self::West),
			5 => Ok(Self::Center),
			6 => Ok(Self::East),
			7 => Ok(Self::SouthWest),
			8 => Ok(Self::South),
			9 => Ok(Self::SouthEast),
			10 => Ok(Self::Static),
			// If none of those values have matched, then we don't know how to
			// read this as a [`WinGravity`].
			_ => Err(ReadError::InvalidData),
		}
	}

	fn read_2b(bytes: u16) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::read_1b(bytes as u8)
	}

	fn read_4b(bytes: u32) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::read_1b(bytes as u8)
	}
}

impl WriteValue for HostFamily {
	fn write_1b(self) -> WriteResult<u8> {
		Ok(match self {
			Self::Internet => 0,
			Self::Decnet => 1,
			Self::Chaos => 2,
			Self::ServerInterpreted => 5,
			Self::InternetV6 => 6,
		})
	}

	fn write_2b(self) -> WriteResult<u16> {
		Ok(self.write_1b()? as u16)
	}

	fn write_4b(self) -> WriteResult<u32> {
		Ok(self.write_1b()? as u32)
	}
}

impl ReadValue for HostFamily {
	fn read_1b(byte: u8) -> ReadResult<Self>
	where
		Self: Sized,
	{
		match byte {
			0 => Ok(Self::Internet),
			1 => Ok(Self::Decnet),
			2 => Ok(Self::Chaos),
			5 => Ok(Self::ServerInterpreted),
			6 => Ok(Self::InternetV6),
			// If the given byte is not one of those values, then it is not a
			// valid [`HostFamily`] byte.
			_ => Err(ReadError::InvalidData),
		}
	}

	fn read_2b(bytes: u16) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::read_1b(bytes as u8)
	}

	fn read_4b(bytes: u32) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::read_1b(bytes as u8)
	}
}
