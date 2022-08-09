// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bytes::BufMut;

use crate::{
    general::{BitmapBitOrder, ByteOrder, Format, Keycode, Screen},
    serialization::{Deserialize, Serialize},
};

/// A marker trait implemented for all 'raw' representations of events defined in XRB.
///
/// You may use this trait as you see fit in distinguishing 'raw' events from XRB.
pub trait RawEvent {}

/// A request to initiate a connection to the X server.
pub struct ConnectionInitRequest<'a> {
    pub byte_order: ByteOrder,
    /// Should always be 11.
    pub protocol_major_version: u16,
    /// Should always be 0.
    pub protocol_minor_version: u16,
    pub auth_protocol_name: &'a str,
    pub auth_data: &'a str,
}

impl RawEvent for ConnectionInitRequest<'_> {}

/// The result of a [ConnectionInitRequest].
pub enum ConnectionInitResult<'a> {
    Failed {
        /// Should always be 11.
        protocol_major_version: u16,
        /// Should always be 0.
        protocol_minor_version: u16,
        reason: &'a str,
    },
    Success {
        /// Should always be 11.
        protocol_major_version: u16,
        /// Should always be 0.
        protocol_minor_version: u16,
        /// Identifies the owner of the X server implementation.
        vendor: &'a str,
        /// The vendor controls the semantics of the release number.
        release_number: u32,
        resource_id_base: u32,
        /// A contiguous set of bits, at least 18, used to allocate [ResourceId]s.
        ///
        /// Clients allocate [ResourceId]s for [Window]s, [Pixmap]s, [Cursor]s, [Font]s,
        /// [GContext]s, and [Colormap]s by choosing a value with only some subset of these bits
        /// set and ORing it with `resource_id_base`. Only values constructed in this way can be
        /// used to name newly created resources over the connection. [ResourceId]s never have the
        /// top three bits set. The client is not restricted to linear or contiguous allocation of
        /// [ResourceId]s. Once a [ResourceId] has been freed, it can be reused. A [ResourceId]
        /// must be unique with respect to the [ResourceId]s of all other resources, not just other
        /// resources of the same type. However, note that the value spaces of [ResourceId]s,
        /// [Atom]s, [VisualId]s, and [KeySym]s are distinguished by context, and, as such, are not
        /// required to be disjoint; for example, a given numeric value might be both a valid
        /// [Window] ID, a valid [Atom], and a valid [KeySym].
        resource_id_mask: u32,
        image_byte_order: ByteOrder,
        /// Can be `8u8`, `16u8`, or `32u8`.
        bitmap_scanline_unit: u8,
        /// Can be `8u8`, `16u8`, or `32u8`.
        bitmap_scanline_pad: u8,
        bitmap_bit_order: BitmapBitOrder,
        pixmap_formats: &'a [Format],
        roots: &'a [Screen<'a>],
        motion_buffer_size: u32,
        /// Specifies the maximum length of a request accepted by the server, in 4-byte units.
        ///
        /// That is, length is the maximum value that can appear in the lenght field of a request.
        /// Reqeusts larger than this maximum generate a `Length` error, and the server will read
        /// and simply discard the entire request. `maximum_request_length` will always be at least
        /// `4096` (that is, requests of length up to and including 16384 bytes will be accepted by
        /// all servers).
        maximum_request_length: u16,
        /// Specifies the smallest keycode value transmitted by the server. Never less than 8.
        min_keycode: Keycode,
        /// Specifies the biggest keycode value transmitted by the server. Never more than 255.
        max_keycode: Keycode,
    },
    Authenticate {
        reason: &'a str,
    },
}

/// The reply to a [ConnectionInitRequest] from the X server.
pub struct ConnectionInitReply<'a> {
    pub result: ConnectionInitResult<'a>,
}

impl RawEvent for ConnectionInitReply<'_> {}

////// INCORRECT (DE)SERIALIZATION FORMATS ////////////////////////////////////////////////////////
// The [Request], [Reply], [Error], [Notification] formats are explicitly defined, along with    //
// all protocol formats, in                                                                      //
// [Appendix B](https://x.org/releases/X11R7.7/doc/xproto/x11protocol.html#protocol_encoding) of //
// the X11 specification.                                                                        //
//                                                                                               //
// These, well, guessed formats are obviously not correct.                                       //
///////////////////////////////////////////////////////////////////////////////////////////////////

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
    fn write_request_data(&self, buf: &mut impl BufMut);
}

// impl Serialize for dyn Request {
//     fn write(&self, buf: &mut impl BufMut) {
//         // Write the request data to an intermediary buffer to calculate the length field.
//         let mut data = BytesMut::new();
//         self.write_request_data(&mut data);

//         // The length field is in 4-byte units: `.div_ceil(4)` will return the exact minimum number
//         // of 4-byte units that can hold the data. We add 1 for the header.
//         let length: u16 = ((data.len() + 3) / 4) as u16 + 1;

//         // Write the header //
//         Self::major_opcode().write(buf); // Byte 1: major opcode

//         if Self::minor_opcode().is_some() {
//             Self::minor_opcode().unwrap().write(buf); // Byte 2: minor opcode
//         } else {
//             0u8.write(buf); // Byte 2: pad?
//         }

//         length.write(buf); // Bytes 3 and 4: length

//         // Write the additional data //
//         buf.put(data);
//     }
// }

/// TODO: Brief description.
///
/// Every _reply_ contains a 32-bit length field expressed in units of four bytes. Every reply
/// consists of 32 bytes followed by zero or more additional bytes of data, as specified in the
/// length field. Unused bytes within a reply are not guaranteed to be zero. Every reply also
/// contains the least significant 16 bits of the sequence number of the corresponding request.
pub trait Reply: Serialize + Deserialize {
    /// The least significant 16 bits of the corresponding request's sequence.
    fn sequence(&self) -> u16;

    fn write_reply_data(&self, buf: &mut impl BufMut);
}

// impl Serialize for &dyn Reply {
//     fn write(&self, buf: &mut impl BufMut) {
//         // Write the reply data to an intermediary buffer to calculate the length field.
//         let mut data = BytesMut::new();
//         self.write_reply_data(&mut data);

//         // The length field is measured in 4-byte units, but the field itself takes up two bytes,
//         // so the additional length because of additional reply data shouldn't include the first
//         // two bytes, as those first two bytes can fit after the length field in the first 4 bytes.
//         //
//         // We add 1 as the reply, including the length field, must take up at least one 4-byte
//         // unit.
//         // TODO: What does "Every reply consists of 32 bytes followed by zero or more additional
//         //       bytes of data" mean? Should the length field count the total length, or only that
//         //       of additional data?
//         let length = ((data.len() + 3) / 4) as u16 + 1;

//         length.write(buf);
//         buf.put(data);
//         self.sequence().write(buf); // TODO: Is this where the sequence is meant to go?

//         // TODO: Write unused bytes.
//     }
// }

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

    fn write_error_data(&self, buf: &mut impl BufMut);
}

// impl Serialize for dyn Error {
//     fn write(&self, buf: &mut impl BufMut) {
//         Self::error_code().write(buf);
//         Self::major_opcode().write(buf);
//         Self::minor_opcode().write(buf);
//         self.sequence().write(buf);

//         // Write the additional data.
//         self.write_error_data(buf);

//         // TODO: Write unused bytes.
//     }
// }

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

// impl Serialize for dyn Notification {}
