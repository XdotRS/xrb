// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod read_value;
mod write_value;

use bytes::{Buf, BufMut};

pub use read_value::ReadValue;
pub use write_value::WriteValue;

/// Serializes a _data structure_ to bytes. _Values_ should implement [`WriteValue`] instead.
pub trait Serialize {
	/// Serializes the data structure to bytes.
	fn serialize(self) -> &'static [u8];

	/// Serializes the data structure to bytes in a [`BufMut`].
	fn serialize_to(self, buf: &mut impl BufMut)
	where
		Self: Sized,
	{
		buf.put(self.serialize());
	}
}

/// Deserializes a _data structure_ from bytes. _Values_ should implement [`ReadValue`] instead.
///
/// Implementations of [`Deserialize`] must be compatible with the existing implementation of
/// [`Serialize`] for their type.
pub trait Deserialize: Serialize {
	/// Deserializes the data structure from bytes in a [`Buf`].
	///
	/// Note that bytes representing the type of data structure may not be present if they were
	/// used to determine which data structure's `deserialize` function to call.
	fn deserialize(buf: &mut impl Buf) -> Self
	where
		Self: Sized;

	/// Deserializes the data structure from bytes.
	///
	/// Note that bytes representing the type of data structure may not be present if they were
	/// used to determine which data structure's `deserialize` function to call.
	fn deserialize_bytes(bytes: &[u8]) -> Self
	where
		Self: Sized,
	{
		Self::deserialize(&mut bytes.clone())
	}
}
