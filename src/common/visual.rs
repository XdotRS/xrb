// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(
	clippy::too_many_arguments,
	reason = "It makes sense for `Screen` to have many arguments because it has many fields."
)]

use crate::{
	unit::{Mm, Px},
	Colormap,
	EventMask,
	MaintainContents,
	Window,
};
use derive_more::{From, Into};
use xrbk_macro::{derive_xrb, new, unwrap, ConstantX11Size, Readable, Wrap, Writable, X11Size};

/// A color in the X Window System.
///
/// ***Note: you may be looking for [`RgbColor`].***
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
pub struct ColorId(u32);

impl ColorId {
	/// A `ColorId` where all bits are zero: `0x0000_0000`.
	pub const ZERO: Self = Self(0x0000_0000);

	/// A `ColorId` with a value of `1`.
	pub const ONE: Self = Self(1);
}

/// A color comprised of red, green, and blue color channels.
///
/// Each of the channels is a `u16` value, where `0` is the minimum intensity
/// and `65535` is the maximum intensity. The X server scales the values to
/// match the display hardware.
#[doc(alias("Color", "Rgb"))]
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

impl RgbColor {
	/// Black (#000000).
	pub const BLACK: Self = Self(0x0000, 0x0000, 0x0000);

	/// Dark gray (#404040).
	pub const DARK_GRAY: Self = Self(0x4000, 0x4000, 0x4000);
	/// Dark grey (#404040).
	pub const DARK_GREY: Self = Self(0x4000, 0x4000, 0x4000);

	/// Gray (#808080).
	pub const GRAY: Self = Self(0x8000, 0x8000, 0x8000);
	/// Grey (#808080).
	pub const GREY: Self = Self(0x8000, 0x8000, 0x8000);

	/// Light gray (#c0c0c0).
	pub const LIGHT_GRAY: Self = Self(0xc000, 0xc000, 0xc000);
	/// Light grey (#c0c0c0).
	pub const LIGHT_GREY: Self = Self(0xc000, 0xc000, 0xc000);

	/// White (#ffffff).
	pub const WHITE: Self = Self(0xffff, 0xffff, 0xffff);

	/// Red (#ff0000).
	pub const RED: Self = Self(0xffff, 0x0000, 0x0000);
	/// Green (#00ff00).
	pub const GREEN: Self = Self(0x0000, 0xffff, 0x0000);
	/// Blue (#0000ff).
	pub const BLUE: Self = Self(0x0000, 0x0000, 0xffff);

	/// Yellow (#ffff00).
	pub const YELLOW: Self = Self(0xffff, 0xffff, 0x0000);
	/// Cyan (#00ffff).
	pub const CYAN: Self = Self(0x0000, 0xffff, 0xffff);
	/// Magenta (#ff00ff).
	pub const MAGENTA: Self = Self(0xffff, 0x0000, 0xffff);

	/// Orange (#ff8000).
	pub const ORANGE: Self = Self(0xffff, 0x8000, 0x0000);
	/// Pink (#ff0080).
	pub const PINK: Self = Self(0xffff, 0x0000, 0x8000);
	/// Lime  (#80ff00).
	pub const LIME: Self = Self(0x8000, 0xffff, 0x0000);
	/// Mint (#00ff80).
	pub const MINT: Self = Self(0x0000, 0xffff, 0x8000);
	/// Purple (#8000ff).
	pub const PURPLE: Self = Self(0x8000, 0x0000, 0xffff);
	/// Sky blue (#0080ff).
	pub const SKY_BLUE: Self = Self(0x0000, 0x8000, 0xffff);

	/// Dark red (#800000).
	pub const DARK_RED: Self = Self(0x8000, 0x0000, 0x0000);
	/// Dark green (#008000).
	pub const DARK_GREEN: Self = Self(0x0000, 0x8000, 0x0000);
	/// Dark blue (#000080).
	pub const DARK_BLUE: Self = Self(0x0000, 0x0000, 0x8000);

	/// Dark yellow (#808000).
	pub const DARK_YELLOW: Self = Self(0x8000, 0x8000, 0x0000);
	/// Dark cyan (#008080).
	pub const DARK_CYAN: Self = Self(0x0000, 0x8000, 0x8000);
	/// Dark magenta (#800080).
	pub const DARK_MAGENTA: Self = Self(0x8000, 0x0000, 0x8000);
}

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
	/// use xrb::visual::{RgbColor, RgbColorTooHigh};
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

	/// Converts an `RgbColor` to a hex color code.
	///
	/// # Lossy
	/// This function is lossy: a `Color` is made up of three `u16` values,
	/// while a hex color code represents three `u8` values. The least
	/// significant byte of each color channel will be lost during conversion.
	///
	/// # Examples
	/// ```
	/// use xrb::visual::RgbColor;
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
pub struct VisualId(u32);

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

derive_xrb! {
	#[derive(Clone, Eq, PartialEq, Hash, Debug, new, X11Size, Readable, Writable)]
	pub struct Screen {
		pub root: Window,
		pub default_colormap: Colormap,

		pub white: ColorId,
		pub black: ColorId,

		pub current_input_masks: EventMask,

		pub width_px: Px<u16>,
		pub height_px: Px<u16>,
		pub width_mm: Mm<u16>,
		pub height_mm: Mm<u16>,

		pub min_installed_colormaps: u16,
		pub max_installed_colormaps: u16,

		pub root_visual: VisualId,
		pub maintain_contents_mode: MaintainContents,
		pub maintain_windows_under: bool,
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
