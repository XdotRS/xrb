// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::message::Error;

use xrbk_macro::derive_xrb;
extern crate self as xrb;

derive_xrb! {
	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	/// An [error] generated when the [major opcode] and [minor opcode]
	/// combination provided in a [request] does not specify a valid [request].
	///
	/// [error]: Error
	/// [request]: crate::message::Request
	/// [major opcode]: crate::message::Request::MAJOR_OPCODE
	/// [minor opcode]: crate::message::Request::MINOR_OPCODE
	pub struct Request: Error(1) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[minor_opcode]
		/// The [minor opcode] meant to refer to the type of [request] that was
		/// sent.
		///
		/// In this case, the combination of this [minor opcode] and the
		/// `major_opcode` did not actually refer to a valid [request].
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		pub minor_opcode: u16,

		#[major_opcode]
		/// The [major opcode] meant to refer to the type of [request] that was
		/// sent.
		///
		/// In this case, the combination of this [major opcode] and the
		/// `minor_opcode` did not actually refer to a valid [request].
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Value: Error(2) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[error_data]
		pub bad_value: [u8; 4],

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Window: Error(3) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Pixmap: Error(4) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Atom: Error(5) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[error_data]
		pub bad_atom_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct CursorAppearance: Error(6) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Font: Error(7) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Match: Error(8) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Drawable: Error(9) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Access: Error(10) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Alloc: Error(11) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Colormap: Error(12) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct GraphicsContext: Error(13) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct ResourceIdChoice: Error(14) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Name: Error(15) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Length: Error(16) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Implementation: Error(17) {
		#[sequence]
		/// The [sequence number][sequence] identifying the [request] that was
		/// sent.
		///
		/// See [`Request::sequence`][sequence] for more information.
		///
		/// [request]: crate::message::Request
		/// [sequence]: crate::message::Request::sequence
		pub sequence: u16,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}
}
