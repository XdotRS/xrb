// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{
	mask::{ConfigureWindowMask, ModifierMask},
	Atom,
	Button,
	Colormap,
	CurrentableTime,
	Drawable,
	GrabMode,
	Keycode,
	Point,
	StackMode,
	Timestamp,
	Window,
};
use bitflags::bitflags;

use xrbk_macro::{derive_xrb, DataSize, Readable, StaticDataSize, Writable};
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
		/// This window is found by beginning with the window which the cursor
		/// is located within, then looking up the window hierarchy for the
		/// first window on which any client has selected interest in this
		/// event (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs or the currently focused window may modify how the
		/// `event_window` is chosen.
		pub event_window: Window,
		/// The direct child of the `event_window` which is an ancestor of the
		/// window in which the cursor was located when this event was
		/// generated, if one exists.
		///
		/// If the window in which the cursor was located within (the source
		/// window) when this event was generated is a descendant of the
		/// `event_window` (that is, it was a child of it, or a child of a
		/// child of it, or a child of a child of a child of it, etc.), then
		/// this is set to the direct child of the `event_window` which is the
		/// ancestor, or is, the source window. Otherwise, if the source window
		/// was not a descendent of the `event_window`, then this is set to
		/// `None`.
		///
		/// That means if the source window was a child of a child of the
		/// `event_window`, then this would be set to the source window's
		/// parent, as that is an ancestor of the source window and a direct
		/// child of the `event_window`.
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the time this event was
		/// generated, relative to the `root` window's origin.
		pub root_coords: Point,
		/// The coordinates of the cursor at the time this event was
		/// generated, relative to the `event_window`'s origin.
		pub event_coords: Point,

		/// The logical state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,
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
		/// This window is found by beginning with the window which the cursor
		/// is located within, then looking up the window hierarchy for the
		/// first window on which any client has selected interest in this
		/// event (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs or the currently focused window may modify how the
		/// `event_window` is chosen.
		pub event_window: Window,
		/// The direct child of the `event_window` which is an ancestor of the
		/// window in which the cursor was located when this event was
		/// generated, if one exists.
		///
		/// If the window in which the cursor was located within (the source
		/// window) when this event was generated is a descendant of the
		/// `event_window` (that is, it was a child of it, or a child of a
		/// child of it, or a child of a child of a child of it, etc.), then
		/// this is set to the direct child of the `event_window` which is the
		/// ancestor, or is, the source window. Otherwise, if the source window
		/// was not a descendent of the `event_window`, then this is set to
		/// `None`.
		///
		/// That means if the source window was a child of a child of the
		/// `event_window`, then this would be set to the source window's
		/// parent, as that is an ancestor of the source window and a direct
		/// child of the `event_window`.
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the time this event was
		/// generated, relative to the `root` window's origin.
		pub root_coords: Point,
		/// The coordinates of the cursor at the time this event was
		/// generated, relative to the `event_window`'s origin.
		pub event_coords: Point,

		/// The logical state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,
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
		/// This window is found by beginning with the window which the cursor
		/// is located within, then looking up the window hierarchy for the
		/// first window on which any client has selected interest in this
		/// event (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs may modify how the `event_window` is chosen.
		pub event_window: Window,
		/// The direct child of the `event_window` which is an ancestor of the
		/// window in which the cursor was located when this event was
		/// generated, if one exists.
		///
		/// If the window in which the cursor was located within (the source
		/// window) when this event was generated is a descendant of the
		/// `event_window` (that is, it was a child of it, or a child of a
		/// child of it, or a child of a child of a child of it, etc.), then
		/// this is set to the direct child of the `event_window` which is the
		/// ancestor, or is, the source window. Otherwise, if the source window
		/// was not a descendent of the `event_window`, then this is set to
		/// `None`.
		///
		/// That means if the source window was a child of a child of the
		/// `event_window`, then this would be set to the source window's
		/// parent, as that is an ancestor of the source window and a direct
		/// child of the `event_window`.
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `root` window's origin.
		pub root_coords: Point,
		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `event_window`'s origin.
		pub event_coords: Point,

		/// The logical state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,
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
		/// This window is found by beginning with the window which the cursor
		/// is located within, then looking up the window hierarchy for the
		/// first window on which any client has selected interest in this
		/// event (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs may modify how the `event_window` is chosen.
		pub event_window: Window,
		/// The direct child of the `event_window` which is an ancestor of the
		/// window in which the cursor was located when this event was
		/// generated, if one exists.
		///
		/// If the window in which the cursor was located within (the source
		/// window) when this event was generated is a descendant of the
		/// `event_window` (that is, it was a child of it, or a child of a
		/// child of it, or a child of a child of a child of it, etc.), then
		/// this is set to the direct child of the `event_window` which is the
		/// ancestor, or is, the source window. Otherwise, if the source window
		/// was not a descendent of the `event_window`, then this is set to
		/// `None`.
		///
		/// That means if the source window was a child of a child of the
		/// `event_window`, then this would be set to the source window's
		/// parent, as that is an ancestor of the source window and a direct
		/// child of the `event_window`.
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `root` window's origin.
		pub root_coords: Point,
		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `event_window`'s origin.
		pub event_coords: Point,

		/// The logical state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,
		pub same_screen: bool,
		_,
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum MotionNotifyType {
	Normal,
	Hint,
}

derive_xrb! {
	pub struct MotionNotify: Event(6) {
		#[sequence]
		pub sequence: u16,
		#[metabyte]
		pub notification_type: MotionNotifyType,

		pub time: Timestamp,

		pub root: Window,
		pub window: Window,
		pub child_window: Option<Window>,

		pub root_coords: Point,
		pub window_coords: Point,

		pub modifiers: ModifierMask,
		pub same_screen: bool,
		_,
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum EnterLeaveDetail {
	Ancestor,
	Virtual,
	Inferior,
	Nonlinear,
	NonlinearVirtual,
}

bitflags! {
	#[derive(Default, DataSize, Readable, StaticDataSize, Writable)]
	pub struct EnterLeaveMask: u8 {
		const FOCUS = 0x01;
		const SAME_SCREEN = 0x02;
	}
}

derive_xrb! {
	pub struct EnterNotify: Event(7) {
		#[sequence]
		pub sequence: u16,
		#[metabyte]
		pub detail: EnterLeaveDetail,

		pub time: Timestamp,

		pub root: Window,
		pub window: Window,
		pub child_window: Option<Window>,

		pub root_coords: Point,
		pub window_coords: Point,

		pub modifiers: ModifierMask,
		pub grab_mode: GrabMode,
		pub mask: EnterLeaveMask,
	}

	pub struct LeaveNotify: Event(8) {
		#[sequence]
		pub sequence: u16,
		#[metabyte]
		pub detail: EnterLeaveDetail,

		pub time: Timestamp,

		pub root: Window,
		pub window: Window,
		pub child_window: Option<Window>,

		pub root_coords: Point,
		pub window_coords: Point,

		pub modifiers: ModifierMask,
		pub grab_mode: GrabMode,
		pub mask: EnterLeaveMask,
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum FocusDetail {
	Ancestor,
	Virtual,
	Inferior,
	Nonlinear,
	NonlinearVirtual,
	Pointer,
	PointerRoot,
	None,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum FocusGrabMode {
	Normal,
	Grab,
	Ungrab,
	WhileGrabbed,
}

derive_xrb! {
	pub struct FocusIn: Event(9) {
		#[sequence]
		pub sequence: u16,
		#[metabyte]
		pub detail: FocusDetail,

		pub window: Window,
		pub grab_mode: FocusGrabMode,
		[_; ..],
	}

	pub struct FocusOut: Event(10) {
		#[sequence]
		pub sequence: u16,
		#[metabyte]
		pub detail: FocusDetail,

		pub window: Window,
		pub grab_mode: FocusGrabMode,
		[_; ..],
	}

	pub struct KeymapNotify: Event(11) {
		pub keys: [Keycode; 31],
	}

	pub struct Expose: Event(12) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,

		pub x: u16,
		pub y: u16,
		pub width: u16,
		pub height: u16,

		pub count: u16,
		[_; ..],
	}

	pub struct GraphicsExposure: Event(13) {
		#[sequence]
		pub sequence: u16,

		pub drawable: Drawable,

		pub x: u16,
		pub y: u16,
		pub width: u16,
		pub height: u16,

		pub minor_opcode: u16,
		pub count: u16,
		pub major_opcode: u8,
		[_; ..],
	}

	pub struct NoExposure: Event(14) {
		#[sequence]
		pub sequence: u16,

		pub drawable: Drawable,
		pub minor_opcode: u16,
		pub major_opcode: u8,
		[_; ..],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum Visibility {
	Unobscured,
	PartiallyObscured,
	FullyObscured,
}

derive_xrb! {
	pub struct VisibilityNotify: Event(15) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,
		pub visibility: Visibility,
		[_; ..],
	}

	pub struct CreateNotify: Event(16) {
		#[sequence]
		pub sequence: u16,

		pub parent: Window,
		pub window: Window,

		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,

		pub border_width: u16,

		pub override_redirect: bool,
		[_; ..],
	}

	pub struct DestroyNotify: Event(17) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,
		pub destroyed_window: Window,
		[_; ..],
	}

	pub struct UnmapNotify: Event(18) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,
		pub unmapped_window: Window,

		/// Whether the window was unmapped with a [`ConfigureWindow`] request.
		///
		/// [`ConfigureWindow`]: crate::x11::request::ConfigureWindow
		pub from_configure: bool,
		[_; ..],
	}

	pub struct MapNotify: Event(19) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,
		pub mapped_window: Window,

		pub override_redirect: bool,
		[_; ..],
	}

	pub struct MapRequest: Event(20) {
		#[sequence]
		pub sequence: u16,

		pub parent: Window,
		pub window: Window,
		[_; ..],
	}

	pub struct ReparentNotify: Event(21) {
		#[sequence]
		pub sequence: u16,

		// TODO: name these fields better; work out what they mean
		pub window: Window,
		pub reparented_window: Window,
		pub parent: Window,

		pub x: i16,
		pub y: i16,

		pub override_redirect: bool,
		[_; ..],
	}

	pub struct ConfigureNotify: Event(22) {
		#[sequence]
		pub sequence: u16,

		pub event: Window,
		pub window: Window,
		pub above_sibling: Option<Window>,

		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,

		pub border_width: u16,

		pub override_redirect: bool,
		[_; ..],
	}

	pub struct ConfigureRequest: Event(23) {
		#[sequence]
		pub sequence: u16,
		#[metabyte]
		pub stack_mode: StackMode,

		pub parent: Window,
		pub window: Window,
		pub sibling: Option<Window>,

		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,

		pub mask: ConfigureWindowMask,
		[_; ..],
	}

	pub struct GravityNotify: Event(24) {
		#[sequence]
		pub sequence: u16,

		// TODO: name these fields better
		pub event: Window,
		pub window: Window,

		pub x: i16,
		pub y: i16,
		[_; ..],
	}

	pub struct ResizeRequest: Event(25) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,

		pub width: u16,
		pub height: u16,
		[_; ..],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum Placement {
	Top,
	Bottom,
}

derive_xrb! {
	pub struct CirculateNotify: Event(26) {
		#[sequence]
		pub sequence: u16,

		// TODO: name these better
		pub event: Window,
		pub window: Window,
		// FIXME: in the protocol it says this is a window with the name
		//        `unused`... I think that is a mistake, especially given the
		//        next event not having such a field, but we should make sure.
		[_; 4],

		pub placement: Placement,
		[_; ..],
	}

	pub struct CirculateRequest: Event(27) {
		#[sequence]
		pub sequence: u16,

		// TODO: name these better
		pub event: Window,
		pub window: Window,
		[_; 4],

		pub placement: Placement,
		[_; ..],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum PropertyChange {
	// This might be for new properties added too, if so mention that in the docs when written.
	Modified,
	Deleted,
}

derive_xrb! {
	pub struct PropertyNotify: Event(28) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,
		pub property: Atom,
		pub time: Timestamp,
		pub change: PropertyChange,
		[_; ..],
	}

	pub struct SelectionClear: Event(29) {
		#[sequence]
		pub sequence: u16,

		pub time: Timestamp,
		pub owner: Window,
		pub selection: Atom,
		[_; ..],
	}

	pub struct SelectionRequest: Event(30) {
		#[sequence]
		pub sequence: u16,

		pub time: CurrentableTime,

		pub owner: Window,
		pub requestor: Window,

		pub selection: Atom,
		pub target: Atom,
		pub property: Option<Atom>,
		[_; ..],
	}

	pub struct SelectionNotify: Event(31) {
		#[sequence]
		pub sequence: u16,

		pub time: CurrentableTime,

		pub requestor: Window,

		pub selection: Atom,
		pub target: Atom,
		pub property: Option<Atom>,
		[_; ..],
	}

	pub struct ColormapNotify: Event(32) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,
		pub colormap: Option<Colormap>,
	}

	pub struct ClientMessage: Event(33) {
		#[sequence]
		pub sequence: u16,
		#[metabyte]
		pub format: u8,

		pub window: Window,
		pub r#type: Atom,

		pub data: [u8; 20],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum MappingNotifyRequest {
	Modifier,
	Keyboard,
	Pointer,
}

derive_xrb! {
	pub struct MappingNotify: Event(34) {
		#[sequence]
		pub sequence: u16,

		pub request: MappingNotifyRequest,

		pub first_keycode: Keycode,
		pub count: u8,
		[_; ..],
	}
}
