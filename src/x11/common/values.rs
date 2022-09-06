// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use cornflakes::*;
use std::io::{Error, ErrorKind};

use xrb_proc_macros::StaticByteSize;

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

pub type Char1b = u8;
pub type Char2b = u16;

#[derive(StaticByteSize)]
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

#[derive(StaticByteSize)]
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

#[derive(StaticByteSize)]
pub enum HostFamily {
	Internet,
	Decnet,
	Chaos,
	ServerInterpreted,
	InternetV6,
}

impl FromBytes for BitGravity {
	fn read_from(reader: &mut impl ByteReader) -> Result<Self, Error>
	where
		Self: Sized,
	{
		match reader.read_u8() {
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
			_ => Err(Error::new(
				ErrorKind::InvalidData,
				"data did not match a known BitGravity variant",
			)),
		}
	}
}

impl ToBytes for BitGravity {
	fn write_to(&self, writer: &mut impl ByteWriter) -> Result<(), Error> {
		match self {
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
		}
		.write_to(writer)
	}
}

impl FromBytes for WinGravity {
	fn read_from(reader: &mut impl ByteReader) -> Result<Self, Error>
	where
		Self: Sized,
	{
		match reader.read_u8() {
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
			_ => Err(Error::new(
				ErrorKind::InvalidData,
				"data did not match a known WinGravity variant",
			)),
		}
	}
}

impl ToBytes for WinGravity {
	fn write_to(&self, writer: &mut impl ByteWriter) -> Result<(), Error> {
		match self {
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
		}
		.write_to(writer)
	}
}
