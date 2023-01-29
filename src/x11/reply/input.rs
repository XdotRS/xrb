// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Replies] defined in the [core X11 protocol] for
//! [requests that relate to input devices, grabs, and coordinates].
//!
//! [Replies] are messages sent from the X server to an X client in response to
//! a [request].
//!
//! [Replies]: Reply
//! [request]: crate::message::Request
//! [core X11 protocol]: crate::x11
//!
//! [requests that relate to input devices, grabs, and coordinates]: request::input

extern crate self as xrb;

use derivative::Derivative;

use xrbk_macro::{derive_xrb, Readable, Writable, X11Size};

use crate::{
	message::Reply,
	x11::{request, request::RevertFocus},
	Coords,
	FocusWindow,
	GrabStatus,
	ModifierMask,
	Timestamp,
	Window,
};

derive_xrb! {
	/// The [reply] to a [`GrabCursor` request].
	///
	/// [reply]: Reply
	///
	/// [`GrabCursor` request]: request::GrabCursor
	#[doc(alias = "GrabPointer")]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GrabCursor: Reply for request::GrabCursor {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// The status of the attempted grab.
		///
		/// See [`GrabStatus`] for more information.
		#[doc(alias = "status")]
		#[metabyte]
		pub grab_status: GrabStatus,

		[_; ..],
	}

	/// The [reply] to a [`GrabKeyboard` request].
	///
	/// [reply]: Reply
	///
	/// [`GrabKeyboard` request]: request::GrabKeyboard
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GrabKeyboard: Reply for request::GrabKeyboard {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// The status of the attempted grab.
		///
		/// See [`GrabStatus`] for more information.
		#[doc(alias = "status")]
		#[metabyte]
		pub grab_status: GrabStatus,

		[_; ..],
	}


	/// The [reply] to a [`QueryCursorLocation` request].
	///
	/// [reply]: Reply
	///
	/// [`QueryCursorLocation` request]: request::QueryCursorLocation
	#[doc(alias("QueryPointer, QueryCursor, GetCursorPos, GetCursorLocation"))]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct QueryCursorLocation: Reply for request::QueryCursorLocation {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// Whether the cursor is on the `same_screen` as the given `target`
		/// [window].
		///
		/// [window]: Window
		#[metabyte]
		pub same_screen: bool,

		/// The root [window] which the cursor is located within.
		///
		/// [window]: Window
		pub root: Window,
		/// The child [window] containing the cursor, if any.
		///
		/// If the cursor is not on the `same_screen` (i.e., `same_screen` is
		/// `false`), this will always be [`None`].
		///
		/// [window]: Window
		// TODO: should always be [`None`] if `same_screen` is false
		pub child: Option<Window>,

		/// The coordinates of the cursor relative to the top-left corner of the
		/// `root` [window].
		///
		/// [window]: Window
		pub root_coords: Coords,
		/// The coordinates of the cursor relative to the top-left corner of the
		/// given `target` [window].
		///
		/// [window]: Window
		// TODO: should always be [`None`] if `same_screen` is false
		pub target_coords: Coords,

		/// The currently held mouse buttons and modifier keys.
		pub modifiers: ModifierMask,
		[_; ..],
	}
}

/// The coordinates of the cursor at a certain [time].
///
/// This is used in the [`GetMotionHistory` reply].
///
/// [time]: Timestamp
///
/// [`GetMotionHistory` reply]: GetMotionHistory
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub struct TimeCoords {
	/// The [time] at which the cursor was at the `coords`.
	///
	/// [time]: Timestamp
	pub time: Timestamp,
	/// The coordinates of the cursor at the `time`.
	pub coords: Coords,
}

derive_xrb! {
	/// The [reply] to a [`GetMotionHistory` request].
	///
	/// [reply]: Reply
	///
	/// [`GetMotionHistory` request]: request::GetMotionHistory
	#[doc(alias = "GetMotionEvents")]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetMotionHistory: Reply for request::GetMotionHistory {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		// The length of `motion_history`.
		#[allow(clippy::cast_possible_truncation)]
		let motion_history_len: u32 = motion_history => motion_history.len() as u32,
		[_; 20],

		/// The recorded cursor motion between the `start` and `end` times
		/// (inclusive) for the given `target` [window].
		///
		/// [window]: Window
		#[context(motion_history_len => *motion_history_len as usize)]
		pub motion_history: Vec<TimeCoords>,
	}

	/// The [reply] to a [`ConvertCoordinates` request].
	///
	/// [reply]: Reply
	///
	/// [`ConvertCoordinates` request]: request::ConvertCoordinates
	#[doc(alias = "TranslateCoordinates")]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct ConvertCoordinates: Reply for request::ConvertCoordinates {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// Whether the `original` [window] and the `output` [window] are on the
		/// same [screen].
		///
		/// [window]: Window
		/// [screen]: crate::visual::Screen
		#[metabyte]
		pub same_screen: bool,

		/// If the `output_coords` are contained within a mapped child of the
		/// `output` [window], this is that child.
		///
		/// If `same_screen` is `false`, this is always [`None`].
		///
		/// [window]: Window
		// TODO: should always be [`None`] if `same_screen` is false
		pub child: Option<Window>,

		/// The converted coordinates which are now relative to the top-left
		/// corner of the `output` [window].
		///
		// FIXME: should always be [`None`], but requires overriding the reading
		//        behavior here
		/// If `same_screen` is `false`, these are always zero.
		///
		/// [window]: Window
		#[doc(alias("dst_x", "dst_y", "dst_coords", "destination_coords"))]
		pub output_coords: Coords,
		[_; ..],
	}

	/// The [reply] to a [`GetFocus` request].
	///
	/// [reply]: Reply
	///
	/// [`GetFocus` request]: request::GetFocus
	#[doc(alias = "GetInputFocus")]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetFocus: Reply for request::GetFocus {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// What the focus will retain to if the focused [window] becomes
		/// unviewable.
		///
		/// [window]: Window
		#[metabyte]
		pub revert_to: RevertFocus,

		/// The current focus.
		pub focus: FocusWindow,
		[_; ..],
	}

	/// The [reply] to a [`QueryKeyboard` request].
	///
	/// [reply]: Reply
	///
	/// [`QueryKeyboard` request]: request::QueryKeyboard
	#[doc(alias = "QueryKeymap")]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct QueryKeyboard: Reply for request::QueryKeyboard {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// A bit vector representing the currently held keys of the keyboard.
		///
		/// A bit is `0` if the key is not held, and `1` if it is held. Byte
		/// `N`, starting at `0`, contains the bits for keys `8N` to `8N + 7`.
		/// The least significant bit in the byte represents key `8N`.
		pub keys: [u8; 32],
	}
}
