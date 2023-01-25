// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Replies] defined in the [core X11 protocol].
//!
//! [Replies] are messages sent from the X server to an X client in response to
//! a [request].
//!
//! [Replies]: Reply
//! [request]: crate::message::Request
//! [core X11 protocol]: super

extern crate self as xrb;

use derivative::Derivative;

use xrbk::{
	Buf,
	BufMut,
	ConstantX11Size,
	ReadResult,
	Readable,
	ReadableWithContext,
	Writable,
	WriteResult,
	X11Size,
};
use xrbk_macro::{derive_xrb, Readable, Writable, X11Size};

use crate::{
	message::Reply,
	unit::Px,
	visual::{ColorId, VisualId},
	x11::request::{self, DataFormat, DataList, RevertFocus},
	Atom,
	BitGravity,
	Colormap,
	Coords,
	DeviceEventMask,
	EventMask,
	FocusWindow,
	GrabStatus,
	LengthString8,
	MaintainContents,
	ModifierMask,
	Rectangle,
	String8,
	Timestamp,
	Window,
	WindowClass,
	WindowGravity,
};

/// The state of the [window] regarding how it is mapped.
///
/// [window]: Window
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum MapState {
	/// The [window] is not mapped.
	///
	/// [window]: Window
	Unmapped,

	/// The [window] is mapped but one of its ancestors is unmapped.
	///
	/// [window]: Window
	Unviewable,

	/// The [window] is mapped and all of its ancestors are mapped.
	///
	/// [window]: Window
	Viewable,
}

derive_xrb! {
	/// The [reply] to a [`GetWindowAttributes` request].
	///
	/// [reply]: Reply
	///
	/// [`GetWindowAttributes` request]: request::GetWindowAttributes
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetWindowAttributes: Reply for request::GetWindowAttributes {
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

		/// The conditions under which the X server should maintain the obscured
		/// [regions] of the [window].
		///
		/// See [`Attributes::maintain_contents`] for more information.
		///
		/// [regions]: crate::Region
		/// [window]: Window
		///
		/// [`Attributes::maintain_contents`]: crate::set::Attributes::maintain_contents
		#[doc(alias = "backing_store")]
		#[metabyte]
		pub maintain_contents: MaintainContents,

		/// The visual used by the [window].
		///
		/// See [`VisualType`] for more information.
		///
		/// [window]: Window
		///
		/// [`VisualType`]: crate::visual::VisualType
		pub visual: VisualId,
		/// The [window]'s [class].
		///
		/// [window]: Window
		/// [class]: WindowClass
		pub class: WindowClass,

		/// Defines the [region] of the [window] which is retained when the
		/// [window] is resized.
		///
		/// See [`Attributes::bit_gravity`] for more information.
		///
		/// [region]: crate::Region
		/// [window]: Window
		///
		/// [`Attributes::bit_gravity`]: crate::set::Attributes::bit_gravity
		pub bit_gravity: BitGravity,
		/// Defines how the [window] is repositioned if its parent is resized.
		///
		/// See [`Attributes::window_gravity`] for more information.
		///
		/// [window]: Window
		///
		/// [`Attributes::window_gravity`]: crate::set::Attributes::window_gravity
		#[doc(alias = "win_gravity")]
		pub window_graivty: WindowGravity,

		/// Defines which bit planes of the [window] hold dynamic data which is
		/// maintained for `maintain_contents` and `maintain_windows_under`.
		///
		/// See [`Attributes::maintained_planes`] for more information.
		///
		/// [window]: Window
		///
		/// [`Attributes::maintained_planes`]: crate::set::Attributes::maintained_planes
		#[doc(alias = "backing_planes")]
		pub maintained_planes: u32,
		/// Defines the [color] used for bit planes which are not preserved for
		/// `maintain_contents` and `maintain_windows_under` (see
		/// `maintained_planes`).
		///
		/// See [`Attributes::maintenance_fallback_color`] for more information.
		///
		/// [color]: ColorId
		///
		/// [`Attributes::maintenance_fallback_color`]: crate::set::Attributes::maintenance_fallback_color
		#[doc(alias = "backing_pixel")]
		pub maintenance_fallback_color: ColorId,
		/// Whether the X server should maintain the contents of [windows] under
		/// this [window].
		///
		/// See [`Attributes::maintain_windows_under`] for more information.
		///
		/// [window]: Window
		///
		/// [`Attributes::maintain_windows_under`]: crate::set::Attributes::maintain_windows_under
		#[doc(alias = "save_under")]
		pub maintain_windows_under: bool,

		/// Whether the [window]'s `colormap` is an installed [colormap] for
		/// the [screen].
		///
		/// [window]: Window
		/// [colormap]: Colormap
		/// [screen]: crate::visual::Screen
		pub map_installed: bool,
		/// The [window]'s [map state].
		///
		/// See [`MapState`] for more information.
		///
		/// [window]: Window
		/// [map state]: MapState
		pub map_state: MapState,

		/// Whether [`MapWindow`] and [`ConfigureWindow`] requests on the
		/// [window] override a [`SUBSTRUCTURE_REDIRECT`] selection on its
		/// parent.
		///
		/// This is typically used to inform a window manager not to tamper with
		/// the [window].
		///
		/// See [`Attributes::override_redirect`] for more information.
		///
		/// [window]: Window
		///
		/// [`MapWindow`]: request::MapWindow
		/// [`ConfigureWindow`]: request::ConfigureWindow
		///
		/// [`SUBSTRUCTURE_REDIRECT`]: EventMask::SUBSTRUCTURE_REDIRECT
		///
		/// [`Attributes::override_redirect`]: crate::set::Attributes::override_redirect
		pub override_redirect: bool,

		/// The [colormap] which best reflects the true colors of the [window].
		///
		/// See [`Attributes::colormap`] for more information.
		///
		/// [window]: Window
		/// [colormap]: Colormap
		///
		/// [`Attributes::colormap`]: crate::set::Attributes::colormap
		pub colormap: Option<Colormap>,

		/// All of the [events] selected by all clients on the [window].
		///
		/// This is the bitwise OR of every client's [`event_mask`] on the
		/// [window].
		///
		/// [window]: Window
		/// [events]: crate::message::Event
		///
		/// [`event_mask`]: crate::set::Attributes::event_mask
		pub all_event_masks: EventMask,
		/// The [events] selected by you on the [window].
		///
		/// This is your [`event_mask`] on the [window].
		///
		/// [window]: Window
		/// [events]: crate::message::Event
		///
		/// [`event_mask`]: crate::set::Attributes::event_mask
		pub your_event_mask: EventMask,
		/// Defines the [events][event] which should not be propagated to
		/// ancestors of the [window] if no client has selected the [event] on
		/// the [window].
		///
		/// See [`Attributes::do_not_propagate_mask`] for more information.
		///
		/// [event]: crate::message::Event
		/// [window]: Window
		///
		/// [`Attributes::do_not_propagate_mask`]: crate::set::Attributes::do_not_propagate_mask
		pub do_not_propagate_mask: DeviceEventMask,
		[_; ..],
	}

	/// The [reply] to a [`GetGeometry` request].
	///
	/// [reply]: Reply
	///
	/// [`GetGeometry` request]: request::GetGeometry
	#[doc(alias("GetX", "GetY", "GetWidth", "GetHeight", "GetBorderWidth"))]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetGeometry: Reply for request::GetGeometry {
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

		/// The number of bits per pixel for the [drawable].
		///
		/// [drawable]: crate::Drawable
		#[metabyte]
		pub depth: u8,

		/// The [drawable]'s root [window].
		///
		/// [window]: Window
		/// [drawable]: crate::Drawable
		pub root: Window,

		/// The [drawable]'s geometry.
		///
		/// For a [pixmap], the `x` and `y` coordinates will always be zero.
		///
		/// For a [window], the coordinates are relative to the top-left corner
		/// of the [window]'s parent.
		///
		/// [window]: Window
		/// [pixmap]: create::Pixmap
		/// [drawable]: crate::Drawable
		#[doc(alias("x", "y", "width", "height"))]
		pub geometry: Rectangle,
		/// The width of the [drawable]'s border.
		///
		/// For a [pixmap], this will always be zero.
		///
		/// [pixmap]: crate::Pixmap
		/// [drawable]: crate::Drawable
		pub border_width: Px<u16>,
		[_; ..],
	}

	/// The [reply] to a [`QueryWindowTree` request].
	///
	/// [reply]: Reply
	///
	/// [`QueryWindowTree` request]: request::QueryWindowTree
	#[doc(alias("QueryTree", "GetTree", "GetWindowTree"))]
	#[doc(alias("QueryParent", "QueryChildren", "QueryRoot"))]
	#[doc(alias("GetParent", "GetChildren", "GetRoot"))]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct QueryWindowTree: Reply for request::QueryWindowTree {
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

		/// The `target` [window]'s root [window].
		///
		/// [window]: Window
		pub root: Window,
		/// The `target` [window]'s parent.
		///
		/// [window]: Window
		pub parent: Option<Window>,

		// The length of `children`.
		#[allow(clippy::cast_possible_truncation)]
		let children_len: u16 = children => children.len() as u16,
		[_; 14],

		/// The `target` [window]'s children.
		///
		/// [window]: Window
		#[context(children_len => usize::from(*children_len))]
		pub children: Vec<Window>,
	}

	/// The [reply] to a [`GetAtom` request].
	///
	/// [reply]: Reply
	///
	/// [`GetAtom` request]: request::GetAtom
	#[doc(alias("InternAtom", "CreateAtom"))]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetAtom: Reply for request::GetAtom {
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

		/// The returned [atom].
		///
		/// If `no_creation` was set to `true` and an [atom] by the given `name`
		/// didn't already exist, this will be [`None`].
		///
		/// [atom]: Atom
		pub atom: Option<Atom>,
		[_; ..],
	}

	/// The [reply] to a [`GetAtomName` request].
	///
	/// [reply]: crate::message
	///
	/// [`GetAtomName` request]: request::GetAtomName
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetAtomName: Reply for request::GetAtomName {
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

		// The length of `name`.
		#[allow(clippy::cast_possible_truncation)]
		let name_len: u16 = name => name.len() as u16,
		[_; 22],

		/// The name of the [atom].
		///
		/// [atom]: Atom
		#[context(name_len => usize::from(*name_len))]
		pub name: String8,
		[_; ..],
	}

	/// The [reply] to a [`GetProperty` request].
	///
	/// [reply]: Reply
	///
	/// [`GetProperty` request]: request::GetProperty
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetProperty: Reply for request::GetProperty {
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

		/// Whether the `value` is empty ([`None`]), or made up of `i8` values,
		/// `i16` values, or `i32` values.
		#[metabyte]
		pub format: Option<DataFormat>,

		/// The actual type of the property.
		pub r#type: Option<Atom>,
		/// The number of bytes remaining in the `property`'s data.
		///
		/// If the specified `property` does not exist for the `target`
		/// [window], this is zero.
		///
		/// If the specified `property` exists but its `type` does not match the
		/// specified type, this is the size of the property's data in bytes.
		///
		/// If the specified `property` exists and the type is either [`Any`] or
		/// matches the actual `type` of the property, this is the number of
		/// bytes remaining in the `property`'s data after the end of the
		/// returned `value`.
		///
		/// [window]: Window
		///
		/// [`Any`]: crate::Any::Any
		#[doc(alias = "bytes_after")]
		pub bytes_remaining: u32,

		// The length of `value`.
		#[allow(clippy::cast_possible_truncation)]
		let value_len: u32 = value => value.len() as u32,
		[_; 12],

		/// The property's value.
		///
		/// If `format` is [`None`], this will be [`DataList::I8`], but with an
		/// empty list.
		#[context(format, value_len => (format.unwrap_or(DataFormat::I8), *value_len))]
		pub value: DataList,
	}

	/// The [reply] for a [`ListProperties` request].
	///
	/// [reply]: Reply
	///
	/// [`ListProperties` request]: request::ListProperties
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct ListProperties: Reply for request::ListProperties {
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

		// The length of `properties`.
		#[allow(clippy::cast_possible_truncation)]
		let properties_len: u16 = properties => properties.len() as u16,
		[_; 22],

		/// The properties defined for the given [window].
		///
		/// [window]: Window
		#[doc(alias = "atoms")]
		#[context(properties_len => usize::from(*properties_len))]
		pub properties: Vec<Atom>,
	}

	/// The [reply] to a [`GetSelectionOwner` request].
	///
	/// [reply]: Reply
	///
	/// [`GetSelectionOwner` request]: request::GetSelectionOwner
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetSelectionOwner: Reply for request::GetSelectionOwner {
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

		/// The owner of the given `selection`.
		///
		/// If this is [`None`], then the selection has no owner.
		pub owner: Option<Window>,
		[_; ..],
	}

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

/// A property of a font.
///
/// The value of this property is uninterpreted by XRB.
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub struct FontProperty {
	/// The name of the font property.
	pub name: Atom,
	/// The value of the property.
	///
	/// This is represented as four individual `u8` values because it is not
	/// necessarily one numerical value; it must not be subject to the byte
	/// swapping that would occur for a `u32` value.
	pub value: [u8; 4],
}

/// Information about a particular character within a font.
///
/// For a nonexistent character, all of these fields are zero.
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub struct CharacterInfo {
	/// The extent of this character's appearance beyond its left edge.
	///
	/// If this is negative, the character's appearance extends to the left of
	/// its x coordinate. If this is positive, the character's appearance starts
	/// after its x coordinate.
	pub left_side_bearing: i16,
	/// The extent of this character's appearance beyond its right edge.
	///
	/// If this is negative, the character's appearance ends before its width.
	/// If this is positive, the character's appearance extends beyond its
	/// width.
	pub right_side_bearing: i16,

	/// The width of this character - positive if it is read [`LeftToRight`],
	/// negative if it is read [`RightToLeft`].
	///
	/// [`LeftToRight`]: DrawDirection::LeftToRight
	/// [`RightToLeft`]: DrawDirection::RightToLeft
	#[doc(alias = "character_width")]
	pub width: i16,

	/// The extent of this character above the baseline.
	pub ascent: i16,
	/// The extent of this character at or below the baseline.
	pub descent: i16,

	/// The interpretation of these character attributes depends on the X
	/// server.
	pub attributes: u16,
}

impl ConstantX11Size for CharacterInfo {
	const X11_SIZE: usize = 12;
}

/// A hint as to whether most [`CharacterInfo`]s in a font have a positive or
/// negative width.
///
/// A positive width means the character is [`LeftToRight`]. A negative width
/// means the character is [`RightToLeft`].
///
/// [`LeftToRight`]: DrawDirection::LeftToRight
/// [`RightToLeft`]: DrawDirection::RightToLeft
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum DrawDirection {
	/// Most [`CharacterInfo`]s in the font have a positive width.
	LeftToRight,
	/// Most [`CharacterInfo`]s in the font have a negative width.
	RightToLeft,
}

impl ConstantX11Size for DrawDirection {
	const X11_SIZE: usize = 1;
}

derive_xrb! {
	/// The [reply] to a [`QueryFont` request].
	///
	/// [reply]: Reply
	///
	/// [`QueryFont` request]: request::QueryFont
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct QueryFont: Reply for request::QueryFont {
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

		/// A [`CharacterInfo`] representing the minimum bounds of all fields in
		/// each [`CharacterInfo`] in `character_infos`.
		pub min_bounds: CharacterInfo,
		[_; 4],

		/// A [`CharacterInfo`] representing the maximum bounds of all fields in
		/// each [`CharacterInfo`] in `character_infos`.
		pub max_bounds: CharacterInfo,
		[_; 4],

		/// If `min_major_index` and `max_major_index` are both zero, this is
		/// the character index of the first element in `character_infos`.
		/// Otherwise, this is a [`u8`] value used to index characters.
		///
		/// If either `min_major_index` or `max_major_index` aren't zero, the
		/// two indexes used to retrieve `character_infos` element `i` (counting
		/// from `i = 0`) are:
		/// ```
		/// # let i = 0;
		/// #
		/// # let first_character_or_min_minor_index = 0;
		/// # let last_character_or_max_minor_index = 1;
		/// #
		/// # let min_major_index = 2;
		/// #
		/// let major_index_range = {
		///     last_character_or_max_minor_index
		///     - first_character_or_min_minor_index
		///     + 1
		/// };
		///
		/// let major_index = i / major_index_range + min_major_index;
		/// let minor_index = i % major_index_range + first_character_or_min_minor_index;
		/// ```
		#[doc(alias = "min_char_or_byte2")]
		pub first_character_or_min_minor_index: u16,
		/// If `min_major_index` and `max_major_index` are both zero, this is
		/// the character index of the last element in `character_infos`.
		/// Otherwise, this is a [`u8`] value used to index characters.
		///
		/// If either `min_major_index` or `max_major_index` aren't zero, the
		/// two indexes used to retrieve `character_infos` element `i` (counting
		/// from `i = 0`) are:
		/// ```
		/// # let i = 0;
		/// #
		/// # let first_character_or_min_minor_index = 0;
		/// # let last_character_or_max_minor_index = 1;
		/// #
		/// # let min_major_index = 2;
		/// #
		/// let major_index_range = {
		///     last_character_or_max_minor_index
		///     - first_character_or_min_minor_index
		///     + 1
		/// };
		///
		/// let major_index = i / major_index_range + min_major_index;
		/// let minor_index = i % major_index_range + first_character_or_min_minor_index;
		/// ```
		#[doc(alias = "max_char_or_byte2")]
		pub last_character_or_max_minor_index: u16,

		/// The character used when an undefined or nonexistent character is
		/// used.
		///
		/// If a font uses two bytes to index its characters (such as that used
		/// for [`Char16`]), the first of the two bytes is found in the most
		/// significant byte of this `fallback_character`, and the second of the
		/// two bytes if found in the least significant byte.
		///
		/// [`Char16`]: crate::Char16
		#[doc(alias("default_char", "default_character", "fallback_char"))]
		pub fallback_character: u16,

		// The length of `properties`.
		#[allow(clippy::cast_possible_truncation)]
		let properties_len: u16 = properties => properties.len() as u16,

		/// A hint as to whether most [`CharacterInfo`s] in a font have a
		/// positive or negative width.
		///
		/// See [`DrawDirection`] for more information.
		///
		/// [`CharacterInfo`s]: CharacterInfo
		pub draw_direction: DrawDirection,

		/// The value of the major index used to retrieve the first element in
		/// `character_infos`.
		#[doc(alias = "min_byte1")]
		pub min_major_index: u8,
		/// The value of the major index used to retrieve the last element in
		/// `character_infos`.
		#[doc(alias = "max_byte1")]
		pub max_major_index: u8,

		/// Whether all of the [`CharacterInfo`s] in `character_infos` have
		/// nonzero bounds.
		///
		/// [`CharacterInfo`s]: CharacterInfo
		pub all_characters_exist: bool,

		/// The extent of the font above the baseline, used for determining line
		/// spacing.
		///
		/// Some specific characters may extend above this.
		pub font_ascent: i16,
		/// The extent of the font at or below the baseline, used for
		/// determining line spacing.
		///
		/// Some specific characters may extend below this.
		pub font_descent: i16,

		// The length of `character_infos`.
		#[allow(clippy::cast_possible_truncation)]
		let character_infos_len: u32 = character_infos => character_infos.len() as u32,

		/// A list of [`FontProperty`s] associated with the font.
		///
		/// [`FontProperty`s]: FontProperty
		#[context(properties_len => usize::from(*properties_len))]
		pub properties: Vec<FontProperty>,
		/// A list of the characters associated with the font, represented by
		/// [`CharacterInfo`s].
		///
		/// [`CharacterInfo`s]: CharacterInfo
		#[doc(alias = "char_infos")]
		#[context(character_infos_len => *character_infos_len as usize)]
		pub character_infos: Vec<CharacterInfo>,
	}

	/// The [reply] to a [`QueryTextExtents` request].
	///
	/// [reply]: Reply
	///
	/// [`QueryTextExtents` request]: request::QueryTextExtents
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct QueryTextExtents: Reply for request::QueryTextExtents {
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

		/// A hint as to whether most characters in a font have a positive
		/// `width` or a negative `width`.
		///
		/// See [`DrawDirection`] for more information.
		#[metabyte]
		pub draw_direction: DrawDirection,

		/// The extent of the font above the baseline, used for determining line
		/// spacing.
		///
		/// Some specific characters may extend above this.
		pub font_ascent: i16,
		/// The extent of the font at or below the baseline, used for
		/// determining line spacing.
		///
		/// Some specific characters may extend below this.
		pub font_descent: i16,

		/// The highest individual `ascent` of any character in `text`.
		pub overall_ascent: i16,
		/// The lowest individual `descent` of any character in `text`.
		pub overall_descent: i16,

		/// The sum of the `width`s of each character in the `text`.
		pub overall_width: i32,

		/// If the 'left side' of each character is the sum of the `width`s of
		/// all characters before it plus its `left_side_bearing`, this is the
		/// leftmost left side.
		pub overall_left: i32,
		/// If the 'right side' of each character is the sum of the `width`s of
		/// all characters before it, plus its `width` and `right_side_bearing`,
		/// this is the rightmost right side.
		pub overall_right: i32,
		[_; ..],
	}

	/// The [reply] to a [`ListFonts` request].
	///
	/// [reply]: Reply
	///
	/// [`ListFonts` request]: request::ListFonts
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct ListFonts: Reply for request::ListFonts {
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

		// The length of `names`.
		#[allow(clippy::cast_possible_truncation)]
		let names_len: u16 = names => names.len() as u16,
		[_; 22],

		/// The names of available fonts (no more than the given
		/// `max_names_count` will appear here, though).
		#[context(names_len => usize::from(*names_len))]
		pub names: Vec<LengthString8>,
		[_; ..],
	}
}

/// The [reply] to a [`ListFontsWithInfo` request].
///
/// The [`ListFontsWithInfo` request] is unique in that it has a series of
/// multiple replies, followed by a reply to terminate that series.
///
/// [reply]: Reply
///
/// [`ListFontsWithInfo` request]: request::ListFontsWithInfo
pub enum ListFontsWithInfo {
	/// Information about one of the available fonts.
	Font(FontWithInfo),
	/// Indicates the end of the series of replies to the
	/// [`ListFontsWithInfo` request].
	///
	/// [`ListFontsWithInfo` request]: request::ListFontsWithInfo
	Terminate(TerminateListFontsWithInfo),
}

impl Reply for ListFontsWithInfo {
	type Request = request::ListFontsWithInfo;

	fn sequence(&self) -> u16 {
		match self {
			Self::Font(FontWithInfo { sequence, .. })
			| Self::Terminate(TerminateListFontsWithInfo { sequence, .. }) => *sequence,
		}
	}
}

impl X11Size for ListFontsWithInfo {
	fn x11_size(&self) -> usize {
		match self {
			Self::Font(reply) => reply.x11_size(),
			Self::Terminate(last) => last.x11_size(),
		}
	}
}

impl Readable for ListFontsWithInfo {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		let name_len = buf.get_u8();
		let sequence = buf.get_u16();

		Ok(match name_len {
			zero if zero == 0 => Self::Terminate(<_>::read_with(buf, &sequence)?),

			other => Self::Font(<_>::read_with(buf, &(other, sequence))?),
		})
	}
}

impl Writable for ListFontsWithInfo {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		match self {
			Self::Font(reply) => reply.write_to(buf)?,

			Self::Terminate(last) => last.write_to(buf)?,
		}

		Ok(())
	}
}

/// A [reply] to a [`ListFontsWithInfo` request] that provides information about
/// one of the available fonts.
///
/// A `FontWithInfo` [reply] is sent for every available font. A
/// [`TerminateListFontsWithInfo` reply] terminates the series.
///
/// [reply]: Reply
///
/// [`ListFontsWithInfo` request]: request::ListFontsWithInfo
/// [`TerminateListFontsWithInfo` reply]: TerminateListFontsWithInfo
#[derive(Derivative, Debug)]
#[derivative(Hash, PartialEq, Eq)]
pub struct FontWithInfo {
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

	/// A [`CharacterInfo`] representing the minimum bounds of all fields in
	/// each [`CharacterInfo`] in `character_infos`.
	pub min_bounds: CharacterInfo,
	/// A [`CharacterInfo`] representing the maximum bounds of all fields in
	/// each [`CharacterInfo`] in `character_infos`.
	pub max_bounds: CharacterInfo,

	/// If `min_major_index` and `max_major_index` are both zero, this is the
	/// character index of the first element in `character_infos`. Otherwise,
	/// this is a [`u8`] value used to index characters.
	///
	/// If either `min_major_index` or `max_major_index` aren't zero, the
	/// two indexes used to retrieve `character_infos` element `i` (counting
	/// from `i = 0`) are:
	/// ```
	/// # let i = 0;
	/// #
	/// # let first_character_or_min_minor_index = 0;
	/// # let last_character_or_max_minor_index = 1;
	/// #
	/// # let min_major_index = 2;
	/// #
	/// let major_index_range = {
	///     last_character_or_max_minor_index
	///     - first_character_or_min_minor_index
	///     + 1
	/// };
	///
	/// let major_index = i / major_index_range + min_major_index;
	/// let minor_index = i % major_index_range + first_character_or_min_minor_index;
	/// ```
	#[doc(alias = "min_char_or_byte2")]
	pub first_character_or_min_minor_index: u16,
	/// If `min_major_index` and `max_major_index` are both zero, this is
	/// the character index of the last element in `character_infos`.
	/// Otherwise, this is a [`u8`] value used to index characters.
	///
	/// If either `min_major_index` or `max_major_index` aren't zero, the
	/// two indexes used to retrieve `character_infos` element `i` (counting
	/// from `i = 0`) are:
	/// ```
	/// # let i = 0;
	/// #
	/// # let first_character_or_min_minor_index = 0;
	/// # let last_character_or_max_minor_index = 1;
	/// #
	/// # let min_major_index = 2;
	/// #
	/// let major_index_range = {
	///     last_character_or_max_minor_index
	///     - first_character_or_min_minor_index
	///     + 1
	/// };
	///
	/// let major_index = i / major_index_range + min_major_index;
	/// let minor_index = i % major_index_range + first_character_or_min_minor_index;
	/// ```
	#[doc(alias = "max_char_or_byte2")]
	pub last_character_or_max_minor_index: u16,

	/// The character used when an undefined or nonexistent character is
	/// used.
	///
	/// If a font uses two bytes to index its characters (such as that used
	/// for [`Char16`]), the first of the two bytes is found in the most
	/// significant byte of this `fallback_character`, and the second of the
	/// two bytes if found in the least significant byte.
	///
	/// [`Char16`]: crate::Char16
	#[doc(alias = "default_char")]
	pub fallback_character: u16,

	/// A hint as to whether most [`CharacterInfo`s] in a font have a
	/// positive or negative width.
	///
	/// See [`DrawDirection`] for more information.
	///
	/// [`CharacterInfo`s]: CharacterInfo
	pub draw_direction: DrawDirection,

	/// The value of the major index used to retrieve the first character in the
	/// font.
	#[doc(alias = "min_byte1")]
	pub min_major_index: u8,
	/// The value of the major index used to retrieve the last character in the
	/// font.
	#[doc(alias = "max_byte1")]
	pub max_major_index: u8,

	/// Whether all of the characters in the font have nonzero bounds.
	pub all_chars_exist: bool,

	/// The extent of the font above the baseline, used for determining line
	/// spacing.
	///
	/// Some specific characters may extend above this.
	pub font_ascent: i16,
	/// The extent of the font at or below the baseline, used for
	/// determining line spacing.
	///
	/// Some specific characters may extend below this.
	pub font_descent: i16,

	/// A hint as to how many more [`FontWithInfo` replies] there will be.
	///
	/// Note that this is only a hint: there may be more or less replies than
	/// this number. A `replies_hint` of zero does not guarantee that there will
	/// be no more [`FontWithInfo` replies]: the only way to know that is to
	/// receive a [`TerminateListFontsWithInfo` reply].
	///
	/// [`FontWithInfo` replies]: FontWithInfo
	/// [`TerminateListFontsWithInfo` reply]: TerminateListFontsWithInfo
	pub replies_hint: u32,

	/// A list of [`FontProperty`s] associated with the font.
	///
	/// [`FontProperty`s]: FontProperty
	pub properties: Vec<FontProperty>,

	/// The name of this font.
	pub name: String8,
}

fn pad<T: X11Size>(thing: &T) -> usize {
	(4 - (thing.x11_size() % 4)) % 4
}

impl X11Size for FontWithInfo {
	fn x11_size(&self) -> usize {
		const CONSTANT_SIZES: usize = u8::X11_SIZE // `1`
			+ u8::X11_SIZE // length of `name`
			+ u16::X11_SIZE // `sequence`
			+ u32::X11_SIZE // length
			+ CharacterInfo::X11_SIZE // `min_bounds`
			+ 4 // 4 unused bytes
			+ CharacterInfo::X11_SIZE // `max_bounds`
			+ 4 // 4 unused bytes
			+ u16::X11_SIZE // `first_character_or_min_minor_index`
			+ u16::X11_SIZE // `last_character_or_max_minor_index`
			+ u16::X11_SIZE // `fallback_character`
			+ u16::X11_SIZE // length of `properties`
			+ DrawDirection::X11_SIZE // `draw_direction`
			+ u8::X11_SIZE // `min_major_index`
			+ u8::X11_SIZE // `max_major_index`
			+ bool::X11_SIZE // `all_chars_exist`
			+ i16::X11_SIZE // `font_ascent`
			+ i16::X11_SIZE // `font_descent`
			+ u32::X11_SIZE; // `replies_hint`

		CONSTANT_SIZES + self.properties.x11_size() + self.name.x11_size() + pad(&self.name)
	}
}

impl ReadableWithContext for FontWithInfo {
	type Context = (u8, u16);

	fn read_with(buf: &mut impl Buf, (name_len, sequence): &(u8, u16)) -> ReadResult<Self> {
		let name_len = usize::from(*name_len);

		// We skip the first 4 bytes because:
		// - the first, `1`, was required to know this is a reply
		// - the second was required to know the `name_len`
		// - the third and fourth - the sequence - were required to know that this is a
		//   `ListFontsWithInfo` reply

		// Read the length - take away the 8 bytes we've already read.
		let length = ((buf.get_u32() as usize) * 4) + (32 - 8);
		// Limit `buf` by the read `length`.
		let buf = &mut buf.take(length);

		let min_bounds = CharacterInfo::read_from(buf)?;
		buf.advance(4); // 4 unused bytes

		let max_bounds = CharacterInfo::read_from(buf)?;
		buf.advance(4); // 4 unused bytes

		let first_character_or_min_minor_index = u16::read_from(buf)?;
		let last_character_or_max_minor_index = u16::read_from(buf)?;

		let fallback_character = u16::read_from(buf)?;

		let properties_len = usize::from(u16::read_from(buf)?);

		let draw_direction = DrawDirection::read_from(buf)?;

		let min_major_index = u8::read_from(buf)?;
		let max_major_index = u8::read_from(buf)?;

		let all_chars_exist = bool::read_from(buf)?;

		let font_ascent = i16::read_from(buf)?;
		let font_descent = i16::read_from(buf)?;

		let replies_hint = u32::read_from(buf)?;

		let properties = <Vec<FontProperty>>::read_with(buf, &properties_len)?;

		let name = String8::read_with(buf, &name_len)?;
		buf.advance(pad(&name));

		Ok(Self {
			sequence: *sequence,

			min_bounds,
			max_bounds,

			first_character_or_min_minor_index,
			last_character_or_max_minor_index,

			fallback_character,

			draw_direction,

			min_major_index,
			max_major_index,

			all_chars_exist,

			font_ascent,
			font_descent,

			replies_hint,

			properties,

			name,
		})
	}
}

impl Writable for FontWithInfo {
	#[allow(clippy::cast_possible_truncation)]
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		buf.put_u8(1);
		buf.put_u8(self.name.len() as u8);
		self.sequence.write_to(buf)?;

		buf.put_u32(((self.x11_size() - 32) / 4) as u32);

		self.min_bounds.write_to(buf)?;
		// 4 unused bytes.
		buf.put_bytes(0, 4);

		self.max_bounds.write_to(buf)?;
		// 4 unused bytes.
		buf.put_bytes(0, 4);

		self.first_character_or_min_minor_index.write_to(buf)?;
		self.last_character_or_max_minor_index.write_to(buf)?;

		self.fallback_character.write_to(buf)?;

		buf.put_u16(self.properties.len() as u16);

		self.draw_direction.write_to(buf)?;

		self.min_major_index.write_to(buf)?;
		self.max_major_index.write_to(buf)?;

		self.all_chars_exist.write_to(buf)?;

		self.font_ascent.write_to(buf)?;
		self.font_descent.write_to(buf)?;

		self.replies_hint.write_to(buf)?;

		self.properties.write_to(buf)?;

		self.name.write_to(buf)?;
		// Padding bytes for `name`.
		buf.put_bytes(0, pad(&self.name));

		Ok(())
	}
}

/// A [reply] to a [`ListFontsWithInfo` request] that represents the final
/// [reply] sent for that [request].
///
/// [reply]: Reply
/// [request]: crate::message::Request
///
/// [`ListFontsWithInfo` request]: request::ListFontsWithInfo
#[derive(Derivative, Debug)]
#[derivative(Hash, PartialEq, Eq)]
pub struct TerminateListFontsWithInfo {
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
}

impl ConstantX11Size for TerminateListFontsWithInfo {
	const X11_SIZE: usize = 60;
}

impl X11Size for TerminateListFontsWithInfo {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Writable for TerminateListFontsWithInfo {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		// Indicates that this is a reply.
		buf.put_u8(1);
		// Indicates that this is the last `ListFontsWithInfo` reply.
		buf.put_u8(0);

		// The sequence number.
		buf.put_u16(self.sequence);

		// Length - the number of 4-byte units after the 32nd byte in this
		// reply.
		buf.put_u32(7);

		// 52 unused bytes.
		buf.put_bytes(0, 52);

		Ok(())
	}
}

impl ReadableWithContext for TerminateListFontsWithInfo {
	type Context = u16;

	fn read_with(buf: &mut impl Buf, sequence: &u16) -> ReadResult<Self> {
		// We skip the first 4 bytes because:
		// - the first, `1`, was required to know this is a reply
		// - the second was required to know this is the last reply for
		//   ListFontsWithInfo
		// - the third and fourth - the sequence - were required to know that this is a
		//   `ListFontsWithInfo` reply

		// Then we skip the length because we know what it is meant to be... should
		// probably verify that...
		buf.advance(4);

		// And then skip the 52 remaining unused bytes.
		buf.advance(52);

		Ok(Self {
			sequence: *sequence,
		})
	}
}
