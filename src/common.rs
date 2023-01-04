// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate self as xrb;

use bytes::Buf;
use cornflakes::{ContextualReadable, ReadResult};
use derive_more::{From, Into};
use xrbk_macro::{derive_xrb, DataSize, Readable, StaticDataSize, Writable};

pub mod atom;
pub mod mask;
mod wrapper;

pub use atom::Atom;
pub use wrapper::*;

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
pub struct Color(
	/// Red.
	u32,
	/// Green.
	u32,
	/// Blue.
	u32,
);

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
pub struct Drawable(pub(crate) u32);

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
pub struct Window(pub(crate) u32);

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
pub struct Pixmap(pub(crate) u32);

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
pub struct Cursor(pub(crate) u32);

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
pub struct Fontable(pub(crate) u32);

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
pub struct Font(pub(crate) u32);

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
pub struct GraphicsContext(pub(crate) u32);

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
pub struct Colormap(pub(crate) u32);

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
pub struct Timestamp(pub(crate) u32);

impl Timestamp {
	pub const fn new(timestamp: u32) -> Self {
		Self(timestamp)
	}

	pub const fn unwrap(self) -> u32 {
		self.0
	}
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
pub struct VisualId(pub(crate) u32);

impl VisualId {
	pub const fn new(id: u32) -> Self {
		Self(id)
	}

	pub const fn unwrap(self) -> u32 {
		self.0
	}
}

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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum BackingStores {
	Never,
	WhenMapped,
	Always,
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
pub struct Keysym(pub(crate) u32);

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
pub struct Keycode(pub(crate) u8);

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
pub struct Button(pub(crate) u8);

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
pub struct Char8(pub(crate) u8);

#[derive(Clone, Eq, PartialEq, Hash, Debug, From, Into, DataSize, Writable)]
pub struct String8(Vec<Char8>);

impl String8 {
	#[must_use]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl ContextualReadable for String8 {
	type Context = usize;

	fn read_with(reader: &mut impl Buf, length: &usize) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(<Vec<Char8>>::read_with(reader, length)?))
	}
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
pub struct Char16(pub(crate) u8, pub(crate) u8);

#[derive(Clone, Eq, PartialEq, Hash, Debug, From, Into, DataSize, Writable)]
pub struct String16(Vec<Char16>);

impl String16 {
	#[must_use]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl ContextualReadable for String16 {
	type Context = usize;

	fn read_with(reader: &mut impl Buf, length: &usize) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(<Vec<Char16>>::read_with(reader, length)?))
	}
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
pub struct Point {
	pub x: i16,
	pub y: i16,
}

/// A rectangle with coordinates and dimensions.
#[derive(Clone, Eq, PartialEq, Hash, Debug, DataSize, StaticDataSize, Readable, Writable)]
pub struct Rectangle {
	/// The `x` coordinate of the upper left corner of the `Rectangle`.
	pub x: i16,
	/// The `y` coordinate of the upper left corner of the `Rectangle`.
	pub y: i16,
	/// The `width` of the `Rectangle`.
	pub width: u16,
	/// The `height` of the `Rectangle`.
	pub height: u16,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, DataSize, StaticDataSize, Readable, Writable)]
pub struct Arc {
	pub x: i16,
	pub y: i16,
	pub width: u16,
	pub height: u16,

	/// Specifies the start of the `Arc`.
	///
	/// The angle is measured in degrees scaled by 64. Positive indicates
	/// counterclockwise motion and negative indicates clockwise motion.
	/// The angle is measured relative to the three-o'clock position from
	/// the center of the rectangle.
	pub start_angle: i16,
	/// Specifies the extent of the `Arc` relative to the `start_angle`.
	///
	/// The angle is measured in degrees scaled by 64. If greater than 360
	/// degrees, this angle is truncated to 360 degrees.
	pub end_angle: i16,
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
