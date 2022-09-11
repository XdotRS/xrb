// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xrb_proc_macros::{ByteSize, StaticByteSize};

/// A unique ID corresponding to a defined string name.
///
/// Atoms exist to provide a fixed-length representation of common strings. They
/// are used to identify properties, types, and selection.
///
/// An [`InternAtom`] request can be sent to the X server to get or create a
/// corresponding `Atom` for a given string of text.
///
/// # Examples
/// [`WM_NAME`] is an `Atom` representing a property used for a window's title.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub struct Atom {
	/// The ID for the `Atom`.
	pub id: u32,
}

impl Atom {
	/// Creates a new [`Atom`] with the given ID.
	#[must_use] pub const fn new(id: u32) -> Self {
		Self { id }
	}
}

/// An [`Atom`] representing the string "PRIMARY".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const PRIMARY: Atom = Atom::new(1);
/// An [`Atom`] representing the string "SECONDARY".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const SECONDARY: Atom = Atom::new(2);
/// An [`Atom`] representing the string "ARC".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const ARC: Atom = Atom::new(3);
/// An [`Atom`] representing the string "ATOM".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const ATOM: Atom = Atom::new(4);
/// An [`Atom`] representing the string "BITMAP".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const BITMAP: Atom = Atom::new(5);
/// An [`Atom`] representing the string "CARDINAL".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const CARDINAL: Atom = Atom::new(6);
/// An [`Atom`] representing the string "COLORMAP".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const COLORMAP: Atom = Atom::new(7);
/// An [`Atom`] representing the string "CURSOR".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const CURSOR: Atom = Atom::new(8);
/// An [`Atom`] representing the string "CUR_BUFFER0".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const CUT_BUFFER0: Atom = Atom::new(9);
/// An [`Atom`] representing the string "CUT_BUFFER1".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const CUT_BUFFER1: Atom = Atom::new(10);
/// An [`Atom`] representing the string "CUT_BUFFER2".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const CUT_BUFFER2: Atom = Atom::new(11);
/// An [`Atom`] representing the string "CUT_BUFFER3".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const CUT_BUFFER3: Atom = Atom::new(12);
/// An [`Atom`] representing the string "CUT_BUFFER4".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const CUT_BUFFER4: Atom = Atom::new(13);
/// An [`Atom`] representing the string "CUT_BUFFER5".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const CUT_BUFFER5: Atom = Atom::new(14);
/// An [`Atom`] representing the string "CUT_BUFFER6".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const CUT_BUFFER6: Atom = Atom::new(15);
/// An [`Atom`] representing the string "CUT_BUFFER7".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const CUT_BUFFER7: Atom = Atom::new(16);
/// An [`Atom`] representing the string "DRAWABLE".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const DRAWABLE: Atom = Atom::new(17);
/// An [`Atom`] representing the string "FONT".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const FONT: Atom = Atom::new(18);
/// An [`Atom`] representing the string "INTEGER".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const INTEGER: Atom = Atom::new(19);
/// An [`Atom`] representing the string "PIXMAP".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const PIXMAP: Atom = Atom::new(20);
/// An [`Atom`] representing the string "POINT".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const POINT: Atom = Atom::new(21);
/// An [`Atom`] representing the string "RECTANGLE".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const RECTANGLE: Atom = Atom::new(22);
/// An [`Atom`] representing the string "RESOURCE_MANAGER".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const RESOURCE_MANAGER: Atom = Atom::new(23);
/// An [`Atom`] representing the string "RGB_COLOR_MAP".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const RGB_COLOR_MAP: Atom = Atom::new(24);
/// An [`Atom`] representing the string "RGB_BEST_MAP".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const RGB_BEST_MAP: Atom = Atom::new(25);
/// An [`Atom`] representing the string "RGB_BLUE_MAP".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const RGB_BLUE_MAP: Atom = Atom::new(26);
/// An [`Atom`] representing the string "RGB_DEFAULT".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const RGB_DEFAULT_MAP: Atom = Atom::new(27);
/// An [`Atom`] representing the string "RGB_GRAY_MAP".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const RGB_GRAY_MAP: Atom = Atom::new(28);
/// An [`Atom`] representing the string "RGB_GREEN_MAP".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const RGB_GREEN_MAP: Atom = Atom::new(29);
/// An [`Atom`] representing the string "RGB_RED_MAP".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const RGB_RED_MAP: Atom = Atom::new(30);
/// An [`Atom`] representing the string "STRING".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const STRING: Atom = Atom::new(31);
/// An [`Atom`] representing the string "VISUALID".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const VISUALID: Atom = Atom::new(32);
/// An [`Atom`] representing the string "WINDOW".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const WINDOW: Atom = Atom::new(33);
/// An [`Atom`] representing the string "WM_COMMAND".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const WM_COMMAND: Atom = Atom::new(34);
/// An [`Atom`] representing the string "WM_HINTS".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const WM_HINTS: Atom = Atom::new(35);
/// An [`Atom`] representing the string "WM_CLIENT_MACHINE".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const WM_CLIENT_MACHINE: Atom = Atom::new(36);
/// An [`Atom`] representing the string "WM_ICON_NAME".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const WM_ICON_NAME: Atom = Atom::new(37);
/// An [`Atom`] representing the string "WM_ICON_SIZE".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const WM_ICON_SIZE: Atom = Atom::new(38);
/// An [`Atom`] representing the string "WM_NAME".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const WM_NAME: Atom = Atom::new(39);
/// An [`Atom`] representing the string "WM_NORMAL_HINTS".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const WM_NORMAL_HINTS: Atom = Atom::new(40);
/// An [`Atom`] representing the string "WM_SIZE_HINTS".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const WM_SIZE_HINTS: Atom = Atom::new(41);
/// An [`Atom`] representing the string "WM_ZOOM_HINTS".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const WM_ZOOM_HINTS: Atom = Atom::new(42);
/// An [`Atom`] representing the string "MIN_SPACE".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const MIN_SPACE: Atom = Atom::new(43);
/// An [`Atom`] representing the string "NORM_SPACE".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const NORM_SPACE: Atom = Atom::new(44);
/// An [`Atom`] representing the string "MAX_SPACE".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const MAX_SPACE: Atom = Atom::new(45);
/// An [`Atom`] representing the string "END_SPACE".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const END_SPACE: Atom = Atom::new(46);
/// An [`Atom`] representing the string "SUPERSCRIPT_X".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const SUPERSCRIPT_X: Atom = Atom::new(47);
/// An [`Atom`] representing the string "SUPERSCRIPT_Y".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const SUPERSCRIPT_Y: Atom = Atom::new(48);
/// An [`Atom`] representing the string "SUBSCRIPT_X".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const SUBSCRIPT_X: Atom = Atom::new(49);
/// An [`Atom`] representing the string "SUBSCRIPT_Y".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const SUBSCRIPT_Y: Atom = Atom::new(50);
/// An [`Atom`] representing the string "UNDERLINE_POSITION".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const UNDERLINE_POSITION: Atom = Atom::new(51);
/// An [`Atom`] representing the string "UNDERLINE_THICKNESS".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const UNDERLINE_THICKNESS: Atom = Atom::new(52);
/// An [`Atom`] representing the string "STRIKEOUT_ASCENT".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const STRIKEOUT_ASCENT: Atom = Atom::new(53);
/// An [`Atom`] representing the string "STRIKEOUT_DESCENT".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const STRIKEOUT_DESCENT: Atom = Atom::new(54);
/// An [`Atom`] representing the string "ITALIC_ANGLE".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const ITALIC_ANGLE: Atom = Atom::new(55);
/// An [`Atom`] representing the string "X_HEIGHT".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const X_HEIGHT: Atom = Atom::new(56);
/// An [`Atom`] representing the string "QUAD_WIDTH".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const QUAD_WIDTH: Atom = Atom::new(57);
/// An [`Atom`] representing the string "WEIGHT".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const WEIGHT: Atom = Atom::new(58);
/// An [`Atom`] representing the string "POINT_SIZE".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const POINT_SIZE: Atom = Atom::new(59);
/// An [`Atom`] representing the string "RESOLUTION".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const RESOLUTION: Atom = Atom::new(60);
/// An [`Atom`] representing the string "COPYRIGHT".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const COPYRIGHT: Atom = Atom::new(61);
/// An [`Atom`] representing the string "NOTICE".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const NOTICE: Atom = Atom::new(62);
/// An [`Atom`] representing the string "FONT_NAME".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const FONT_NAME: Atom = Atom::new(63);
/// An [`Atom`] representing the string "FAMILY_NAME".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const FAMILY_NAME: Atom = Atom::new(64);
/// An [`Atom`] representing the string "FULL_NAME".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const FULL_NAME: Atom = Atom::new(65);
/// An [`Atom`] representing the string "CAP_HEIGHT".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const CAP_HEIGHT: Atom = Atom::new(66);
/// An [`Atom`] representing the string "WM_CLASS".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const WM_CLASS: Atom = Atom::new(67);
/// An [`Atom`] representing the string "WM_TRANSIENT_FOR".
///
/// This atom is predefined in the X protocol; that is, it is not defined per
/// connection and is always known to be the same ID.
pub const WM_TRANSIENT_FOR: Atom = Atom::new(68);
