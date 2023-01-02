// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use xrbk_macro::derive_xrb;

derive_xrb! {
	/// A unique ID corresponding to a string name.
	///
	/// `Atom`s are used to identify properties, types, and selections.
	pub struct Atom(u32);

	impl Atom {
		/// Creates a new `Atom`, wrapping the given `id`.
		#[must_use]
		pub const fn new(id: u32) -> Self {
			Self(id)
		}

		/// Unwraps the wrapped numerical `id`.
		#[must_use]
		pub const fn unwrap(self) -> u32 {
			self.0
		}
	}

	impl From<u32> for Atom {
		fn from(id: u32) -> Self {
			Self(id)
		}
	}

	impl From<Atom> for u32 {
		fn from(atom: Atom) -> Self {
			atom.0
		}
	}
}

pub const PRIMARY: Atom = Atom::new(1);
pub const SECONDARY: Atom = Atom::new(2);
pub const ARC: Atom = Atom::new(3);
pub const ATOM: Atom = Atom::new(4);
pub const BITMAP: Atom = Atom::new(5);
pub const CARDINAL: Atom = Atom::new(6);
pub const COLORMAP: Atom = Atom::new(7);
pub const CURSOR: Atom = Atom::new(8);
pub const CUT_BUFFER0: Atom = Atom::new(9);
pub const CUT_BUFFER1: Atom = Atom::new(10);
pub const CUT_BUFFER2: Atom = Atom::new(11);
pub const CUT_BUFFER3: Atom = Atom::new(12);
pub const CUT_BUFFER4: Atom = Atom::new(13);
pub const CUT_BUFFER5: Atom = Atom::new(14);
pub const CUT_BUFFER6: Atom = Atom::new(15);
pub const CUT_BUFFER7: Atom = Atom::new(16);
pub const DRAWABLE: Atom = Atom::new(17);
pub const FONT: Atom = Atom::new(18);
pub const INTEGER: Atom = Atom::new(19);
pub const PIXMAP: Atom = Atom::new(20);
pub const POINT: Atom = Atom::new(21);
pub const RECTANGLE: Atom = Atom::new(22);
pub const RESOURCE_MANAGER: Atom = Atom::new(23);
pub const RGB_COLOR_MAP: Atom = Atom::new(24);
pub const RGB_BEST_MAP: Atom = Atom::new(25);
pub const RGB_BLUE_MAP: Atom = Atom::new(26);
pub const RGB_DEFAULT_MAP: Atom = Atom::new(27);
pub const RGB_GRAY_MAP: Atom = Atom::new(28);
pub const RGB_GREEN_MAP: Atom = Atom::new(29);
pub const RGB_RED_MAP: Atom = Atom::new(30);
pub const STRING: Atom = Atom::new(31);
pub const VISUALID: Atom = Atom::new(32);
pub const WINDOW: Atom = Atom::new(33);
pub const WM_COMMAND: Atom = Atom::new(34);
pub const WM_HINTS: Atom = Atom::new(35);
pub const WM_CLIENT_MACHINE: Atom = Atom::new(36);
pub const WM_ICON_NAME: Atom = Atom::new(37);
pub const WM_ICON_SIZE: Atom = Atom::new(38);
pub const WM_NAME: Atom = Atom::new(39);
pub const WM_NORMAL_HINTS: Atom = Atom::new(40);
pub const WM_SIZE_HINTS: Atom = Atom::new(41);
pub const WM_ZOOM_HINTS: Atom = Atom::new(42);
pub const MIN_SPACE: Atom = Atom::new(43);
pub const NORM_SPACE: Atom = Atom::new(44);
pub const MAX_SPACE: Atom = Atom::new(45);
pub const END_SPACE: Atom = Atom::new(46);
pub const SUPERSCRIPT_X: Atom = Atom::new(47);
pub const SUPERSCRIPT_Y: Atom = Atom::new(48);
pub const SUBSCRIPT_X: Atom = Atom::new(49);
pub const SUBSCRIPT_Y: Atom = Atom::new(50);
pub const UNDERLINE_POSITION: Atom = Atom::new(51);
pub const UNDERLINE_THICKNESS: Atom = Atom::new(52);
pub const STRIKEOUT_ASCENT: Atom = Atom::new(53);
pub const STRIKEOUT_DESCENT: Atom = Atom::new(54);
pub const ITALIC_ANGLE: Atom = Atom::new(55);
pub const X_HEIGHT: Atom = Atom::new(56);
pub const QUAD_WIDTH: Atom = Atom::new(57);
pub const WEIGHT: Atom = Atom::new(58);
pub const POINT_SIZE: Atom = Atom::new(59);
pub const RESOLUTION: Atom = Atom::new(60);
pub const COPYRIGHT: Atom = Atom::new(61);
pub const NOTICE: Atom = Atom::new(62);
pub const FONT_NAME: Atom = Atom::new(63);
pub const FAMILY_NAME: Atom = Atom::new(64);
pub const FULL_NAME: Atom = Atom::new(65);
pub const CAP_HEIGHT: Atom = Atom::new(66);
pub const WM_CLASS: Atom = Atom::new(67);
pub const WM_TRANSIENT_FOR: Atom = Atom::new(68);
