// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate self as xrb;

use derive_more::{From, Into};
use xrbk_macro::{derive_xrb, DataSize, Readable, StaticDataSize, Writable};

pub mod atom;
pub mod mask;

pub use atom::Atom;

/// A resource ID referring to either a [`Window`] or a [`Pixmap`].
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct Drawable(u32);

impl From<Window> for Drawable {
	fn from(window: Window) -> Self {
		let Window(id) = window;
		Self(id)
	}
}

impl From<Pixmap> for Drawable {
	fn from(pixmap: Pixmap) -> Self {
		let Pixmap(id) = pixmap;
		Self(id)
	}
}

/// A resource ID referring to a particular window resource.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct Window(u32);

impl From<Drawable> for Window {
	fn from(drawable: Drawable) -> Self {
		let Drawable(id) = drawable;
		Self(id)
	}
}

/// A resource ID referring to a particular pixmap resource.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct Pixmap(u32);

impl From<Drawable> for Pixmap {
	fn from(drawable: Drawable) -> Self {
		let Drawable(id) = drawable;
		Self(id)
	}
}

/// A resource ID referring to a particular cursor resource.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct Cursor(u32);

/// A resource ID referring to either a [`Font`] or a [`GraphicsContext`].
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct Fontable(u32);

impl From<Font> for Fontable {
	fn from(font: Font) -> Self {
		let Font(id) = font;
		Self(id)
	}
}

impl From<GraphicsContext> for Fontable {
	fn from(context: GraphicsContext) -> Self {
		let GraphicsContext(id) = context;
		Self(id)
	}
}

/// A resource ID referring to a particular font resource.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct Font(u32);

impl From<Fontable> for Font {
	fn from(fontable: Fontable) -> Self {
		let Fontable(id) = fontable;
		Self(id)
	}
}

/// A resource ID referring to a particular graphics context resource.
///
/// Information relating to graphics output is stored in a graphics
/// context such as foreground pixel, background pixel, line width,
/// clipping region, etc. A graphics context can only be used with
/// [`Drawable`]s that have the same `root` and `depth` as the
/// `GraphicsContext`.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct GraphicsContext(u32);

impl From<Fontable> for GraphicsContext {
	fn from(fontable: Fontable) -> Self {
		let Fontable(id) = fontable;
		Self(id)
	}
}

/// A resource ID referring to a particular colormap resource.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct Colormap(u32);

/// Represents a particular time, expressed in milliseconds.
///
/// Timestamps are typically the time since the last server reset. After
/// approximately 49.7 days, the time will wrap around back to 0.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct Timestamp(u32);

#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct VisualId(u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
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

#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct Keysym(u32);

#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct Keycode(u8);

#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct Button(u8);

#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct Char8(u8);

#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct Char16(u8, u8);

#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// XRBK traits
	DataSize,
	StaticDataSize,
	Readable,
	Writable,
)]
pub struct Point {
	x: i16,
	y: i16,
}

/// A rectangle with coordinates and dimensions.
#[derive(Clone, Eq, PartialEq, Hash, Debug, DataSize, StaticDataSize, Readable, Writable)]
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

#[derive(Clone, Eq, PartialEq, Hash, Debug, DataSize, StaticDataSize, Readable, Writable)]
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum HostFamily {
	Internet,
	Decnet,
	Chaos,
	ServerInterpreted = 5,
	InternetV6,
}

derive_xrb! {
	#[derive(Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Host {
		pub family: HostFamily,
		_,

		#[allow(clippy::cast_possible_truncation)]
		let address_len: u16 = address => address.len() as u16,

		#[context(address_len => *address_len as usize)]
		pub address: Vec<u8>,
		[_; ..],
	}
}
