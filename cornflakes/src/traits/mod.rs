// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod byte_reader;
mod byte_writer;

pub use byte_reader::ByteReader;
pub use byte_writer::ByteWriter;

use bytes::{Buf, BufMut};
use std::io::Error;

/// Defines the size of a type in bytes.
pub trait ByteSize {
	/// The size of this type in bytes.
	fn byte_size(&self) -> usize;
}

/// Defines the size of a type in bytes for types whose byte size does not vary.
pub trait StaticByteSize {
	/// The size of this type in bytes.
	fn static_byte_size() -> usize
	where
		Self: Sized;
}

/// Allows a type to be read from bytes in a [`ByteReader`], given a byte size
/// to read with.
pub trait FromBytesWithSize {
	/// Reads [`Self`] from a [`ByteReader`] with the given byte `size`.
	///
	/// # Errors
	/// Returns [`ErrorKind::InvalidInput`] if the given byte size is not
	/// supported.
	///
	/// [`ErrorKind::InvalidInput`]: std::io::ErrorKind::InvalidInput
	fn read_from_with_size(reader: &mut impl ByteReader, size: usize) -> Result<Self, Error>
	where
		Self: Sized;
}

/// Allows a type to be written to bytes in a [`ByteWriter`], given a byte size
/// to write with.
pub trait ToBytesWithSize: ToBytes {
	/// Writes [`Self`] to a [`ByteWriter`] with the given byte `size`.
	///
	/// # Errors
	/// Returns [`ErrorKind::InvalidInput`] if the given byte size is not
	/// supported.
	///
	/// [`ErrorKind::InvalidInput`]: std::io::ErrorKind::InvalidInput
	fn write_to_with_size(&self, writer: &mut impl ByteWriter, size: usize) -> Result<(), Error>
	where
		Self: Sized;
}

/// Allows a type to be read from bytes in a [`ByteReader`].
pub trait FromBytes {
	/// Reads [`Self`] from a [`ByteReader`].
	fn read_from(reader: &mut impl ByteReader) -> Result<Self, Error>
	where
		Self: Sized;

	/// Reads [`Self`]s until the end of a [`ByteReader`].
	///
	/// # Implementors note
	/// It is recommended to override this method if it can be optimized for
	/// this type.
	fn read_vectored_from(reader: &mut impl ByteReader) -> Result<Vec<Self>, Error>
	where
		Self: Sized,
	{
		let mut things: Vec<Self> = vec![];

		while reader.has_remaining() {
			things.push(reader.read()?);
		}

		Ok(things)
	}
}

/// Allows a type to be written as bytes to a [`ByteWriter`].
pub trait ToBytes: ByteSize {
	/// Writes `self` to a [`ByteWriter`].
	///
	/// # Implementors note
	/// The number of bytes written must be equal to the number of bytes given
	/// by `self`'s [`ByteSize`] implementation.
	fn write_to(&self, writer: &mut impl ByteWriter) -> Result<(), Error>
	where
		Self: Sized;

	/// Writes all of the provided `selves` to a [`ByteWriter`].
	///
	/// # Implementors note
	/// It is recommended to override this method if it can be optimized for
	/// this type.
	fn write_vectored_to(selves: &[Self], writer: &mut impl ByteWriter) -> Result<(), Error>
	where
		Self: Sized,
	{
		for selv in selves {
			// Limit `writer` to `selv`'s given `byte_size()`.
			selv.write_to(&mut writer.limit(selv.byte_size()))?;
		}

		Ok(())
	}

	/// Writes `self` as bytes.
	fn to_bytes(&self) -> Result<Vec<u8>, Error>
	where
		Self: Sized,
	{
		let mut bytes: Vec<u8> = vec![];
		self.write_to(&mut bytes)?;

		Ok(bytes)
	}
}

// This is just here so that `cornflakes` won't compile if any of these traits
// are not object safe, to make sure they haven't accidentally been made
// object unsafe.
fn _assert_object_safety(
	_byte_size: &dyn ByteSize,
	_static_byte_size: &dyn StaticByteSize,
	_from_bytes_with_size: &dyn FromBytesWithSize,
	_to_bytes_with_size: &dyn ToBytesWithSize,
	_from_bytes: &dyn FromBytes,
	_to_bytes: &dyn ToBytes,
) {
}
