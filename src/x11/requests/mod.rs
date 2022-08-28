// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod create_window;

use crate::rw::Serialize;

/// A request is a message sent from an X client to the X server.
///
/// Since an X client will never receive an actual request message,
/// deserialization is not implemented for requests for the sake of simplicity.
pub trait Request<REPLY = ()>: Serialize {
	/// The major opcode that uniquely identifies this request or extension.
	///
	/// X core protocol requests have unique major opcodes, but each extension
	/// is only assigned one major opcode. Extensions are assigned major opcodes
	/// from 127 through to 255.
	fn opcode() -> u8;

	/// The minor opcode that uniquely identifies this request within its
	/// extension.
	///
	/// As each extension is only assigned one major opcode, the minor opcode
	/// can be used to distinguish different requests contained within an
	/// extension.
	///
	/// [`None`] means that either this request is not from an extension, or the
	/// extension does not make use of the minor opcode, likely because it only
	/// has one request.
	///
	/// [`Some`] means that there is indeed a minor opcode associated with this
	/// request. This request is therefore from an extension.
	fn minor_opcode() -> Option<u16> {
		None
	}

	/// The length of this request, including the header, in 4-byte units.
	///
	/// Every request contains a header whcih is 4 bytes long. This header is
	/// included in the length, so the minimum length is 1 unit (4 bytes). The
	/// length represents the _exact_ length of the request: padding bytes may
	/// need to be added to the end of the request to ensure its length is
	/// brought up to a multiple of 4, if it is not already.
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
