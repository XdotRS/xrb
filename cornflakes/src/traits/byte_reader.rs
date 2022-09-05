// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::FromBytes;
use crate::IoResult;

use bytes::Buf;

pub trait ByteReader: Buf {
	/// Reads a [`FromBytes`] implementing type.
	///
	/// This is equivalent to calling [`T::read_from`] with `self`.
	///
	/// [`T::read_from`]: FromBytes::read_from
	fn read<T>(&mut self) -> IoResult<T>
	where
		T: FromBytes,
		Self: Sized,
	{
		T::read_from(self)
	}

	/// Continuously reads `T`s to a vector until the end of this [`ByteReader`].
	///
	/// This will call [`T::read_vectored_from`] with `self`.
	///
	/// [`T::read_vectored_from`]: FromBytes::read_vectored_from
	fn read_all<T>(&mut self) -> IoResult<Vec<T>>
	where
		T: FromBytes,
		Self: Sized,
	{
		T::read_vectored_from(self)
	}

	// These are a single byte. They have no endianness. {{{

	/// Reads a single [`u8`] value from one byte in `self`.
	fn read_u8(&mut self) -> u8 {
		self.get_u8()
	}

	/// Reads a single [`i8`] value from one byte in `self`.
	fn read_i8(&mut self) -> i8 {
		self.get_i8()
	}

	// }}}

	// Native-endianness {{{
	//     Unsigned {{{

	/// Reads a single [`u16`] value from two bytes in `self` using native
	/// endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn read_u16_ne(&mut self) -> u16 {
		if cfg!(target_endian = "big") {
			self.get_u16()
		} else {
			self.get_u16_le()
		}
	}

	/// Reads a single [`u32`] value from four bytes in `self` using native
	/// endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn read_u32_ne(&mut self) -> u32 {
		if cfg!(target_endian = "big") {
			self.get_u32()
		} else {
			self.get_u32()
		}
	}

	/// Reads a single [`u64`] value from eight bytes in `self` using native
	/// endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn read_u64_ne(&mut self) -> u64 {
		if cfg!(target_endian = "big") {
			self.get_u64()
		} else {
			self.get_u64()
		}
	}

	/// Reads a single [`u128`] value from sixteen bytes in `self` using native
	/// endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn read_u128_ne(&mut self) -> u128 {
		if cfg!(target_endian = "big") {
			self.get_u128()
		} else {
			self.get_u128_le()
		}
	}

	//     }}}

	//     Signed {{{

	/// Reads a single [`i16`] value from two bytes in `self` using native
	/// endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn read_i16_ne(&mut self) -> i16 {
		if cfg!(target_endian = "big") {
			self.get_i16()
		} else {
			self.get_i16_le()
		}
	}

	/// Reads a single [`i32`] value from four bytes in `self` using native
	/// endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn read_i32_ne(&mut self) -> i32 {
		if cfg!(target_endian = "big") {
			self.get_i32()
		} else {
			self.get_i32()
		}
	}

	/// Reads a single [`i64`] value from eight bytes in `self` using native
	/// endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn read_i64_ne(&mut self) -> i64 {
		if cfg!(target_endian = "big") {
			self.get_i64()
		} else {
			self.get_i64()
		}
	}

	/// Reads a single [`i128`] value from sixteen bytes in `self` using native
	/// endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn read_i128_ne(&mut self) -> i128 {
		if cfg!(target_endian = "big") {
			self.get_i128()
		} else {
			self.get_i128_le()
		}
	}

	//     }}}

	//     Floating-point {{{

	/// Reads a single [`f32`] value from four bytes in `self` using native
	/// endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn read_f32_ne(&mut self) -> f32 {
		if cfg!(target_endian = "big") {
			self.get_f32()
		} else {
			self.get_f32_le()
		}
	}

	/// Reads a single [`f64`] value from eight bytes in `self` using native
	/// endianness.
	///
	/// Native endianness means that the endianness of the system is used to
	/// write the value. On most systems this will be little endian, but
	/// sometimes it will be big endian.
	fn read_f64_ne(&mut self) -> f64 {
		if cfg!(target_endian = "big") {
			self.get_f64()
		} else {
			self.get_f64()
		}
	}

	//     }}}
	// }}}

	// Big-endianness {{{
	//     Unsigned {{{

	/// Reads a single [`u16`] value from two bytes in `self` using big
	/// endianness.
	fn read_u16_be(&mut self) -> u16 {
		self.get_u16()
	}

	/// Reads a single [`u32`] value from four bytes in `self` using big
	/// endianness.
	fn read_u32_be(&mut self) -> u32 {
		self.get_u32()
	}

	/// Reads a single [`u64`] value from eight bytes in `self` using big
	/// endianness.
	fn read_u64_be(&mut self) -> u64 {
		self.get_u64()
	}

	/// Reads a single [`u128`] value from sixteen bytes in `self` using big
	/// endianness.
	fn read_u128_be(&mut self) -> u128 {
		self.get_u128()
	}

	//     }}}

	//     Signed {{{

	/// Reads a single [`i16`] value from two bytes in `self` using big
	/// endianness.
	fn read_i16_be(&mut self) -> i16 {
		self.get_i16()
	}

	/// Reads a single [`i32`] value from four bytes in `self` using big
	/// endianness.
	fn read_i32_be(&mut self) -> i32 {
		self.get_i32()
	}

	/// Reads a single [`i64`] value from eight bytes in `self` using big
	/// endianness.
	fn read_i64_be(&mut self) -> i64 {
		self.get_i64()
	}

	/// Reads a single [`i128`] value from sixteen bytes in `self` using big
	/// endianness.
	fn read_i128_be(&mut self) -> i128 {
		self.get_i128()
	}

	//     }}}

	//     Floating-point {{{

	/// Reads a single [`f32`] value from four bytes in `self` using big
	/// endianness.
	fn read_f32_be(&mut self) -> f32 {
		self.get_f32()
	}

	/// Reads a single [`f64`] value from eight bytes in `self` using big
	/// endianness.
	fn read_f64_be(&mut self) -> f64 {
		self.get_f64()
	}

	//     }}}
	// }}}

	// Little-endianness {{{
	//     Unsigned {{{

	/// Reads a single [`u16`] value from two bytes in `self` using little
	/// endianness.
	fn read_u16_le(&mut self) -> u16 {
		self.get_u16_le()
	}

	/// Reads a single [`u32`] value from four bytes in `self` using little
	/// endianness.
	fn read_u32_le(&mut self) -> u32 {
		self.get_u32_le()
	}

	/// Reads a single [`u64`] value from eight bytes in `self` using little
	/// endianness.
	fn read_u64_le(&mut self) -> u64 {
		self.get_u64_le()
	}

	/// Reads a single [`u128`] value from sixteen bytes in `self` using little
	/// endianness.
	fn read_u128_le(&mut self) -> u128 {
		self.get_u128_le()
	}

	//     }}}

	//     Signed {{{

	/// Reads a single [`i16`] value from two bytes in `self` using little
	/// endianness.
	fn read_i16_le(&mut self) -> i16 {
		self.get_i16_le()
	}

	/// Reads a single [`i32`] value from four bytes in `self` using little
	/// endianness.
	fn read_i32_le(&mut self) -> i32 {
		self.get_i32_le()
	}

	/// Reads a single [`i64`] value from eight bytes in `self` using little
	/// endianness.
	fn read_i64_le(&mut self) -> i64 {
		self.get_i64_le()
	}

	/// Reads a single [`i128`] value from sixteen bytes in `self` using little
	/// endianness.
	fn read_i128_le(&mut self) -> i128 {
		self.get_i128_le()
	}

	//     }}}

	//     Floating-point

	/// Reads a single [`f32`] value from four bytes in `self` using little
	/// endianness.
	fn read_f32_le(&mut self) -> f32 {
		self.get_f32_le()
	}

	/// Reads a single [`f64`] value from eight bytes in `self` using little
	/// endianness.
	fn read_f64_le(&mut self) -> f64 {
		self.get_f64_le()
	}

	//     }}}
	// }}}
}
