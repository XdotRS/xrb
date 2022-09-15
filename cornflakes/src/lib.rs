// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(doc_notable_trait)]
// Deny the following clippy lints to enforce them:
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
// Warn for these lints, rather than denying them.
#![warn(clippy::use_self)]
// Warn for pedantic & cargo lints. They are allowed completely by default.
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
// Continue to allow these though.
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::module_name_repetitions)]

mod reader;
mod writer;

use reader::{ReadError, Reader};
use writer::{WriteError, Writer};

pub trait ByteCount {
	/// Returns the number of bytes that `self` will be [written] as.
	///
	/// [written]: Writable::write_to
	fn byte_count(&self) -> usize;
}

/// Reads a type from bytes.
#[doc(notable_trait)]
pub trait Readable {
	/// Reads [`Self`] from a [`Reader`].
	fn read_from(reader: &mut impl Reader) -> Result<Self, ReadError>
	where
		Self: Sized;
}

/// Reads a type from a specific number of bytes.
pub trait ReadableWithCount {
	/// Reads [`Self`] from a [`Reader`] using the given number of bytes.
	fn read_from_with_count(reader: &mut impl Reader, num_bytes: usize) -> Result<Self, ReadError>
	where
		Self: Sized;
}

/// Reads a list of elements from bytes.
#[doc(notable_trait)]
pub trait ReadableWithLength {
	/// Reads a list of values from a [`Reader`] using the given length of the
	/// list.
	fn read_from_with_length(reader: &mut impl Reader, length: usize) -> Result<Self, ReadError>
	where
		Self: Sized;
}

/// Writes a type as bytes.
#[doc(notable_trait)]
pub trait Writable {
	/// Writes `self` as bytes to a [`Writer`].
	fn write_to(&self, writer: &mut impl Writer) -> Result<(), WriteError>
	where
		Self: Sized;
}

/// Writes a type as a specific number of bytes.
pub trait WritableWithCount {
	/// Writes `self` as the given number of bytes to a [`Writer`].
	fn write_to_with_count(
		&self,
		writer: &mut impl Writer,
		num_bytes: usize,
	) -> Result<(), WriteError>
	where
		Self: Sized;
}

// This function is unused, but writing it here asserts that these traits are
// _object safe_; that is, that the Rust compiler will generate an error if any
// of these traits are accidentally made _object unsafe_, which means that they
// cannot be used with the `dyn` keyword.
fn _assert_object_safety(
	_byte_count: &dyn ByteCount,
	_read_bytes: &dyn Readable,
	_read_sized: &dyn ReadableWithCount,
	_read_list: &dyn ReadableWithLength,
	_write_bytes: &dyn Writable,
	_write_sized: &dyn WritableWithCount,
) {
}
