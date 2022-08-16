// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod read_value;
mod write_value;

use bytes::{Buf, BufMut};

pub use read_value::ReadValue;
pub use write_value::WriteValue;

/// An error in reading or writing a value with [`ReadValue`] or [`WriteValue`].
pub enum ReadWriteError {
	/// There was not enough space to contain the value.
	NotEnoughSpace,
}

/// Shorthand for `Result<T>, ReadWriteError>`.
pub type ReadWriteResult<T = ()> = Result<T, ReadWriteError>;

pub enum SerializeError {
	/// There was not enough capacity in the buffer to write the serialized data.
	///
	/// No changes were made to the buffer.
	CapacityTooLow,
}

// TODO: Redo `DeserializeError`s. An early data termination is indeed critical, but should be able
//       to be handled for most of the X11 protocol: if the length of the current message being
//       deserialized is known, then the whole message can be skipped. For example, an `Error` in X
//       will always be 32 bytes, so the buffer can be skipped to the end of that event. Otherwise,
//       the length field associated with the message can be used to skip to the end of the
//       message. Only if no length field could be read will early data termination become
//       critical.
//
//       Reminder: This is _not_ general deserialization, this is _only_ for X. Design it with that
//       in mind.

/// Wraps an error with whether deserialization terminated early (error `E`) or not (error `F`).
///
/// If deserialization returns without reaching the end of the data, then no more data can be read
/// from the buffer at all: there's no way to tell where the next object starts. An effort should
/// be made to always reach the end of the data for the given object, even if an error was
/// encountered, so that that error can returned as `FullyTerminated(E)`.
pub enum DeserializeError<E, F> {
	/// Deserialization terminated before the definitive end of the data.
	///
	/// Consider this the end of the buffer data was being read from. The location where the next
	/// object's data starts has been lost.
	EarlyTermination(E),
	/// The end of the data was reached, but there was another error encountered reading the data.
	///
	/// Always try to reach the end of the object's data if at all possible, even if an error was
	/// encountered, and return that error wrapped in `FullyTerminated(F)`.
	FullyTerminated(F),
}

pub type SerializeResult = Result<(), SerializeError>;
pub type DeserializeResult<T, E, F> = Result<T, DeserializeError<E, F>>;

/// Serializes a _data structure_ to bytes. _Values_ should implement [`WriteValue`] instead.
pub trait Serialize {
	/// Serializes the data structure to bytes.
	///
	/// Should have zero side effects.
	fn serialize(self) -> &'static [u8];

	/// Serializes the data structure to bytes in a [`BufMut`].
	///
	/// Do not implement this method directly: implement [`serialize()`](Serialize::serialize)
	/// instead.
	///
	/// Should have zero effects.
	fn serialize_to(self, buf: &mut impl BufMut) -> SerializeResult
	where
		Self: Sized,
	{
		let data = self.serialize();

		if buf.remaining_mut() < data.len() {
			// If there is not enough space remaining in the buffer, return a `NotEnoughCapacity`
			// error.
			return Err(SerializeError::CapacityTooLow);
		}

		Ok(buf.put(data))
	}
}

/// Deserializes a _data structure_ from bytes. _Values_ should implement [`ReadValue`] instead.
///
/// Implementations of [`Deserialize`] must be compatible with the existing implementation of
/// [`Serialize`] for their type.
///
/// `E` is the type of error returned if deserialization is terminated early, in which case the
/// buffer from which data was being read is no longer valid and should be discarded. If
/// deserialization reached the end of the object's data but encountered an error in the process,
/// error type `F` is returned.
pub trait Deserialize<E = (), F = ()>: Serialize {
	/// Deserializes the data structure from bytes in a [`Buf`].
	///
	/// Note that bytes representing the type of data structure may not be present if they were
	/// used to determine which data structure's `deserialize` function to call.
	///
	/// Should have zero side effects.
	fn deserialize(buf: &mut impl Buf) -> DeserializeResult<Self, E, F>
	where
		Self: Sized;

	/// Deserializes the data structure from bytes.
	///
	/// Do not implement this function: implement
	/// [`deserialize(buf)`](Deserialize::deserialize) instead.
	///
	/// An [`EarlyTermination`](DeserializeError::EarlyTermination) error returned by this function
	/// is not critical, as there is no next object in the provided bytes.
	fn deserialize_bytes(bytes: &[u8]) -> DeserializeResult<Self, E, F>
	where
		Self: Sized,
	{
		Self::deserialize(&mut bytes.clone())
	}
}
