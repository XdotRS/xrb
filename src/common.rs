// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate self as xrb;

use derive_more::{From, Into};

pub use atom::Atom;
pub use mask::*;
pub use res_id::*;
pub use wrapper::*;
use xrbk::{Buf, ConstantX11Size, ReadError, ReadResult, ReadableWithContext, Wrap};
use xrbk_macro::{derive_xrb, new, unwrap, ConstantX11Size, Readable, Wrap, Writable, X11Size};

use crate::unit::Px;

pub mod atom;
pub mod set;
pub mod visual;

mod mask;
mod res_id;
mod wrapper;

/// Whether something is enabled or disabled.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
pub enum Toggle {
	/// The thing is disabled.
	Disabled,
	/// The thing is enabled.
	Enabled,
}

/// Whether something is enabled, disabled, or the default is chosen.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
pub enum ToggleOrDefault {
	/// The thing is disabled.
	Disabled,
	/// The thing is enabled.
	Enabled,

	/// The default choice (out of [`Disabled`] or [`Enabled`]) is chosen.
	///
	/// Which is the default depends on what this `ToggleOrDefault` is applied
	/// to.
	Default,
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
	/// A [window]'s class; whether it has a visual output form.
	///
	/// [window]: Window
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
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
pub enum MaintainContents {
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

/// Whether a grab causes a freeze in [event] processing.
///
/// [event]: crate::message::Event
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
pub enum FreezeMode {
	/// [Event] processing is not frozen.
	///
	/// [Event]: crate::message::Event
	#[doc(alias = "Asynchronous")]
	Unfrozen,

	/// [Event] processing is frozen.
	///
	/// [Event]: crate::message::Event
	#[doc(alias = "Synchronous")]
	Frozen,
}

/// The status of an attempted grab.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
pub enum GrabStatus {
	/// The grab was successful.
	Success,

	/// Another client already had a grab.
	AlreadyGrabbed,
	/// Another client already had an active grab and had frozen [event]
	/// processing.
	///
	/// [event]: crate::message::Event
	Frozen,
	/// The given time was either earlier than the previous grab, or later than
	/// the X server's [current time].
	///
	/// [current time]: CurrentableTime::CurrentTime
	InvalidTime,
	/// The grabbed [window] or the [window] which the cursor was confined to is
	/// not viewable, or the [window] which the cursor was confined to is
	/// completely outside of the root [window].
	///
	/// [window]: Window
	NotViewable,
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

derive_xrb! {
	#[derive(
		Clone,
		Eq,
		PartialEq,
		Hash,
		Debug,
		From,
		Into,
		// XRBK traits
		X11Size,
		Readable,
		Writable,
	)]
	pub struct LengthString8 {
		#[allow(clippy::cast_possible_truncation)]
		let len: u8 = string => string.len() as u8,

		#[context(len => usize::from(*len))]
		string: String8,
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
pub struct Coords {
	#[allow(missing_docs)]
	pub x: Px<i16>,
	#[allow(missing_docs)]
	pub y: Px<i16>,
}

/// A rectangle with coordinates and dimensions.
#[derive(Clone, Eq, PartialEq, Hash, Debug, new, X11Size, ConstantX11Size, Readable, Writable)]
pub struct Rectangle {
	/// The x-coordinate of the upper left corner of the `Rectangle`.
	pub x: Px<i16>,
	/// The y-coordinate of the upper left corner of the `Rectangle`.
	pub y: Px<i16>,
	/// The width of the `Rectangle`.
	pub width: Px<u16>,
	/// The height of the `Rectangle`.
	pub height: Px<u16>,
}

/// Same as a [`Rectangle`], but with unsigned coordinates.
#[derive(Clone, Eq, PartialEq, Hash, Debug, new, X11Size, ConstantX11Size, Readable, Writable)]
pub struct Region {
	/// The x-coordinate of the upper left corner of the `Region`.
	pub x: Px<u16>,
	/// The y-coordinate of the upper left corner of the `Region`.
	pub y: Px<u16>,

	/// The width of the `Region`.
	pub width: Px<u16>,
	/// The height of the `Region`.
	pub height: Px<u16>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, new, X11Size, ConstantX11Size, Readable, Writable)]
pub struct Arc {
	pub x: Px<i16>,
	pub y: Px<i16>,
	pub width: Px<u16>,
	pub height: Px<u16>,

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
