// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{ByteReader, ByteSize, ToBytes};
use crate::IoResult;

use bytes::BufMut;

/// Utilities to extend the functionality of [bytes::BufMut] for serialization.
pub trait ByteWriter: BufMut {
	/// Writes a [`ToBytes`] implementing type as bytes.
	///
	/// This is equivalent to calling [`thing.write_to`] with this writer.
	///
	/// [`thing.write_to`]: ToBytes::write_to
	fn write<T>(&mut self, thing: T) -> IoResult
	where
		T: ToBytes + ByteSize,
		Self: Sized,
	{
		// Limit `self` to `thing`'s given `byte_size()`.
		thing.write_to(&mut self.limit(thing.byte_size()))?;

		Ok(())
	}

	/// Writes a slice of a [`ToBytes`] implementing type as bytes.
	///
	/// This uses [`T::write_vectored_to`].
	///
	/// [`T::write_vectored_to`]: ToBytes::write_vectored_to
	fn write_all<T>(&mut self, things: &[T]) -> IoResult
	where
		T: ToBytes + ByteSize,
		Self: Sized,
	{
		T::write_vectored_to(things, self)
	}

	/// Writes the given bytes.
	fn write_bytes<T>(&mut self, bytes: impl ByteReader)
	where
		Self: Sized,
	{
		self.put(bytes);
	}

	/// Writes the given bytes but stops at the given limit.
	///
	/// # Examples
	/// In this example, messages must be written on the wire as no more than
	/// 32 bytes:
	/// ```rust
	/// wire.write_bytes_limited(message.to_bytes_vec(), 32);
	/// ```
	fn write_bytes_limited(&mut self, bytes: impl ByteReader, limit: usize) {
		self.limit(limit).put(bytes);
	}

	/// Writes the same byte `count` many times.
	///
	/// # Examples
	/// In order to fill unused bytes in a message, it is convenient to be able
	/// to write an exact number of the same byte:
	/// ```rust
	/// bytes.write_u32_ne(73);
	///
	/// // Since this message must have a total length of 32 bytes, and a single
	/// // [`u32`] value is merely 4 bytes, we must fill the rest with unused
	/// // bytes. The exact value of these bytes does not matter, but remember
	/// // that they might not necessarily be zero.
	/// bytes.write_mant(0, 28);
	/// ```
	fn write_many(&mut self, byte: u8, count: usize) {
		self.put_bytes(byte, count);
	}

	// These are a single byte. They have no endianness. {{{

	/// Writes a single [`u8`] value as one byte.
	fn write_u8(&mut self, val: u8) {
		self.put_u8(val);
	}

	/// Writes a single [`i8`] value as one byte.
	fn write_i8(&mut self, val: i8) {
		self.put_i8(val);
	}

	// }}}

	// Native-endianness {{{
	//     Unsigned {{{

	/// Writes a single [`u16`] value as two bytes using native endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn write_u16_ne(&mut self, val: u16) {
		self.put_slice(&val.to_ne_bytes());
	}

	/// Writes a single [`u32`] value as four bytes using native endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn write_u32_ne(&mut self, val: u32) {
		self.put_slice(&val.to_ne_bytes());
	}

	/// Writes a single [`u64`] value as eight bytes using native endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn write_u64_ne(&mut self, val: u64) {
		self.put_slice(&val.to_ne_bytes());
	}

	/// Writes a single [`u128`] value as sixteen bytes using native endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn write_u128_ne(&mut self, val: u128) {
		self.put_slice(&val.to_ne_bytes());
	}

	//     }}}

	//     Signed {{{
	/// Writes a single [`i16`] value as two bytes using native endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn write_i16_ne(&mut self, val: i16) {
		self.put_slice(&val.to_ne_bytes());
	}

	/// Writes a single [`i32`] value as four bytes using native endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn write_i32_ne(&mut self, val: i32) {
		self.put_slice(&val.to_ne_bytes());
	}

	/// Writes a single [`i64`] value as eight bytes using native endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn write_i64_ne(&mut self, val: i64) {
		self.put_slice(&val.to_ne_bytes());
	}

	/// Writes a single [`i128`] value as sixteen bytes using native endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn write_i128_ne(&mut self, val: i128) {
		self.put_slice(&val.to_ne_bytes());
	}

	//     }}}

	//     Floating-point {{{

	/// Writes a single [`f32`] value as four bytes using native endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn write_f32_ne(&mut self, val: f32) {
		self.put_slice(&val.to_ne_bytes());
	}

	/// Writes a single [`f64`] value as eight bytes using native endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn write_f64_ne(&mut self, val: f64) {
		self.put_slice(&val.to_ne_bytes());
	}

	//     }}}
	// }}}

	// Big-endianness {{{
	//     Unsigned {{{

	/// Writes a single [`u16`] value as two bytes using big endianness.
	fn write_u16_be(&mut self, val: u16) {
		self.put_slice(&val.to_be_bytes());
	}

	/// Writes a single [`u32`] value as four bytes using big endianness.
	fn write_u32_be(&mut self, val: u32) {
		self.put_slice(&val.to_be_bytes());
	}

	/// Writes a single [`u64`] value as eight bytes using big endianness.
	fn write_u64_be(&mut self, val: u64) {
		self.put_slice(&val.to_be_bytes());
	}

	/// Writes a single [`u128`] value as sixteen bytes using big endianness.
	fn write_u128_be(&mut self, val: u128) {
		self.put_slice(&val.to_be_bytes());
	}

	//     }}}

	//     Signed {{{

	/// Writes a single [`i16`] value as two bytes using big endianness.
	fn write_i16_be(&mut self, val: i16) {
		self.put_slice(&val.to_be_bytes());
	}

	/// Writes a single [`i32`] value as four bytes using big endianness.
	fn write_i32_be(&mut self, val: i32) {
		self.put_slice(&val.to_be_bytes());
	}

	/// Writes a single [`i64`] value as eight bytes using big endianness.
	fn write_i64_be(&mut self, val: i64) {
		self.put_slice(&val.to_be_bytes());
	}

	/// Writes a single [`i128`] value as sixteen bytes using big endianness.
	fn write_i128_be(&mut self, val: i128) {
		self.put_slice(&val.to_be_bytes());
	}

	//     }}}

	//     Floating-point {{{

	/// Writes a single [`f32`] value as four bytes using big endianness.
	fn write_f32_be(&mut self, val: f32) {
		self.put_slice(&val.to_be_bytes());
	}

	/// Writes a single [`f64`] value as eight bytes using big endianness.
	fn write_f64_be(&mut self, val: f64) {
		self.put_slice(&val.to_be_bytes());
	}

	//     }}}
	// }}}

	// Little-endianness {{{
	//     Unsigned {{{

	/// Writes a single [`u16`] value as two bytes using little endianness.
	fn write_u16_le(&mut self, val: u16) {
		self.put_slice(&val.to_le_bytes());
	}

	/// Writes a single [`u32`] value as four bytes using little endianness.
	fn write_u32_le(&mut self, val: u32) {
		self.put_slice(&val.to_le_bytes());
	}

	/// Writes a single [`u64`] value as eight bytes using little endiannes.
	fn write_u64_le(&mut self, val: u64) {
		self.put_slice(&val.to_le_bytes());
	}

	/// Writes a single [`u128`] value as sixteen bytes using little endiannes.
	fn write_u128_le(&mut self, val: u128) {
		self.put_slice(&val.to_le_bytes());
	}

	//     }}}

	//     Signed {{{

	/// Writes a single [`i16`] value as two bytes using little endianness.
	fn write_i16_le(&mut self, val: i16) {
		self.put_slice(&val.to_le_bytes());
	}

	/// Writes a single [`i32`] value as four bytes using little endianness.
	fn write_i32_le(&mut self, val: i32) {
		self.put_slice(&val.to_le_bytes());
	}

	/// Writes a single [`i64`] value as eight bytes using little endiannes.
	fn write_i64_le(&mut self, val: i64) {
		self.put_slice(&val.to_le_bytes());
	}

	/// Writes a single [`i128`] value as sixteen bytes using little endiannes.
	fn write_i128_le(&mut self, val: i128) {
		self.put_slice(&val.to_le_bytes());
	}

	//     }}}

	//     Floating point {{{

	/// Writes a single [`f32`] value as four bytes using little endianness.
	fn write_f32_le(&mut self, val: f32) {
		self.put_slice(&val.to_le_bytes());
	}

	/// Writes a single [`f64`] value as eight bytes using little endianness.
	fn write_f64_le(&mut self, val: f64) {
		self.put_slice(&val.to_le_bytes());
	}

	//     }}}
	// }}}
}
