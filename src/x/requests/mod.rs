// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod create_window;

use crate::rw::Serialize;

pub trait Request<REPLY = ()>: Serialize {
	fn opcode() -> u8;
	fn length(&self) -> u16;
}

#[macro_export]
/// Implements [`WriteValue`](crate::WriteValue) and a `mask` method for a
/// request values enum.
macro_rules! values {
	(
		$(
			$(#[$outer:meta])* // attributes
			$vis:vis enum $Value:ident<$Mask:ty> { // pub enum Value<Mask> {
				$(
					$(#[$inner:meta])* // variant attributes
					$Variant:ident($type:ty): $mask:ident // Variant(u32): VARIANT
				),+$(,)? // comma separated, with optional final comma
			}
		)+
	) => {
		$(
			$(#[$outer])* // attributes
			$vis enum $Value { // pub enum Value {
				$(
					$(#[$inner])* // variant attributes
					$Variant($type) // Variant(u32)
				),+
			}

			impl $Value {
				/// Get the value mask associated with this field.
				pub fn mask(&self) -> $Mask {
					match self {
						$(
							// Self::Variant(_) => Mask::VARIANT
							Self::$Variant(_) => <$Mask>::$mask
						),+
					}
				}
			}

			impl $crate::rw::WriteValue for $Value { // impl WriteValue for Value {
				// fn write_1b(self) -> WriteResult<u8> {
				fn write_1b(self) -> $crate::rw::WriteResult<u8> {
					match self {
						$(
							// Self::Variant(val) => val.write_1b()
							Self::$Variant(val) =>
								<$type as $crate::rw::WriteValue>::write_1b(val)
						),+
					}
				}

				// fn write_2b(self) -> WriteResult<u16> {
				fn write_2b(self) -> $crate::rw::WriteResult<u16> {
					match self {
						$(
							// Self::Variant(val) => val.write_2b()
							Self::$Variant(val) =>
								<$type as $crate::rw::WriteValue>::write_2b(val)
						),+
					}
				}

				// fn write_4b(self) -> WriteResult<u32> {
				fn write_4b(self) -> $crate::rw::WriteResult<u32> {
					match self {
						$(
							// Self::Variant(val) => val.write_4b()
							Self::$Variant(val) =>
								<$type as $crate::rw::WriteValue>::write_4b(val)
						),+
					}
				}
			}
		)+
	};
}
