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
use xrbk::{Buf, BufMut, ConstantX11Size, ReadResult, Readable, Writable, WriteResult, X11Size};

use xrbk_macro::{derive_xrb, Readable, Writable, X11Size};

use crate::{
	message::Reply,
	unit::{Hz, Ms, Percentage, Px},
	x11::{
		request,
		request::{Fraction, RevertFocus},
	},
	Coords,
	FocusWindow,
	GrabStatus,
	Keysym,
	ModifierMask,
	Timestamp,
	Toggle,
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

/// The [keysyms] mapped to a particular [keycode].
///
/// [keysyms]: Keysym
/// [keycode]: crate::Keycode
pub type KeyMapping = Vec<Keysym>;

/// The [reply] to a [`GetKeyboardMapping` request].
///
/// [reply]: Reply
///
/// [`GetKeyboardMapping` request]: request::GetKeyboardMapping
#[derive(Derivative, Debug)]
#[derivative(Hash, PartialEq, Eq)]
pub struct GetKeyboardMapping {
	/// The sequence number identifying the [request] that generated this
	/// [reply].
	///
	/// See [`Reply::sequence`] for more information.
	///
	/// [request]: crate::message::Request
	/// [reply]: Reply
	///
	/// [`Reply::sequence`]: Reply::sequence
	#[derivative(Hash = "ignore", PartialEq = "ignore")]
	pub sequence: u16,

	/// The mapping of [keysyms] for each [keycode] in the specified `range`.
	///
	/// [keycode]: crate::Keycode
	/// [keysyms]: Keysym
	pub mappings: Vec<KeyMapping>,
}

impl Reply for GetKeyboardMapping {
	type Request = request::GetKeyboardMapping;

	fn sequence(&self) -> u16 {
		self.sequence
	}
}

impl X11Size for GetKeyboardMapping {
	fn x11_size(&self) -> usize {
		const HEADER: usize = 8;
		const CONSTANT_SIZES: usize = HEADER + 24;

		CONSTANT_SIZES + self.mappings.x11_size()
	}
}

impl Readable for GetKeyboardMapping {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		const HEADER: usize = 8;

		// Header {{{

		// FIXME: actually, replies need to have their first 4 bytes read before
		//        the type of reply can be determined, so `keysyms_per_keycode`
		//        and `sequence` should be context for `ReadableWithContext`.
		//
		// FIXME: This is a change that needs to be done for all replies...
		buf.advance(1);
		let keysyms_per_keycode = buf.get_u8();
		let sequence = buf.get_u16();

		let length = (buf.get_u32() as usize) * 4;
		let buf = &mut buf.take(length - HEADER);

		// }}}

		// 24 unused bytes.
		buf.advance(24);

		let mappings = {
			let mapping_size = usize::from(keysyms_per_keycode) * Keysym::X11_SIZE;
			let mappings_len = buf.remaining() / mapping_size;

			let mut mappings = vec![];

			for _ in 0..mappings_len {
				let mut keysyms = vec![];

				for _ in 0..keysyms_per_keycode {
					keysyms.push(Keysym::read_from(buf)?);
				}

				mappings.push(keysyms);
			}

			mappings
		};

		Ok(Self { sequence, mappings })
	}
}

impl Writable for GetKeyboardMapping {
	#[allow(clippy::cast_possible_truncation)]
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let buf = &mut buf.limit(self.x11_size());

		// Header {{{

		// Indicates that this is a reply.
		buf.put_u8(1);
		// The number of keysyms in each mapping.
		let mapping_size = self.mappings.x11_size() / self.mappings.len();
		let keysyms_per_keycode = (mapping_size / Keysym::X11_SIZE) as u8;
		keysyms_per_keycode.write_to(buf)?;
		// The sequence number.
		self.sequence.write_to(buf)?;

		// The message length.
		self.length().write_to(buf)?;

		// }}}

		// 24 unused bytes.
		buf.put_bytes(0, 24);

		self.mappings.write_to(buf)?;

		Ok(())
	}
}

derive_xrb! {
	/// The [reply] to a [`GetKeyboardOptions` request].
	///
	/// [reply]: Reply
	///
	/// [`GetKeyboardOptions` request]: request::GetKeyboardOptions
	#[doc(alias("GetKeyboardControl"))]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetKeyboardOptions: Reply for request::GetKeyboardOptions {
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

		/// Whether the global [auto repeat mode] is enabled.
		///
		/// If the global auto repeat mode is disabled, no keys can have auto
		/// repeat applied. If the global auto repeat mode is enabled, keys
		/// which have their own auto repeat enabled will be repeated.
		///
		/// [auto repeat mode]: crate::set::KeyboardOptions::auto_repeat_mode
		#[doc(alias("global_auto_repeat"))]
		#[metabyte]
		pub global_auto_repeat_mode: Toggle,

		/// A bitmask representing whether each [LED] is lit.
		///
		/// The least significant bit represents the state of [LED] 1. The most
		/// significant bit represents the state of [LED] 32.
		///
		/// [LED]: crate::set::Led
		pub led_mask: u32,

		/// The volume of key clicks.
		///
		/// See [`KeyboardOptions::key_click_volume`] for more information.
		///
		/// [`KeyboardOptions::key_click_volume`]: crate::set::KeyboardOptions::key_click_volume
		#[doc(alias("key_click_percent"))]
		pub key_click_volume: Percentage,

		/// The volume of the bell.
		///
		/// See [`KeyboardOptions::bell_volume`] for more information.
		///
		/// [`KeyboardOptions::bell_volume`]: crate::set::KeyboardOptions::bell_volume
		#[doc(alias("bell_percent"))]
		pub bell_volume: Percentage,
		/// The pitch of the bell.
		///
		/// See [`KeyboardOptions::bell_pitch`] for more information.
		///
		/// [`KeyboardOptions::bell_pitch`]: crate::set::KeyboardOptions::bell_pitch
		pub bell_pitch: Hz<u16>,
		/// The duration for which the bell rings.
		///
		/// See [`KeyboardOptions::bell_duration`] for more information.
		///
		/// [`KeyboardOptions::bell_duration`]: crate::set::KeyboardOptions::bell_duration
		pub bell_duration: Ms<u16>,
		[_; 2],

		/// A bit vector representing whether each key has [auto repeat mode]
		/// enabled.
		///
		/// Byte `N`, starting at `0`, contains the bits for [keycodes] `8N` to
		/// `8N + 7`. The least significant bit in each byte represents key
		/// `8N`.
		///
		/// See [`KeyboardOptions::auto_repeat_mode`] for more information.
		///
		/// [keycodes]: crate::Keycode
		/// [auto repeat mode]: crate::set::KeyboardOptions::auto_repeat_mode
		///
		/// [`KeyboardOptions::auto_repeat_mode`]: crate::set::KeyboardOptions::auto_repeat_mode
		#[doc(alias("auto_repeats"))]
		pub auto_repeat_modes: [u8; 32],
	}

	/// The [reply] to a [`GetCursorOptions` request].
	///
	/// [reply]: Reply
	///
	/// [`GetCursorOptions` request]: request::GetCursorOptions
	#[doc(alias("GetPointerControl", "GetPointerOptions", "GetCursorControl"))]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetCursorOptions: Reply for request::GetCursorOptions {
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

		/// The multiplier applied to the acceleration of the cursor when the
		/// [`threshold`] is exceeded.
		///
		/// [`threshold`]: GetCursorOptions::threshold
		pub acceleration: Fraction<Px<u16>>,
		/// The threshold speed which the cursor must exceed for the
		/// [`acceleration`] multiplier to be applied.
		///
		/// [`acceleration`]: GetCursorOptions::acceleration
		pub threshold: Px<u16>,
		[_; ..],
	}
}

/// Whether a [`SetButtonMapping` request] was successful.
///
/// This is used in the [`SetButtonMapping` reply].
///
/// [`SetButtonMapping` request]: request::SetButtonMapping
/// [`SetButtonMapping` reply]: SetButtonMapping
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum SetButtonMappingStatus {
	/// The [`SetButtonMapping` request] was successful.
	///
	/// [`SetButtonMapping` request]: request::SetButtonMapping
	Success,

	/// The [`SetButtonMapping` request] was unsuccessful because it specified
	/// buttons which are currently held.
	///
	/// The mapping of mouse buttons cannot be changed while they are held.
	///
	/// [`SetButtonMapping` request]: request::SetButtonMapping
	Busy,
}

derive_xrb! {
	/// The [reply] to a [`SetButtonMapping` request].
	///
	/// [reply]: Reply
	///
	/// [`SetButtonMapping` request]: request::SetButtonMapping
	#[doc(alias("SetPointerMapping", "SetCursorMapping"))]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct SetButtonMapping: Reply for request::SetButtonMapping {
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

		/// Whether the [`SetButtonMapping` request] was successful.
		///
		/// See [`SetButtonMappingStatus`] for more information.
		///
		/// [`SetButtonMapping` request]: request::SetButtonMapping
		pub status: SetButtonMappingStatus,
		[_; ..],
	}
}
