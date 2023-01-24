// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Events] defined in the [core X11 protocol].
//!
//! [Events] are messages sent from the X server to an X client.
//!
//! [Events]: crate::message::Event
//! [core X11 protocol]: super

use crate::{
	atom::Atom,
	set::WindowConfigMask,
	unit::Px,
	Button,
	Coords,
	CurrentableTime,
	Drawable,
	GrabMode,
	Keycode,
	ModifierMask,
	Rectangle,
	Region,
	StackMode,
	Timestamp,
	Window,
};

use bitflags::bitflags;
use derivative::Derivative;
use xrbk::{Buf, ConstantX11Size, ReadResult, Readable, ReadableWithContext, X11Size};

use xrbk_macro::{derive_xrb, ConstantX11Size, Readable, Writable, X11Size};
extern crate self as xrb;

derive_xrb! {
	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a key is pressed.
	///
	/// This [event] is generated for all keys: that includes modifier keys.
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`KEY_PRESS`] on the
	/// `event_window`.
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`KEY_PRESS`]: crate::EventMask::KEY_PRESS
	pub struct KeyPress: Event(2) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		#[metabyte]
		/// The keycode of the key that was pressed.
		pub keycode: Keycode,

		/// The time at which this [event] was generated.
		///
		/// [event]: crate::message::Event
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this [event] was generated.
		///
		/// [event]: crate::message::Event
		pub root: Window,
		/// The window which this [event] was generated in relation to.
		///
		/// This window is found by beginning with the window in which the
		/// cursor is located, then searching up the window hierarchy (starting
		/// with that window, then going to its parent, etc.) to find the first
		/// window which any client has selected interest in this event
		/// (provided no window between the two prohibits this event from
		/// generating in its [`do_not_propagate_mask`]).
		///
		/// Active grabs or the currently focused window may modify how the
		/// `event_window` is chosen.
		///
		/// [event]: crate::message::Event
		/// [`do_not_propagate_mask`]: crate::Attributes::do_not_propagate_mask
		pub event_window: Window,
		/// If a child of the `event_window` contains the cursor, this is that
		/// child.
		///
		/// Otherwise, this is [`None`].
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the `time` this [event] was
		/// generated, relative to the `root` [window]'s origin.
		///
		/// [event]: crate::message::Event
		/// [window]: Window
		pub root_coords: Coords,
		/// The coordinates of the cursor at the `time` this [event] was
		/// generated, relative to the `event_window`'s origin.
		///
		/// [event]: crate::message::Event
		pub event_coords: Coords,

		/// The state of mouse buttons and modifier keys immediately
		/// before this [event] was generated.
		///
		/// [event]: crate::message::Event
		pub modifiers: ModifierMask,

		/// Whether the cursor is on the same screen as the `event_window`.
		pub same_screen: bool,
		_,
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a key is released.
	///
	/// This [event] is generated for all keys: that includes modifier keys.
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`KEY_RELEASE`] on the
	/// `event_window`.
	///
	/// [event]: crate::message::Event
	/// [`KEY_RELEASE`]: crate::EventMask::KEY_RELEASE
	pub struct KeyRelease: Event(3) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		#[metabyte]
		/// The keycode of the key which was released.
		pub keycode: Keycode,

		/// The time at which this [event] was generated.
		///
		/// [event]: crate::message::Event
		pub time: Timestamp,

		/// The root [window] containing the [window] in which the cursor was located
		/// within when this [event] was generated.
		///
		/// [window]: Window
		/// [event]: crate::message::Event
		pub root: Window,
		/// The window which this [event] was generated in relation to.
		///
		/// This window is found by beginning with the window in which the
		/// cursor is located, then searching up the window hierarchy (starting
		/// with that window, then going to its parent, etc.) to find the first
		/// window which any client has selected interest in this event
		/// (provided no window between the two prohibits this event from
		/// generating in its [`do_not_propagate_mask`]).
		///
		/// Active grabs or the currently focused window may modify how the
		/// `event_window` is chosen.
		///
		/// [event]: crate::message::Event
		/// [`do_not_propagate_mask`]: crate::Attributes::do_not_propagate_mask
		pub event_window: Window,
		/// If a child of the `event_window` contains the cursor, this is that
		/// child.
		///
		/// Otherwise, this is [`None`].
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the `time` this [event] was
		/// generated, relative to the `root` [window]'s origin.
		///
		/// [window]: Window
		/// [event]: crate::message::Event
		pub root_coords: Coords,
		/// The coordinates of the cursor at the `time` this [event] was
		/// generated, relative to the `event_window`'s origin.
		///
		/// [event]: crate::message::Event
		pub event_coords: Coords,

		/// The state of [mouse buttons] and modifier keys immediately
		/// before this [event] was generated.
		///
		/// [event]: crate::message::Event
		/// [mouse buttons]: Button
		pub modifiers: ModifierMask,

		/// Whether the cursor is on the same [screen] as the `event_window`.
		///
		/// [screen]: crate::Screen
		pub same_screen: bool,
		_,
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [mouse button] is pressed.
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`BUTTON_PRESS`] on the
	/// `event_window`.
	///
	/// [event]: crate::message::Event
	/// [mouse button]: Button
	/// [`BUTTON_PRESS`]: crate::EventMask::BUTTON_PRESS
	pub struct ButtonPress: Event(4) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		#[metabyte]
		/// The mouse button which was pressed.
		pub button: Button,

		/// The time at which this [event] was generated.
		///
		/// [event]: crate::message::Event
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this [event] was generated.
		///
		/// [event]: crate::message::Event
		pub root: Window,
		/// The window which this [event] was generated in relation to.
		///
		/// This window is found by beginning with the window in which the
		/// cursor is located, then searching up the window hierarchy (starting
		/// with that window, then going to its parent, etc.) to find the first
		/// window which any client has selected interest in this event
		/// (provided no window between the two prohibits this event from
		/// generating in its [`do_not_propagate_mask`]).
		///
		/// Active grabs may modify how the `event_window` is chosen.
		///
		/// [event]: crate::message::Event
		/// [`do_not_propagate_mask`]: crate::Attributes::do_not_propagate_mask
		pub event_window: Window,
		/// If a child of the `event_window` contains the cursor, this is that
		/// child.
		///
		/// Otherwise, this is [`None`].
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the `time` this [event] was generated,
		/// relative to the `root` [window]'s origin.
		///
		/// [event]: crate::message::Event
		/// [window]: Window
		pub root_coords: Coords,
		/// The coordinates of the cursor at the `time` this [event] was generated,
		/// relative to the `event_window`'s origin.
		///
		/// [event]: crate::message::Event
		pub event_coords: Coords,

		/// The state of [mouse buttons] and modifier keys immediately
		/// before this [event] was generated.
		///
		/// [mouse buttons]: Button
		/// [event]: crate::message::Event
		pub modifiers: ModifierMask,

		/// Whether the cursor is on the same [screen] as the `event_window`.
		///
		/// [screen]: crate::Screen
		pub same_screen: bool,
		_,
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [mouse button] is released.
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`BUTTON_RELEASE`] on the
	/// `event_window`.
	///
	/// [event]: crate::message::Event
	/// [mouse button]: Button
	/// [`BUTTON_RELEASE`]: crate::EventMask::BUTTON_RELEASE
	pub struct ButtonRelease: Event(5) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		#[metabyte]
		/// The mouse button which was released.
		pub button: Button,

		/// The time at which this [event] was generated.
		///
		/// [event]: crate::message::Event
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this [event] was generated.
		///
		/// [event]: crate::message::Event
		pub root: Window,
		/// The window which this [event] was generated in relation to.
		///
		/// This window is found by beginning with the window in which the
		/// cursor is located, then searching up the window hierarchy (starting
		/// with that window, then going to its parent, etc.) to find the first
		/// window which any client has selected interest in this event
		/// (provided no window between the two prohibits this event from
		/// generating in its [`do_not_propagate_mask`]).
		///
		/// Active grabs may modify how the `event_window` is chosen.
		///
		/// [event]: crate::message::Event
		/// [`do_not_propagate_mask`]: crate::Attributes::do_not_propagate_mask
		pub event_window: Window,
		/// If a child of the `event_window` contains the cursor, this is that
		/// child.
		///
		/// Otherwise, this is [`None`].
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the `time` this [event] was generated,
		/// relative to the `root` window's origin.
		///
		/// [event]: crate::message::Event
		pub root_coords: Coords,
		/// The coordinates of the cursor at the `time` this [event] was generated,
		/// relative to the `event_window`'s origin.
		///
		/// [event]: crate::message::Event
		pub event_coords: Coords,

		/// The state of [mouse buttons] and modifier keys immediately
		/// before this [event] was generated.
		///
		/// [mouse buttons]: Button
		/// [event]: crate::message::Event
		pub modifiers: ModifierMask,

		/// Whether the cursor is on the same [screen] as the `event_window`.
		///
		/// [screen]: crate::Screen
		pub same_screen: bool,
		_,
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, X11Size, Readable, Writable)]
/// The type of [`Motion` event] sent.
///
/// This is used in the [`Motion` event].
///
/// [`Motion` event]: Motion
pub enum MotionNotificationType {
	/// The [`Motion` event] was not one generated for a client selecting
	/// [`MOTION_HINT`].
	///
	/// [`Motion` event]: Motion
	/// [`MOTION_HINT`]: crate::EventMask::MOTION_HINT
	Normal,

	/// The [`Motion` event] was generated for a client selecting
	/// [`MOTION_HINT`].
	///
	/// The X server is free to send only one [`Motion` event] to the client
	/// until:
	/// - a [mouse button] or key is pressed or released; or
	/// - the cursor leaves the `event_window`; or
	/// - the client sends a [`QueryCursor`] or [`GetMotionEvents`] request.
	///
	/// [`Motion` event]: Motion
	/// [`MOTION_HINT`]: crate::EventMask::MOTION_HINT
	/// [mouse button]: Button
	///
	/// [`QueryCursor`]: super::request::QueryCursorLocation
	/// [`GetMotionEvents`]: super::request::GetMotionEvents
	Hint,
}

derive_xrb! {
	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when the cursor moves within a [window].
	///
	/// `Motion` events are only generated when the cursor motion begins and ends
	/// in the same window. If the cursor leaves the window, a [`LeaveWindow` event]
	/// will be generated instead, accompanied by an [`EnterWindow` event] for the
	/// window which it moves into.
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`ANY_MOTION`] or the button
	/// motion event masks ([`BUTTON_1_MOTION`] to [`BUTTON_5_MOTION`], as well
	/// as [`ANY_BUTTON_MOTION`]).
	///
	/// Selecting for [`ANY_MOTION` events] means `Motion` events will be received
	/// independently of the currently pressed mouse buttons. Selecting for
	/// button motion events ([`BUTTON_1_MOTION`]..[`BUTTON_5_MOTION`] and
	/// [`ANY_BUTTON_MOTION`]), however, means `Motion` events will only be
	/// received while at least one of the selected mouse buttons is pressed.
	///
	/// If [`MOTION_HINT`] is selected, the server is free to send only one
	/// `Motion` event with a [`MotionNotificationType`] of [`Hint`] until:
	/// - a mouse button or key is pressed or released; or
	/// - the pointer leaves the `event_window`; or
	/// - the client sends a [`QueryCursor`] or [`GetMotionEvents`].
	///
	/// [`EnterWindow` event]: EnterWindow
	/// [`LeaveWindow` event]: LeaveWindow
	///
	/// [`ANY_MOTION`]: crate::EventMask::ANY_MOTION
	/// [`BUTTON_1_MOTION`]: crate::EventMask::BUTTON_1_MOTION
	/// [`BUTTON_5_MOTION`]: crate::EventMask::BUTTON_5_MOTION
	/// [`ANY_BUTTON_MOTION`]: crate::EventMask::ANY_BUTTON_MOTION
	/// [`MOTION_HINT`]: crate::EventMask::MOTION_HINT
	///
	/// [`Hint`]: MotionNotificationType::Hint
	/// [`QueryCursor`]: super::request::QueryCursorLocation
	/// [`GetMotionEvents`]: super::request::GetMotionEvents
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	pub struct Motion: Event(6) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		#[metabyte]
		/// The type of `Motion` event sent.
		pub notification_type: MotionNotificationType,

		/// The time at which this event was generated.
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this [event] was generated.
		///
		/// [event]: crate::message::Event
		pub root: Window,
		/// The window which this [event] was generated in relation to.
		///
		/// This window is found by beginning with the window in which the
		/// cursor is located, then searching up the window hierarchy (starting
		/// with that window, then going to its parent, etc.) to find the first
		/// window which any client has selected interest in this event
		/// (provided no window between the two prohibits this event from
		/// generating in its [`do_not_propagate_mask`]).
		///
		/// Active grabs may modify how the `event_window` is chosen.
		///
		/// [event]: crate::message::Event
		/// [`do_not_propagate_mask`]: crate::Attributes::do_not_propagate_mask
		pub event_window: Window,
		/// If a child of the `event_window` contains the cursor, this is that
		/// child.
		///
		/// Otherwise, this is [`None`].
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the `time` this [event] was generated,
		/// relative to the `root` [window]'s origin.
		///
		/// [event]: crate::message::Event
		/// [window]: Window
		pub root_coords: Coords,
		/// The coordinates of the cursor at the `time` this [event] was generated,
		/// relative to the `event_window`'s origin.
		///
		/// [event]: crate::message::Event
		pub event_coords: Coords,

		/// The state of [mouse buttons] and modifier keys immediately
		/// before this [event] was generated.
		///
		/// [mouse buttons]: Button
		/// [event]: crate::message::Event
		pub modifiers: ModifierMask,

		/// Whether the cursor is on the same [screen] as the `event_window`.
		///
		/// [screen]: crate::Screen
		pub same_screen: bool,
		_,
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, X11Size, Readable, Writable)]
/// Detail that describes how a [window] receiving a [`LeaveWindow`] or
/// [`EnterWindow`] event relates to the [event] which took place.
///
/// If the cursor moves from window A to window B and A is a descendent of B:
/// - A [`LeaveWindow` event] is generated on A with a detail of [`Ancestor`].
/// - A [`LeaveWindow` event] is generated on each window between A and B
///   exclusive (in that order) with a detail of [`Intermediate`].
/// - An [`EnterWindow` event] is generated on B with a detail of
///   [`Descendent`].
///
/// If the cursor moves from window A to window B and A is an ancestor of B:
/// - A [`LeaveWindow` event] is generated on A with a detail of [`Descendent`].
/// - An [`EnterWindow` event] is generated on each window between A and B
///   exclusive (in that order) with a detail of [`Intermediate`]
/// - An [`EnterWindow` event] is generated on B with a detail of [`Ancestor`].
///
/// If the cursor moves from window A to window B and window C is their least
/// common ancestor:
/// - A [`LeaveWindow` event] is generated on A with a detail of [`Nonlinear`].
/// - A [`LeaveWindow` event] is generated on each window between A and C
///   exclusive (in that order) with a detail of [`NonlinearIntermediate`].
/// - An [`EnterWindow` event] is generated on each window between C and B
///   exclusive (in that order) with a detail of [`NonlinearIntermediate`].
/// - An [`EnterWindow` event] is generated on B with a detail of [`Nonlinear`].
///
/// If the cursor moves from window A to window B and A and B are on different
/// screens:
/// - A [`LeaveWindow` event] is generated on A with a detail of [`Nonlinear`].
/// - If A is not a root window, a [`LeaveWindow` event] is generated on each
///   ancestor of A including its root, in order from A's parent to its root,
///   with a detail of [`NonlinearIntermediate`].
/// - If B is not a root window, an [`EnterWindow` event] is generated on each
///   ancestor of B including its root, in order from B's root to B's parent,
///   with a detail of [`NonlinearIntermediate`].
/// - An [`EnterWindow` event] is generated on B with a detail of [`Nonlinear`].
///
/// [`LeaveWindow` event]: LeaveWindow
/// [`EnterWindow` event]: EnterWindow
///
/// [`Ancestor`]: EnterLeaveDetail::Ancestor
/// [`Intermediate`]: EnterLeaveDetail::Intermediate
/// [`Descendent`]: EnterLeaveDetail::Descendant
///
/// [`Nonlinear`]: EnterLeaveDetail::Nonlinear
/// [`NonlinearIntermediate`]: EnterLeaveDetail::NonlinearIntermediate
///
/// [event]: crate::message::Event
/// [window]: Window
pub enum EnterLeaveDetail {
	/// Used for [`LeaveWindow` events] when the cursor leaves a [window] and
	/// enters an ancestor of that [window], and for [`EnterWindow` events]
	/// when the cursor enters a [window] and leaves an ancestor of that
	/// [window].
	///
	/// [`LeaveWindow` events]: LeaveWindow
	/// [`EnterWindow` events]: EnterWindow
	/// [window]: Window
	Ancestor,
	/// Used in [`LeaveWindow`] and [`EnterWindow`] events for all [windows]
	/// between the newly entered [window] and the previous [window] if one is a
	/// descendent of the other.
	///
	/// [window]: Window
	/// [windows]: Window
	Intermediate,
	/// Used for [`LeaveWindow` events] when the cursor leaves a [window] and
	/// enters a descendent of that [window], and for [`EnterWindow` events]
	/// when the cursor enters a [window] and leaves a descendent of that
	/// [window].
	///
	/// [`LeaveWindow` events]: LeaveWindow
	/// [`EnterWindow` events]: EnterWindow
	/// [window]: Window
	Descendant,

	/// Used for [`LeaveWindow`] and [`EnterWindow`] events for the newly
	/// entered [window] and the previous [window] if neither is a descendent of
	/// the other.
	///
	/// [window]: Window
	Nonlinear,
	/// Used for [`LeaveWindow`] and [`EnterWindow`] events when neither the
	/// [window] that was left nor the [window] that was entered are a
	/// descendent of the other.
	///
	/// This is the detail for the [`LeaveWindow` events] generated for all the
	/// [windows] between the [window] that was left and the least common
	/// ancestor of that [window] and the [window] that was entered (exclusive).
	///
	/// This is the detail for the [`EnterWindow` events] generated for all the
	/// [windows] between the [window] that was entered and the least common
	/// ancestor of that [window] and the [window] that was left (exclusive).
	///
	/// [window]: Window
	/// [windows]: Window
	/// [`LeaveWindow` events]: LeaveWindow
	/// [`EnterWindow` events]: EnterWindow
	NonlinearIntermediate,
}

bitflags! {
	#[derive(Default, X11Size, Readable, ConstantX11Size, Writable)]
	/// A bitmask used in the [`EnterWindow`] and [`LeaveWindow`] events.
	pub struct EnterLeaveMask: u8 {
		/// Whether the `event_window` is the focused [window] or a descendant
		/// of the focused [window].
		///
		/// [window]: Window
		const FOCUS = 0x01;
		/// Whether the cursor is on the same [screen] as the `event_window`.
		///
		/// [screen]: crate::Screen
		const SAME_SCREEN = 0x02;
	}
}

derive_xrb! {
	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when the cursor enters a [window].
	///
	/// This [event] is triggered both when the cursor moves to be in a different
	/// [window] than it was before, as well as when the [window] under the cursor
	/// changes due to a change in the window hierarchy (i.e. [`Unmap`],
	/// [`Map`], [`Configure`], [`Gravity`],
	/// [`Circulate`]).
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`ENTER_WINDOW`] on the
	/// [window].
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`ENTER_WINDOW`]: crate::EventMask::ENTER_WINDOW
	pub struct EnterWindow: Event(7) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		#[metabyte]
		/// Detail about how the [event] was generated.
		///
		/// See [`EnterLeaveDetail`] for more information.
		///
		/// [event]: crate::message::Event
		pub detail: EnterLeaveDetail,

		/// The time at which this [event] was generated.
		///
		/// [event]: crate::message::Event
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this [event] was generated.
		///
		/// [event]: crate::message::Event
		pub root: Window,
		/// The window that the cursor entered.
		pub event_window: Window,
		/// If a child of the `event_window` contains the final cursor position
		/// (`event_coords`), this is that child.
		///
		/// Otherwise, this is [`None`].
		pub child_window: Option<Window>,

		/// The position of the cursor at the time this event was generated,
		/// relative to the `root` window's origin.
		///
		/// This is always the final position of the cursor, not its initial
		/// position.
		pub root_coords: Coords,
		/// The position of the cursor at the time this event was generated,
		/// relative to the `event_window`'s origin, if the `event_window` is on
		/// the [`SAME_SCREEN`].
		///
		/// If the `event_window` is on a different screen, these coordinates
		/// are zero.
		///
		/// This is always the final position of the cursor, not its initial
		/// position.
		///
		/// [`SAME_SCREEN`]: EnterLeaveMask::SAME_SCREEN
		pub event_coords: Coords,

		/// The state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,
		/// [`Normal`] for normal `EnterWindow` events, [`Grab`] and
		/// [`Ungrab`] for events generated by grabs and ungrabs.
		///
		/// [`Normal`]: GrabMode::Normal
		/// [`Grab`]: GrabMode::Grab
		/// [`Ungrab`]: GrabMode::Ungrab
		pub grab_mode: GrabMode,

		/// A bitmask containing two boolean fields, [`FOCUS`] and [`SAME_SCREEN`].
		///
		/// [`FOCUS`]: EnterLeaveMask::FOCUS
		/// [`SAME_SCREEN`]: EnterLeaveMask::SAME_SCREEN
		pub mask: EnterLeaveMask,
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when the cursor leaves a [window].
	///
	/// This event is triggered both when the cursor moves to be in a different
	/// window than it was before, as well as when the window under the cursor
	/// changes due to a change in the window hierarchy (i.e. [`Unmap`],
	/// [`Map`], [`Configure`], [`Gravity`],
	/// [`Circulate`]).
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`LEAVE_WINDOW`] on the
	/// [window].
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`LEAVE_WINDOW`]: crate::EventMask::LEAVE_WINDOW
	pub struct LeaveWindow: Event(8) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		#[metabyte]
		/// Detail about how the [event] was generated.
		///
		/// See [`EnterLeaveDetail`] for more information.
		///
		/// [event]: crate::message::Event
		pub detail: EnterLeaveDetail,

		/// The time at which this [event] was generated.
		///
		/// [event]: crate::message::Event
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this [event] was generated.
		///
		/// [event]: crate::message::Event
		pub root: Window,
		/// The window which the cursor left.
		pub event_window: Window,
		/// If a child of the `event_window` contains the initial cursor position
		/// (`event_coords`), this is that child.
		///
		/// Otherwise, this is [`None`].
		pub child_window: Option<Window>,

		/// The position of the cursor at the `time` this [event] was generated,
		/// relative to the `root` [window]'s origin.
		///
		/// This is always the final position of the cursor, not its initial
		/// position.
		///
		/// [event]: crate::message::Event
		/// [window]: Window
		pub root_coords: Coords,
		/// The position of the cursor at the `time` this [event] was generated,
		/// relative to the `event_window`'s origin, if the `event_window` is on
		/// the [`SAME_SCREEN`].
		///
		/// If the `event_window` is on a different [screen], these coordinates
		/// are zero.
		///
		/// This is always the final position of the cursor, not its initial
		/// position.
		///
		/// [event]: crate::message::Event
		/// [`SAME_SCREEN`]: EnterLeaveMask::SAME_SCREEN
		/// [screen]: crate::Screen
		pub event_coords: Coords,

		/// The state of [mouse buttons] and modifier keys immediately
		/// before this [event] was generated.
		///
		/// [mouse buttons]: Button
		/// [event]: crate::message::Event
		pub modifiers: ModifierMask,
		/// [`Normal`] for normal `LeaveWindow` events, [`Grab`] and
		/// [`Ungrab`] for events generated by grabs and ungrabs.
		///
		/// [`Normal`]: GrabMode::Normal
		/// [`Grab`]: GrabMode::Grab
		/// [`Ungrab`]: GrabMode::Ungrab
		pub grab_mode: GrabMode,

		/// A bitmask containing two boolean fields, [`FOCUS`] and [`SAME_SCREEN`].
		///
		/// [`FOCUS`]: EnterLeaveMask::FOCUS
		/// [`SAME_SCREEN`]: EnterLeaveMask::SAME_SCREEN
		pub mask: EnterLeaveMask,
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, X11Size, Readable, Writable)]
/// Detail describing how a [window] that receives a [`Focus`] or [`Unfocus`]
/// event relates to the [event] that occurred.
///
/// If the focus moves from window A to window B, A is a descendent of B, and
/// the cursor is in window C:
/// - An [`Unfocus` event] is generated on A with a detail of [`Ancestor`].
/// - An [`Unfocus` event] is generated on each window between A and B exclusive
///   (in that order) with a detail of [`Intermediate`].
/// - A [`Focus` event] is generated on B with a detail of [`Descendent`].
/// - If C is a descendent of B but C is not A, nor a descendent of A, nor an
///   ancestor of A, a [`Focus` event] is generated on each descendent of B down
///   to and including C (in that order) with a detail of [`Cursor`].
///
/// If the focus moves from window A to window B, A is an ancestor of B, and the
/// cursor is in window C:
/// - If C is a descendent of A but C is not a descendent of B nor an ancestor
///   of B, an [`Unfocus` event] is generated on C and each ancestor of C up to
///   but not including A (in that order) with a detail of [`Cursor`].
/// - An [`Unfocus` event] is generated on A with a detail of [`Descendent`].
/// - A [`Focus` event] is generated on  each window between A and B exclusive
///   (in that order) with a detail of [`Intermediate`].
/// - A [`Focus` event] is generated on B with a detail of [`Ancestor`].
///
/// If the focus moves from with A to window B, the cursor is in window C, and
/// window D is their least common ancestor:
/// - If C is a descendent of A, an [`Unfocus` event] is generated on C and each
///   ancestor of C up to and including A (in that order) with a detail of
///   [`Cursor`].
/// - An [`Unfocus` event] is generated on A with a detail of [`Nonlinear`].
/// - An [`Unfocus` event] is generated on each window between A and D exclusive
///   (in that order) with a detail of [`NonlinearIntermediate`].
/// - A [`Focus` event] is generated on each window between D and B exclusive
///   (in that order) with a detail of [`NonlinearIntermediate`].
/// - A [`Focus` event] is generated on B with a detail of [`Nonlinear`].
/// - If C is a descendent of B, a [`Focus` event] is generated on each
///   descendent of B down to and including C (in that order) with a detail of
///   [`Cursor`].
///
/// If the focus moves from window A to window B, A and B are on different
/// screens, and the cursor is in window C:
/// - If C is a descendent of A, an [`Unfocus` event] is generated on C and each
///   ancestor of C up to but not including A (in that order) with a detail of
///   [`Cursor`].
/// - An [`Unfocus` event] is generated on A with a detail of [`Nonlinear`].
/// - If A is not a root window, an [`Unfocus` event] is generated on each
///   ancestor of A up to and including its root (in that order) with a detail
///   of [`NonlinearIntermediate`].
/// - If B is not a root window, a [`Focus` event] is generated on each ancestor
///   of B, starting with B's root and ending with B's parent, with a detail of
///   [`NonlinearIntermediate`].
/// - A [`Focus` event] is generated on B with a detail of [`Nonlinear`].
/// - If C is a descendent of B, a [`Focus` event] is generated on each
///   descendent of B down to and including C (in that order) with a detail of
///   [`Cursor`].
///
/// If the focus moves from window A to [`CursorRoot`] or [`None`] and the
/// cursor is in window C:
/// - If C is a descendent of A, an [`Unfocus` event] is generated on C and each
///   ancestor of C up to but not including A (in that order) with a detail of
///   [`Cursor`].
/// - An [`Unfocus` event] is generated on A with a detail of [`Nonlinear`].
/// - If A is not a root window, an [`Unfocus` event] is generated on each
///   ancestor of A up to and including its root (in that order) with a detail
///   of [`NonlinearIntermediate`].
/// - A [`Focus` event] is generated on all root windows with a detail of
///   [`CursorRoot`] or [`None`] respectively.
/// - If the new focus is [`CursorRoot`], a [`Focus` event] is generated on C
///   and each ancestor of C, starting with C's root and ending with C, with a
///   detail of [`Cursor`].
///
/// If the focus moves from [`CursorRoot`] or [`None`] to window A and the
/// cursor is in window C:
/// - If the old focus is [`CursorRoot`], an [`Unfocus` event] is generated on C
///   and each ancestor of C up to and including C's root (in that order) with a
///   detail of [`Cursor`].
/// - An [`Unfocus` event] is generated on all root windows with a detail of
///   [`CursorRoot`] or [`None`] respectively.
/// - If A is not a root window, a [`Focus` event] is generated on each ancestor
///   of A, starting with A's root and ending with A's parent, with a detail of
///   [`NonlinearIntermediate`].
/// - A [`Focus` event] is generated on A with a detail of [`Nonlinear`].
/// - If C is a descendent of A, a [`Focus` event] is generated on each
///   descendent of A down to and including C (in that order) with a detail of
///   [`Cursor`].
///
/// If the focus moves from [`CursorRoot`] to [`None`] and the cursor is in
/// window C:
/// - An [`Unfocus` event] is generated on C and each ancestor of C up to and
///   including C's root (in that order) with a detail of [`Cursor`].
/// - An [`Unfocus` event] is generated on all root windows with a detail of
///   [`CursorRoot`].
/// - A [`Focus` event] is generated on all root windows with a detail of
///   [`None`].
///
/// If the focus moves from [`None`] to [`CursorRoot`] and the cursor is in
/// window C:
/// - An [`Unfocus` event] is generated on all root windows with a detail of
///   [`None`].
/// - A [`Focus` event] is generated on all root windows with a detail of
///   [`CursorRoot`].
/// - A [`Focus` event] is generated on C and each ancestor of C, starting with
///   C's root and ending with C, with a detail of [`Cursor`].
///
/// [`Unfocus` event]: Unfocus
/// [`Focus` event]: Focus
///
/// [`Ancestor`]: FocusDetail::Ancestor
/// [`Intermediate`]:  FocusDetail::Intermediate
/// [`Descendent`]: FocusDetail::Descendent
///
/// [`Nonlinear`]: FocusDetail::Nonlinear
/// [`NonlinearIntermediate`]: FocusDetail::NonlinearIntermediate
///
/// [`Cursor`]: FocusDetail::Cursor
/// [`CursorRoot`]: FocusDetail::CursorRoot
///
/// [`None`]: FocusDetail::None
///
/// [event]: crate::message::Event
/// [window]: Window
pub enum FocusDetail {
	/// Used for [`Unfocus` events] for the [window] which has been unfocused if
	/// the newly focused [window] is an ancestor of that [window], and for
	/// [`Focus` events] for the [window] which has been focused if the newly
	/// unfocused [window] is an ancestor of that [window].
	///
	/// [window]: Window
	/// [`Unfocus` events]: Unfocus
	/// [`Focus` events]: Focus
	Ancestor,
	/// Used for [`Unfocus`] and [`Focus`] events for each [window] between the
	/// [window] that was unfocused and the [window] that was focused if one is
	/// a descendent of the other.
	///
	/// [window]: Window
	Intermediate,
	/// Used for [`Unfocus` events] for the [window] which has been unfocused if
	/// the newly focused [window] is a descendent of that [window], and for
	/// [`Focus` events] for the [window] which has been focused if the newly
	/// unfocused [window] is a descendent of that [window].
	///
	/// [`Unfocus` events]: Unfocus
	/// [`Focus` events]: Focus
	/// [window]: Window
	Descendent,

	/// Used for [`Unfocus`] and [`Focus`] events for both the [window] that was
	/// unfocused and the [window] that was focused if neither [window] is a
	/// descendent of the other.
	///
	/// [window]: Window
	Nonlinear,
	/// Used for [`Unfocus`] and [`Focus`] events for each [window] between the
	/// unfocused and focused [windows]' least common ancestor and the unfocused
	/// [window] and focused [window] respectively.
	///
	/// [window]: Window
	/// [windows]: Window
	NonlinearIntermediate,

	Cursor,

	CursorRoot,
	None,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, X11Size, Readable, Writable)]
/// Detail about how an [`Unfocus`] or [`Focus`] event was generated in relation
/// to grabs.
pub enum FocusGrabMode {
	/// Used for [`Unfocus`] and [`Focus`] events generated when the keyboard is
	/// not grabbed.
	Normal,

	/// Used for [`Unfocus`] and [`Focus`] events generated by the activation of
	/// a keyboard grab.
	Grab,
	/// Used for [`Unfocus`] and [`Focus`] events generated by the deactivation
	/// of a keyboard grab.
	Ungrab,

	/// Used for [`Unfocus`] and [`Focus`] events generated while the keyboard
	/// is grabbed.
	WhileGrabbed,
}

derive_xrb! {
	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [window] is focused.
	///
	/// `Focus` events generated when the keyboard is not grabbed have
	/// [`FocusGrabMode::Normal`], `Focus` events generated when the keyboard
	/// is grabbed have [`FocusGrabMode::WhileGrabbed`], `Focus` events
	/// generated by a keyboard grab activating have [`FocusGrabMode::Grab`],
	/// and `Focus` events generated by a keyboard grab deactivating have
	/// [`FocusGrabMode::Ungrab`].
	///
	/// # Recipients
	/// `Focus` events are reported to clients selecting [`FOCUS_CHANGE`] on the
	/// [window] that was focused.
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`FOCUS_CHANGE`]: crate::EventMask::FOCUS_CHANGE
	pub struct Focus: Event(9) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		#[metabyte]
		/// Detail about how the [event] was generated.
		///
		/// See [`FocusDetail`] for more information.
		///
		/// [event]: crate::message::Event
		pub detail: FocusDetail,

		/// The window which was focused.
		pub window: Window,

		/// How this [event] was generated in relation to grabs.
		///
		/// [`Normal`] for normal `Focus` events, [`Grab`] and [`Ungrab`] for
		/// events generated by grabs and ungrabs, [`WhileGrabbed`] for events
		/// generated by a [`SetInputFocus` request] while the keyboard is
		/// grabbed.
		///
		/// [event]: crate::message::Event
		///
		/// [`Normal`]: FocusGrabMode::Normal
		/// [`Grab`]: FocusGrabMode::Grab
		/// [`Ungrab`]: FocusGrabMode::Ungrab
		/// [`WhileGrabbed`]: FocusGrabMode::WhileGrabbed
		///
		/// [`SetInputFocus` request]: super::request::SetInputFocus
		pub grab_mode: FocusGrabMode,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [window] is unfocused.
	///
	/// `Unfocus` events generated when the keyboard is not grabbed have
	/// [`FocusGrabMode::Normal`], `Unfocus` events generated when the keyboard
	/// is grabbed have [`FocusGrabMode::WhileGrabbed`], `Unfocus` events
	/// generated by a keyboard grab activating have [`FocusGrabMode::Grab`],
	/// and `Unfocus` events generated by a keyboard grab deactivating have
	/// [`FocusGrabMode::Ungrab`].
	///
	/// # Recipients
	/// `Unfocus` events are reported to clients selecting [`FOCUS_CHANGE`] on the
	/// [window] that was unfocused.
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`FOCUS_CHANGE`]: crate::EventMask::FOCUS_CHANGE
	pub struct Unfocus: Event(10) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		#[metabyte]
		/// Detail about how the [event] was generated.
		///
		/// See [`FocusDetail`] for more information.
		///
		/// [event]: crate::message::Event
		pub detail: FocusDetail,

		/// The window which was unfocused.
		pub window: Window,

		/// How this [event] was generated in relation to grabs.
		///
		/// [`Normal`] for normal `Focus` events, [`Grab`] and [`Ungrab`] for
		/// events generated by grabs and ungrabs, [`WhileGrabbed`] for events
		/// generated by a [`SetInputFocus` request] while the keyboard is
		/// grabbed.
		///
		/// [event]: crate::message::Event
		///
		/// [`Normal`]: FocusGrabMode::Normal
		/// [`Grab`]: FocusGrabMode::Grab
		/// [`Ungrab`]: FocusGrabMode::Ungrab
		/// [`WhileGrabbed`]: FocusGrabMode::WhileGrabbed
		///
		/// [`SetInputFocus` request]: super::request::SetInputFocus
		pub grab_mode: FocusGrabMode,
		[_; ..],
	}

	#[derive(Debug, Hash, X11Size, Readable, Writable)]
	/// An [event] describing the current state of the keyboard.
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`KEYBOARD_STATE`] on a
	/// [window] immediately after every [`EnterWindow`] and [`Focus`] event.
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`KEYBOARD_STATE`]: crate::EventMask::KEYBOARD_STATE
	pub struct KeyboardState: Event(11) {
		/// A bit vector representing the current keyboard state.
		///
		/// Each bit set to 1 indicates that the corresponding key is currently
		/// pressed. Byte `N` (starting at 1 - [keycodes] 0 to 7 are not present)
		/// contains the bits for [keycodes] `8N` to `8N + 7`, with the least
		/// significant bit in the byte representing key `8N`.
		///
		/// [keycodes]: Keycode
		pub keys: [u8; 31],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a rectangular area of a [window] needs to be
	/// rendered.
	///
	/// This event is generated when no valid contents are available for regions
	/// of a window, and either:
	/// - the regions are visible; or
	/// - the regions are viewable and the server is maintaining the contents of
	///   the window; or
	/// - the window is not viewable but the server is honoring the window's
	///   [`maintain_contents` attribute] of [`Always`] or [`WhenMapped`].
	///
	/// The regions are decomposed into an arbitrary set of rectangles, and an
	/// `Expose` event is generated for each one.
	///
	/// `Expose` events are never generated on [`WindowClass::InputOnly`]
	/// windows.
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`EXPOSURE`] on a [window].
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	///
	/// [`maintain_contents` attribute]: crate::set::Attributes::maintain_contents
	/// [`Always`]: crate::MaintainContents::Always
	/// [`WhenMapped`]: crate::MaintainContents::WhenMapped
	/// [`WindowClass::InputOnly`]: crate::WindowClass::InputOnly
	///
	/// [`EXPOSURE`]: crate::EventMask::EXPOSURE
	pub struct Expose: Event(12) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [event]: crate::message::Event
		/// [request]: crate::message::Request
		pub sequence: u16,

		/// The window which this `Expose` event applies to.
		pub window: Window,
		/// The region of the `window` which this `Expose` event applies to.
		pub region: Region,

		/// The minimum number of `Expose` events that follow for this `window`.
		///
		/// A `count` of `0` is guaranteed to mean no more `Expose` events for
		/// this `window` follow.
		pub count: u16,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when using graphics operations when a region of a
	/// source [`Drawable`] is obscured.
	///
	/// This [event] is generated when a region of the `destination` could not be
	/// computed because part of the `source` was obscured or out of bounds.
	///
	/// # Recipients
	/// This [event] is reported to a client using a [`GraphicsContext`] with
	/// [`graphics_exposure`] enabled.
	///
	/// [event]: crate::message::Event
	/// [`GraphicsContext`]: crate::GraphicsContext
	/// [`graphics_exposure`]: crate::set::GraphicsOptions::graphics_exposure
	pub struct GraphicsExposure: Event(13) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [event]: crate::message::Event
		/// [request]: crate::message::Request
		pub sequence: u16,

		/// The [`Drawable`] this `GraphicsExposure` event applies to.
		pub drawable: Drawable,
		/// The obscured or out-of-bounds source region.
		pub region: Region,

		/// The [minor opcode] identifying the graphics request used.
		///
		/// For the core protocol, this is always zero.
		///
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		pub minor_opcode: u16,

		/// The minimum number of `GraphicsExposure` events that follow for this
		/// [`Drawable`].
		///
		/// A `count` of `0` is guaranteed to mean no more `GraphicsExposure`
		/// events for this [`Drawable`] follow.
		pub count: u16,

		/// The [major opcode] identifying the graphics request used.
		///
		/// For the core protocol, this always refers to [`CopyArea`] or
		/// [`CopyPlane`].
		///
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		/// [`CopyArea`]: super::request::CopyArea
		/// [`CopyPlane`]: super::request::CopyPlane
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a graphics request which might generate
	/// [`GraphicsExposure` events] doesn't generate any.
	///
	/// # Recipients
	/// This [event] is reported to a client using a [`GraphicsContext`] with
	/// [`graphics_exposure`] enabled.
	///
	/// [event]: crate::message::Event
	/// [`GraphicsExposure` events]: GraphicsExposure
	/// [`GraphicsContext`]: crate::GraphicsContext
	/// [`graphics_exposure`]: crate::set::GraphicsOptions::graphics_exposure
	pub struct NoExposure: Event(14) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The [`Drawable`] this `NoExposure` event applies to.
		///
		/// This is the `destination` of the graphics request.
		pub drawable: Drawable,

		/// The [minor opcode] identifying the graphics request used.
		///
		/// For the core protocol, this is always zero.
		///
		/// [minor opcode]: crate::message::Request::MINOR_OPCODE
		pub minor_opcode: u16,
		/// The [major opcode] identifying the graphics request used.
		///
		/// For the core protocol, this always refers to [`CopyArea`] or
		/// [`CopyPlane`].
		///
		/// [major opcode]: crate::message::Request::MAJOR_OPCODE
		/// [`CopyArea`]: super::request::CopyArea
		/// [`CopyPlane`]: super::request::CopyPlane
		pub major_opcode: u8,
		[_; ..],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, X11Size, Readable, Writable)]
/// The state of a [window]'s visibility.
///
/// This is used in the [`Visibility` event].
///
/// [window]: Window
/// [`Visibility` event]: Visibility
pub enum VisibilityState {
	/// There is nothing obscuring the `window`.
	///
	/// This is used in the [`Visibility` event] when a [window] changes state
	/// to be `Unobscured`.
	///
	/// [window]: Window
	/// [`Visibility` event]: Visibility
	Unobscured,

	/// The `window` is partially, but not fully, obscured.
	///
	/// This is used in the [`Visibility` event] when a [window] changes state
	/// to be `PartiallyObscured`.
	///
	/// [window]: Window
	/// [`Visibility` event]: Visibility
	PartiallyObscured,

	/// The `window` is fully obscured.
	///
	/// This is used in the [`Visibility` event] when a [window] changes state
	/// to be `FullyObscured`.
	///
	/// [window]: Window
	/// [`Visibility` event]: Visibility
	FullyObscured,
}

derive_xrb! {
	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when changes to a [window]'s visibility occur.
	///
	/// The [window]'s visibility is calculated ignoring all of its subwindows.
	///
	/// When a [window] changes state from not viewable, [`PartiallyObscured`],
	/// or [`FullyObscured`] to viewable and [`Unobscured`], a `Visibility`
	/// event with [`VisibilityState::Unobscured`] is generated.
	///
	/// When a [window] changes state from viewable and [`Unobscured`], viewable
	/// and [`FullyObscured`], or not viewable, to viewable and
	/// [`PartiallyObscured`], a `Visibility` event with
	/// [`VisibilityState::PartiallyObscured`] is generated.
	///
	/// When a [window] changes state from viewable and [`Unobscured`], viewable
	/// and [`PartiallyObscured`], or not viewable to viewable and
	/// [`FullyObscured`], a `Visibility` event with
	/// [`VisibilityState::FullyObscured`] is generated.
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`VISIBILITY_CHANGE`] on
	/// the [window].
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	///
	/// [`Unobscured`]: VisibilityState::Unobscured
	/// [`PartiallyObscured`]: VisibilityState::PartiallyObscured
	/// [`FullyObscured`]: VisibilityState::FullyObscured
	///
	/// [`VISIBILITY_CHANGE`]: crate::EventMask::VISIBILITY_CHANGE
	pub struct Visibility: Event(15) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The window this `Visibility` event applies to.
		pub window: Window,
		/// The new [`VisibilityState`] of the window.
		pub visibility: VisibilityState,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [window] is created.
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`SUBSTRUCTURE_NOTIFY`] on
	/// the [window]'s parent.
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::EventMask::SUBSTRUCTURE_NOTIFY
	pub struct Create: Event(16) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The parent of the `window` that was created.
		pub parent: Window,
		/// The window that was created.
		pub window: Window,

		/// The geometry (coordinates and dimensions) of the `window`.
		///
		/// The `window`'s coordinates are relative to its `parent`'s origin.
		///
		/// The `window`'s dimensions exclude its border.
		pub geometry: Rectangle,
		/// The width of the border of the `window` that was created.
		///
		/// This is zero for [`WindowClass::InputOnly`] windows.
		///
		/// [`WindowClass::InputOnly`]: crate::WindowClass::InputOnly
		pub border_width: Px<u16>,

		/// Whether [`MapWindow`] and [`ConfigureWindow`] requests on the newly
		/// created `window` should override a [`SUBSTRUCTURE_REDIRECT`] on the
		/// window's `parent`.
		///
		/// This is typically set to inform the window manager not to tamper
		/// with the `window`.
		///
		/// [`MapWindow`]: super::request::MapWindow
		/// [`ConfigureWindow`]: super::request::ConfigureWindow
		///
		/// [`SUBSTRUCTURE_REDIRECT`]: crate::EventMask::SUBSTRUCTURE_REDIRECT
		pub override_redirect: bool,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [window] is destroyed.
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`STRUCTURE_NOTIFY`] on the
	/// [window], as well as to clients selecting [`SUBSTRUCTURE_NOTIFY`] on its
	/// parent.
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`STRUCTURE_NOTIFY`]: crate::EventMask::STRUCTURE_NOTIFY
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::EventMask::SUBSTRUCTURE_NOTIFY
	pub struct Destroy: Event(17) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The window on which this `Destroy` event was generated.
		///
		/// For clients selecting [`STRUCTURE_NOTIFY`] on the window that was
		/// destroyed, this is that window. For clients selecting
		/// [`SUBSTRUCTURE_NOTIFY`] on the window's parent, this is that parent.
		///
		/// [`STRUCTURE_NOTIFY`]: crate::EventMask::STRUCTURE_NOTIFY
		/// [`SUBSTRUCTURE_NOTIFY`]: crate::EventMask::SUBSTRUCTURE_NOTIFY
		pub event_window: Window,
		/// The window which was destroyed.
		pub window: Window,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [window] is unmapped.
	///
	/// Unmapping a [window] is the X term for hiding it. This is commonly used to
	/// minimize a [window], for example.
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`STRUCTURE_NOTIFY`] on the
	/// window, and to clients selecting [`SUBSTRUCTURE_NOTIFY`] on its parent.
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`STRUCTURE_NOTIFY`]: crate::EventMask::STRUCTURE_NOTIFY
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::EventMask::SUBSTRUCTURE_NOTIFY
	pub struct Unmap: Event(18) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The window on which this `Unmap` event was generated.
		///
		/// For clients selecting [`STRUCTURE_NOTIFY`] on the `window` that was
		/// unmapped, this is that `window`. For clients selecting
		/// [`SUBSTRUCTURE_NOTIFY`] on the `window`'s parent, this is that parent.
		///
		/// [`STRUCTURE_NOTIFY`]: crate::EventMask::STRUCTURE_NOTIFY
		/// [`SUBSTRUCTURE_NOTIFY`]: crate::EventMask::SUBSTRUCTURE_NOTIFY
		pub event_window: Window,
		/// The window that was unmapped.
		pub window: Window,

		/// Whether this [event] was generated as a result of its parent being
		/// resized when the unmapped [window] had [`WindowGravity::Unmap`].
		///
		/// [event]: crate::message::Event
		/// [window]: Window
		/// [`WindowGravity::Unmap`]: crate::WindowGravity::Unmap
		pub from_configure: bool,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [window] is mapped.
	///
	/// Mapping a [window] is the X term for showing it. It is the reverse of
	/// 'minimizing' the [window].
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`STRUCTURE_NOTIFY`] on the
	/// [window] and to clients selecting [`SUBSTRUCTURE_NOTIFY`] on the parent.
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`STRUCTURE_NOTIFY`]: crate::EventMask::STRUCTURE_NOTIFY
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::EventMask::SUBSTRUCTURE_NOTIFY
	pub struct Map: Event(19) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The window on which this `Map` event was generated.
		///
		/// For clients selecting [`STRUCTURE_NOTIFY`] on the `window` that was
		/// mapped, this is that `window`. For clients selecting
		/// [`SUBSTRUCTURE_NOTIFY`] on the `window`'s parent, this is that parent.
		///
		/// [`STRUCTURE_NOTIFY`]: crate::EventMask::STRUCTURE_NOTIFY
		/// [`SUBSTRUCTURE_NOTIFY`]: crate::EventMask::SUBSTRUCTURE_NOTIFY
		pub event_window: Window,
		/// The window that was mapped.
		pub window: Window,

		/// Whether [`MapWindow`] and [`ConfigureWindow`] requests on the
		/// `window` should override a [`SUBSTRUCTURE_REDIRECT`] on the
		/// `window`'s parent.
		///
		/// This is typically set to inform the window manager not to tamper
		/// with the `window`.
		///
		/// [`MapWindow`]: super::request::MapWindow
		/// [`ConfigureWindow`]: super::request::ConfigureWindow
		///
		/// [`SUBSTRUCTURE_REDIRECT`]: crate::EventMask::SUBSTRUCTURE_REDIRECT
		pub override_redirect: bool,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when an unmapped [window] with an
	/// [`override_redirect` attribute] of `false` sends a [`MapWindow` request].
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`SUBSTRUCTURE_REDIRECT`]
	/// on the [window]'s parent. The [window] would not actually be mapped unless
	/// the client selecting [`SUBSTRUCTURE_REDIRECT`] sends its own
	/// [`MapWindow` request] for the [window].
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`override_redirect` attribute]: crate::Attributes::override_redirect
	/// [`MapWindow` request]: super::request::MapWindow
	///
	/// [`SUBSTRUCTURE_REDIRECT`]: crate::EventMask::SUBSTRUCTURE_REDIRECT
	pub struct MapWindowRequest: Event(20) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The `window`'s parent.
		pub parent: Window,
		/// The window that sent the [`MapWindow` request].
		///
		/// [`MapWindow` request]: super::request::MapWindow
		pub window: Window,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [window] is reparented.
	///
	/// Reparenting a [window] means to remove it from its current position in
	/// the window hierarchy and place it as the child of a new parent [window].
	///
	/// # Recipients
	/// This [event] is reported to client selecting [`SUBSTRUCTURE_NOTIFY`] on
	/// either the old parent or the `new_parent`, and to clients selecting
	/// [`STRUCTURE_NOTIFY`] on the `window` itself.
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::EventMask::SUBSTRUCTURE_NOTIFY
	/// [`STRUCTURE_NOTIFY`]: crate::EventMask::STRUCTURE_NOTIFY
	pub struct Reparent: Event(21) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The window on which this `Reparent` event was generated.
		///
		/// For clients selecting [`STRUCTURE_NOTIFY`] on the `window` that was
		/// reparented, this is that `window`. For clients selecting
		/// [`SUBSTRUCTURE_NOTIFY`] on the `window`'s old parent or
		/// `new_parent`, this is that parent.
		///
		/// [`STRUCTURE_NOTIFY`]: crate::EventMask::STRUCTURE_NOTIFY
		/// [`SUBSTRUCTURE_NOTIFY`]: crate::EventMask::SUBSTRUCTURE_NOTIFY
		pub event_window: Window,
		/// The window which was reparented.
		pub window: Window,
		/// The `window`'s new parent.
		pub new_parent: Window,

		/// The `window`'s new coordinates relative to its `new_parent`'s origin.
		pub coords: Coords,

		/// Whether [`MapWindow`] and [`ConfigureWindow`] requests on the
		/// `window` should override a [`SUBSTRUCTURE_REDIRECT`] on the
		/// `window`'s parent.
		///
		/// This is typically set to inform the window manager not to tamper
		/// with the `window`.
		///
		/// [`MapWindow`]: super::request::MapWindow
		/// [`ConfigureWindow`]: super::request::ConfigureWindow
		///
		/// [`SUBSTRUCTURE_REDIRECT`]: crate::EventMask::SUBSTRUCTURE_REDIRECT
		pub override_redirect: bool,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [`ConfigureWindow` request] changes the state
	/// of a [window].
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`STRUCTURE_NOTIFY`] on the
	/// window, and to clients selecting [`SUBSTRUCTURE_NOTIFY`] on its parent.
	///
	/// [event]: crate::message::Event
	/// [`ConfigureWindow` request]: super::request::ConfigureWindow
	/// [window]: Window
	/// [`STRUCTURE_NOTIFY`]: crate::EventMask::STRUCTURE_NOTIFY
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::EventMask::SUBSTRUCTURE_NOTIFY
	pub struct Configure: Event(22) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The window on which this `Configure` event was generated.
		///
		/// For clients selecting [`STRUCTURE_NOTIFY`] on the `window` that was
		/// configured, this is that `window`. For clients selecting
		/// [`SUBSTRUCTURE_NOTIFY`] on the `window`'s parent, this is that parent.
		///
		/// [`STRUCTURE_NOTIFY`]: crate::EventMask::STRUCTURE_NOTIFY
		/// [`SUBSTRUCTURE_NOTIFY`]: crate::EventMask::SUBSTRUCTURE_NOTIFY
		pub event_window: Window,
		/// The window which was configured in the [`ConfigureWindow` request].
		///
		/// [`ConfigureWindow` request]: super::request::ConfigureWindow
		pub window: Window,
		/// The `window`'s sibling which is directly below it in the window
		/// stack.
		///
		/// If the `window` has no siblings or the `window` is lower than all
		/// its siblings in the window stack, this is [`None`].
		pub sibling_below: Option<Window>,

		/// The geometry (coordinates and dimensions) of the `window`.
		///
		/// The `window`'s coordinates are relative to its `parent`'s origin.
		///
		/// The `window`'s dimensions exclude its border.
		pub geometry: Rectangle,
		/// The width of the configured `window`'s border.
		///
		/// This is zero for [`WindowClass::InputOnly`] windows.
		///
		/// [`WindowClass::InputOnly`]: crate::WindowClass::InputOnly
		pub border_width: Px<u16>,

		/// Whether [`MapWindow`] and [`ConfigureWindow`] requests on the
		/// configured `window` should override a [`SUBSTRUCTURE_REDIRECT`] on
		/// its `parent`.
		///
		/// This is typically set to inform the window manager not to tamper
		/// with the `window`.
		///
		/// [`MapWindow`]: super::request::MapWindow
		/// [`ConfigureWindow`]: super::request::ConfigureWindow
		///
		/// [`SUBSTRUCTURE_REDIRECT`]: crate::EventMask::SUBSTRUCTURE_REDIRECT
		pub override_redirect: bool,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [window] sends a [`ConfigureWindow` request].
	///
	/// This [event] is generated when a client other than the one selecting
	/// [`SUBSTRUCTURE_REDIRECT`] sends a [`ConfigureWindow` request] for that
	/// [window].
	///
	/// The `mask` and corresponding values are reported as given in the
	/// request. The remaining values are filled in from the current geometry of
	/// the `window`, except for `sibling` and `stack_mode`, which are reported as
	/// [`None`] and [`StackMode::Above`] respectively if not given in the
	/// request.
	///
	/// # Recipients
	/// This [event] is reported to the client selecting [`SUBSTRUCTURE_REDIRECT`]
	/// on the window's parent.
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`ConfigureWindow` request]: super::request::ConfigureWindow
	///
	/// [`SUBSTRUCTURE_REDIRECT`]: crate::EventMask::SUBSTRUCTURE_REDIRECT
	pub struct ConfigureWindowRequest: Event(23) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		#[metabyte]
		/// The [`StackMode`] to use to restack the window.
		///
		/// See [`StackMode`] for more information.
		///
		/// [stacking mode]: StackMode
		pub stack_mode: StackMode,

		/// The `window`'s parent, on which [`SUBSTRUCTURE_REDIRECT`] is
		/// selected.
		///
		/// [`SUBSTRUCTURE_REDIRECT`]: crate::EventMask::SUBSTRUCTURE_REDIRECT
		pub parent: Window,
		/// The window for which the [`ConfigureWindow` request] was sent.
		///
		/// [`ConfigureWindow` request]: super::request::ConfigureWindow
		pub window: Window,

		/// The `window`'s sibling that the `stack_mode` is applied to, if
		/// provided.
		///
		/// See [`ConfigureWindow`] for more information.
		///
		/// [`ConfigureWindow`]: super::request::ConfigureWindow
		pub sibling: Option<Window>,

		/// The geometry (coordinates and dimensions) of the `window`.
		///
		/// The `window`'s coordinates are relative to its `parent`'s origin.
		///
		/// The `window`'s dimensions exclude its border.
		pub geometry: Rectangle,

		/// A bitmask representing which attributes were configured in the
		/// [`ConfigureWindow` request].
		///
		/// [`ConfigureWindow` request]: super::request::ConfigureWindow
		pub mask: WindowConfigMask,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [window] is moved because its parent is
	/// resized.
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`STRUCTURE_NOTIFY`] on the
	/// window, and to clients selecting [`SUBSTRUCTURE_NOTIFY`] on its parent.
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`STRUCTURE_NOTIFY`]: crate::EventMask::STRUCTURE_NOTIFY
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::EventMask::SUBSTRUCTURE_NOTIFY
	pub struct Gravity: Event(24) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The window which this `Gravity` event was generated on.
		///
		/// For clients selecting [`STRUCTURE_NOTIFY`] on the `window` that was
		/// moved, this is that `window`. For clients selecting
		/// [`SUBSTRUCTURE_NOTIFY`] on the `window`'s parent, this is that parent.
		///
		/// [`STRUCTURE_NOTIFY`]: crate::EventMask::STRUCTURE_NOTIFY
		/// [`SUBSTRUCTURE_NOTIFY`]: crate::EventMask::SUBSTRUCTURE_NOTIFY
		pub event_window: Window,
		/// The window which was moved.
		pub window: Window,

		/// The new coordinates of the `window`, relative to its parent's
		/// origin.
		pub coords: Coords,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [window] on which a client is selecting
	/// [`RESIZE_REDIRECT`] has a [`ConfigureWindow` request] sent by another
	/// client attempt to change the [window]'s size.
	///
	/// # Recipients
	/// This [event] is reported to the client selecting [`RESIZE_REDIRECT`] on
	/// the [window].
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`RESIZE_REDIRECT`]: crate::EventMask::RESIZE_REDIRECT
	/// [`ConfigureWindow` request]: super::request::ConfigureWindow
	pub struct ResizeRequest: Event(25) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The window which the [`ConfigureWindow` request] attempted to
		/// resize.
		///
		/// [`ConfigureWindow` request]: super::request::ConfigureWindow
		pub window: Window,

		/// The width which the [`ConfigureWindow` request] is attempting to
		/// resize the `window` to.
		///
		/// [`ConfigureWindow` request]: super::request::ConfigureWindow
		pub width: Px<u16>,
		/// The height which the [`ConfigureWindow` request] is attempting to
		/// resize the `window` to.
		///
		/// [`ConfigureWindow` request]: super::request::ConfigureWindow
		pub height: Px<u16>,
		[_; ..],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, X11Size, Readable, Writable)]
/// The new placement of a [window] restacked in a [`CirculateWindow` request].
///
/// This is used in [`Circulate` events].
///
/// [window]: Window
/// [`CirculateWindow` request]: super::request::CirculateWindow
/// [`Circulate` events]: Circulate
pub enum Placement {
	/// The `window` is now above all its siblings in the stack.
	Top,
	/// The `window` is now below all its siblings in the stack.
	Bottom,
}

derive_xrb! {
	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [window] is restacked due to a
	/// [`CirculateWindow` request].
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`STRUCTURE_NOTIFY`] on the
	/// [window], and to clients selecting [`SUBSTRUCTURE_NOTIFY`] on its parent.
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`CirculateWindow` request]: super::request::CirculateWindow
	///
	/// [`STRUCTURE_NOTIFY`]: crate::EventMask::STRUCTURE_NOTIFY
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::EventMask::SUBSTRUCTURE_NOTIFY
	pub struct Circulate: Event(26) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The window which this `Circulate` event was generated on.
		///
		/// For clients selecting [`STRUCTURE_NOTIFY`] on the `window` that was
		/// restacked, this is that `window`. For clients selecting
		/// [`SUBSTRUCTURE_NOTIFY`] on the `window`'s parent, this is that parent.
		///
		/// [`STRUCTURE_NOTIFY`]: crate::EventMask::STRUCTURE_NOTIFY
		/// [`SUBSTRUCTURE_NOTIFY`]: crate::EventMask::SUBSTRUCTURE_NOTIFY
		pub event_window: Window,
		/// The window which was restacked.
		pub window: Window,
		[_; 4],

		/// The new placement in the window stack of the `window` in relation to
		/// its siblings.
		pub placement: Placement,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [`CirculateWindow` request] is sent for a
	/// [window] and that [window] actually needs to be restacked.
	///
	/// # Recipients
	/// This [event] is reported to the client selecting [`SUBSTRUCTURE_REDIRECT`]
	/// on the [window]'s parent.
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`SUBSTRUCTURE_REDIRECT`]: crate::EventMask::SUBSTRUCTURE_REDIRECT
	/// [`CirculateWindow` request]: super::request::CirculateWindow
	pub struct CirculateWindowRequest: Event(27) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The parent of the `window` the [`CirculateWindow` request] applies
		/// to.
		///
		/// This is the window that this `CirculateWindowRequest` event was
		/// generated on.
		///
		/// [`CirculateWindow` request]: super::request::CirculateWindow
		pub parent: Window,
		/// The window which the [`CirculateWindow` request] is attempting to
		/// restack.
		///
		/// [`CirculateWindow` request]: super::request::CirculateWindow
		pub window: Window,
		[_; 4],

		/// The requested placement in the window stack of the `window` in
		/// relation to its siblings.
		pub placement: Placement,
		[_; ..],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, X11Size, Readable, Writable)]
/// Whether a `property` was [`Modified`] or [`Deleted`] in a [`Property`
/// event].
///
/// [`Property` event]: Property
/// [`Modified`]: PropertyChange::Modified
/// [`Deleted`]: PropertyChange::Deleted
pub enum PropertyChange {
	/// The `property` was added or its value was changed.
	Modified,
	/// The `property` was removed.
	Deleted,
}

derive_xrb! {
	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [window] property is added, modified, or
	/// removed.
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`PROPERTY_CHANGE`] on the
	/// [window].
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [`PROPERTY_CHANGE`]: crate::EventMask::PROPERTY_CHANGE
	pub struct Property: Event(28) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The window on which the `property` was changed.
		pub window: Window,

		/// The property that was changed.
		pub property: Atom,
		/// The time at which the `property` was changed.
		pub time: Timestamp,
		/// Whether the `property` was [`Modified`] or [`Deleted`].
		///
		/// [`Modified`]: PropertyChange::Modified
		/// [`Deleted`]: PropertyChange::Deleted
		pub change: PropertyChange,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a new selection owner is defined for a
	/// selection.
	///
	/// A new selection owner is defined via the use of the
	/// [`SetSelectionOwner` request].
	///
	/// # Recipients
	/// This [event] is reported to the current owner of a selection.
	///
	/// [event]: crate::message::Event
	/// [`SetSelectionOwner` request]: super::request::SetSelectionOwner
	pub struct SelectionClear: Event(29) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The time at which the new `selection` owner was defined.
		pub time: Timestamp,
		/// The current owner of the `selection`.
		pub owner: Window,
		/// The selection which had a new selection owner defined.
		pub selection: Atom,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [`ConvertSelection` request] is sent.
	///
	/// The owner should convert the selection based on the specified target
	/// type and send a [`Selection` event] back to the requester using the
	/// [`SendEvent` request].
	///
	/// A complete specification for using selections is given in the
	/// _Inter-Client Communication Conventions Manual._
	///
	/// # Recipients
	/// This [event] is reported to the owner of the selection.
	///
	/// [event]: crate::message::Event
	/// [`ConvertSelection` request]: super::request::ConvertSelection
	/// [`Selection` event]: Selection
	/// [`SendEvent` request]: super::request::SendEvent
	pub struct ConvertSelectionRequest: Event(30) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The time at which the [`ConvertSelection` request] was sent.
		///
		/// [`ConvertSelection` request]: super::request::ConvertSelection
		pub time: CurrentableTime,

		/// The owner of the `selection`.
		pub owner: Window,
		/// The window that sent the [`ConvertSelection` request].
		///
		/// [`ConvertSelection` request]: super::request::ConvertSelection
		pub requester: Window,

		/// The selection to be converted.
		pub selection: Atom,
		/// The type that the `selection` should be converted into.
		pub target_type: Atom,
		// TODO: undocumented in core protocol, check ICCCM
		pub property: Option<Atom>,
		[_; ..],
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// A reply to the [`ConvertSelection` request].
	///
	/// If the selection has no owner, this is generated by the X server. If the
	/// selection does have an owner, that owner should generate this [event]
	/// using the [`SendEvent` request].
	///
	/// # Recipients
	/// This [event] is reported to the `requester` of a
	/// [`ConvertSelection` request].
	///
	/// [event]: crate::message::Event
	/// [`ConvertSelection` request]: super::request::ConvertSelection
	/// [`SendEvent` request]: super::request::SendEvent
	pub struct Selection: Event(31) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The time at which this `Selection` event was generated.
		pub time: CurrentableTime,

		/// The window that sent the [`ConvertSelection` request].
		///
		/// [`ConvertSelection` request]: super::request::ConvertSelection
		pub requester: Window,

		/// The selection which the [`ConvertSelection` request] applied to.
		///
		/// [`ConvertSelection` request]: super::request::ConvertSelection
		pub selection: Atom,
		/// The type that the `selection` was to be converted into.
		///
		/// The `selection` may or may not have been converted.
		pub target_type: Atom,
		// TODO: undocumented in core protocol, check ICCCM.
		pub property: Option<Atom>,
		[_; ..],
	}

	#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, X11Size, Readable, Writable)]
	/// The reason why a [`Colormap` event] was generated.
	///
	/// [`Colormap` event]: Colormap
	pub enum ColormapDetail {
		/// The `window`'s [`colormap` attribute] was changed.
		///
		/// [`colormap` attribute]: crate::Attributes::colormap
		AttributeChanged,
		/// The `window`'s [colormap] was installed or uninstalled.
		///
		/// [colormap]: crate::Colormap
		InstalledOrUninstalled,
	}

	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	/// Whether a [window]'s [colormap] is currently installed.
	///
	/// [window]: Window
	/// [colormap]: crate::Colormap
	pub enum ColormapState {
		/// The [window]'s [colormap] is not currently installed.
		///
		/// [window]: Window
		/// [colormap]: crate::Colormap
		Uninstalled,
		/// The [window]'s [colormap] is currently installed.
		///
		/// [window]: Window
		/// [colormap]: crate::Colormap
		Installed,
	}

	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [window]'s [colormap] is installed,
	/// uninstalled, or its [`colormap` attribute] is changed.
	///
	/// # Recipients
	/// This [event] is reported to clients selecting [`COLORMAP_CHANGE`] on the
	/// window.
	///
	/// [event]: crate::message::Event
	/// [window]: Window
	/// [colormap]: crate::Colormap
	/// [`colormap` attribute]: crate::Attributes::colormap
	///
	/// [`COLORMAP_CHANGE`]: crate::EventMask::COLORMAP_CHANGE
	pub struct Colormap: Event(32) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The window that this [event] relates to.
		///
		/// [event]: crate::message::Event
		pub window: Window,
		/// The `window`'s [colormap].
		///
		/// [colormap]: crate::Colormap
		pub colormap: Option<crate::Colormap>,

		/// Whether this [event] was generated because the `window`'s
		/// [`colormap` attribute] was changed or because the `window`'s
		/// `colormap` was installed or uninstalled.
		///
		/// [event]: crate::message::Event
		/// [`colormap` attribute]: crate::Attributes::colormap
		pub detail: ColormapDetail,
		/// Whether the `window`'s `colormap` is currently installed.
		pub state: ColormapState,
		[_; ..],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, X11Size, Readable, Writable)]
/// Used in the [`ClientMessage` event] to represent whether its `data` is 20
/// `i8` values, 10 `i16` values, or 5 `i32` values.
///
/// [`ClientMessage` event]: ClientMessage
pub enum ClientMessageFormat {
	/// 20 `i8` values: [`ClientMessageData::I8`].
	I8 = 8,
	/// 10 `i16` values: [`ClientMessageData::I16`].
	I16 = 16,
	/// 5 `i32` values: [`ClientMessageData::I32`].
	I32 = 32,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Writable)]
/// The `data` contained in a [`ClientMessage` event].
///
/// [`ClientMessage` event]: ClientMessage
#[no_discrim]
pub enum ClientMessageData {
	/// Data comprised of 20 `i8` values.
	I8([i8; 20]),
	/// Data comprised of 10 `i16` values.
	I16([i16; 10]),
	/// Data comprised of 5 `i32` values.
	I32([i32; 5]),
}

impl ConstantX11Size for ClientMessageData {
	const X11_SIZE: usize = 20;
}

impl X11Size for ClientMessageData {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl ReadableWithContext for ClientMessageData {
	type Context = ClientMessageFormat;

	fn read_with(buf: &mut impl Buf, format: &ClientMessageFormat) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(match format {
			ClientMessageFormat::I8 => Self::I8(<_>::read_from(buf)?),
			ClientMessageFormat::I16 => Self::I16(<_>::read_from(buf)?),
			ClientMessageFormat::I32 => Self::I32(<_>::read_from(buf)?),
		})
	}
}

derive_xrb! {
	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated by a [`SendEvent` request].
	///
	/// # Recipients
	/// This [event] is reported to the [`SendEvent` request]'s `destination`
	/// [window].
	///
	/// [event]: crate::message::Event
	/// [`SendEvent` request]: super::request::SendEvent
	/// [window]: Window
	pub struct ClientMessage: Event(33) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		#[metabyte]
		/// Whether `data` is `[i8; 20]`, `[i16; 10]`, or `[i32; 5]`.
		let format: ClientMessageFormat = data => match data {
			ClientMessageData::I8(_) => ClientMessageFormat::I8,
			ClientMessageData::I16(_) => ClientMessageFormat::I16,
			ClientMessageData::I32(_) => ClientMessageFormat::I32,
		},

		/// The recipient of this `ClientMessage` event.
		pub window: Window,
		/// How the `data` is to be interpreted by the `recipient`.
		pub r#type: Atom,

		#[context(format => *format)]
		/// The data contained in this [event].
		///
		/// [event]: crate::message::Event
		pub data: ClientMessageData,
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, X11Size, Readable, Writable)]
/// Detail about which [request] generated a [`MappingChange` event].
///
/// [request]: crate::message::Request
/// [`MappingChange` event]: MappingChange
pub enum MappingRequest {
	/// The [`MappingChange` event] was generated by a
	/// [`SetModifierMapping` request].
	///
	/// [`MappingChange` event]: MappingChange
	/// [`SetModifierMapping` request]: super::request::SetModifierMapping
	Modifier,

	/// The [`MappingChange` event] was generated by a
	/// [`ChangeKeyboardMapping` request].
	///
	/// [`MappingChange` event]: MappingChange
	/// [`ChangeKeyboardMapping` request]: super::request::ChangeKeyboardMapping
	Keyboard,

	/// The [`MappingChange` event] was generated by a
	/// [`SetCursorMapping` request].
	///
	/// [`MappingChange` event]: MappingChange
	/// [`ChangeKeyboardMapping` request]: super::request::ChangeKeyboardMapping
	Cursor,
}

derive_xrb! {
	#[derive(Debug, Derivative, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	/// An [event] generated when a [`SetModifierMapping`],
	/// [`ChangeKeyboardMapping`], or [`SetCursorMapping`] request is successful.
	///
	/// # Recipients
	/// This [event] is reported to all clients.
	///
	/// [event]: crate::message::Event
	/// [`SetModifierMapping`]: super::request::SetModifierMapping
	/// [`ChangeKeyboardMapping`]: super::request::ChangeKeyboardMapping
	/// [`SetCursorMapping`]: super::request::SetCursorMapping
	pub struct MappingChange: Event(34) {
		#[sequence]
		#[derivative(PartialEq = "ignore", Hash = "ignore")]
		/// The [sequence number] associated with the last [request] related
		/// to this [event] that was received before this [event] was generated.
		///
		/// [sequence number]: crate::message::Event::sequence
		/// [request]: crate::message::Request
		/// [event]: crate::message::Event
		pub sequence: u16,

		/// The [request] that generated this `MappingChange` event.
		///
		/// See [`MappingRequest`] for more information.
		///
		/// [request]: crate::message::Request
		pub request: MappingRequest,

		/// The first [keycode] in the range of altered keycodes, if this event
		/// was generated by a [`ChangeKeyboardMapping` request].
		///
		/// [keycode]: Keycode
		/// [`ChangeKeyboardMapping` request]: super::request::ChangeKeyboardMapping
		pub first_keycode: Keycode,
		/// The number of [keycodes] altered if this event was generated by a
		/// [`ChangeKeyboardMapping` request].
		///
		/// [keycodes]: Keycode
		/// [`ChangeKeyboardMapping` request]: super::request::ChangeKeyboardMapping
		pub count: u8,
		[_; ..],
	}
}
