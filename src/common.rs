// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate self as xrb;

use bytes::Buf;
use derive_more::{From, Into};
use xrbk::{ConstantX11Size, ReadError, ReadResult, ReadableWithContext, Wrap};
use xrbk_macro::{derive_xrb, new, unwrap, ConstantX11Size, Readable, Wrap, Writable, X11Size};

pub mod atom;
pub mod mask;
pub mod res_id;
pub mod wrapper;

/// A color comprised of red, green, and blue color channels.
///
/// Each of the channels is a `u16` value, where `0` is the minimum intensity
/// and `65535` is the maximum intensity. The X server scales the values to
/// match the display hardware.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
)]
pub struct Color(
	/// Red.
	u16,
	/// Green.
	u16,
	/// Blue.
	u16,
);

/// An error returned when a value meant to be interpreted as a hex color code
/// is greater than `0xffffff`.
///
/// This is returned from [`Color::from_hex`].
pub struct ColorValueTooHigh;

impl Color {
	/// Converts a hex color code to a `Color`.
	///
	/// # Errors
	/// If the provided `u32` color value is greater than `0x_ffffff`, a
	/// [`ColorValueToHigh`] error will be generated.
	#[allow(clippy::unreadable_literal)]
	pub fn from_hex(hex: u32) -> Result<Self, ColorValueTooHigh> {
		if hex > 0x00ff_ffff {
			return Err(ColorValueTooHigh);
		}

		// Red color channel gets moved over 16 bits so that it is represented as one
		// byte.
		let red = ((hex & 0x00ff_0000) >> 16) as u16;
		// Green color channel gets moved over 8 bits so that is is represented as one
		// byte.
		let green = ((hex & 0x0000_ff00) >> 8) as u16;
		// Blue color channel is already represented as one byte.
		let blue = (hex & 0x0000_00ff) as u16;

		// Since the color channels for `Color` are actually `u16` values, not `u8`,
		// they are shifted one byte to the left.
		Ok(Self(red << 8, green << 8, blue << 8))
	}

	/// Converts a `Color` to a hex color code.
	///
	/// # Lossy
	/// This function is lossy: a `Color` is made up of three `u16` values,
	/// while a hex color code represents three `u8` values. The least
	/// significant byte of each color channel will be lost during conversion.
	#[must_use]
	pub fn to_hex(&self) -> u32 {
		let Self(red, green, blue) = self;

		// The color channels are all shifted 8 bits to the right to scale their values
		// from `u16` to `u8`. They are then cast to `u32` so they can be combined into
		// one `u32` value.
		let (red, green, blue) = (
			u32::from(red >> 8),
			u32::from(green >> 8),
			u32::from(blue >> 8),
		);

		// We can now union the channels into one `u32` value - red and green are
		// shifted over into `0xff0000` and `0x00ff00` positions respectively to do so.
		(red << 16) | (green << 8) | blue
	}
}

impl From<(u32, u32, u32)> for Color {
	#[allow(
		clippy::cast_possible_truncation,
		reason = "for purposes of the X11 protocol, a color represented by (u32, u32, u32) is \
		          just a (u16, u16, u16) color with two unused bytes for each channel - \
		          truncation is intended behavior"
	)]
	fn from((red, green, blue): (u32, u32, u32)) -> Self {
		Self(red as u16, green as u16, blue as u16)
	}
}

impl From<Color> for (u32, u32, u32) {
	fn from(Color(red, green, blue): Color) -> Self {
		(u32::from(red), u32::from(green), u32::from(blue))
	}
}

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
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
)]
pub struct Timestamp(pub(crate) u32);

/// The ID of a [`VisualType`].
///
/// [`VisualType`]: crate::connection::VisualType
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
)]
pub struct VisualId(pub(crate) u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
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

// The `derive_xrb!` attribute here is used to write the discriminants as `u16`.
derive_xrb! {
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
	/// A [window]'s class; whether it has a visual output form.
	///
	/// [window]: Window
	pub enum WindowClass: u16 {
		/// A [window] that both receives input and has a visual output (i.e. what
		/// one would normally consider a window to be).
		///
		/// [window]: Window
		InputOutput = 1,
		/// A [window] that receives input but does not have a visual form.
		///
		/// [window]: Window
		InputOnly = 2,
	}

	impl ConstantX11Size for WindowClass {
		const X11_SIZE: usize = 2;
	}

	impl Wrap for WindowClass {
		type Integer = u16;
	}

	impl TryFrom<u16> for WindowClass {
		type Error = ReadError;

		fn try_from(val: u16) -> ReadResult<Self> {
			match val {
				discrim if discrim == 1 => Ok(Self::InputOutput),
				discrim if discrim == 2 => Ok(Self::InputOnly),

				other_discrim => Err(ReadError::UnrecognizedDiscriminant(other_discrim as usize)),
			}
		}
	}

	impl From<WindowClass> for u16 {
		fn from(class: WindowClass) -> Self {
			match class {
				WindowClass::InputOutput => 1,
				WindowClass::InputOnly => 2,
			}
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
pub enum BackingStores {
	Never,
	WhenMapped,
	Always,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
pub enum GrabMode {
	Normal,
	Grab,
	Ungrab,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
pub enum StackMode {
	Above,
	Below,
	TopIf,
	BottomIf,
	Opposite,
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
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
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
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
)]
pub struct Keycode(pub u8);

#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
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
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
)]
pub struct Char8(pub(crate) u8);

#[derive(Clone, Eq, PartialEq, Hash, Debug, From, Into, X11Size, Writable)]
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

impl ReadableWithContext for String8 {
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
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
)]
pub struct Char16(pub(crate) u8, pub(crate) u8);

#[derive(Clone, Eq, PartialEq, Hash, Debug, From, Into, X11Size, Writable)]
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

impl ReadableWithContext for String16 {
	type Context = usize;

	fn read_with(reader: &mut impl Buf, length: &usize) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(<Vec<Char16>>::read_with(reader, length)?))
	}
}

/// A 2D point with an `x`-coordinate and a `y`-coordinate.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
)]
pub struct Point {
	#[allow(missing_docs)]
	pub x: i16,
	#[allow(missing_docs)]
	pub y: i16,
}

/// A rectangle with coordinates and dimensions.
#[derive(Clone, Eq, PartialEq, Hash, Debug, new, X11Size, ConstantX11Size, Readable, Writable)]
pub struct Rectangle {
	/// The x-coordinate of the upper left corner of the `Rectangle`.
	pub x: i16,
	/// The y-coordinate of the upper left corner of the `Rectangle`.
	pub y: i16,
	/// The width of the `Rectangle`.
	pub width: u16,
	/// The height of the `Rectangle`.
	pub height: u16,
}

/// Same as a [`Rectangle`], but with unsigned coordinates.
#[derive(Clone, Eq, PartialEq, Hash, Debug, new, X11Size, ConstantX11Size, Readable, Writable)]
pub struct Region {
	/// The x-coordinate of the upper left corner of the `Region`.
	pub x: u16,
	/// The y-coordinate of the upper left corner of the `Region`.
	pub y: u16,

	/// The width of the `Region`.
	pub width: u16,
	/// The height of the `Region`.
	pub height: u16,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, new, X11Size, ConstantX11Size, Readable, Writable)]
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
pub enum HostFamily {
	Internet,
	Decnet,
	Chaos,
	ServerInterpreted = 5,
	InternetV6,
}

derive_xrb! {
	#[derive(Clone, Eq, PartialEq, Hash, Debug, new, X11Size, Readable, Writable)]
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
