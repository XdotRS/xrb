// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate self as xrb;

use derive_more::{From, Into};
use xrbk::{Buf, ConstantX11Size, ReadError, ReadResult, ReadableWithContext, Wrap};
use xrbk_macro::{derive_xrb, new, unwrap, ConstantX11Size, Readable, Wrap, Writable, X11Size};

pub mod atom;

pub mod attribute;
pub mod mask;
pub mod res_id;
pub mod visual;
pub mod wrapper;

pub use atom::Atom;
pub use attribute::*;
pub use mask::*;
pub use res_id::*;
pub use visual::*;
pub use wrapper::*;

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
pub enum WindowGravity {
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
pub enum BackingStore {
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
