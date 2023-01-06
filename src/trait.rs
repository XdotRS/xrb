// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// A request is a message sent from an X client to the X server.
///
/// A request may have a specific reply associated with it. That reply is
/// indicated by `Reply`.
#[doc(notable_trait)]
pub trait Request {
	type Reply;

	/// The major opcode that uniquely identifies this request or extension.
	///
	/// X core protocol requests have unique major opcodes, but each extension
	/// is only assigned one major opcode. Extensions are assigned major opcodes
	/// from 127 through to 255.
	fn major_opcode() -> u8;

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
	fn minor_opcode() -> Option<u8>;

	/// The length of this request, including the header, in 4-byte units.
	///
	/// Every request contains a header which is 4 bytes long. This header is
	/// included in the length, so the minimum length is 1 unit (4 bytes). The
	/// length represents the _exact_ length of the request: padding bytes may
	/// need to be added to the end of the request to ensure its length is
	/// brought up to a multiple of 4, if it is not already.
	fn length(&self) -> u16;
}

/// A reply is a message sent from the X server to an X client in response to a
/// request.
///
/// The request associated with a reply is indicated by `Request`.
#[doc(notable_trait)]
pub trait Reply
where
	Self: Sized,
{
	type Req: Request<Reply = Self>;

	/// The length of this reply in 4-byte units minus 8.
	///
	/// Every reply always consists of 32 bytes followed by zero or more
	/// additional bytes of data; this method indicates the number of additional
	/// bytes of data within this reply.
	///
	/// |'Actual' length in bytes|`length()`|
	/// |------------------------|----------|
	/// |32                      |0         |
	/// |36                      |1         |
	/// |40                      |2         |
	/// |44                      |3         |
	/// |...                     |...       |
	/// |`32 + 4n`               |`n`       |
	fn length(&self) -> u32;

	/// The sequence number associated with the request that this reply is for.
	///
	/// Every request on a given connection is assigned a sequence number when
	/// it is sent, starting with one. This sequence number can therefore be
	/// used to keep track of exactly which request generated this reply.
	fn sequence(&self) -> u16;
}

// An event is sent in a SendEvent request. It is 32 bytes long.
//
// TODO: docs!
#[doc(notable_trait)]
pub trait Event
where
	Self: Sized,
{
	// The code that uniquely identifies the event.
	fn code() -> u8;

	// The sequence number associated with the last request sent by the X
	// server that relates to the event.
	fn sequence(&self) -> Option<u16>;
}
