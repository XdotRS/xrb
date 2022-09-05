// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod byte_reader;
mod byte_writer;

pub use byte_reader::ByteReader;
pub use byte_writer::ByteWriter;

use crate::IoResult;
use bytes::{Buf, BufMut};

pub trait ByteSize {
	/// The size of this type in bytes.
	fn byte_size(&self) -> usize;
}

/// The size of a type in bytes, for types whose size can be known at compile
/// time.
pub trait StaticByteSize {
	/// The size of this type in bytes.
	fn static_byte_size() -> usize;
}

/// Allows a type to be read from bytes in a [`ByteReader`].
pub trait FromBytes {
	/// Reads [`Self`] from a [`ByteReader`].
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized;

	/// Reads [`Self`]s until the end of a [`ByteReader`].
	///
	/// # Implementors note
	/// It is recommended to override this method if it can be optimized for
	/// this type.
	fn read_vectored_from(reader: &mut impl ByteReader) -> IoResult<Vec<Self>>
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
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult;

	/// Writes all of the provided `selves` to a [`ByteWriter`].
	///
	/// # Implementors note
	/// It is recommended to override this method if it can be optimized for
	/// this type.
	fn write_vectored_to(selves: &[Self], writer: &mut impl ByteWriter) -> IoResult
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
	///
	/// Equivalent to:
	/// ```rust
	/// let mut bytes: Vec<u8> = vec![];
	/// self.to_bytes(&mut bytes);
	/// ```
	fn to_bytes(&self) -> IoResult<Vec<u8>> {
		let mut bytes: Vec<u8> = vec![];
		self.write_to(&mut bytes)?;

		Ok(bytes)
	}
}
