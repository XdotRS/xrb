// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bytes::BufMut;

use crate::error_handling::WriteResult;

/// Write a _value_ as a 1-byte, 2-byte, or 4-byte unsigned integer ([`u8`], [`u16`], or [`u32`]).
///
/// A _value_ is considered to be data that refers to one individual 'piece' of information. A
/// _value_ __does not__ contain any other values.
///
/// A _value_ is no more than four bytes in length, and can be expressed as a [`u8`], [`u16`], or
/// [`u32`] value.
///
/// The term _value_ is used here to refer to the concept of a value or field: an [`Option`] of a
/// a value can still be considered a value, because it is effectively just the value itself, even
/// though it might technically have an unnamed field. The point is that [`WriteValue`] should not
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
pub trait WriteValue {
	/// Writes [`Self`] to a single byte ([`u8`]).
	fn write_1b(self) -> WriteResult<u8>;
	/// Writes [`Self`] to two bytes ([`u16`]) using the system's native endianness.
	fn write_2b(self) -> WriteResult<u16>;
	/// Writes [`Self`] to four bytes ([`u32`]) using the system's native endianness.
	fn write_4b(self) -> WriteResult<u32>;

	fn write_1b_to(self, buf: &mut impl BufMut) -> WriteResult
	where
		Self: Sized,
	{
		buf.put_u8(self.write_1b()?);

		Ok(())
	}

	fn write_2b_to(self, buf: &mut impl BufMut) -> WriteResult
	where
		Self: Sized,
	{
		if cfg!(target_endian = "big") {
			buf.put_u16(self.write_2b()?); // system is big endian
		} else {
			buf.put_u16_le(self.write_2b()?); // system is little endian
		}

		Ok(())
	}

	fn write_4b_to(self, buf: &mut impl BufMut) -> WriteResult
	where
		Self: Sized,
	{
		if cfg!(target_endian = "big") {
			buf.put_u32(self.write_4b()?); // system is big endian
		} else {
			buf.put_u32_le(self.write_4b()?); // system is little endian
		}

		Ok(())
	}
}

/// Implements [`WriteValue`] for multiple primitive types that can cast to `u8`, `u16` and `u32`.
macro_rules! writer {
	(
		$($T:ty),+ // List of types to implement `WriteValue` on.
		$(,)* // Last comma is optional.
	) => {
		$(
			impl WriteValue for $T {
				fn write_1b(self) -> WriteResult<u8> {
					Ok(self as u8)
				}

				fn write_2b(self) -> WriteResult<u16> {
					Ok(self as u16)
				}

				fn write_4b(self) -> WriteResult<u32> {
					Ok(self as u32)
				}
			}
		)+
	};
}

// Implement [`WriteValue`] for 1-byte, 2-byte, 4-byte, and `size` primitives //
writer! {
	u8, u16, u32, usize, // unsigned
	i8, i16, i32, isize, // signed
}

writer!(bool, char);

