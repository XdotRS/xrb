// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub trait Request {
	/// The major opcode is a unique identifier for this request.
	///
	/// Major opcodes 128 through 255 are reserved for extensions. An extension may contain
	/// multiple requests, and as such will represent multiple requests with the same major opcode.
	/// An extension may choose to encode an additional _minor opcode_ in the
	/// [`metadata()`](Request::metadata) byte.
	fn major_opcode() -> u8;

	/// Metadata encoded in the second byte of the request.
	///
	/// This may be the minor opcode for extensions, though it is up to the individual extension
	/// as to where it wishes to place minor opcodes, if at all. If this metadata byte is unused,
	/// it is not guaranteed to be zero. The metadata byte may also be used for any purpose
	/// relevant to the request, as defined by the request itself.
	fn metadata(&self) -> u8 {
		0u8
	}

	/// The length of the request in units of four bytes.
	///
	/// Includes the length of the header, which is one unit of 4 bytes that contains the
	/// [`major_opcode()`](Request::major_opcode), [`metadata()`](Request::metadata) byte, and
	/// these two [`length()`](Request::length) bytes, as well as any additional data associated
	/// with the request.
	///
	/// The `length()` must be equal to the minimum length required to contain the request. That
	/// is, if the length of the request is not an exact multiple of 4 bytes, it should be rounded
	/// up to the nearest 4-byte unit by including however many padding bytes is necessary. Any
	/// unused padding bytes are not guaranteed to be zero; they may be set to anything.
	fn length(&self) -> u16;
}
