// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Events] defined in the [core X11 protocol].
//!
//! [Events] are messages sent from the X server to an X client.
//!
//! [Events]: crate::Event
//! [core X11 protocol]: super

use crate::{
	mask::{ConfigureWindowMask, ModifierMask},
	Atom,
	Button,
	CurrentableTime,
	Drawable,
	GrabMode,
	Keycode,
	Point,
	Rectangle,
	Region,
	StackMode,
	Timestamp,
	Window,
};

use bitflags::bitflags;
use bytes::{Buf, BufMut};
use xrbk::{
	ConstantX11Size,
	ReadResult,
	Readable,
	ReadableWithContext,
	Writable,
	WriteResult,
	X11Size,
};

use xrbk_macro::{derive_xrb, ConstantX11Size, Readable, Writable, X11Size};
extern crate self as xrb;

derive_xrb! {
	/// An event generated when a key is pressed.
	///
	/// This event is generated for all keys: that includes modifier keys.
	pub struct KeyPress: Event(2) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// The keycode of the key which was pressed.
		pub keycode: Keycode,

		/// The time at which this event was generated.
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this event was generated.
		pub root: Window,
		/// The window which this event was generated in relation to.
		///
		/// This window is found by beginning with the window in which the
		/// cursor is located, then searching up the window hierarchy (starting
		/// with that window, then going to its parent, etc.) to find the first
		/// window which any client has selected interest in this event
		/// (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs or the currently focused window may modify how the
		/// `event_window` is chosen.
		pub event_window: Window,
		/// If a child of the `event_window` contains the cursor, this is that
		/// child.
		///
		/// Otherwise, this is [`None`].
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the time this event was
		/// generated, relative to the `root` window's origin.
		pub root_coords: Point,
		/// The coordinates of the cursor at the time this event was
		/// generated, relative to the `event_window`'s origin.
		pub event_coords: Point,

		/// The state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,

		/// Whether the cursor is on the same screen as the `event_window`.
		pub same_screen: bool,
		_,
	}

	/// An event generated when a key is released.
	///
	/// This event is generated for all keys: that includes modifier keys.
	pub struct KeyRelease: Event(3) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// The keycode of the key which was released.
		pub keycode: Keycode,

		/// The time at which this event was generated.
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was located
		/// within when this event was generated.
		pub root: Window,
		/// The window which this event was generated in relation to.
		///
		/// This window is found by beginning with the window in which the
		/// cursor is located, then searching up the window hierarchy (starting
		/// with that window, then going to its parent, etc.) to find the first
		/// window which any client has selected interest in this event
		/// (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs or the currently focused window may modify how the
		/// `event_window` is chosen.
		pub event_window: Window,
		/// If a child of the `event_window` contains the cursor, this is that
		/// child.
		///
		/// Otherwise, this is [`None`].
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the time this event was
		/// generated, relative to the `root` window's origin.
		pub root_coords: Point,
		/// The coordinates of the cursor at the time this event was
		/// generated, relative to the `event_window`'s origin.
		pub event_coords: Point,

		/// The state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,

		/// Whether the cursor is on the same screen as the `event_window`.
		pub same_screen: bool,
		_,
	}

	/// An event generated when a mouse button is pressed.
	pub struct ButtonPress: Event(4) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// The mouse button which was pressed.
		pub button: Button,

		/// The time at which this event was generated.
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this event was generated.
		pub root: Window,
		/// The window which this event was generated in relation to.
		///
		/// This window is found by beginning with the window in which the
		/// cursor is located, then searching up the window hierarchy (starting
		/// with that window, then going to its parent, etc.) to find the first
		/// window which any client has selected interest in this event
		/// (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs may modify how the `event_window` is chosen.
		pub event_window: Window,
		/// If a child of the `event_window` contains the cursor, this is that
		/// child.
		///
		/// Otherwise, this is [`None`].
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `root` window's origin.
		pub root_coords: Point,
		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `event_window`'s origin.
		pub event_coords: Point,

		/// The state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,

		/// Whether the cursor is on the same screen as the `event_window`.
		pub same_screen: bool,
		_,
	}

	/// An event generated when a mouse button is released.
	pub struct ButtonRelease: Event(5) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// The mouse button which was released.
		pub button: Button,

		/// The time at which this event was generated.
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this event was generated.
		pub root: Window,
		/// The window which this event was generated in relation to.
		///
		/// This window is found by beginning with the window in which the
		/// cursor is located, then searching up the window hierarchy (starting
		/// with that window, then going to its parent, etc.) to find the first
		/// window which any client has selected interest in this event
		/// (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs may modify how the `event_window` is chosen.
		pub event_window: Window,
		/// If a child of the `event_window` contains the cursor, this is that
		/// child.
		///
		/// Otherwise, this is [`None`].
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `root` window's origin.
		pub root_coords: Point,
		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `event_window`'s origin.
		pub event_coords: Point,

		/// The state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,

		/// Whether the cursor is on the same screen as the `event_window`.
		pub same_screen: bool,
		_,
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
/// The type of [`Motion`] event sent.
pub enum MotionNotificationType {
	/// The [`Motion`] event was not one generated for a client selecting
	/// [`MOTION_HINT`].
	///
	/// [`MOTION_HINT`]: crate::mask::EventMask::MOTION_HINT
	Normal,

	/// The [`Motion`] event was generated for a client selecting
	/// [`MOTION_HINT`].
	///
	/// The X server is free to send only one [`Motion`] event to the client
	/// until:
	/// - a mouse button or key is pressed or released; or
	/// - the pointer leaves the `event_window`; or
	/// - the client sends a [`QueryCursor`] or [`GetMotionEvents`] request.
	///
	/// [`MOTION_HINT`]: crate::mask::EventMask::MOTION_HINT
	///
	/// [`QueryCursor`]: super::request::QueryCursor
	/// [`GetMotionEvents`]: super::request::GetMotionEvents
	Hint,
}

derive_xrb! {
	/// An event generated when the cursor moves within a [window].
	///
	/// Motion events are only generated when the cursor motion begins and ends
	/// in the same window. If the cursor leaves the window, a [`LeaveWindow`] event
	/// will be generated instead, accompanied by an [`EnterWindow`] event for the
	/// window which it moves into.
	///
	/// Selecting for [`ANY_MOTION`] events means `Motion` events will be received
	/// independently of the currently pressed mouse buttons. Selecting for
	/// button motion events ([`BUTTON_1_MOTION`]..[`BUTTON_5_MOTION`] and
	/// [`ANY_BUTTON_MOTION`]), however, means `Motion` events will only be
	/// received while at least one of the selected mouse buttons is pressed.
	///
	/// If [`MOTION_HINT`] is selected, the server is free to send only one
	/// `Motion` event with a [`MotionNotificationType`] of [`Hint`] until:
	/// - a mouse button or key is pressed or released; or
	/// - the pointer leaves the `event_window`; or
	/// - the client sends a [`QueryCursor`] or [`GetMotionEvents`] request.
	///
	/// [`ANY_MOTION`]: crate::mask::EventMask::ANY_MOTION
	/// [`BUTTON_1_MOTION`]: crate::mask::EventMask::BUTTON_1_MOTION
	/// [`BUTTON_5_MOTION`]: crate::mask::EventMask::BUTTON_5_MOTION
	/// [`ANY_BUTTON_MOTION`]: crate::mask::EventMask::ANY_BUTTON_MOTION
	/// [`MOTION_HINT`]: crate::mask::EventMask::MOTION_HINT
	///
	/// [`Hint`]: MotionNotificationType::Hint
	/// [`QueryCursor`]: super::request::QueryCursor
	/// [`GetMotionEvents`]: super::request::GetMotionEvents
	///
	/// [window]: Window
	pub struct Motion: Event(6) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// The type of `Motion` event sent.
		pub notification_type: MotionNotificationType,

		/// The time at which this event was generated.
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this event was generated.
		pub root: Window,
		/// The window which this event was generated in relation to.
		///
		/// This window is found by beginning with the window in which the
		/// cursor is located, then searching up the window hierarchy (starting
		/// with that window, then going to its parent, etc.) to find the first
		/// window which any client has selected interest in this event
		/// (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs may modify how the `event_window` is chosen.
		pub event_window: Window,
		/// If a child of the `event_window` contains the cursor, this is that
		/// child.
		///
		/// Otherwise, this is [`None`].
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `root` window's origin.
		pub root_coords: Point,
		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `event_window`'s origin.
		pub event_coords: Point,

		/// The state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,

		/// Whether the cursor is on the same screen as the `event_window`.
		pub same_screen: bool,
		_,
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
/// Detail that describes how a [window] receiving a [`LeaveWindow`] or
/// [`EnterWindow`] event relates to the event which took place.
///
/// If the cursor moves from window A to window B and A is a descendent of B:
/// - A [`LeaveWindow`] event is generated on A with a detail of [`Ancestor`].
/// - A [`LeaveWindow`] event is generated on each window between A and B
///   exclusive (in that order) with a detail of [`Intermediate`].
/// - An [`EnterWindow`] event is generated on B with a detail of
///   [`Descendent`].
///
/// If the cursor moves from window A to window B and A is an ancestor of B:
/// - A [`LeaveWindow`] event is generated on A with a detail of [`Descendent`].
/// - An [`EnterWindow`] event is generated on each window between A and B
///   exclusive (in that order) with a detail of [`Intermediate`]
/// - An [`EnterWindow`] event is generated on B with a detail of [`Ancestor`].
///
/// If the cursor moves from window A to window B and window C is their least
/// common ancestor:
/// - A [`LeaveWindow`] event is generated on A with a detail of [`Nonlinear`].
/// - A [`LeaveWindow`] event is generated on each window between A and C
///   exclusive (in that order) with a detail of [`NonlinearIntermediate`].
/// - An [`EnterWindow`] event is generated on each window between C and B
///   exclusive (in that order) with a detail of [`NonlinearIntermediate`].
/// - An [`EnterWindow`] event is generated on B with a detail of [`Nonlinear`].
///
/// If the cursor moves from window A to window B and A and B are on different
/// screens:
/// - A [`LeaveWindow`] event is generated on A with a detail of [`Nonlinear`].
/// - If A is not a root window, a [`LeaveWindow`] event is generated on each
///   ancestor of A including its root, in order from A's parent to its root,
///   with a detail of [`NonlinearIntermediate`].
/// - If B is not a root window, an [`EnterWindow`] event is generated on each
///   ancestor of B including its root, in order from B's root to B's parent,
///   with a detail of [`NonlinearIntermediate`].
/// - An [`EnterWindow`] event is generated on B with a detail of [`Nonlinear`].
///
/// [`Ancestor`]: EnterLeaveDetail::Ancestor
/// [`Intermediate`]: EnterLeaveDetail::Intermediate
/// [`Descendent`]: EnterLeaveDetail::Descendant
///
/// [`Nonlinear`]: EnterLeaveDetail::Nonlinear
/// [`NonlinearIntermediate`]: EnterLeaveDetail::NonlinearIntermediate
///
/// [window]: Window
pub enum EnterLeaveDetail {
	/// Used for [`LeaveWindow`] events when the cursor leaves a window and
	/// enters an ancestor of that window, and for [`EnterWindow`] events
	/// when the cursor enters a window and leaves an ancestor of that window.
	Ancestor,
	/// Used in [`LeaveWindow`] and [`EnterWindow`] events for all windows
	/// between the newly entered window and the previous window if one is a
	/// descendent of the other.
	Intermediate,
	/// Used for [`LeaveWindow`] events when the cursor leaves a window and
	/// enters a descendent of that window, and for [`EnterWindow`] events
	/// when the cursor enters a window and leaves a descendent of that window.
	Descendant,

	/// Used for [`LeaveWindow`] and [`EnterWindow`] events for the newly
	/// entered window and the previous window if neither is a descendent of the
	/// other.
	Nonlinear,
	/// Used for [`LeaveWindow`] and [`EnterWindow`] events when neither the
	/// window that was left nor the window that was entered are a descendent of
	/// the other.
	///
	/// This is the detail for the [`LeaveWindow`] events generated for all the
	/// windows between the window that was left and the least common ancestor
	/// of that window and the window that was entered (exclusive).
	///
	/// This is the detail for the [`EnterWindow`] events generated for all the
	/// windows between the window that was entered and the least common
	/// ancestor of that window and the window that was left (exclusive).
	NonlinearIntermediate,
}

bitflags! {
	#[derive(Default, X11Size, Readable, ConstantX11Size, Writable)]
	/// A bitmask used in the [`EnterWindow`] and [`LeaveWindow`] events.
	pub struct EnterLeaveMask: u8 {
		/// Whether the `event_window` is the focused window or a descendant
		/// of the focused window.
		const FOCUS = 0x01;
		/// Whether the cursor is on the same screen as the `event_window`.
		const SAME_SCREEN = 0x02;
	}
}

derive_xrb! {
	/// An event generated when the cursor enters a [window].
	///
	/// This event is triggered both when the cursor moves to be in a different
	/// window than it was before, as well as when the window under the cursor
	/// changes due to a change in the window hierarchy (i.e. [`Unmap`],
	/// [`Map`], [`Configure`], [`Gravity`],
	/// [`Circulate`]).
	///
	/// This event is received only by clients selecting [`ENTER_WINDOW`] on a
	/// window.
	///
	/// `EnterWindow` events caused by a hierarchy change are generated after
	/// that hierarchy change event (see above), but there is no restriction
	/// as to whether `EnterWindow` events should be generated before or
	/// after [`Unfocus`], [`Visibility`], or [`Expose`] events.
	///
	/// [window]: Window
	/// [`ENTER_WINDOW`]: crate::mask::EventMask::ENTER_WINDOW
	pub struct EnterWindow: Event(7) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// Detail about how the event was generated.
		///
		/// See [`EnterLeaveDetail`] for more information.
		pub detail: EnterLeaveDetail,

		/// The time at which this event was generated.
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this event was generated.
		pub root: Window,
		/// The window which this event was generated in relation to.
		///
		/// This window is found by beginning with the window in which the
		/// cursor is located, then searching up the window hierarchy (starting
		/// with that window, then going to its parent, etc.) to find the first
		/// window which any client has selected interest in this event
		/// (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs may modify how the `event_window` is chosen.
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
		pub root_coords: Point,
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
		pub event_coords: Point,

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

	/// An event generated when the cursor leaves a [window].
	///
	/// This event is triggered both when the cursor moves to be in a different
	/// window than it was before, as well as when the window under the cursor
	/// changes due to a change in the window hierarchy (i.e. [`Unmap`],
	/// [`Map`], [`Configure`], [`Gravity`],
	/// [`Circulate`]).
	///
	/// This event is received only by clients selecting [`LEAVE_WINDOW`] on a
	/// window.
	///
	/// `LeaveWindow` events caused by a hierarchy change are generated after
	/// that hierarchy change event (see above), but there is no restriction
	/// as to whether `LeaveWindow` events should be generated before or
	/// after [`Unfocus`], [`Visibility`], or [`Expose`] events.
	///
	/// [window]: Window
	/// [`LEAVE_WINDOW`]: crate::mask::EventMask::LEAVE_WINDOW
	pub struct LeaveWindow: Event(8) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// Detail about how the event was generated.
		///
		/// See [`EnterLeaveDetail`] for more information.
		pub detail: EnterLeaveDetail,

		/// The time at which this event was generated.
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this event was generated.
		pub root: Window,
		/// The window which this event was generated in relation to.
		///
		/// This window is found by beginning with the window in which the
		/// cursor is located, then searching up the window hierarchy (starting
		/// with that window, then going to its parent, etc.) to find the first
		/// window which any client has selected interest in this event
		/// (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs may modify how the `event_window` is chosen.
		pub event_window: Window,
		/// If a child of the `event_window` contains the initial cursor position
		/// (`event_coords`), this is that child.
		///
		/// Otherwise, this is [`None`].
		pub child_window: Option<Window>,

		/// The position of the cursor at the time this event was generated,
		/// relative to the `root` window's origin.
		///
		/// This is always the final position of the cursor, not its initial
		/// position.
		pub root_coords: Point,
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
		pub event_coords: Point,

		/// The state of mouse buttons and modifier keys immediately
		/// before this event was generated.
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
/// Detail describing how a [window] that receives a [`Focus`] or [`Unfocus`]
/// event relates to the event that occurred.
///
/// If the focus moves from window A to window B, A is a descendent of B, and
/// the cursor is in window C:
/// - An [`Unfocus`] event is generated on A with a detail of [`Ancestor`].
/// - An [`Unfocus`] event is generated on each window between A and B exclusive
///   (in that order) with a detail of [`Intermediate`].
/// - A [`Focus`] event is generated on B with a detail of [`Descendent`].
/// - If C is a descendent of B but C is not A, nor a descendent of A, nor an
///   ancestor of A, a [`Focus`] event is generated on each descendent of B down
///   to and including C (in that order) with a detail of [`Cursor`].
///
/// If the focus moves from window A to window B, A is an ancestor of B, and the
/// cursor is in window C:
/// - If C is a descendent of A but C is not a descendent of B nor an ancestor
///   of B, an [`Unfocus`] event is generated on C and each ancestor of C up to
///   but not including A (in that order) with a detail of [`Cursor`].
/// - An [`Unfocus`] event is generated on A with a detail of [`Descendent`].
/// - A [`Focus`] event is generated on  each window between A and B exclusive
///   (in that order) with a detail of [`Intermediate`].
/// - A [`Focus`] event is generated on B with a detail of [`Ancestor`].
///
/// If the focus moves from with A to window B, the cursor is in window C, and
/// window D is their least common ancestor:
/// - If C is a descendent of A, an [`Unfocus`] event is generated on C and each
///   ancestor of C up to and including A (in that order) with a detail of
///   [`Cursor`].
/// - An [`Unfocus`] event is generated on A with a detail of [`Nonlinear`].
/// - An [`Unfocus`] event is generated on each window between A and D exclusive
///   (in that order) with a detail of [`NonlinearIntermediate`].
/// - A [`Focus`] event is generated on each window between D and B exclusive
///   (in that order) with a detail of [`NonlinearIntermediate`].
/// - A [`Focus`] event is generated on B with a detail of [`Nonlinear`].
/// - If C is a descendent of B, a [`Focus`] event is generated on each
///   descendent of B down to and including C (in that order) with a detail of
///   [`Cursor`].
///
/// If the focus moves from window A to window B, A and B are on different
/// screens, and the cursor is in window C:
/// - If C is a descendent of A, an [`Unfocus`] event is generated on C and each
///   ancestor of C up to but not including A (in that order) with a detail of
///   [`Cursor`].
/// - An [`Unfocus`] event is generated on A with a detail of [`Nonlinear`].
/// - If A is not a root window, an [`Unfocus`] event is generated on each
///   ancestor of A up to and including its root (in that order) with a detail
///   of [`NonlinearIntermediate`].
/// - If B is not a root window, a [`Focus`] event is generated on each ancestor
///   of B, starting with B's root and ending with B's parent, with a detail of
///   [`NonlinearIntermediate`].
/// - A [`Focus`] event is generated on B with a detail of [`Nonlinear`].
/// - If C is a descendent of B, a [`Focus`] event is generated on each
///   descendent of B down to and including C (in that order) with a detail of
///   [`Cursor`].
///
/// If the focus moves from window A to [`CursorRoot`] or [`None`] and the
/// cursor is in window C:
/// - If C is a descendent of A, an [`Unfocus`] event is generated on C and each
///   ancestor of C up to but not including A (in that order) with a detail of
///   [`Cursor`].
/// - An [`Unfocus`] event is generated on A with a detail of [`Nonlinear`].
/// - If A is not a root window, an [`Unfocus`] event is generated on each
///   ancestor of A up to and including its root (in that order) with a detail
///   of [`NonlinearIntermediate`].
/// - A [`Focus`] event is generated on all root windows with a detail of
///   [`CursorRoot`] or [`None`] respectively.
/// - If the new focus is [`CursorRoot`], a [`Focus`] event is generated on C
///   and each ancestor of C, starting with C's root and ending with C, with a
///   detail of [`Cursor`].
///
/// If the focus moves from [`CursorRoot`] or [`None`] to window A and the
/// cursor is in window C:
/// - If the old focus is [`CursorRoot`], an [`Unfocus`] event is generated on C
///   and each ancestor of C up to and including C's root (in that order) with a
///   detail of [`Cursor`].
/// - An [`Unfocus`] event is generated on all root windows with a detail of
///   [`CursorRoot`] or [`None`] respectively.
/// - If A is not a root window, a [`Focus`] event is generated on each ancestor
///   of A, starting with A's root and ending with A's parent, with a detail of
///   [`NonlinearIntermediate`].
/// - A [`Focus`] event is generated on A with a detail of [`Nonlinear`].
/// - If C is a descendent of A, a [`Focus`] event is generated on each
///   descendent of A down to and including C (in that order) with a detail of
///   [`Cursor`].
///
/// If the focus moves from [`CursorRoot`] to [`None`] and the cursor is in
/// window C:
/// - An [`Unfocus`] event is generated on C and each ancestor of C up to and
///   including C's root (in that order) with a detail of [`Cursor`].
/// - An [`Unfocus`] event is generated on all root windows with a detail of
///   [`CursorRoot`].
/// - A [`Focus`] event is generated on all root windows with a detail of
///   [`None`].
///
/// If the focus moves from [`None`] to [`CursorRoot`] and the cursor is in
/// window C:
/// - An [`Unfocus`] event is generated on all root windows with a detail of
///   [`None`].
/// - A [`Focus`] event is generated on all root windows with a detail of
///   [`CursorRoot`].
/// - A [`Focus`] event is generated on C and each ancestor of C, starting with
///   C's root and ending with C, with a detail of [`Cursor`].
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
/// [window]: Window
pub enum FocusDetail {
	/// Used for [`Unfocus`] events for the window which has been unfocused if
	/// the newly focused window is an ancestor of that window, and for
	/// [`Focus`] events for the window which has been focused if the newly
	/// unfocused window is an ancestor of that window.
	Ancestor,
	/// Used for [`Unfocus`] and [`Focus`] events for each window between the
	/// window that was unfocused and the window that was focused if one is a
	/// descendent of the other.
	Intermediate,
	/// Used for [`Unfocus`] events for the window which has been unfocused if
	/// the newly focused window is a descendent of that window, and for
	/// [`Focus`] events for the window which has been focused if the newly
	/// unfocused window is a descendent of that window.
	Descendent,

	/// Used for [`Unfocus`] and [`Focus`] events for both the window that was
	/// unfocused and the window that was focused if neither window is a
	/// descendent of the other.
	Nonlinear,
	/// Used for [`Unfocus`] and [`Focus`] events for each window between the
	/// unfocused and focused windows' least common ancestor and the unfocused
	/// window and focused window respectively.
	NonlinearIntermediate,

	Cursor,

	CursorRoot,
	None,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
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
	/// An event generated when a window is focused.
	///
	/// `Focus` events are reported to clients selecting [`FOCUS_CHANGE`] on the
	/// window that was focused.
	///
	/// `Focus` events generated when the keyboard is not grabbed have
	/// [`FocusGrabMode::Normal`], `Focus` events generated when the keyboard
	/// is grabbed have [`FocusGrabMode::WhileGrabbed`], `Focus` events
	/// generated by a keyboard grab activating have [`FocusGrabMode::Grab`],
	/// and `Focus` events generated by a keyboard grab deactivating have
	/// [`FocusGrabMode::Ungrab`].
	///
	/// [`FOCUS_CHANGE`]: crate::mask::EventMask::FOCUS_CHANGE
	pub struct Focus: Event(9) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// Detail about how the event was generated.
		///
		/// See [`FocusDetail`] for more information.
		pub detail: FocusDetail,

		/// The window which was focused.
		pub window: Window,

		/// How this event was generated in relation to grabs.
		///
		/// [`Normal`] for normal `Focus` events, [`Grab`] and [`Ungrab`] for
		/// events generated by grabs and ungrabs, [`WhileGrabbed`] for events
		/// generated by a [`SetInputFocus`] request while the keyboard is
		/// grabbed.
		///
		/// [`Normal`]: FocusGrabMode::Normal
		/// [`Grab`]: FocusGrabMode::Grab
		/// [`Ungrab`]: FocusGrabMode::Ungrab
		/// [`WhileGrabbed`]: FocusGrabMode::WhileGrabbed
		///
		/// [`SetInputFocus`]: super::request::SetInputFocus
		pub grab_mode: FocusGrabMode,
		[_; ..],
	}

	/// An event generated when a window is unfocused.
	///
	/// `Unfocus` events are reported to clients selecting [`FOCUS_CHANGE`] on the
	/// window that was unfocused.
	///
	/// `Unfocus` events generated when the keyboard is not grabbed have
	/// [`FocusGrabMode::Normal`], `Unfocus` events generated when the keyboard
	/// is grabbed have [`FocusGrabMode::WhileGrabbed`], `Unfocus` events
	/// generated by a keyboard grab activating have [`FocusGrabMode::Grab`],
	/// and `Unfocus` events generated by a keyboard grab deactivating have
	/// [`FocusGrabMode::Ungrab`].
	///
	/// [`FOCUS_CHANGE`]: crate::mask::EventMask::FOCUS_CHANGE
	pub struct Unfocus: Event(10) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// Detail about how the event was generated.
		///
		/// See [`FocusDetail`] for more information.
		pub detail: FocusDetail,

		/// The window which was unfocused.
		pub window: Window,

		/// How this event was generated in relation to grabs.
		///
		/// [`Normal`] for normal `Focus` events, [`Grab`] and [`Ungrab`] for
		/// events generated by grabs and ungrabs, [`WhileGrabbed`] for events
		/// generated by a [`SetInputFocus`] request while the keyboard is
		/// grabbed.
		///
		/// [`Normal`]: FocusGrabMode::Normal
		/// [`Grab`]: FocusGrabMode::Grab
		/// [`Ungrab`]: FocusGrabMode::Ungrab
		/// [`WhileGrabbed`]: FocusGrabMode::WhileGrabbed
		///
		/// [`SetInputFocus`]: super::request::SetInputFocus
		pub grab_mode: FocusGrabMode,
		[_; ..],
	}

	/// An event describing the current state of the keyboard.
	///
	/// This event is reported to clients selecting [`KEYS_STATE`] on a [window]
	/// immediately after every [`EnterWindow`] and [`Focus`] event.
	///
	/// [window]: Window
	/// [`KEYS_STATE`]: crate::mask::EventMask::KEYS_STATE
	pub struct KeysState: Event(11) {
		/// A bit vector representing the current keyboard state.
		///
		/// Each bit set to 1 indicates that the corresponding key is currently
		/// pressed. Byte `N` (starting at 1) contains the bits for keycodes `8N`
		/// to `8N + 7`, with the least significant bit in the byte representing
		/// key `8N`.
		pub keys: [u8; 31],
	}

	/// An event generated when a rectangular area of a [window] needs to be
	/// rendered.
	///
	/// This event is reported to clients selecting [`EXPOSURE`] on a [window].
	///
	/// This event is generated when no valid contents are available for regions
	/// of a window, and either:
	/// - the regions are visible; or
	/// - the regions are viewable and the server is maintaining a backing store
	///   on the window; or
	/// - the window is not viewable but the server is honoring the window's
	///   [`BackingStore` attribute] of [`Always`] or [`WhenMapped`].
	///
	/// The regions are decomposed into an arbitrary set of rectangles, and an
	/// `Expose` event is generated for each one.
	///
	/// `Expose` events are never generated on [`WindowClass::InputOnly`]
	/// windows.
	///
	/// [window]: Window
	///
	/// [`BackingStore` attribute]: crate::Attribute::BackingStore
	/// [`Always`]: crate::BackingStores::Always
	/// [`WhenMapped`]: crate::BackingStores::WhenMapped
	/// [`WindowClass::InputOnly`]: crate::WindowClass::InputOnly
	///
	/// [`EXPOSURE`]: crate::mask::EventMask::EXPOSURE
	pub struct Expose: Event(12) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The window which this `Expose` event applies to.
		pub window: Window,
		/// The region of the `window` which this `Expose` event applies to.
		pub region: Region,

		/// The minimum number of `Expose` events that follow for this [window].
		///
		/// A `count` of `0` is guaranteed to mean no more `Expose` events for
		/// this window follow.
		///
		/// [window]: Window
		pub count: u16,
		[_; ..],
	}

	/// An event generated when using graphics operations when a region of a
	/// source [`Drawable`] is obscured.
	///
	/// This event is reported to a client using a [`GraphicsContext`] with
	/// [`GRAPHICS_EXPOSURE`] selected.
	///
	/// This event is generated when a region of the `destination` could not be
	/// computed because part of the `source` was obscured or out of bounds.
	///
	/// [`GraphicsContext`]: crate::GraphicsContext
	/// [`GRAPHICS_EXPOSURE`]: crate::mask::GraphicsContextMask::GRAPHICS_EXPOSURE
	pub struct GraphicsExposure: Event(13) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The [`Drawable`] this `GraphicsExposure` event applies to.
		pub drawable: Drawable,
		/// The obscured or out-of-bounds source region.
		pub region: Region,

		/// The minor opcode identifying the graphics request used.
		///
		/// For the core protocol, this is always zero.
		pub minor_opcode: u16,

		/// The minimum number of `GraphicsExposure` events that follow for this
		/// [`Drawable`].
		///
		/// A `count` of `0` is guaranteed to mean no more `GraphicsExposure`
		/// events for this [`Drawable`] follow.
		pub count: u16,

		/// The major opcode identifying the graphics request used.
		///
		/// For the core protocol, this always refers to [`CopyArea`] or
		/// [`CopyPlane`].
		///
		/// [`CopyArea`]: super::request::CopyArea
		/// [`CopyPlane`]: super::request::CopyPlane
		pub major_opcode: u8,
		[_; ..],
	}

	/// An event generated when a graphics request which might generate
	/// [`GraphicsExposure`] events doesn't generate any.
	///
	/// This event is reported to a client using a [`GraphicsContext`] with
	/// [`GRAPHICS_EXPOSURE`] selected.
	///
	/// [`GraphicsContext`]: crate::GraphicsContext
	/// [`GRAPHICS_EXPOSURE`]: crate::mask::GraphicsContextMask::GRAPHICS_EXPOSURE
	pub struct NoExposure: Event(14) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The [`Drawable`] this `NoExposure` event applies to.
		///
		/// This is the `destination` of the graphics request.
		pub drawable: Drawable,

		/// The minor opcode identifying the graphics request used.
		///
		/// For the core protocol, this is always zero.
		pub minor_opcode: u16,
		/// The major opcode identifying the graphics request used.
		///
		/// For the core protocol, this always refers to [`CopyArea`] or
		/// [`CopyPlane`].
		///
		/// [`CopyArea`]: super::request::CopyArea
		/// [`CopyPlane`]: super::request::CopyPlane
		pub major_opcode: u8,
		[_; ..],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
/// The state of a [window]'s visibility.
///
/// This is used in the [`Visibility`] event.
///
/// [window]: Window
pub enum VisibilityState {
	/// There is nothing obscuring the window.
	///
	/// This is used in the [`Visibility`] event when a window changes state to
	/// be `Unobscured`.
	Unobscured,
	/// The window is partially, but not fully, obscured.
	///
	/// This is used in the [`Visibility`] event when a window changes state to
	/// be `PartiallyObscured`.
	PartiallyObscured,
	/// The window is fully obscured.
	///
	/// This is used in the [`Visibility`] event when a window changes state to
	/// be `FullyObscured`.
	FullyObscured,
}

derive_xrb! {
	/// An event generated when changes to a [window]'s visibility occur.
	///
	/// This event is reported to clients selecting [`VISIBILITY_CHANGE`] on
	/// the window.
	///
	/// The window's visibility is calculated ignoring all of its subwindows.
	///
	/// When a window changes state from not viewable, [`PartiallyObscured`],
	/// or [`FullyObscured`] to viewable and [`Unobscured`], a `Visibility`
	/// event with [`VisibilityState::Unobscured`] is generated.
	///
	/// When a window changes state from viewable and [`Unobscured`], viewable
	/// and [`Obscured`], or not viewable, to viewable and
	/// [`PartiallyObscured`], a `Visibility` event with
	/// [`VisibilityState::PartiallyObscured`] is generated.
	///
	/// When a window changes state from viewable and [`Unobscured`], viewable
	/// and [`PartiallyObscured`], or not viewable to viewable and
	/// [`FullyObscured`], a `Visibility` event with
	/// [`VisibilityState::FullyObscured`] is generated.
	///
	/// [window]: Window
	///
	/// [`Unobscured`]: VisibilityState::Unobscured
	/// [`PartiallyObscured`]: VisibilityState::PartiallyObscured
	/// [`FullyObscured`]: VisibilityState::FullyObscured
	///
	/// [`VISIBILITY_CHANGE`]: crate::mask::EventMask::VISIBILITY_CHANGE
	pub struct Visibility: Event(15) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The window this `Visibility` event applies to.
		pub window: Window,
		/// The new [`VisibilityState`] of the window.
		pub visibility: VisibilityState,
		[_; ..],
	}

	/// An event generated when a [window] is created.
	///
	/// This event is reported to clients selecting [`SUBSTRUCTURE_NOTIFY`] on
	/// the window's parent.
	///
	/// [window]: Window
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::mask::EventMask::SUBSTRUCTURE_NOTIFY
	pub struct Create: Event(16) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
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
		pub border_width: u16,

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
		/// [`SUBSTRUCTURE_REDIRECT`]: crate::mask::EventMask::SUBSTRUCTURE_REDIRECT
		pub override_redirect: bool,
		[_; ..],
	}

	/// An event generated when a [window] is destroyed.
	///
	/// This event is reported to clients selecting [`STRUCTURE_NOTIFY`] on the
	/// window, as well as to clients selecting [`SUBSTRUCTURE_NOTIFY`] on its
	/// parent.
	///
	/// [window]: Window
	/// [`STRUCTURE_NOTIFY`]: crate::mask::EventMask::STRUCTURE_NOTIFY
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::mask::EventMask::SUBSTRUCTURE_NOTIFY
	pub struct Destroy: Event(17) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The window on which this `WindowDestroyed` was generated.
		///
		/// For clients selecting [`STRUCTURE_NOTIFY`] on the window that was
		/// destroyed, this is that window. For clients selecting
		/// [`SUBSTRUCTURE_NOTIFY`] on the window's parent, this is that parent.
		///
		/// [`STRUCTURE_NOTIFY`]: crate::mask::EventMask::STRUCTURE_NOTIFY
		/// [`SUBSTRUCTURE_NOTIFY`]: crate::mask::EventMask::SUBSTRUCTURE_NOTIFY
		pub event_window: Window,
		/// The window which was destroyed.
		pub window: Window,
		[_; ..],
	}

	/// An event generated when a [window] is unmapped.
	///
	/// This event is reported to clients selecting [`STRUCTURE_NOTIFY`] on the
	/// window, and to clients selecting [`SUBSTRUCTURE_NOTIFY`] on its parent.
	///
	/// Unmapping a window is the X term for hiding it. This is commonly used to
	/// minimize a window, for example.
	///
	/// [window]: Window
	/// [`STRUCTURE_NOTIFY`]: crate::mask::EventMask::STRUCTURE_NOTIFY
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::mask::EventMask::SUBSTRUCTURE_NOTIFY
	pub struct Unmap: Event(18) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The window on which this `Unmap` event was generated.
		///
		/// For clients selecting [`STRUCTURE_NOTIFY`] on the window that was
		/// unmapped, this is that window. For clients selecting
		/// [`SUBSTRUCTURE_NOTIFY`] on the window's parent, this is that parent.
		///
		/// [`STRUCTURE_NOTIFY`]: crate::mask::EventMask::STRUCTURE_NOTIFY
		/// [`SUBSTRUCTURE_NOTIFY`]: crate::mask::EventMask::SUBSTRUCTURE_NOTIFY
		pub event_window: Window,
		/// The window that was unmapped.
		pub window: Window,

		/// Whether this event was generated as a result of its parent being
		/// resized when the unmapped window had [`WinGravity::Unmap`].
		///
		/// [`WinGravity::Unmap`]: crate::WinGravity::Unmap
		pub from_configure: bool,
		[_; ..],
	}

	/// An event generated when a [window] is mapped.
	///
	/// This event is reported to clients selecting [`STRUCTURE_NOTIFY`] on the
	/// window and to clients selecting [`SUBSTRUCTURE_NOTIFY`] on the parent.
	///
	/// Mapping a window is the X term for showing it. It is the reverse of
	/// 'minimizing' the window.
	///
	/// [window]: Window
	/// [`STRUCTURE_NOTIFY`]: crate::mask::EventMask::STRUCTURE_NOTIFY
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::mask::EventMask::SUBSTRUCTURE_NOTIFY
	pub struct Map: Event(19) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The window on which this `Map` event was generated.
		///
		/// For clients selecting [`STRUCTURE_NOTIFY`] on the window that was
		/// mapped, this is that window. For clients selecting
		/// [`SUBSTRUCTURE_NOTIFY`] on the window's parent, this is that parent.
		///
		/// [`STRUCTURE_NOTIFY`]: crate::mask::EventMask::STRUCTURE_NOTIFY
		/// [`SUBSTRUCTURE_NOTIFY`]: crate::mask::EventMask::SUBSTRUCTURE_NOTIFY
		pub event_window: Window,
		/// The window that was mapped.
		pub window: Window,

		/// Whether [`MapWindow`] and [`ConfigureWindow`] requests on the
		/// `window` should override a [`SUBSTRUCTURE_REDIRECT`] on the
		/// window's parent.
		///
		/// This is typically set to inform the window manager not to tamper
		/// with the window.
		///
		/// [`MapWindow`]: super::request::MapWindow
		/// [`ConfigureWindow`]: super::request::ConfigureWindow
		///
		/// [`SUBSTRUCTURE_REDIRECT`]: crate::mask::EventMask::SUBSTRUCTURE_REDIRECT
		pub override_redirect: bool,
		[_; ..],
	}

	/// An event generated when an unmapped window with an
	/// [`OverrideRedirect` attribute] of `false` sends a [`MapWindow` request].
	///
	/// This event is reported to clients selecting [`SUBSTRUCTURE_REDIRECT`]
	/// on the window's parent. The window would not actually be mapped unless
	/// the client selecting [`SUBSTRUCTURE_REDIRECT`] sends its own
	/// [`MapWindow` request] for the window.
	///
	/// [`OverrideRedirect` attribute]: crate::WinAttribute::OverrideRedirect
	/// [`MapWindow` request]: super::request::MapWindow
	///
	/// [`SUBSTRUCTURE_REDIRECT`]: crate::mask::EventMask::SUBSTRUCTURE_REDIRECT
	pub struct MapRequest: Event(20) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The `window`'s parent.
		pub parent: Window,
		/// The window that sent the [`MapWindow` request].
		///
		/// [`MapWindow` request]: super::request::MapWindow
		pub window: Window,
		[_; ..],
	}

	/// An event generated when a [window] is reparented.
	///
	/// This event is reported to client selecting [`SUBSTRUCTURE_NOTIFY`] on
	/// either the old parent or the `new_parent`, and to clients selecting
	/// [`STRUCTURE_NOTIFY`] on the `window` itself.
	///
	/// Reparenting a window means to remove it from its current position in
	/// the window hierarchy and place it as the child of a `new_parent` window.
	///
	/// [window]: Window
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::mask::EventMask::SUBSTRUCTURE_NOTIFY
	/// [`STRUCTURE_NOTIFY`]: crate::mask::EventMask::STRUCTURE_NOTIFY
	pub struct Reparent: Event(21) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The window on which this `Reparent` event was generated.
		///
		/// For clients selecting [`STRUCTURE_NOTIFY`] on the `window` that was
		/// reparented, this is that `window`. For clients selecting
		/// [`SUBSTRUCTURE_NOTIFY`] on the `window`'s old parent or
		/// `new_parent`, this is that parent.
		///
		/// [`STRUCTURE_NOTIFY`]: crate::mask::EventMask::STRUCTURE_NOTIFY
		/// [`SUBSTRUCTURE_NOTIFY`]: crate::mask::EventMask::SUBSTRUCTURE_NOTIFY
		pub event_window: Window,
		/// The window which was reparented.
		pub window: Window,
		/// The `window`'s new parent.
		pub new_parent: Window,

		/// The `window`'s new coordinates relative to its `new_parent`'s origin.
		pub coords: Point,

		/// Whether [`MapWindow`] and [`ConfigureWindow`] requests on the
		/// `window` should override a [`SUBSTRUCTURE_REDIRECT`] on the
		/// window's parent.
		///
		/// This is typically set to inform the window manager not to tamper
		/// with the window.
		///
		/// [`MapWindow`]: super::request::MapWindow
		/// [`ConfigureWindow`]: super::request::ConfigureWindow
		///
		/// [`SUBSTRUCTURE_REDIRECT`]: crate::mask::EventMask::SUBSTRUCTURE_REDIRECT
		pub override_redirect: bool,
		[_; ..],
	}

	/// An event generated when a [`ConfigureWindow` request] changes the state
	/// of a [window].
	///
	/// This event is reported to clients selecting [`STRUCTURE_NOTIFY`] on the
	/// window, and to clients selecting [`SUBSTRUCTURE_NOTIFY`] on its parent.
	///
	/// [`ConfigureWindow` request]: super::request::ConfigureWindow
	/// [window]: Window
	/// [`STRUCTURE_NOTIFY`]: crate::mask::EventMask::STRUCTURE_NOTIFY
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::mask::EventMask::SUBSTRUCTURE_NOTIFY
	pub struct Configure: Event(22) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The window on which this `Configure` event was generated.
		///
		/// For clients selecting [`STRUCTURE_NOTIFY`] on the window that was
		/// mapped, this is that window. For clients selecting
		/// [`SUBSTRUCTURE_NOTIFY`] on the window's parent, this is that parent.
		///
		/// [`STRUCTURE_NOTIFY`]: crate::mask::EventMask::STRUCTURE_NOTIFY
		/// [`SUBSTRUCTURE_NOTIFY`]: crate::mask::EventMask::SUBSTRUCTURE_NOTIFY
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
		pub border_width: u16,

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
		/// [`SUBSTRUCTURE_REDIRECT`]: crate::mask::EventMask::SUBSTRUCTURE_REDIRECT
		pub override_redirect: bool,
		[_; ..],
	}

	/// An event generated when a [window] sends a [`ConfigureWindow` request].
	///
	/// This event is reported to the client selecting [`SUBSTRUCTURE_REDIRECT`]
	/// on the window's parent.
	///
	/// This event is generated when a client other than the one selecting
	/// [`SUBSTRUCTURE_REDIRECT`] sends a [`ConfigureWindow` request] for that
	/// window.
	///
	/// The `mask` and corresponding values are reported as given in the
	/// request. The remaining values are filled in from the current geometry of
	/// the window, except for `sibling` and `stack_mode`, which are reported as
	/// [`None`] and [`StackMode::Above`] respectively if not given in the
	/// request.
	///
	/// [window]: Window
	/// [`ConfigureWindow` request]: super::request::ConfigureWindow
	///
	/// [`SUBSTRUCTURE_REDIRECT`]: crate::mask::EventMask::SUBSTRUCTURE_REDIRECT
	pub struct ConfigureWindowRequest: Event(23) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// The [stacking mode] to use to restack the window.
		///
		/// See [`StackMode`] for more information.
		///
		/// [stacking mode]: StackMode
		pub stack_mode: StackMode,

		/// The `window`'s parent, on which [`SUBSTRUCTURE_REDIRECT`] is
		/// selected.
		///
		/// [`SUBSTRUCTURE_REDIRECT`]: crate::mask::EventMask::SUBSTRUCTURE_REDIRECT
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
		pub mask: ConfigureWindowMask,
		[_; ..],
	}

	/// An event generated when a [window] is moved because its parent is
	/// resized.
	///
	/// This event is reported to clients selecting [`STRUCTURE_NOTIFY`] on the
	/// window, and to clients selecting [`SUBSTRUCTURE_NOTIFY`] on its parent.
	///
	/// [window]: Window
	/// [`STRUCTURE_NOTIFY`]: crate::mask::EventMask::STRUCTURE_NOTIFY
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::mask::EventMask::SUBSTRUCTURE_NOTIFY
	pub struct Gravity: Event(24) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The window which this `Gravity` event was generated on.
		///
		/// For clients selecting [`STRUCTURE_NOTIFY`] on the `window` that was
		/// moved, this is that `window`. For clients selecting
		/// [`SUBSTRUCTURE_NOTIFY`] on the `window`'s parent, this is that parent.
		///
		/// [`STRUCTURE_NOTIFY`]: crate::mask::EventMask::STRUCTURE_NOTIFY
		/// [`SUBSTRUCTURE_NOTIFY`]: crate::mask::EventMask::SUBSTRUCTURE_NOTIFY
		pub event_window: Window,
		/// The window which was moved.
		pub window: Window,

		/// The new coordinates of the `window`, relative to its parent's
		/// origin.
		pub coords: Point,
		[_; ..],
	}

	/// An event generated when a [window] on which a client is selecting
	/// [`RESIZE_REDIRECT`] has a [`ConfigureWindow` request] sent by another
	/// client attempt to change the window's size.
	///
	/// This event is reported to the client selecting [`RESIZE_REDIRECT`] on
	/// the window.
	///
	/// [window]: Window
	/// [`RESIZE_REDIRECT`]: crate::mask::EventMask::RESIZE_REDIRECT
	/// [`ConfigureWindow` request]: super::request::ConfigureWindow
	pub struct ResizeRequest: Event(25) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The window which the [`ConfigureWindow` request] attempted to
		/// resize.
		///
		/// [`ConfigureWindow` request]: super::request::ConfigureWindow
		pub window: Window,

		/// The width which the [`ConfigureWindow` request] is attempting to
		/// resize the window to.
		///
		/// [`ConfigureWindow` request]: super::request::ConfigureWindow
		pub width: u16,
		/// The height which the [`ConfigureWindow` request] is attempting to
		/// resize the window to.
		///
		/// [`ConfigureWindow` request]: super::request::ConfigureWindow
		pub height: u16,
		[_; ..],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
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
	/// An event generated when a [window] is restacked due to a
	/// [`CirculateWindow` request].
	///
	/// This event is reported to clients selecting [`STRUCTURE_NOTIFY`] on the
	/// window, and to clients selecting [`SUBSTRUCTURE_NOTIFY`] on its parent.
	///
	/// [window]: Window
	/// [`CirculateWindow` request]: super::request::CirculateWindow
	///
	/// [`STRUCTURE_NOTIFY`]: crate::mask::EventMask::STRUCTURE_NOTIFY
	/// [`SUBSTRUCTURE_NOTIFY`]: crate::mask::EventMask::SUBSTRUCTURE_NOTIFY
	pub struct Circulate: Event(26) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The window which this `Circulate` event was generated on.
		///
		/// For clients selecting [`STRUCTURE_NOTIFY`] on the `window` that was
		/// restacked, this is that `window`. For clients selecting
		/// [`SUBSTRUCTURE_NOTIFY`] on the `window`'s parent, this is that parent.
		///
		/// [`STRUCTURE_NOTIFY`]: crate::mask::EventMask::STRUCTURE_NOTIFY
		/// [`SUBSTRUCTURE_NOTIFY`]: crate::mask::EventMask::SUBSTRUCTURE_NOTIFY
		pub event_window: Window,
		/// The window which was restacked.
		pub window: Window,
		[_; 4],

		/// The new placement in the window stack of the `window` in relation to
		/// its siblings.
		pub placement: Placement,
		[_; ..],
	}

	/// An event generated when a [`CirculateWindow` request] is sent for a
	/// [window] and that window actually needs to be restacked.
	///
	/// This event is reported to the client selecting [`SUBSTRUCTURE_REDIRECT`]
	/// on the window's parent.
	///
	/// [window]: Window
	/// [`CirculateWindow` request]: super::request::CirculateWindow
	pub struct CirculateRequest: Event(27) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The parent of the `window` the [`CirculateWindow` request] applies
		/// to.
		///
		/// This is the window that this `CirculateRequest` event was generated
		/// on.
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
/// Whether a `property` was [`Modified`] or [`Deleted`] in a [`Property`]
/// event.
///
/// [`Modified`]: PropertyChange::Modified
/// [`Deleted`]: PropertyChange::Deleted
pub enum PropertyChange {
	/// The `property` was added or its value was changed.
	Modified,
	/// The `property` was removed.
	Deleted,
}

derive_xrb! {
	/// An event generated when a [window] property is added, modified, or
	/// removed.
	///
	/// This event is reported to clients selecting [`PROPERTY_CHANGE`] on the
	/// window.
	///
	/// [window]: Window
	/// [`PROPERTY_CHANGE`]: crate::mask::EventMask::PROPERTY_CHANGE
	pub struct Property: Event(28) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The window on which the `property` was changed.
		pub window: Window,

		/// The property that was changed.
		pub property: Atom,
		/// The time at which the property was changed.
		pub time: Timestamp,
		/// Whether the property was [`Modified`] or [`Deleted`].
		///
		/// [`Modified`]: PropertyChange::Modified
		/// [`Deleted`]: PropertyChange::Deleted
		pub change: PropertyChange,
		[_; ..],
	}

	/// An event generated when a new selection owner is defined for a
	/// selection.
	///
	/// This event is reported to the current owner of a selection.
	///
	/// A new selection owner is defined via the use of the
	/// [`SetSelectionOwner` request].
	///
	/// [`SetSelectionOwner` request]: super::request::SetSelectionOwner
	pub struct SelectionClear: Event(29) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The time at which the new `selection` owner was defined.
		pub time: Timestamp,
		/// The current owner of the `selection`.
		pub owner: Window,
		/// The selection which had a new selection owner defined.
		pub selection: Atom,
		[_; ..],
	}

	/// An event generated when a [`ConvertSelection` request] is sent.
	///
	/// This event is reported to the owner of the selection.
	///
	/// The owner should convert the selection based on the specified target
	/// type and send a [`Selection` event] back to the requester using the
	/// [`SendEvent` request].
	///
	/// A complete specification for using selections is given in the
	/// _Inter-Client Communication Conventions Manual._
	///
	/// [`ConvertSelection` request]: super::request::ConvertSelection
	/// [`Selection` event]: Selection
	/// [`SendEvent` request]: super::request::SendEvent
	pub struct ConvertSelectionRequest: Event(30) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
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

	/// A reply to the [`ConvertSelection` request].
	///
	/// This event is reported to the `requester` of a
	/// [`ConvertSelection` request].
	///
	/// If the selection has no owner, this is generated by the X server. If the
	/// selection does have an owner, that owner should generate this event
	/// using the [`SendEvent` request].
	///
	/// [`ConvertSelection` request]: super::request::ConvertSelection
	/// [`SendEvent` request]: super::request::SendEvent
	pub struct Selection: Event(31) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
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

	/// The reason why a [`Colormap` event] was generated.
	///
	/// [`Colormap` event]: Colormap
	pub enum ColormapDetail {
		/// The `window`'s [`COLORMAP` attribute] was changed.
		///
		/// [`COLORMAP` attribute]: crate::mask::AttributeMask::COLORMAP
		AttributeChanged,
		/// The `window`'s [colormap] was installed or uninstalled.
		///
		/// [colormap]: crate::Colormap
		InstalledOrUninstalled,
	}

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

	/// An event generated when a [window]'s [colormap] is installed,
	/// uninstalled, or its [`COLORMAP` attribute] is changed.
	///
	/// This event is reported to clients selecting [`COLORMAP_CHANGE`] on the
	/// window.
	///
	/// [window]: Window
	/// [colormap]: crate::Colormap
	/// [`COLORMAP` attribute]: crate::mask::AttributeMask::COLORMAP
	///
	/// [`COLORMAP_CHANGE`]: crate::mask::EventMask::COLORMAP_CHANGE
	pub struct Colormap: Event(32) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		/// The window that this event relates to.
		pub window: Window,
		/// The `window`'s [colormap].
		pub colormap: Option<crate::Colormap>,

		/// Whether this event was generated because the `window`'s
		/// [`COLORMAP` attribute] was changed or because the `window`'s
		/// `colormap` was installed or uninstalled.
		///
		/// [`COLORMAP` attribute]: crate::mask::AttributeMask::COLORMAP
		pub detail: ColormapDetail,
		/// Whether the `window`'s `colormap` is currently installed.
		pub state: ColormapState,
		[_; ..],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
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
	/// An event generated by a [`SendEvent` request].
	///
	/// This event is reported to the [`SendEvent` request]'s `destination`
	/// [window].
	///
	/// [`SendEvent` request]: super::request::SendEvent
	/// [window]: Window
	pub struct ClientMessage: Event(33) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
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
		/// The data contained in this event.
		pub data: ClientMessageData,
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
pub enum KeyMappingRequest {
	Modifier,
	Keyboard,
	Pointer,
}

derive_xrb! {
	pub struct KeyMapping: Event(34) {
		#[sequence]
		pub sequence: u16,

		pub request: KeyMappingRequest,

		pub first_keycode: Keycode,
		pub count: u8,
		[_; ..],
	}
}
