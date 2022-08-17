// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io;

use super::Word;

/// An error in reading or writing a value with [`ReadValue`] or [`WriteValue`].
pub enum ReadWriteError {
	/// There was not enough space to contain the value.
	NotEnoughSpace,
}

/// Shorthand for `Result<T>, ReadWriteError>`.
pub type ReadWriteResult<T = ()> = Result<T, ReadWriteError>;

#[allow(dead_code)]
pub enum SerialError {
	/// There was not enough capacity in the buffer to write the serialized data.
	///
	/// No changes were made to the buffer.
	CapacityTooLow,
}

/// Shorthand for `Result<(), SerialError>`.
#[allow(dead_code)]
pub type SerialResult = Result<(), SerialError>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum DeserialError {
	/// Unexpectedly reached the end of the provided data.
	///
	/// This could mean not enough data was provided.
	UnexpectedEndOfData,
	/// The data provided was invalid and could not be used to construct the object sucessfully.
	InvalidData,
}

/// Shorthand for `Result<T, DeserialError>`;
#[allow(dead_code)]
pub type DeserialResult<T> = Result<T, DeserialError>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum WordError {
	/// A slice of less than four bytes was provided.
	NotEnoughBytes,
	/// A slice of more than four bytes was provided.
	TooManyBytes,
}

#[allow(dead_code)]
pub type WordResult<T = Word> = Result<T, WordError>;

/// A trait object that can represent any error from XRB, or a [std::io::Error].
pub trait Error {}

/// A trait object that can represent any serialization error from XRB.
pub trait SerializationError {}

impl Error for ReadWriteError {}
impl Error for SerialError {}
impl Error for io::Error {}

impl SerializationError for ReadWriteError {}
impl SerializationError for SerialError {}

/// Shorthand for `Result<T, Box<dyn Error>>`.
#[allow(dead_code)]
pub type XrbResult<T> = Result<T, Box<dyn Error>>;

/// Shorthand for `Result<T, Box<dyn SerializationError>>`.
#[allow(dead_code)]
pub type SerializationResult<T> = Result<T, Box<dyn SerializationError>>;
