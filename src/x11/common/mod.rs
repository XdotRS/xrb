// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xrb_proc_macros::{ByteSize, StaticByteSize};

mod id;
mod masks;
mod string;
mod values;
mod wrappers;

pub use id::*;
pub use masks::*;
pub use string::*;
pub use values::*;
pub use wrappers::*;

pub use id::atoms::Atom;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum Status {
	Success,
	Busy,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum ScreenSaverMode {
	Reset,
	Activate,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum CloseDownMode {
	Destroy,
	RetainPermanent,
	RetainTemporary,
}

/// The 'type' of 'best size' being queried in a [`QueryBestSize`] request.
///
/// [`QueryBestSize`]: super::QueryBestSize
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum QueryBestSizeClass {
	Cursor,
	Tile,
	Stipple,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum ColormapAlloc {
	None,
	All,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum Shape {
	Complex,
	Nonconvex,
	Convex,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum CoordinateMode {
	Origin,
	Previous,
}

impl Default for CoordinateMode {
	fn default() -> Self {
		Self::Origin
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize, Default)]
pub struct Segment {
	pub start: (i16, i16),
	pub end: (i16, i16),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum Ordering {
	Unsorted,
	Ysorted,
	YxSorted,
	YxBanded,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum DrawDirection {
	LeftToRight,
	RightToLeft,
}

impl Default for DrawDirection {
	fn default() -> Self {
		Self::LeftToRight
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub struct FontProperty {
	pub name: Atom,
	pub value: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub struct CharInfo {
	pub left_side_bearing: i16,
	pub right_side_bearing: i16,
	pub character_width: i16,
	pub ascent: i16,
	pub descent: i16,
	pub attributes: u16,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum AllowEventsMode {
	AsyncPointer,
	SyncPointer,
	ReplayPointer,
	AsyncKeyboard,
	SyncKeyboard,
	ReplayKeyboard,
	AsyncBoth,
	SyncBoth,
}

impl Default for AllowEventsMode {
	fn default() -> Self {
		Self::AsyncBoth
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum HostFamilyA {
	Internet,
	Decnet,
	Chaos,
}

impl Default for HostFamilyA {
	fn default() -> Self {
		Self::Internet
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum GrabMode {
	Synchronous,
	Asynchronous,
}

impl Default for GrabMode {
	fn default() -> Self {
		Self::Asynchronous
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum GrabStatus {
	Success,
	AlreadyGrabbed,
	InvalidTime,
	NotViewable,
	Frozen,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum CirculateDirection {
	RaiseLowest,
	RaiseHighest,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum EditMode {
	Insert,
	Delete,
}

impl Default for EditMode {
	fn default() -> Self {
		Self::Insert
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum Format {
	XyPixmap = 1,
	Zpixmap = 2,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum StackMode {
	Above,
	Below,
	TopIf,
	Bottomif,
	Opposite,
}

impl Default for StackMode {
	fn default() -> Self {
		Self::Above
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum MapState {
	Unmapped,
	Unviewable,
	Viewable,
}

impl Default for MapState {
	fn default() -> Self {
		Self::Unmapped
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum BackingStore {
	NotUseful,
	WhenMapped,
	Always,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum WindowClass {
	InputOutput = 1,
	InputOnly = 2,
}

impl Default for WindowClass {
	fn default() -> Self {
		Self::InputOutput
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum BitGravity {
	Forget,
	NorthWest,
	North,
	NorthEast,
	West,
	Center,
	East,
	SouthWest,
	South,
	SouthEast,
	Static,
}

impl Default for BitGravity {
	fn default() -> Self {
		Self::NorthWest
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum WinGravity {
	Unmap,
	NorthWest,
	North,
	NorthEast,
	West,
	Center,
	East,
	SouthWest,
	South,
	SouthEast,
	Static,
}

impl Default for WinGravity {
	fn default() -> Self {
		Self::NorthWest
	}
}

/// A rectangle with coordinates and dimensions.
///
/// The coordinates are those of the upper-left corner of the rectangle. The
/// units for the coordinates and dimensions are not specified.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub struct Rectangle {
	/// X-coordinate of the upper-left corner of the rectangle.
	pub x: i16,
	/// Y-coordinate of the upper-left corner of the rectangle.
	pub y: i16,
	/// Width of the rectangle.
	pub width: u16,
	/// Height of the rectangle.
	pub height: u16,
}

impl Default for Rectangle {
	fn default() -> Self {
		Self {
			x: 0,
			y: 0,
			width: 1,
			height: 1,
		}
	}
}

/// An arc (the geometry kind) with coordinates, dimensions, and angles.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub struct GeomArc {
	/// X-coordinate of the arc.
	pub x: i16,
	/// Y-coordinate of the arc.
	pub y: i16,
	/// Width of the arc.
	pub width: u16,
	/// Height of the arc.
	pub height: u16,
	/// The start angle of the arc.
	pub start: i16,
	/// The end angle of the arc.
	pub end: i16,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, ByteSize)]
pub struct Host {
	/// The protocol family of the host, e.g. [InternetV6](HostFamily::InternetV6).
	pub family: HostFamily,
	/// The address of the host in question.
	pub address: String8,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub enum HostFamily {
	Internet,
	Decnet,
	Chaos,
	ServerInterpreted,
	InternetV6,
}

impl Default for HostFamily {
	fn default() -> Self {
		Self::ServerInterpreted
	}
}

/// An identifier representing the concept of all possible keys.
///
/// The difference between a `Keysym` and a [`Keycode`] is that the `Keysym`
/// universally represents the concept of any particular key, while the
/// [`Keycode`] refers to the specific position of a key on the user's keyboard,
/// as interpreted by the device drivers.
///
/// For example, the concept of an `F13` key always exists as a `Keysym`, even
/// if there is no such key represented by a [`Keycode`] for the actual keyboard
/// currently in use.
pub type Keysym = u32;
/// An identifier for the location of a key as interepreted by OS drivers.
///
/// The difference between a `Keycode` and a [`Keysym`] is that the `Keycode`
/// refers to the specific position of a key on the user's keyboard, as
/// interpreted by the device drivers, while the [`Keysym`] universally
/// represents the concept of any particular key.
///
/// For example, the concept of an `F13` key always exists as a [`Keysym`], even
/// if there is no such key represented by a `Keycode` for the actual keyboard
/// currently in use.
pub type Keycode = u8;
/// A button on the mouse.
///
/// For example, button 1 is the primary mouse button, commonly found on the
/// left of a mouse.
pub type Button = u8;

pub type Timestamp = u32;

/// Specifies how to pick the window to revert focus to when the current
/// window is unmapped.
//
// Would this be better as a `Parent` unit struct and a type alias for
// `Option<InputFocus<Parent>>`? Did it like this so that you don't have to do:
// ```
// Some(InputFocus::Specific(Parent))
// ```
// and can instead do:
// ```
// RevertTo::Parent
// ```
pub enum RevertTo {
	/// Revert the focus to none at all.
	///
	/// It is recommended to avoid setting this: it might lead to behavior you
	/// don't expect. Only set this as the [`RevertTo`] if you know the
	/// potential consequences.
	None,
	// TODO: What is this?
	PointerRoot,
	/// Revert the focus to the parent of the window.
	///
	/// This is the recommended [`RevertTo`] option for most cases.
	Parent,
}

impl Default for RevertTo {
	fn default() -> Self {
		Self::Parent
	}
}

/// The destination for an [`Event`] in a [`SendEvent`] request.
///
/// This is the window that the event will be sent to.
pub enum Destination {
	/// The [`Window`] the pointer is currently within.
	PointerWindow,
	/// The [`Window`] that currently has input focus.
	InputFocus,
	/// A specific [`Window`].
	Specific(Window),
}

impl Default for Destination {
	fn default() -> Self {
		Self::InputFocus
	}
}
