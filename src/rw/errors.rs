// This Soruce Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/2.0/.

use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum WriteError {
	/// Not enough capacity was given for the writer to write its data.
	#[error("could not write data because the given capacity was too low")]
	CapacityTooLow,

	/// Not enough information about the writer was given to write itself correctly.
	#[error("could not write data because not enough information was provided")]
	MissingInfo,
}

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ReadError {
	/// The given input was not formatted correctly for this data type.
	#[error("could not read data because the input was not formatted correctly")]
	InvalidData,

	/// The reader does not support inputs of this size.
	#[error("could not read data because the input was an incorrect length")]
	UnsupportedSize,
}

pub type ReadResult<T> = Result<T, ReadError>;
pub type WriteResult<T> = Result<T, WriteError>;
