// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod errors;

mod read_value;
mod write_value;

use bytes::{Buf, BufMut};

pub use read_value::ReadValue;
pub use write_value::WriteValue;

pub use errors::Error;
pub use errors::XrbResult;

pub use errors::SerializationError;
pub use errors::SerializationResult;

pub use errors::ReadWriteError;
pub use errors::ReadWriteResult;

pub use errors::SerialError;
pub use errors::SerialResult;

pub use errors::DeserialError;
pub use errors::DeserialResult;

use self::errors::WordError;
use self::errors::WordResult;

#[allow(dead_code)]
pub struct Word {
	pub data: (u8, u8, u8, u8),
}

#[allow(dead_code)]
impl Word {
	fn new(bytes: &[u8]) -> WordResult {
		if bytes.len() < 4 {
			return Err(WordError::NotEnoughBytes);
		}

		if bytes.len() > 4 {
			return Err(WordError::TooManyBytes);
		}

		Ok(Self {
			data: (bytes[0], bytes[1], bytes[2], bytes[3]),
		})
	}

	fn words(bytes: &[u8]) -> WordResult<Vec<Self>> {
		let vec: Vec<Self> = bytes
			.chunks_exact(4)
			.map(|chunk| {
				Self::new(chunk).expect(
					"`.chunks_exact(4)` didn't return a slice of length 4: should be impossible.",
				)
			})
			.collect();

		Ok(vec)
	}
}

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
	fn serialize_to(self, buf: &mut impl BufMut) -> SerialResult
	where
		Self: Sized,
	{
		let data = self.serialize();

		if buf.remaining_mut() < data.len() {
			// If there is not enough space remaining in the buffer, return a `NotEnoughCapacity`
			// error.
			return Err(SerialError::CapacityTooLow);
		}

		Ok(buf.put(data))
	}
}

/// Deserializes a _data structure_ from bytes. _Values_ should implement [`ReadValue`] instead.
///
/// Implementations of [`Deserialize`] must be compatible with the existing implementation of
/// [`Serialize`] for their type.
pub trait Deserialize {
	/// Deserializes the data structure from bytes in a [`Buf`].
	///
	/// Note that bytes representing the type of data structure may not be present if they were
	/// used to determine which data structure's `deserialize` function to call.
	///
	/// Should have zero side effects.
	fn deserialize(header: Word, buf: &mut impl Buf) -> DeserialResult<Self>
	where
		Self: Sized;

	/// Deserializes the data structure from bytes.
	///
	/// Do not implement this function: implement
	/// [`deserialize(buf)`](Deserialize::deserialize) instead.
	fn deserialize_bytes(header: Word, bytes: &[u8]) -> DeserialResult<Self>
	where
		Self: Sized,
	{
		Self::deserialize(header, &mut bytes.clone())
	}
}
