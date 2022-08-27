// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[macro_use]
mod wrapper_macro;

use crate::protocol::common::values::{Timestamp, Window};

use crate::errors::{ReadResult, WriteResult};
use crate::serialization::{ReadValue, WriteValue};

wrappers! {
	/// Allows fields to inherit their values from their 'parent'.
	///
	/// Similar in concept to an [`Option`] enum, this is a wrapper to provide
	/// a [`CopyFromParent`](Inherit::CopyFromParent) alternate value for fields,
	/// as defined in the X11 protocol.
	pub enum Inherit<T> {
		/// Wraps the indicated value, rather than inheriting the value.
		Value(T),
		/// Inherits the value from the 'parent'.
		///
		/// This does not _bind_ the value to that of its parent; that is,
		/// changes to this value in the parent are not reflected. This merely
		/// indicates that the corresponding value in the parent be _copied_.
		CopyFromParent = 0,
	}

	pub enum Relative<T> {
		Value(T),
		ParentRelative = 1,
	}

	/// Allows fields to indicate an 'any' state.
	///
	/// Similar in concept to an [`Option`] enum, this is a wrapper to provide
	/// an [`Any`](Specificity::Any) alternate value for fields, as defined in
	/// the X11 protocol.
	pub enum Specificity<T> {
		/// Wraps the indicated value, rather than representing an
		/// [`Any`](Specificity::Any) state.
		Specific(T),
		/// Indicates an 'any' state; _that any input is allowed_, for example.
		Any = 0,
	}

	/// Represents either a [`Specific`](Time::Specific) [`Timestamp`] or the
	/// [`Current`] time.
	pub enum Time {
		Specific(Timestamp),
		Current = 0,
	}

	pub enum Destination {
		Window(Window),
		PointerWindow = 0,
		InputFocus = 1,
	}

	pub enum Focus {
		Window(Window),
		PointerRoot = 1,
	}
}

// Implement serialization for [`Option`] enum wrappers too. None = 0.
impl<T> WriteValue for Option<T>
where
	T: WriteValue,
{
	fn write_1b(self) -> WriteResult<u8> {
		match self {
			None => Ok(0),
			Some(val) => val.write_1b(),
		}
	}

	fn write_2b(self) -> WriteResult<u16> {
		match self {
			None => Ok(0),
			Some(val) => val.write_2b(),
		}
	}

	fn write_4b(self) -> WriteResult<u32> {
		match self {
			None => Ok(0),
			Some(val) => val.write_4b(),
		}
	}
}

// Implement deserialization for [`Option`] enum wrappers too. 0 = None.
impl<T> ReadValue for Option<T>
where
	T: ReadValue,
{
	fn read_1b(byte: u8) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(match byte {
			0 => None,
			_ => Some(T::read_1b(byte)?),
		})
	}

	fn read_2b(bytes: u16) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(match bytes {
			0 => None,
			_ => Some(T::read_2b(bytes)?),
		})
	}

	fn read_4b(bytes: u32) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(match bytes {
			0 => None,
			_ => Some(T::read_4b(bytes)?),
		})
	}
}
