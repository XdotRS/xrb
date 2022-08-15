// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub trait Event {
	fn event_type(&self) -> u8;

	fn generated_from_send_event_reqeust(&self) -> bool {
		// If the most significant byte is set...
		self.event_type() & 0b10000000 == 0b10000000
	}

	fn sequence_num(&self) -> Option<u16>;
}

// use crate::KeyCode;

// /// A marker trait implemented for all 'raw' representations of events defined in XRB.
// ///
// /// You may use this trait as you see fit in distinguishing 'raw' events from XRB.
// pub trait RawEvent {}

// /// A request to initiate a connection to the X server.
// pub struct ConnectionInitRequest<'a> {
// 	// pub byte_order: ByteOrder,
// 	/// Should always be 11.
// 	pub protocol_major_version: u16,
// 	/// Should always be 0.
// 	pub protocol_minor_version: u16,
// 	pub auth_protocol_name: &'a str,
// 	pub auth_data: &'a str,
// }

// impl RawEvent for ConnectionInitRequest<'_> {}

// /// The result of a [ConnectionInitRequest].
// pub enum ConnectionInitResult<'a> {
// 	Failed {
// 		/// Should always be 11.
// 		protocol_major_version: u16,
// 		/// Should always be 0.
// 		protocol_minor_version: u16,
// 		reason: &'a str,
// 	},
// 	Success {
// 		/// Should always be 11.
// 		protocol_major_version: u16,
// 		/// Should always be 0.
// 		protocol_minor_version: u16,
// 		/// Identifies the owner of the X server implementation.
// 		vendor: &'a str,
// 		/// The vendor controls the semantics of the release number.
// 		release_number: u32,
// 		resource_id_base: u32,
// 		/// A contiguous set of bits, at least 18, used to allocate [ResourceId]s.
// 		///
// 		/// Clients allocate [ResourceId]s for [Window]s, [Pixmap]s, [Cursor]s, [Font]s,
// 		/// [GContext]s, and [Colormap]s by choosing a value with only some subset of these bits
// 		/// set and ORing it with `resource_id_base`. Only values constructed in this way can be
// 		/// used to name newly created resources over the connection. [ResourceId]s never have the
// 		/// top three bits set. The client is not restricted to linear or contiguous allocation of
// 		/// [ResourceId]s. Once a [ResourceId] has been freed, it can be reused. A [ResourceId]
// 		/// must be unique with respect to the [ResourceId]s of all other resources, not just other
// 		/// resources of the same type. However, note that the value spaces of [ResourceId]s,
// 		/// [Atom]s, [VisualId]s, and [KeySym]s are distinguished by context, and, as such, are not
// 		/// required to be disjoint; for example, a given numeric value might be both a valid
// 		/// [Window] ID, a valid [Atom], and a valid [KeySym].
// 		resource_id_mask: u32,
// 		// image_byte_order: ByteOrder,
// 		/// Can be `8u8`, `16u8`, or `32u8`.
// 		bitmap_scanline_unit: u8,
// 		/// Can be `8u8`, `16u8`, or `32u8`.
// 		bitmap_scanline_pad: u8,
// 		// bitmap_bit_order: BitmapBitOrder,
// 		// pixmap_formats: &'a [Format],
// 		// roots: &'a [Screen<'a>],
// 		motion_buffer_size: u32,
// 		/// Specifies the maximum length of a request accepted by the server, in 4-byte units.
// 		///
// 		/// That is, length is the maximum value that can appear in the lenght field of a request.
// 		/// Reqeusts larger than this maximum generate a `Length` error, and the server will read
// 		/// and simply discard the entire request. `maximum_request_length` will always be at least
// 		/// `4096` (that is, requests of length up to and including 16384 bytes will be accepted by
// 		/// all servers).
// 		maximum_request_length: u16,
// 		/// Specifies the smallest keycode value transmitted by the server. Never less than 8.
// 		min_keycode: KeyCode,
// 		/// Specifies the biggest keycode value transmitted by the server. Never more than 255.
// 		max_keycode: KeyCode,
// 	},
// 	Authenticate {
// 		reason: &'a str,
// 	},
// }

// /// The reply to a [ConnectionInitRequest] from the X server.
// pub struct ConnectionInitReply<'a> {
// 	pub result: ConnectionInitResult<'a>,
// }

// impl RawEvent for ConnectionInitReply<'_> {}
