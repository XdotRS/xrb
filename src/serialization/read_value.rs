// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bytes::Buf;

use super::{ReadWriteError, WriteValue};

/// Read a _value_ from a 1-byte, 2-byte, or 4-byte unsigned integer ([`u8`], [`u16`], or [`u32`]).
///
/// A _value_ is considered to be data that refers to one individual 'piece' of information. A
/// _value_ __does not__ contain any other values.
///
/// A _value_ is no more than four bytes in length, and can be expressed as a [`u8`], [`u16`], or
/// [`u32`] value.
///
/// The term _value_ is used here to refer to the concept of a value or field: an [`Option`] of a
/// a value can still be considered a value, because it is effectively just the value itself, even
/// though it might technically have an unnamed field. The point is that [`ReadValue`] should not
/// be implemented for a data _structure_, such as an object, a list, a message, though it might be
/// implemented for the individual elements of an object, list, or message.
///
/// ## Examples of things considered _values_
/// - [`u8`]
/// - [`u16`]
/// - [`u32`]
/// - [`usize`]
/// - [`i8`]
/// - [`i16`]
/// - [`i32`]
/// - [`isize`]
/// - [`bool`]
/// - [`char`]
/// - [`BitGravity`](crate::proto::common::BitGravity)
/// - [`WinGravity`](crate::proto::common::WinGravity)
/// - [`Protocol`](crate::proto::common::Protocol)
/// - [`KeySym`](crate::proto::common::KeySym)
///
/// ## Examples of things _not_ considered _values_
/// - [`&str`](str)
/// - [`String`]
/// - [`&[u8]`](std::slice)
/// - [`Rect`](crate::proto::common::Rect)
/// - [`Host`](crate::proto::common::Host)
pub trait ReadValue: WriteValue {
	/// Read [`Self`] from a single byte ([`u8`]).
	fn read_1b(byte: u8) -> Result<Self, ReadWriteError>
	where
		Self: Sized;

	/// Read [`Self`] from two bytes ([`u16`]) using the system's native endianness.
	fn read_2b(bytes: u16) -> Result<Self, ReadWriteError>
	where
		Self: Sized;

	/// Read [`Self`] from four bytes ([`u32`]) using the system's native endianness.
	fn read_4b(bytes: u32) -> Result<Self, ReadWriteError>
	where
		Self: Sized;

	/// Read [`Self`] from a single byte in a [`Buf`].
	fn read_1b_from(buf: &mut impl Buf) -> Result<Self, ReadWriteError>
	where
		Self: Sized,
	{
		Ok(Self::read_1b(buf.get_u8())?)
	}

	/// Read [`Self`] from two bytes in a [`Buf`] using the system's native endianness.
	fn read_2b_from(buf: &mut impl Buf) -> Result<Self, ReadWriteError>
	where
		Self: Sized,
	{
		if cfg!(target_endian = "big") {
			Ok(Self::read_2b(buf.get_u16())?) // system is big endian
		} else {
			Ok(Self::read_2b(buf.get_u16_le())?) // system is little endian
		}
	}

	/// Read [`Self`] from four bytes in a [`Buf`] using the system's native endianness.
	fn read_4b_from(buf: &mut impl Buf) -> Result<Self, ReadWriteError>
	where
		Self: Sized,
	{
		if cfg!(target_endian = "big") {
			Ok(Self::read_4b(buf.get_u32())?) // system is big endian
		} else {
			Ok(Self::read_4b(buf.get_u32_le())?) // system is little endian
		}
	}
}

/// Implements [`ReadValue`] for one or more primitive types that can cast from `u8`, `u16` and
/// `u32`.
macro_rules! reader {
	(
		$($T:ty),+ // List of types to implement `ReadValue` on.
		$(,)* // Final comma is optional.
	) => {
		$(
			impl ReadValue for $T {
				fn read_1b(byte: u8) -> Result<Self, ReadWriteError> {
					Ok(byte as Self)
				}

				fn read_2b(bytes: u16) -> Result<Self, ReadWriteError> {
					Ok(bytes as Self)
				}

				fn read_4b(bytes: u32) -> Result<Self, ReadWriteError> {
					Ok(bytes as Self)
				}
			}
		)+
	};
}

// Implement [`ReadValue`] for 1-byte, 2-byte, 4-byte, and `size` primitives //
reader! {
	u8, u16, u32, usize, // unsigned
	i8, i16, i32, isize, // signed
}

impl ReadValue for bool {
	fn read_1b(byte: u8) -> Result<Self, ReadWriteError> {
		Ok(byte != 0)
	}

	fn read_2b(bytes: u16) -> Result<Self, ReadWriteError> {
		Ok(bytes != 0)
	}

	fn read_4b(bytes: u32) -> Result<Self, ReadWriteError> {
		Ok(bytes != 0)
	}
}

impl ReadValue for char {
	fn read_1b(byte: u8) -> Result<Self, ReadWriteError> {
		Ok(byte as char)
	}

	fn read_2b(bytes: u16) -> Result<Self, ReadWriteError> {
		Ok((bytes as u8) as char) // `char` can only be cast from `u8`
	}

	fn read_4b(bytes: u32) -> Result<Self, ReadWriteError> {
		Ok((bytes as u8) as char) // `char` can only be cast from `u8`
	}
}
