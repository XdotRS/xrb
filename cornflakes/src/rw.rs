// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::util::reader::ReadError;
use crate::util::reader::Reader;

pub trait Writer {}

/// Reads a type from bytes.
#[doc(notable_trait)]
pub trait ReadBytes {
	/// Reads [`Self`] from a [`Reader`].
	fn read(reader: &mut impl Reader) -> Result<Self, ReadError>
	where
		Self: Sized;
}

/// Reads a type from a specific number of bytes.
pub trait ReadSized {
	/// Reads [`Self`] from a [`Reader`] using the given number of bytes.
	fn read_with_size(num_bytes: usize, reader: &mut impl Reader) -> Result<Self, ReadError>
	where
		Self: Sized;
}

/// Reads a list of elements from bytes.
#[doc(notable_trait)]
pub trait ReadList {
	/// Reads a list of values from a [`Reader`] using the given length of the
	/// list.
	fn read(length: usize, reader: &mut impl Reader) -> Result<Self, ReadError>
	where
		Self: Sized;
}

/// Writes a type as bytes.
#[doc(notable_trait)]
pub trait WriteBytes {
	/// Writes `self` as bytes to a [`Writer`].
	fn write(&self, writer: &mut impl Writer) -> Result<(), ReadError>
	where
		Self: Sized;
}

/// Writes a type as a specific number of bytes.
pub trait WriteSized {
	/// Writes `self` as the given number of bytes to a [`Writer`].
	fn write_with_size(&self, num_bytes: usize, writer: &mut impl Writer) -> Result<(), ReadError>
	where
		Self: Sized;
}

// This function is unused, but writing it here asserts that these traits are
// _object safe_; that is, that the Rust compiler will generate an error if any
// of these traits are accidentally made _object unsafe_, which means that they
// cannot be used with the `dyn` keyword.
fn _assert_object_safety(
	_read_bytes: &dyn ReadBytes,
	_read_sized: &dyn ReadSized,
	_read_list: &dyn ReadList,
	_write_bytes: &dyn WriteBytes,
	_write_sized: &dyn WriteSized,
) {
}
