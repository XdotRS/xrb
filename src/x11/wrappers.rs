// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::x11::common::values::{Timestamp, Window};

/// Automatically implements [`cornflakes::FromBytes`] and [`cornflakes::ToBytes`]
/// for wrapper-like types alike [std]'s [`Option`].
macro_rules! wrappers {
	(
		$(
			$(#[$outer:meta])*
			$vis:vis enum $Name:ident$(<$T:ident>)? {
				$(#[$value_meta:meta])*
				$Value:ident($val:ty),
				$(
					$(#[$variant_meta:meta])*
					$Variant:ident = $encode:expr
				),+$(,)?
			}
		)+
	) => {
		$(
			$vis enum $Name$(<$T>)? {
				$(#[$outer])*
				$Value($val),
				$(
					$(#[$variant_meta])*
					$Variant
				),+
			}

			impl$(<$T>)? cornflakes::StaticByteSize for $Name$(<$T>)?
			$(where
				$T: cornflakes::StaticByteSize,)?
			{
				fn static_byte_size() -> usize {
					<$val>::static_byte_size()
				}
			}

			impl$(<$T>)? cornflakes::ByteSize for $Name$(<$T>)?
			$(where
				$T: cornflakes::StaticByteSize,)?
			{
				fn byte_size(&self) -> usize {
					<$val as cornflakes::StaticByteSize>::static_byte_size()
				}
			}

			impl$(<$T>)? cornflakes::FromBytes for $Name$(<$T>)?
			$(where
				$T: cornflakes::FromBytes + cornflakes::StaticByteSize,)?
			{
				fn read_from(reader: &mut impl cornflakes::ByteReader) -> Result<Self, std::io::Error> {
					let mut bytes = reader.copy_to_bytes(<$val as cornflakes::StaticByteSize>::static_byte_size());

					let sum = bytes.iter().sum::<u8>();

					$(if sum == $encode {
						Ok(Self::$Variant)
					} else)+ {
						Ok(Self::$Value(<$val as cornflakes::FromBytes>::read_from(&mut bytes)?))
					}
				}
			}

			impl$(<$T>)? cornflakes::ToBytes for $Name$(<$T>)?
			$(where
				$T: cornflakes::ToBytes + cornflakes::StaticByteSize,)?
			{
				fn write_to(&self, writer: &mut impl cornflakes::ByteWriter) -> Result<(), std::io::Error> {
					match self {
						// FIXME: this isn't writing $encode as the correct length!
						// need a trait that allows casting to a certain length
						// to be implemented?
						$(Self::$Variant => $encode.write_to(writer)?,)+
						Self::$Value(val) => val.write_to(writer)?,
					};

					Ok(())
				}
			}
		)+
	};
}

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
	/// [`Current`](Time::Current) time.
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
