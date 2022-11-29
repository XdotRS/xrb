// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate self as xrb;

mod masks;

pub use masks::*;

use xrbk_macro::define;

define! {
	/// A resource ID referring to a particular window resource.
	pub struct Window(u32);

	/// A resource ID referring to a particular pixmap resource.
	pub struct Pixmap(u32);

	/// A resource ID referring to a particular cursor resource.
	pub struct Cursor(u32);

	/// A resource ID referring to a particular font resource.
	pub struct Font(u32);

	/// A resource ID referring to a particular graphics context resource.
	///
	/// Information relating to graphics output is stored in a graphics
	/// context such as foreground pixel, background pixel, line width,
	/// clipping region, etc. A graphics context can only be used with
	/// [`Drawable`]s that have the same `root` and `depth` as the
	/// `GraphicsContext`.
	pub struct GraphicsContext(u32);

	/// A resource ID referring to a particular colormap resource.
	pub struct Colormap(u32);

	/// A unique ID corresponding to a string name.
	///
	/// `Atom`s are used to identify properties, types, and selections.
	pub struct Atom(u32);
	pub struct VisualId(u32);

	pub enum BitGravity {
		Forget,
		Static,
		NorthWest,
		North,
		NorthEast,
		West,
		Center,
		East,
		SouthWest,
		South,
		SouthEast,
	}

	pub enum WinGravity {
		Unmap,
		Static,
		NorthWest,
		North,
		NorthEast,
		West,
		Center,
		East,
		SouthWest,
		South,
		SouthEast,
	}

	pub struct Keysym(u32);
	pub struct Keycode(u8);
	pub struct Button(u8);

	pub struct Char8(u8);
	pub struct Char16(u8, u8);

	pub struct Point {
		x: i16,
		y: i16,
	}

	/// A rectangle with coordinates and dimensions.
	pub struct Rectangle {
		/// The `x` coordinate of the upper left corner of the `Rectangle`.
		x: i16,
		/// The `y` coordinate of the upper left corner of the `Rectangle`.
		y: i16,
		/// The `width` of the `Rectangle`.
		width: u16,
		/// The `height` of the `Rectangle`.
		height: u16,
	}

	pub struct Arc {
		x: i16,
		y: i16,
		width: u16,
		height: u16,

		/// Specifies the start of the `Arc`.
		///
		/// The angle is measured in degrees scaled by 64. Positive indicates
		/// counterclockwise motion and negative indicates clockwise motion.
		/// The angle is measured relative to the three-o'clock position from
		/// the center of the rectangle.
		start_angle: i16,
		/// Specifies the extent of the `Arc` relative to the `start_angle`.
		///
		/// The angle is measured in degrees scaled by 64. If greater than 360
		/// degrees, this angle is truncated to 360 degrees.
		end_angle: i16,
	}
}

pub trait DrawableId {}
pub trait FontableId {}

pub type Drawable = Box<dyn DrawableId>;
pub type Fontable = Box<dyn FontableId>;

impl DrawableId for Window {}
impl DrawableId for Pixmap {}

impl FontableId for Font {}
impl FontableId for GraphicsContext {}

fn _assert_object_safety(_drawable: &dyn DrawableId, _fontable: &dyn FontableId) {}
