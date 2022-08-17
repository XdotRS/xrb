// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub trait GenericError: std::fmt::Debug {}

pub trait ReadWriteError: GenericError {}

#[derive(Debug, Eq, PartialEq)]
pub enum WriteError {
	/// Not enough capacity was given for the writer to write its data.
	CapacityTooLow,
	/// Not enough information about the object/value was given to be able write it correctly.
	MissingInfo,
}
impl ReadWriteError for WriteError {}
impl GenericError for WriteError {}

#[derive(Debug, Eq, PartialEq)]
pub enum ReadError {
	/// The data being read was formatted incorrectly.
	InvalidData,
	/// Reached the end of the provided data before reading was complete.
	///
	/// Likely indicates that not enough data was provided to the reader.
	UnexpectedEndOfData,
	/// The data supplied to the reader was of an unsupported length.
	UnsupportedLength,
}
impl ReadWriteError for ReadError {}
impl GenericError for ReadError {}

/// A generic result; shorthand for `Result<T, Box<dyn GenericError>>`.
pub type GenResult<T = ()> = Result<T, Box<dyn GenericError>>;

/// A result potentially generating either a [`ReadError`] or a [`WriteError`].
///
/// Shorthand for `Result<T, Box<dyn ReadWriteError>>`.
pub type ReadWriteResult<T = ()> = Result<T, Box<dyn ReadWriteError>>;
/// A result generating a [`WriteError`]. Shorthand for `Result<T, WriteError>`.
pub type WriteResult<T = ()> = Result<T, WriteError>;
/// A result generating a [`ReadError`]. Shorthand for `Result<T, ReadError>`.
pub type ReadResult<T> = Result<T, ReadError>;
