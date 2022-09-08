// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xrb_proc_macros::{ByteSize, StaticByteSize};

mod string;
pub use string::*;

pub mod masks;
pub mod structures;
pub mod values;

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
