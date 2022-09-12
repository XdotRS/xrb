// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::x11::*;

use cornflakes::{ByteSize, StaticByteSize};

/// Allows a value to be copied from the parent at its initialization.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Inheritable<T> {
	/// Initialise this value by copying it from the parent.
	///
	/// The value will be _copied_ at initialization: if there are changes in
	/// the equivalent of this value in the parent, they are not reflected here.
	CopyFromParent,
	/// Provides a specific value, rather than copying from the parent.
	Specific(T),
}

impl<T> Default for Inheritable<T> {
	fn default() -> Self {
		Self::CopyFromParent
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Relatable<T> {
	ParentRelative,
	Specific(T),
}

impl<T> Default for Relatable<T> {
	fn default() -> Self {
		Self::ParentRelative
	}
}

/// Allows a value to be represented as an `Any` state.
///
/// The meaning of `Any` is dependent on the nature of the value in question.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Any<T> {
	Any,
	/// Provides a specific value, rather than representing `Any`.
	Specific(T),
}

impl<T> Default for Any<T> {
	fn default() -> Self {
		Self::Any
	}
}

/// Allows a field to be implicitly initialized as its default value.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Defaultable<T> {
	/// The default for this particular field.
	///
	/// *This is not the same as [`Default`] in [`std`].*
	///
	/// [`Default`]: std::default::Default
	Default,
	/// Provides a specific value, rather than initializing as the default.
	Specific(T),
}

impl<T> Default for Defaultable<T> {
	fn default() -> Self {
		Self::Default
	}
}

/// Represents a point in time.
///
/// This enum allows [`Current`] to be specified as in place of a [specifc]
/// [`Timestamp`], if that is desired. Otherwise, [`Specific`] can be used to
/// represent a specific [`Timestamp`].
///
/// [`Current`]: Time::Current
/// [`Specific`]: Time::Specific
/// [specific]: Time::Specific
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Time {
	/// Represents the current time.
	///
	/// The X server replaces this value with the actual current [`Timestamp`].
	Current,
	/// Represents a specific [`Timestamp`], rather than being replaced by the
	/// [current time].
	///
	/// [current time]: Time::Current
	Specific(Timestamp),
}

impl Default for Time {
	fn default() -> Self {
		Self::Current
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum InputFocus {
	PointerRoot,
	Specific(Window),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum BitmapFormat {
	Bitmap,
	Specific(Format),
}

// Byte size implementations {{{

impl<T> StaticByteSize for Inheritable<T>
where
	T: StaticByteSize,
{
	fn static_byte_size() -> usize {
		T::static_byte_size()
	}
}

impl<T> StaticByteSize for Relatable<T>
where
	T: StaticByteSize,
{
	fn static_byte_size() -> usize {
		T::static_byte_size()
	}
}

impl<T> StaticByteSize for Any<T>
where
	T: StaticByteSize,
{
	fn static_byte_size() -> usize {
		T::static_byte_size()
	}
}

impl<T> StaticByteSize for Defaultable<T>
where
	T: StaticByteSize,
{
	fn static_byte_size() -> usize {
		T::static_byte_size()
	}
}

impl StaticByteSize for Time {
	fn static_byte_size() -> usize {
		Timestamp::static_byte_size()
	}
}

impl StaticByteSize for InputFocus {
	fn static_byte_size() -> usize {
		Window::static_byte_size()
	}
}

impl StaticByteSize for BitmapFormat {
	fn static_byte_size() -> usize {
		Format::static_byte_size()
	}
}

impl<T> ByteSize for Inheritable<T>
where
	T: StaticByteSize,
{
	fn byte_size(&self) -> usize {
		T::static_byte_size()
	}
}

impl<T> ByteSize for Relatable<T>
where
	T: StaticByteSize,
{
	fn byte_size(&self) -> usize {
		T::static_byte_size()
	}
}

impl<T> ByteSize for Any<T>
where
	T: StaticByteSize,
{
	fn byte_size(&self) -> usize {
		T::static_byte_size()
	}
}

impl<T> ByteSize for Defaultable<T>
where
	T: StaticByteSize,
{
	fn byte_size(&self) -> usize {
		T::static_byte_size()
	}
}

impl ByteSize for Time {
	fn byte_size(&self) -> usize {
		Timestamp::static_byte_size()
	}
}

impl ByteSize for InputFocus {
	fn byte_size(&self) -> usize {
		Window::static_byte_size()
	}
}

impl ByteSize for BitmapFormat {
	fn byte_size(&self) -> usize {
		Format::static_byte_size()
	}
}

// }}}
