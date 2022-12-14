// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xrbk::{Readable, Writable, X11Size};

/// A message sent from an X client to the X server.
#[doc(notable_trait)]
pub trait Request: X11Size + Writable {
	/// The type of [`Reply`] generated by this `Request`.
	///
	/// For `Request`s which do not generate a [reply], this is `()`.
	///
	/// [reply]: Reply
	type Reply;

	/// The major opcode that uniquely identifies this `Request` or extension.
	///
	/// X core protocol `Request`s have unique major opcodes, but each extension
	/// is only assigned one major opcode. Extensions are assigned major opcodes
	/// from 127 through to 255.
	const MAJOR_OPCODE: u8;

	/// The minor opcode that uniquely identifies this `Request` within its
	/// extension.
	///
	/// As each extension is only assigned one major opcode, the minor opcode
	/// can be used to distinguish different `Request`s contained within an
	/// extension.
	///
	/// [`Some`] means that there is indeed a minor opcode associated with this
	/// `Request`. This `Request` is therefore from an extension.
	///
	/// [`None`] means that either this request is not from an extension, or the
	/// extension does not make use of the minor opcode, likely because it only
	/// has one request.
	const MINOR_OPCODE: Option<u8>;

	/// The size of this `Request`, including the header, in 4-byte units.
	///
	/// Every `Request` contains a header which is 4 bytes long. This header is
	/// included in the `length()`, so the minimum `length()` is 1 unit (4
	/// bytes). Since the length is always in multiples of 4 bytes, padding
	/// bytes may need to be added to the end of the `Request` to ensure its
	/// `length()` is a multiple of 4 bytes.
	///
	/// The `Request` header includes the metabyte position, so that will not
	/// contribute toward the data portion.
	///
	/// |Size (excl. header)|Size (incl. header)|`length()`|
	/// |-------------------|-------------------|----------|
	/// |0                  |4                  |1         |
	/// |4                  |8                  |2         |
	/// |8                  |12                 |3         |
	/// |12                 |16                 |4         |
	/// |...                |...                |...       |
	/// |`4n - 4`           |`4n`               |`n`       |
	fn length(&self) -> u16;
}

/// A message sent from the X server to an X client in response to a
/// [`Request`].
#[doc(notable_trait)]
pub trait Reply: X11Size + Readable
where
	Self: Sized,
{
	/// The [request] that generates this `Reply`.
	///
	/// The type indicated here must implement [`Request`] with a
	/// [`Request::Reply`] associated type set to this `Reply`.
	///
	/// [request]: Request
	type Request: Request<Reply = Self>;

	/// The size of this `Reply` in 4-byte units minus 8.
	///
	/// Every `Reply` always consists of an 8-byte-long header followed by 24
	/// bytes of data, followed by zero or more additional bytes of data; this
	/// method indicates the number of additional bytes of data within the
	/// `Reply`.
	///
	/// The `Reply` header includes the metabyte position and sequence number,
	/// so those will not contribute toward the data portion.
	///
	/// |Size (excl. header)|Size (incl. header)|`length()`|
	/// |-------------------|-------------------|----------|
	/// |24                 |32                 |0         |
	/// |28                 |36                 |1         |
	/// |32                 |40                 |2         |
	/// |36                 |44                 |3         |
	/// |...                |...                |...       |
	/// |`4n - 8`           |`4n`               |`n - 8`   |
	fn length(&self) -> u32;

	/// The sequence number associated with the [request] that generated this
	/// `Reply`.
	///
	/// Every [request] on a given connection is assigned a sequence number when
	/// it is sent, starting with `1`. This sequence number can therefore be
	/// used to keep track of exactly which [request] generated this reply.
	///
	/// [request]: Request
	fn sequence(&self) -> u16;
}

/// A message sent from the X server to an X client.
///
/// `Event`s differ from [replies] in that they are not a direct response to a
/// [request] sent by the client receiving them.
///
/// [replies]: Reply
/// [request]: Request
#[doc(notable_trait)]
pub trait Event: X11Size + Readable + Writable
where
	Self: Sized,
{
	/// The code uniquely identifying this `Event`.
	const CODE: u8;

	/// The sequence number associated with the last [request] received that
	/// was related to this `Event`.
	///
	/// [request]: Request
	fn sequence(&self) -> Option<u16>;
}
