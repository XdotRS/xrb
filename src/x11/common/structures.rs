// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::x11::common::values::{Char1b, Char2b, HostFamily};

pub type String8<'a> = &'a [Char1b];
pub type String16<'a> = &'a [Char2b];

/// A rectangle represented by its coordinates and dimensions.
///
/// The coordinates are those of the upper-left corner of the rectangle. The
/// units for the coordinates and dimensions are not specified.
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

/// A geometric arc, represented by its coordinates, dimensions, and start and end angles.
pub struct Arc {
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

pub struct Host {
	/// The protocol family of the host, e.g. [InternetV6](HostFamily::InternetV6).
	pub family: HostFamily,
	/// The address of the host in question.
	pub address: String,
}
