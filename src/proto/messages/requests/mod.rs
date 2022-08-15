// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod create_window;

pub trait Request {
	/// The major opcode is a unique identifier for this request.
	///
	/// Major opcodes 128 through 255 are reserved for extensions. An extension may contain
	/// multiple requests, and as such will represent multiple requests with the same major opcode.
	/// An extension may choose to encode an additional _minor opcode_ in the
	/// [`metadata()`](Request::metadata) byte.
	fn major_opcode() -> u8;

	/// Extensions may specify minor opcodes to differentiate their requests.
	///
	/// It is up to the individual extension as to where the minor opcode is placed, though it is
	/// typically located in the 'metadata' byte of the request header, i.e. the second byte. This
	/// metadata byte may also be used for other purposes, however.
	fn minor_opcode() -> Option<u16> {
		None
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
