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
	pub struct KeyPress: Event(2) {
		#[metabyte]
		pub keycode: Keycode,
		#[sequence]
		pub sequence: u16,

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

	pub struct KeyRelease: Event(3) {
		#[metabyte]
		pub keycode: Keycode,
		#[sequence]
		pub sequence: u16,

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

	pub struct ButtonPress: Event(4) {
		#[metabyte]
		pub button: Button,
		#[sequence]
		pub sequence: u16,

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

	pub struct ButtonRelease: Event(5) {
		#[metabyte]
		pub button: Button,
		#[sequence]
		pub sequence: u16,

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
pub enum MotionNotifyType {
	Normal,
	Hint,
}

derive_xrb! {
	pub struct MotionNotify: Event(6) {
		#[metabyte]
		pub notification_type: MotionNotifyType,
		#[sequence]
		pub sequence: u16,

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
		#[metabyte]
		pub detail: EnterLeaveDetail,
		#[sequence]
		pub sequence: u16,

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
		#[metabyte]
		pub detail: EnterLeaveDetail,
		#[sequence]
		pub sequence: u16,

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
		#[metabyte]
		pub detail: FocusDetail,
		#[sequence]
		pub sequence: u16,

		pub window: Window,
		pub grab_mode: FocusGrabMode,
		[_; ..],
	}

	pub struct FocusOut: Event(10) {
		#[metabyte]
		pub detail: FocusDetail,
		#[sequence]
		pub sequence: u16,

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
		#[metabyte]
		pub stack_mode: StackMode,
		#[sequence]
		pub sequence: u16,

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
		#[metabyte]
		pub format: u8,
		#[sequence]
		pub sequence: u16,

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
