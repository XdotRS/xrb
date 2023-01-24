// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Errors] defined in the [core X11 protocol].
//!
//! [Errors] are messages sent from the X server to an X client in response to
//! a failed [request].
//!
//! [Errors]: Error
//! [request]: crate::message::Request
//! [core X11 protocol]: super

use crate::message::Error;

use derivative::Derivative;
use xrbk_macro::derive_xrb;
extern crate self as xrb;

derive_xrb! {
	/// An [error] generated when the [major opcode] and [minor opcode]
	/// combination provided in a [request] does not specify a valid [request].
	///
	/// [error]: Error
	/// [request]: crate::message::Request
	/// [major opcode]: crate::message::Request::MAJOR_OPCODE
	/// [minor opcode]: crate::message::Request::MINOR_OPCODE
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct Request: Error(1) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The [minor opcode] meant to refer to the type of [request] that was
		/// sent.
		///
		/// In this case, the combination of this [minor opcode] and the
		/// `invalid_major_opcode` did not actually refer to a valid [request].
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub invalid_minor_opcode: u16,

		/// The [major opcode] meant to refer to the type of [request] that was
		/// sent.
		///
		/// In this case, the combination of this [major opcode] and the
		/// `invalid_minor_opcode` did not actually refer to a valid [request].
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub invalid_major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when a numerical value contained in the [request]
	/// falls outside of the range of accepted values.
	///
	/// This [error] is commonly generated for enums, because any value which is
	/// not one of the enum discriminants is invalid.
	///
	/// [request]: crate::message::Request
	/// [error]: Error
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct Value: Error(2) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The numerical value which fell outside of the accepted ranges.
		///
		/// This is represented as four bytes instead of a `u32` value because
		/// it is not specified in the X11 protocol that this value is one
		/// `u32` value. Encoding it as such if it wasn't meant to be could
		/// cause issues with byte-swapping, where the bytes of a value would
		/// be swapped to translate it to a `u32` value on the target platform.
		#[error_data]
		pub invalid_value: [u8; 4],

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when the [`Window`] ID used in the [request] does
	/// not refer to a defined [window].
	///
	/// [error]: Error
	/// [`Window`]: crate::Window
	/// [window]: crate::Window
	/// [request]: crate::message::Request
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct Window: Error(3) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The invalid [`Window`] ID.
		///
		/// This is of type `u32`, not [`Window`], because it does not refer to
		/// a defined [window], and so it shouldn't be used as such.
		///
		/// [`Window`]: crate::Window
		/// [window]: crate::Window
		#[error_data]
		pub invalid_window_id: u32,

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when the [`Pixmap`] ID used in the [request] does
	/// not refer to a defined [pixmap].
	///
	/// [error]: Error
	/// [`Pixmap`]: crate::Pixmap
	/// [pixmap]: crate::Pixmap
	/// [request]: crate::message::Request
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct Pixmap: Error(4) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The invalid [`Pixmap`] ID.
		///
		/// This is of type `u32`, not [`Pixmap`], because it does not refer to
		/// a defined [pixmap], and so it shouldn't be used as such.
		///
		/// [`Pixmap`]: crate::Pixmap
		/// [pixmap]: crate::Pixmap
		#[error_data]
		pub invalid_pixmap_id: u32,

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when the [`Atom`] ID used in the [request] does
	/// not refer to a defined [atom].
	///
	/// [error]: Error
	/// [`Atom`]: crate::Atom
	/// [atom]: crate::Atom
	/// [request]: crate::message::Request
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct Atom: Error(5) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The invalid [`Atom`] ID.
		///
		/// This is of type `u32`, not [`Atom`], because it does not refer to
		/// a defined [atom], and so it shouldn't be used as such.
		///
		/// [`Atom`]: crate::Atom
		/// [atom]: crate::Atom
		#[error_data]
		pub invalid_atom_id: u32,

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when the [`CursorAppearance`] ID used in the
	/// [request] does not refer to a defined [cursor appearance].
	///
	/// [error]: Error
	/// [`CursorAppearance`]: crate::CursorAppearance
	/// [cursor appearance]: crate::CursorAppearance
	/// [request]: crate::message::Request
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct CursorAppearance: Error(6) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The invalid [`CursorAppearance`] ID.
		///
		/// This is of type `u32`, not [`CursorAppearance`], because it does
		/// not refer to a defined [cursor appearance], and so it shouldn't be
		/// used as such.
		///
		/// [`CursorAppearance`]: crate::CursorAppearance
		/// [cursor appearance]: crate::CursorAppearance
		#[error_data]
		pub invalid_cursor_appearance_id: u32,

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when the [`Font`] ID used in the [request] does
	/// not refer to a defined [font].
	///
	/// [error]: Error
	/// [`Font`]: crate::Font
	/// [font]: crate::Font
	/// [request]: crate::message::Request
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct Font: Error(7) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The invalid [`Font`] ID.
		///
		/// This is of type `u32`, not [`Font`], because it does not refer to
		/// a defined [font], and so it shouldn't be used as such.
		///
		/// [`Font`]: crate::Font
		/// [font]: crate::Font
		#[error_data]
		pub invalid_font_id: u32,

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when there is a mismatch of some kind.
	///
	/// This [error] is generated for a number of reasons:
	/// - when an [`InputOnly`] [window] is used as a [drawable],
	/// - in a graphics [request], the [graphics context] does not have the
	///   same `root` [window] and `depth` as the `destination` [`Drawable`],
	/// - or generally when a field or pair of fields has the correct type
	///   and falls in the correct range, but it fails to match is some other
	///   way required by the [request].
	///
	/// [error]: Error
	/// [`InputOnly`]: crate::WindowClass::InputOnly
	/// [window]: crate::Window
	/// [drawable]: crate::Drawable
	/// [request]: crate::message::Request
	/// [graphics context]: crate::GraphicsContext
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct Match: Error(8) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when the [`Drawable`] ID used in the [request]
	/// does not refer to a defined [window] or [pixmap].
	///
	/// [error]: Error
	/// [`Drawable`]: crate::Drawable
	/// [window]: crate::Window
	/// [pixmap]: crate::Pixmap
	/// [request]: crate::message::Request
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct Drawable: Error(9) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The invalid [`Drawable`] ID.
		///
		/// This is of type `u32`, not [`Drawable`], because it does not refer to
		/// a defined [window] nor [pixmap], and so it shouldn't be used as such.
		///
		/// [`Drawable`]: crate::Drawable
		/// [window]: crate::Window
		/// [pixmap]: crate::Pixmap
		#[error_data]
		pub invalid_drawable_id: u32,

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when access is not allowed for what the [request] is
	/// trying to do.
	///
	/// This [error] is generated for a number of reasons:
	/// - an attempt is made to grab a key/[button] combination already grabbed by
	///   another client,
	/// - an attempt is made to free a [colormap] entry not allocated by the
	///   client,
	/// - an attempt is made to free an entry in a [colormap] that was created
	///   with all entries writable,
	/// - an attempt is made to store into a read-only or unallocated [colormap]
	///   entry,
	/// - an attempt is made to modify the access control list from an external
	///   host or otherwise unauthorized client,
	/// - or an attempt is made to [select to receive] an [event] type that only one
	///   client can select at a time when another client has already selected
	///   it.
	///
	/// [error]: Error
	/// [request]: crate::message::Request
	/// [event]: crate::message::Event
	/// [select to receive]: crate::mask::EventMask
	/// [button]: crate::Button
	/// [colormap]: crate::Colormap
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct Access: Error(10) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when the X server failed to allocate the requested
	/// resource.
	///
	/// [error]: Error
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct Alloc: Error(11) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when the [`Colormap`] ID used in the [request]
	/// does not refer to a defined [colormap].
	///
	/// [error]: Error
	/// [`Colormap`]: crate::Colormap
	/// [colormap]: crate::Colormap
	/// [request]: crate::message::Request
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct Colormap: Error(12) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The invalid [`Colormap`] ID.
		///
		/// This is of type `u32`, not [`Colormap`], because it does not refer to
		/// a defined [colormap], and so it shouldn't be used as such.
		///
		/// [`Colormap`]: crate::Colormap
		/// [colormap]: crate::Colormap
		#[error_data]
		pub invalid_colormap_id: u32,

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when the [`GraphicsContext`] ID used in the [request]
	/// does not refer to a defined [graphics context].
	///
	/// [error]: Error
	/// [`GraphicsContext`]: crate::GraphicsContext
	/// [graphics context]: crate::GraphicsContext
	/// [request]: crate::message::Request
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GraphicsContext: Error(13) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The invalid [`GraphicsContext`] ID.
		///
		/// This is of type `u32`, not [`GraphicsContext`], because it does not refer to
		/// a defined [graphics context], and so it shouldn't be used as such.
		///
		/// [`GraphicsContext`]: crate::GraphicsContext
		/// [graphics context]: crate::GraphicsContext
		#[error_data]
		pub invalid_graphics_context_id: u32,

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when a chosen resource ID is not in the range of
	/// resource IDs assigned to the client, or the ID is already in use.
	///
	/// [error]: Error
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct ResourceIdChoice: Error(14) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The resource ID that was either not assigned to the client or was
		/// already in use.
		#[error_data]
		pub unavailable_resource_id: u32,

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when the [request] specifies the name of a [font]
	/// or color which does not exist.
	///
	/// [error]: Error
	/// [request]: crate::message::Request
	/// [font]: crate::Font
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct Name: Error(15) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when a [request] is not of the correct length.
	///
	/// The length may be too short or too long to hold the fields defined for
	/// the [request], or its length might exceed the maximum [request] length
	/// accepted by the X server.
	///
	/// [error]: Error
	/// [request]: crate::message::Request
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct Length: Error(16) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	/// An [error] generated when the X server does not implement some aspect
	/// of the [request].
	///
	/// [error]: Error
	/// [request]: crate::message::Request
	#[derive(Debug, Derivative, Writable, Readable, X11Size)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct Implementation: Error(17) {
		/// The sequence number identifying the [request] that was
		/// sent.
		///
		/// [request]: crate::message::Request
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		pub sequence: u16,

		/// The [minor opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MINOR_OPCODE`][minor opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		#[minor_opcode]
		pub minor_opcode: u16,

		/// The [major opcode] referring to the type of [request] that was sent.
		///
		/// See [`Request::MAJOR_OPCODE`][major opcode] for more information.
		///
		/// [request]: crate::message::Request
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}
}
