// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate self as xrb;

use array_init::array_init;
use derive_more::{From, Into};
use thiserror::Error;

pub use atom::Atom;
pub use mask::*;
pub use res_id::*;
pub use wrapper::*;

use xrbk::{
	pad,
	Buf,
	BufMut,
	ConstantX11Size,
	ReadError,
	ReadError::FailedConversion,
	ReadResult,
	ReadableWithContext,
	Wrap,
	Writable,
	WriteResult,
	X11Size,
};
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
	// `new` const fn
	new,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
)]
pub struct Keysym(pub(crate) u32);

impl Keysym {
	pub const NO_SYMBOL: Self = Self::new(0x0000_0000);
	pub const VOID_SYMBOL: Self = Self::new(0x00ff_ffff);

	/// Returns the raw contained keysym value.
	#[must_use]
	pub const fn unwrap(&self) -> u32 {
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
	// `new` const fn
	new,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
)]
pub struct Keycode(pub(crate) u8);

impl Keycode {
	/// Returns the contained `u8` keycode.
	#[must_use]
	pub const fn unwrap(&self) -> u8 {
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

impl From<u16> for Char16 {
	fn from(value: u16) -> Self {
		let [byte1, byte2] = value.to_be_bytes();

		Self::new(byte1, byte2)
	}
}

impl From<Char16> for u16 {
	fn from(char: Char16) -> Self {
		let (byte1, byte2) = char.unwrap();

		Self::from_be_bytes([byte1, byte2])
	}
}

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
	/// The x coordinate, measured in pixels.
	pub x: Px<i16>,
	/// The y coordinate, measured in pixels.
	pub y: Px<i16>,
}

/// 2D dimensions (width and height), measured in pixels.
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
pub struct Dimensions {
	/// The width, measured in pixels.
	pub width: Px<u16>,
	/// The height, measured in pixels.
	pub height: Px<u16>,
}

/// A rectangle with coordinates and dimensions.
#[derive(
	Copy, Clone, Eq, PartialEq, Hash, Debug, new, X11Size, ConstantX11Size, Readable, Writable,
)]
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

impl Rectangle {
	/// Returns the rectangle's `x` and `y` coordinates as [`Coords`].
	#[must_use]
	pub const fn as_coords(&self) -> Coords {
		Coords::new(self.x, self.y)
	}

	/// Returns the rectangle's `width` and `height` as [`Dimensions`].
	#[must_use]
	pub const fn as_dimensions(&self) -> Dimensions {
		Dimensions::new(self.width, self.height)
	}
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

/// A circular or elliptical arc.
#[derive(Clone, Eq, PartialEq, Hash, Debug, new, X11Size, ConstantX11Size, Readable, Writable)]
pub struct Arc {
	/// The [rectangle] which contains the arc.
	///
	/// The center of the arc is the center of this rectangle. If the arc were
	/// to form a full circle, it would touch this [rectangle] in four places:
	/// the left side, the top side, the right side, and the bottom side. It is
	/// fully contained within this [rectangle].
	///
	/// [rectangle]: Rectangle
	pub bounds: Rectangle,

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

/// The address family of a host.
///
/// This is used in [`Host`].
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
pub enum HostFamily {
	/// An IPv4 address.
	///
	/// See [`HostAddress::Ipv4`] for more information.
	Ipv4,
	/// A DECnet address.
	///
	/// See [`HostAddress::DecNet`] for more information.
	DecNet,
	/// A chaos address.
	///
	/// See [`HostAddress::Chaos`] for more information.
	Chaos,
	/// A server-specific address interpreted by the X server.
	///
	/// See [`HostAddress::ServerInterpreted`] for more information.
	ServerInterpreted = 5,
	/// An IPv6 address.
	///
	/// See [`HostAddress::Ipv6`] for more information.
	Ipv6,
}

/// The string used to create an [`AsciiString`] was not encoded as ASCII.
#[derive(Error, Debug)]
#[error("the provided string was not encoded correctly in ASCII format")]
pub struct NonAsciiEncoding;

/// A string comprised entirely of ASCII bytes.
///
/// This is used for [`HostAddress::ServerInterpreted`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, X11Size, Writable)]
pub struct AsciiString(Vec<u8>);

impl AsciiString {
	/// Creates a new `AsciiString` with the given ASCII-encoded bytes.
	///
	/// # Errors
	/// Returns [`NonAsciiEncoding`] if the given `string` is not encoded
	/// correctly as ASCII.
	pub fn new(string: Vec<u8>) -> Result<Self, NonAsciiEncoding> {
		if string.is_ascii() {
			Ok(Self(string))
		} else {
			Err(NonAsciiEncoding)
		}
	}

	/// Returns a slice of the string as bytes.
	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}

	/// Returns the length of the string.
	#[must_use]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	/// Returns whether the string is empty.
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl ReadableWithContext for AsciiString {
	type Context = usize;

	fn read_with(buf: &mut impl Buf, length: &usize) -> ReadResult<Self> {
		Ok(Self(<Vec<u8>>::read_with(buf, length)?))
	}
}

/// The address used in a [host].
///
/// [host]: Host
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum HostAddress {
	/// An IPv4 address.
	Ipv4([u8; 4]),
	/// A DECnet address.
	///
	/// The first byte contains the least significant 8 bits of the node number.
	///
	/// The second byte contains the most significant 2 bits of the node number
	/// in its least significant 2 bits, and the area in the most significant 6
	/// bits of the byte.
	DecNet([u8; 2]),
	/// A chaos address.
	///
	/// The first byte is the host number.
	///
	/// The second byte is the subnet number.
	Chaos([u8; 2]),
	/// An address type interpreted by the X server.
	ServerInterpreted {
		/// The type of address.
		///
		/// Address types and the syntax for their values are defined elsewhere.
		address_type: AsciiString,
		/// The value of the address.
		///
		/// Address types and the syntax for their values are defined elsewhere.
		address_value: AsciiString,
	},
	/// An IPv6 address.
	Ipv6([u8; 16]),
}

impl HostAddress {
	/// The [`HostFamily`] associated with this address.
	#[must_use]
	pub const fn family(&self) -> HostFamily {
		match self {
			Self::Ipv4(..) => HostFamily::Ipv4,
			Self::DecNet(..) => HostFamily::DecNet,
			Self::Chaos(..) => HostFamily::Chaos,
			Self::ServerInterpreted { .. } => HostFamily::ServerInterpreted,
			Self::Ipv6(..) => HostFamily::Ipv6,
		}
	}
}

impl X11Size for HostAddress {
	fn x11_size(&self) -> usize {
		match self {
			Self::Ipv4(address) => address.x11_size(),
			Self::DecNet(address) | Self::Chaos(address) => address.x11_size(),

			Self::ServerInterpreted {
				address_type,
				address_value,
			} => {
				if address_value.is_empty() {
					address_type.x11_size()
				} else {
					address_type.x11_size() + 1 + address_value.x11_size()
				}
			},

			Self::Ipv6(address) => address.x11_size(),
		}
	}
}

impl ReadableWithContext for HostAddress {
	type Context = (HostFamily, usize);

	fn read_with(buf: &mut impl Buf, (family, length): &(HostFamily, usize)) -> ReadResult<Self> {
		let buf = &mut buf.take(*length);

		match family {
			HostFamily::Ipv4 => Ok(Self::Ipv4([
				buf.get_u8(),
				buf.get_u8(),
				buf.get_u8(),
				buf.get_u8(),
			])),
			HostFamily::DecNet => Ok(Self::DecNet([buf.get_u8(), buf.get_u8()])),
			HostFamily::Chaos => Ok(Self::Chaos([buf.get_u8(), buf.get_u8()])),

			HostFamily::ServerInterpreted => {
				let mut address_type = vec![];
				let mut address_value = vec![];

				while buf.has_remaining() {
					match buf.get_u8() {
						0 => {
							buf.advance(1);
							address_value = <Vec<u8>>::read_with(buf, &buf.remaining())?;

							break;
						},

						byte => address_type.push(byte),
					}
				}

				match (
					AsciiString::new(address_type),
					AsciiString::new(address_value),
				) {
					(Ok(address_type), Ok(address_value)) => Ok(Self::ServerInterpreted {
						address_type,
						address_value,
					}),

					(Err(error), _) | (_, Err(error)) => Err(FailedConversion(Box::new(error))),
				}
			},

			HostFamily::Ipv6 => Ok(Self::Ipv6(array_init(|_| buf.get_u8()))),
		}
	}
}

impl Writable for HostAddress {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		match self {
			Self::Ipv4(address) => address.write_to(buf)?,
			Self::DecNet(address) | Self::Chaos(address) => address.write_to(buf)?,

			Self::ServerInterpreted {
				address_type,
				address_value,
			} => {
				address_type.write_to(buf)?;

				if !address_value.is_empty() {
					buf.put_u8(0);
					address_value.write_to(buf)?;
				}
			},

			Self::Ipv6(address) => address.write_to(buf)?,
		}

		Ok(())
	}
}

derive_xrb! {
	/// A host, as provided in a [`ChangeHosts` request].
	///
	/// [`ChangeHosts` request]: crate::x11::request::ChangeHosts
	#[derive(Clone, Eq, PartialEq, Hash, Debug, new, X11Size, Readable, Writable)]
	pub struct Host {
		// The `address`' family.
		let family: HostFamily = address => address.family(),
		_,

		// The size of `address` in bytes.
		#[allow(clippy::cast_possible_truncation)]
		let address_size: u16 = address => address.x11_size() as u16,
		/// The host's address.
		///
		/// See [`HostAddress`] for more information.
		#[context(family, address_size => (*family, *address_size as usize))]
		pub address: HostAddress,
		[_; address => pad(address)],
	}
}
