// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xrb_proc_macros::{ByteSize, StaticByteSize};

mod string;
mod masks;
mod wrappers;
mod id;

pub use string::*;
pub use masks::*;
pub use wrappers::*;
pub use id::*;

pub use id::atoms::Atom;

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
