// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bytes::{Buf, BufMut, BytesMut};

pub trait Serialize {
    /// Write to a [BufMut](bytes::BufMut).
    fn write(&self, buf: &mut impl BufMut);
}

pub trait Deserialize {
    /// Construct [`Self`] from a [Buf](bytes::Buf). Must be the inverse of [Serialize::write].
    fn read(buf: &mut impl Buf) -> Self;
}

impl Serialize for u8 {
    fn write(&self, buf: &mut impl BufMut) {
        buf.put_u8(*self);
    }
}

impl Deserialize for u8 {
    fn read(buf: &mut impl Buf) -> Self {
        buf.get_u8()
    }
}

impl Serialize for u16 {
    fn write(&self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_u16(*self);
        } else {
            buf.put_u16_le(*self);
        }
    }
}

impl Deserialize for u16 {
    fn read(buf: &mut impl Buf) -> Self {
        if cfg!(target_endian = "big") {
            buf.get_u16()
        } else {
            buf.get_u16_le()
        }
    }
}

impl Serialize for u32 {
    fn write(&self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_u32(*self);
        } else {
            buf.put_u32_le(*self);
        }
    }
}

impl Deserialize for u32 {
    fn read(buf: &mut impl Buf) -> Self {
        if cfg!(target_endian = "big") {
            buf.get_u32()
        } else {
            buf.get_u32_le()
        }
    }
}

/// TODO: What is the second byte used for, and why does XCB call it `pad`? Not just minor opcode.
///
/// Every request contains an 8-bit _major opcode_ and a 16-bit _length field_ expressed in units
/// of four bytes. Every request consists of four bytes of a header (containing the major opcode,
/// the length field, and a data byte) followed by zero or more additional bytes of data. The
/// length field defines the total length of the request, including the header. The length field in
/// a request must equal the minimum length required to contain the request. If the specified
/// length is smaller or greater than the required length, an error is generated. Unused bytes in a
/// request are not requried to be zero. Major opcodes 128 through 255 are reserved for
/// _extensions_. Extensions are intended to contain multiple requests, so extension requests
/// typically have an additional _minor opcode_ encoded in the second data byte in the request
/// header. However, the placement and interpretation of this minor opcode and of all other fields
/// in extension requests are not defined by the core protocol. Every request on a given connection
/// is implicitly assigned a _sequence number_, starting with one, that is used in replies, errors,
/// and events.
pub trait Request: Serialize + Deserialize {
    /// Every request contains an 8-bit major opcode.
    ///
    /// Major opcodes 128 through 255 are reserved for extensions.
    fn major_opcode() -> u8;

    /// Extensions can contain minor opcodes to differentiate their requests.
    ///
    /// If this request comes from an extension, implement this function and return [`Some`] of the
    /// request's minor opcode.
    fn minor_opcode() -> Option<u8> {
        None
    }

    /// Write this request to the [BufMut] with the request format defined in the X11 spec.
    fn write_request(&self, buf: &mut impl BufMut) {
        // Write the request data to an intermediary buffer to calculate the length field.
        let mut data = BytesMut::new();
        self.write(&mut data);

        // If there is no minor opcode, one byte of data can fit in the header.
        let data_length = if Self::minor_opcode().is_none() {
            data.len() - 1
        } else {
            data.len()
        };

        // The length field is in 4-byte units: `.div_ceil(4)` will return the exact minimum number
        // of 4-byte units that can hold the data. We add 1 for the header.
        let length: u16 = ((data_length + 3) / 4) as u16 + 1;

        // Write the header //
        Self::major_opcode().write(buf);
        length.write(buf);

        if Self::minor_opcode().is_some() {
            Self::minor_opcode().unwrap().write(buf);
        }

        // Write the additional data //
        buf.put(data);

        // TODO: Write unused bytes.
    }
}

/// TODO: Brief description.
///
/// Every _reply_ contains a 32-bit length field expressed in units of four bytes. Every reply
/// consists of 32 bytes followed by zero or more additional bytes of data, as specified in the
/// length field. Unused bytes within a reply are not guaranteed to be zero. Every reply also
/// contains the least significant 16 bits of the sequence number of the corresponding request.
pub trait Reply: Serialize + Deserialize {
    /// The least significant 16 bits of the corresponding request's sequence.
    fn sequence(&self) -> u16;

    fn write_reply(&self, buf: &mut impl BufMut) {
        // Write the reply data to an intermediary buffer to calculate the length field.
        let mut data = BytesMut::new();
        self.write(&mut data);

        // The length field is measured in 4-byte units, but the field itself takes up two bytes,
        // so the additional length because of additional reply data shouldn't include the first
        // two bytes, as those first two bytes can fit after the length field in the first 4 bytes.
        //
        // We add 1 as the reply, including the length field, must take up at least one 4-byte
        // unit.
        // TODO: What does "Every reply consists of 32 bytes followed by zero or more additional
        //       bytes of data" mean? Should the length field count the total length, or only that
        //       of additional data?
        let length = ((data.len() + 3) / 4) as u16 + 1;

        length.write(buf);
        buf.put(data);
        self.sequence().write(buf); // TODO: Is this where the sequence is meant to go?

        // TODO: Write unused bytes.
    }
}

/// TODO: Brief description.
///
/// Error reports are 32 bytes long. Every error includes an 8-bit error code. Error codes 128
/// through 255 are reserved for extensions. Every error also includes the major and minor opcodes
/// of the failed request and the least significant 16 bits of the sequence number of the request.
/// For the following errors, the failing resource ID is also returned:
/// - Colormap
/// - Cursor
/// - Drawable
/// - Font
/// - GContext
/// - IDChoice
/// - Pixmap
/// - Window
///
/// For Atom errors, the failing atom is returned. For Value errors, the failing value is returned.
/// Other core errors return no additional data. Unused bytes within an error are not guaranteed to
/// be zero.
pub trait Error: Serialize + Deserialize {
    fn error_code() -> u8;
    fn major_opcode() -> u8;
    fn minor_opcode() -> u8;
    fn sequence(&self) -> u16;

    fn write_error(&self, buf: &mut impl BufMut) {
        Self::error_code().write(buf);
        Self::major_opcode().write(buf);
        Self::minor_opcode().write(buf);
        self.sequence().write(buf);

        // Write the additional data.
        self.write(buf);

        // TODO: Write unused bytes.
    }
}

/// TODO: Brief description.
///
/// _Notifications_ are 32 bytes long. Unused bytes within a notification are not guaranteed to be
/// zero. Every notification contains an 8-bit type code. The most significant bit in this code is
/// set if the notification was generated from a [SendNotification] request. Notification codes 64
/// through 127 are reserved for extensions, although the core protocol does not define a mechanism
/// in such notifications. Every core notification (with the exception of [KeymapNotify]) also
/// contains the last request issued by the client that was (or is currently being) processed by
/// the server.
pub trait Notification: Serialize + Deserialize {
    /// An 8-bit event type code.
    ///
    /// The most significant bit in this code is set if the event was generated from a [SendEvent]
    /// request.
    ///
    /// Event codes 64 through 127 are reserved for extensions.
    fn type_code(&self) -> u8;

    /// The least significant 16 bits of the sequence number of the last request issued by the
    /// client that was (or is currently being) processed by the server.
    fn sequence(&self) -> Option<u16>;

    fn write_notification(&self, buf: &mut impl BufMut) {
        self.type_code().write(buf);

        if self.sequence().is_some() {
            self.sequence().unwrap().write(buf);
        }

        // TODO: Should additional data be written? If so, where?
        // TODO: Write unused bytes.
    }
}
