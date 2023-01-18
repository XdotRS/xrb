// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{BackingStores, Colormap, EventMask, Window};
use derive_more::{From, Into};
use xrbk_macro::{derive_xrb, new, unwrap, ConstantX11Size, Readable, Wrap, Writable, X11Size};

/// A pixel's color.
///
/// The way that this color is interpreted depends on the [`VisualClass`] of the
/// [`VisualType`]:
/// - For [`VisualClass::PseudoColor`], a pixel value indexes a [colormap] to
///   produce independent RGB values.
/// - [`VisualClass::GrayScale`] is the same as [`PseudoColor`], except each
///   color channel must have an equal value (i.e. it must be gray).
/// - For [`VisualClass::DirectColor`], a pixel value is decomposed into
///   separate RGB subfields, each separately indexing the [colormap] for the
///   corresponding value.
/// - [`VisualClass::TrueColor`] is the same as [`DirectColor`], except the
///   [colormap] has predefined read-only RGB values.
/// - [`VisualClass::StaticColor`] is the same as [`PseudoColor`], except the
///   [colormap] has predefined read-only RGB values which are server-dependent.
/// - [`VisualClass::StaticGray`] is the same as [`StaticColor`], except each
///   color channel must have an equal value (i.e. it must be gray).
///
/// [`StaticColor`]: VisualClass::StaticColor
/// [`DirectColor`]: VisualClass::DirectColor
/// [`PseudoColor`]: VisualClass::PseudoColor
/// [colormap]: Colormap
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	new,
	unwrap,
	// XRBK traits
	ConstantX11Size,
	X11Size,
	Readable,
	Writable,
)]
pub struct Pixel(u32);

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
pub struct RgbColor(
	/// Red.
	pub u16,
	/// Green.
	pub u16,
	/// Blue.
	pub u16,
);

/// An error returned when a value meant to be interpreted as a hex color code
/// is greater than `0xffffff`.
///
/// This is returned from [`RgbColor::from_hex`].
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RgbColorTooHigh;

impl RgbColor {
	/// Converts a hex color code to a `Color`.
	///
	/// # Errors
	/// If the provided `u32` color value is greater than `0x_ffffff`, a
	/// [`RgbColorTooHigh`] error will be generated.
	///
	/// # Examples
	/// ```
	/// use xrb::{RgbColor, RgbColorTooHigh};
	///
	/// # fn main() -> Result<(), RgbColorTooHigh> {
	/// #
	/// let red = RgbColor::from_hex(0xff0000)?;
	/// assert_eq!(red, RgbColor(0xff00, 0x0000, 0x0000));
	///
	/// let blue = RgbColor::from_hex(0x0000ff)?;
	/// assert_eq!(blue, RgbColor(0x0000, 0x0000, 0xff00));
	/// #
	/// #     Ok(())
	/// # }
	/// ```
	pub const fn from_hex(hex: u32) -> Result<Self, RgbColorTooHigh> {
		/// The maximum value which a hex color code can be.
		const COLOR_MASK: u32 = 0x00ff_ffff;

		const RED_MASK: u32 = 0x00ff_0000;
		const GREEN_MASK: u32 = 0x0000_ff00;
		const BLUE_MASK: u32 = 0x0000_00ff;

		/// The number of bits in a byte. Used to make the bitshifts more
		/// readable.
		const BYTE: u32 = u8::BITS;

		if hex > COLOR_MASK {
			return Err(RgbColorTooHigh);
		}

		// Red color channel gets moved over 16 bits so that it is represented as one
		// byte.
		let red = ((hex & RED_MASK) >> (2 * BYTE)) as u16;
		// Green color channel gets moved over 8 bits so that is is represented as one
		// byte.
		let green = ((hex & GREEN_MASK) >> BYTE) as u16;
		// Blue color channel is already represented as one byte.
		let blue = (hex & BLUE_MASK) as u16;

		// Since the color channels for `Color` are actually `u16` values, not `u8`,
		// they are shifted one byte to the left.
		Ok(Self(red << BYTE, green << BYTE, blue << BYTE))
	}

	/// Converts a `Color` to a hex color code.
	///
	/// # Lossy
	/// This function is lossy: a `Color` is made up of three `u16` values,
	/// while a hex color code represents three `u8` values. The least
	/// significant byte of each color channel will be lost during conversion.
	///
	/// # Examples
	/// ```
	/// use xrb::RgbColor;
	///
	/// let red: RgbColor = RgbColor(0xff80, 0x00ff, 0x0000);
	/// assert_eq!(red.to_hex(), 0xff0000);
	///
	/// let blue: RgbColor = RgbColor(0x0000, 0x0080, 0xffff);
	/// assert_eq!(blue.to_hex(), 0x0000ff);
	/// ```
	#[must_use]
	pub fn to_hex(&self) -> u32 {
		/// The number of bits in a byte. Used to make the bitshifts more
		/// readable.
		const BYTE: u32 = u8::BITS;

		let Self(red, green, blue) = self;

		// The color channels are all shifted 8 bits to the right to scale their values
		// from `u16` to `u8`. They are then cast to `u32` so they can be combined into
		// one `u32` value.
		let (red, green, blue) = (
			u32::from(red >> BYTE),
			u32::from(green >> BYTE),
			u32::from(blue >> BYTE),
		);

		// We can now union the channels into one `u32` value - red and green are
		// shifted over into `0xff0000` and `0x00ff00` positions respectively to do so.
		(red << (2 * BYTE)) | (green << BYTE) | blue
	}
}

impl From<(u32, u32, u32)> for RgbColor {
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

impl From<RgbColor> for (u32, u32, u32) {
	fn from(RgbColor(red, green, blue): RgbColor) -> Self {
		(u32::from(red), u32::from(green), u32::from(blue))
	}
}

/// The ID of a [`VisualType`].
///
/// [`VisualType`]: VisualType
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

derive_xrb! {
	#[derive(
		Copy,
		Clone,
		Eq,
		PartialEq,
		Hash,
		Debug,
		new,
		// XRBK traits
		X11Size,
		Readable,
		Writable,
	)]
	pub struct Format {
		pub depth: u8,
		pub bits_per_pixel: u8,
		pub scanline_pad: u8,
		[_; 5],
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
	Readable,
	Writable,
)]
pub struct Millimeters(u16);

derive_xrb! {
	#[derive(Clone, Eq, PartialEq, Hash, Debug, new, X11Size, Readable, Writable)]
	pub struct Screen {
		pub root: Window,
		pub default_colormap: Colormap,

		pub white_pixel: u32,
		pub black_pixel: u32,

		pub current_input_masks: EventMask,

		pub width_px: u16,
		pub height_px: u16,
		pub width_mm: Millimeters,
		pub height_mm: Millimeters,

		pub min_installed_maps: u16,
		pub max_installed_maps: u16,

		pub root_visual: VisualId,
		pub backing_stores: BackingStores,
		pub save_unders: bool,
		pub root_depth: u8,

		#[allow(clippy::cast_possible_truncation)]
		let allowed_depths_len: u8 = allowed_depths => allowed_depths.len() as u8,

		#[context(allowed_depths_len => *allowed_depths_len as usize)]
		pub allowed_depths: Vec<Depth>,
	}

	#[derive(Clone, Eq, PartialEq, Hash, Debug, new, X11Size, Readable, Writable)]
	pub struct Depth {
		pub depth: u8,
		_,

		#[allow(clippy::cast_possible_truncation)]
		let visuals_len: u16 = visuals => visuals.len() as u16,
		[_; 4],

		#[context(visuals_len => *visuals_len as usize)]
		pub visuals: Vec<VisualType>,
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
pub enum VisualClass {
	StaticGray,
	GrayScale,
	StaticColor,
	PseudoColor,
	TrueColor,
	DirectColor,
}

derive_xrb! {
	#[derive(Clone, Eq, PartialEq, Hash, Debug, new, X11Size, Readable, Writable)]
	pub struct VisualType {
		pub visual_id: VisualId,
		pub class: VisualClass,
		pub bits_per_rgb_value: u8,
		pub colormap_entries: u16,
		pub color_mask: RgbColor,
		[_; 4],
	}
}
