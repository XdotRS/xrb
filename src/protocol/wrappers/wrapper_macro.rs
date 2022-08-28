// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Creates 'wrapper' enums, a bit like [`Option`]s.
///
/// Includes implementations for [WriteValue](crate::rw::WriteValue) and
/// [ReadValue](crate::rw::ReadValue).
///
/// # Example
/// ```rust
/// wrappers! {
///     pub enum Destination {
///         Window(u32),
///         PointerWindow = 1,
///         InputFocus = 2,
///     }
/// }
/// ```
/// `Destination` will have implementations generated that will read and write
/// `PointerWindow` as the value `1` and `InputFocus` as the value `2`,
/// otherwise the wrapped [`u32`] value will be read or written.
macro_rules! wrappers {
	(
		$(
			$(#[$attr:meta])* // attributes
			$vis:vis enum $Wrapper:ident$(<$A:ident>)? { // pub enum Wrapper<T> {
				$(#[$value_attr:meta])* // value variant attributes
				$Value:ident($B:ty)$(,)? // Value(T),
				$(
					$(#[$variant_attr:meta])* // variant attributes
					$Variant:ident = $val:expr, // Variant = 0,
				)+
			}
		)+
	) => {
		$(
			$(#[$attr])* // attributes
			$vis enum $Wrapper<$($A)?> { // pub enum Wrapper<T> {
				$(#[$value_attr])* // value variant attributes
				$Value($B), // Value(T),
				$(
					$(#[$variant_attr])* // variant attributes
					$Variant, // Variant,
				)+
			}

			// impl<T> WriteValue for Wrapper<T> {
			// where
			//     T: WriteValue,
			// {
			impl<$($A)?> $crate::rw::WriteValue for $Wrapper$(<$A>
			where
				$A: $crate::rw::WriteValue,)?
			{
				// fn write_1b(self) -> WriteResult<u8> {
				fn write_1b(self) -> $crate::errors::WriteResult<u8> {
					Ok(match self {
						// Self::Variant => 0,
						$(Self::$Variant => $val,)+
						// Self::Value(val) => val.write_1b()?,
						Self::$Value(val) =>
							<$B as $crate::rw::WriteValue>::write_1b(val)?,
					})
				}

				// fn write_2b(self) -> WriteResult<u16> {
				fn write_2b(self) -> $crate::errors::WriteResult<u16> {
					Ok(match self {
						// Self::Variant => 0,
						$(Self::$Variant => $val,)+
						// Self::Value(val) => val.write_2b()?,
						Self::$Value(val) =>
							<$B as $crate::rw::WriteValue>::write_2b(val)?,
					})
				}

				// fn write_4b(self) -> WriteResult<u32> {
				fn write_4b(self) -> $crate::errors::WriteResult<u32> {
					Ok(match self {
						// Self::Variant => 0,
						$(Self::$Variant => $val,)+
						// Self::Value(val) => val.write_4b()?,
						Self::$Value(val) =>
							<$B as $crate::rw::WriteValue>::write_4b(val)?,
					})
				}
			}

			// impl<T> ReadValue for Wrapper<T>
			// where
			//     T: ReadValue
			// {
			impl<$($A)?> $crate::rw::ReadValue for $Wrapper$(<$A>
			where
				$A: $crate::rw::ReadValue,)?
			{
				// fn read_1b(byte: u8) -> ReadResult<Self>
				// where
				//     Self: Sized,
				// {
				fn read_1b(byte: u8) -> $crate::errors::ReadResult<Self>
				where
					Self: Sized,
				{
					Ok(match byte {
						// 0 => Self::Variant,
						$($val => Self::$Variant,)+
						// _ => Self::Value(T::read_1b(byte)?),
						_ => Self::$Value(
							<$B as $crate::rw::ReadValue>::read_1b(byte)?
						),
					})
				}

				// fn read_2b(bytes: u16) -> ReadResult<Self>
				// where
				//     Self: Sized,
				// {
				fn read_2b(bytes: u16) -> $crate::errors::ReadResult<Self>
				where
					Self: Sized,
				{
					Ok(match bytes {
						// 0 => Self::Variant,
						$($val => Self::$Variant,)+
						// _ => Self::Value(T::read_2b(bytes)?),
						_ => Self::$Value(
							<$B as $crate::rw::ReadValue>::read_2b(bytes)?
						),
					})
				}

				// fn read_4b(bytes: u32) -> ReadResult<Self>
				// where
				//     Self: Sized,
				// {
				fn read_4b(bytes: u32) -> $crate::errors::ReadResult<Self>
				where
					Self: Sized,
				{
					Ok(match bytes {
						// 0 => Self::Variant,
						$($val => Self::$Variant,)+
						// _ => Self::Value(T::read_4b(bytes)?),
						_ => Self::$Value(
							<$B as $crate::rw::ReadValue>::read_4b(bytes)?
						),
					})
				}
			}
		)+
	};
}
