// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xrbk_macro::{ConstantX11Size, Readable, Writable, X11Size};

use crate::{common::set::__bool, visual::Pixel, Font, Pixmap};
use bitflags::bitflags;
use xrbk::{
	Buf,
	BufMut,
	ConstantX11Size,
	ReadError::UnrecognizedDiscriminant,
	ReadResult,
	Readable,
	Writable,
	WriteResult,
	X11Size,
};

bitflags! {
	#[derive(Default, X11Size, Readable, ConstantX11Size, Writable)]
	pub struct GraphicsOptionsMask: u32 {
		const FUNCTION = 0x0000_0001;

		const PLANE_MASK = 0x0000_0002;

		const FOREGROUND = 0x0000_0004;
		const BACKGROUND = 0x0000_0008;

		const LINE_WIDTH = 0x0000_0010;

		const LINE_STYLE = 0x0000_0020;
		const CAP_STYLE = 0x0000_0040;
		const JOIN_STYLE = 0x0000_0080;
		const FILL_STYLE = 0x0000_0100;
		const FILL_RULE = 0x0000_0200;

		const TILE = 0x0000_0400;
		const STIPPLE = 0x0000_0800;

		const TILE_STIPPLE_X_ORIGIN = 0x0000_1000;
		const TILE_STIPPLE_Y_ORIGIN = 0x0000_2000;

		const FONT = 0x0000_4000;

		const SUBWINDOW_MODE = 0x0000_8000;

		const GRAPHICS_EXPOSURES = 0x0001_0000;

		const CLIP_X_ORIGIN = 0x0002_0000;
		const CLIP_Y_ORIGIN = 0x0004_0000;
		const CLIP_MASK = 0x0008_0000;

		const DASH_OFFSET = 0x0010_0000;
		const DASHES = 0x0020_0000;

		const ARC_MODE = 0x0040_0000;
	}
}

/// Given a source and destination pixel, represents a bitwise operation applied
/// to the source and destination to determine the resultant pixel.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum Function {
	/// The resultant pixel is bitwise zero; that is, it has a `0` for each bit.
	Clear = 0,
	/// The resultant pixel is the bitwise AND of the source and the
	/// destination: `source & destination`.
	And = 1,
	/// The resultant pixel is the bitwise AND of the source and the NOT of
	/// the destination: `source & (!destination)`.
	AndReverse = 2,
	/// The resultant pixel is simply the source pixel: `source`.
	Copy = 3,

	/// The resultant pixel is the bitwise AND of the NOT of the source and
	/// the destination: `(!source) & destination`.
	AndInverted = 4,
	/// The resultant pixel is simply the destination pixel: `destination`.
	NoOp = 5,
	/// The resultant pixel is the bitwise XOR of the source and the
	/// destination: `source ^ destination`.
	Xor = 6,
	/// The resultant pixel is the bitwise OR of the source and the destination
	/// `source | destination`.
	Or = 7,

	/// The resultant pixel is the bitwise AND of the NOT of the source and the
	/// NOT of the destination: `(!source) & (!destination)`.
	Nor = 8,
	/// The resultant pixel is the bitwise XOR of the NOT of the source and the
	/// destination: `(!source) ^ destination`.
	Equiv = 9,
	/// The resultant pixel is the bitwise NOT of the destination:
	/// `!destination`.
	Invert = 10,
	/// The resultant pixel is the bitwise OR of the source and the NOT of the
	/// destination: `source | (!destination)`.
	OrReverse = 11,

	/// The resultant pixel is the bitwise NOT of the source: `!source`.
	CopyInverted = 12,
	/// The resultant pixel is the bitwise OR of the NOT of the source and the
	/// destination: `(!source) | destination`.
	OrInverted = 13,
	/// The resultant pixel is the bitwise OR of the NOT of the source and the
	/// NOT of the destination: `(!source) | (!destination)`.
	Nand = 14,
	/// The resultant pixel is bitwise one; that is, it has a `1` for each bit.
	Set = 15,
}

/// The width of a line.
///
/// The line can either be [`Thin`] (if it has the special value of zero), or
/// [`Wide`] for any width greater than or equal to one.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum LineWidth {
	/// A thin line, having the special width value of zero.
	Thin,
	/// A thick line, having a `LineWidth` of greater than or equal to one.
	Thick(u16),
}

impl LineWidth {
	/// Creates a new `LineWidth` with the given width.
	///
	/// If the width is zero, this is [`LineWidth::Thin`]. Otherwise, it is
	/// [`LineWidth::Thick(width)`].
	///
	/// The width is measured in pixels.
	///
	/// [`LineWidth::Thick(width)`]: LineWidth::Thick
	#[must_use]
	pub const fn new(width: u16) -> Self {
		if width == 0 {
			Self::Thin
		} else {
			Self::Thick(width)
		}
	}

	/// Unwraps the `u16` width of the line.
	///
	/// If this is [`LineWidth::Thin`], this is `0`. Otherwise, it is the width
	/// wrapped by [`LineWidth::Thick`].
	///
	/// The width is measured in pixels.
	#[must_use]
	pub const fn unwrap(&self) -> u16 {
		match self {
			Self::Thin => 0,
			Self::Thick(width) => *width,
		}
	}
}

/// Defines which sections of a line are drawn.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum LineStyle {
	/// The full path of the line is drawn.
	Solid = 0,

	/// Only the even dashes are drawn.
	///
	/// The [`CapStyle`] applies to all internal ends of the individual dashes,
	/// with the exception of [`CapStyle::NotLast`], which is treated as
	/// [`CapStyle::Butt`].
	OnOffDash = 1,

	/// The full path of the line is drawn, but the even dashes are [filled
	/// differently] than the odd dashes.
	///
	/// [`CapStyle::Butt`] is used where even and odd dashes meet.
	///
	/// [filled differently]: FillStyle
	DoubleDash = 2,
}

/// Defines how the endpoints of a path are drawn.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum CapStyle {
	/// Equivalent to [`Butt`], except in the case of [`LineWidth::Thin`], where
	/// the final endpoint is not drawn.
	///
	/// [`Butt`]: CapStyle::Butt
	NotLast = 0,

	/// The end of the path is square, perpendicular to the slope of the line,
	/// with no projection beyond the endpoint.
	Butt = 1,

	/// The end of the path is a circular arc with a diameter equal to the
	/// [`LineWidth`], centered on the endpoint.
	///
	/// For [`LineWidth::Thin`], this is equivalent to [`Butt`].
	///
	/// [`Butt`]: CapStyle::Butt
	Round = 2,

	/// The end of the path is square, but the path projects beyond the endpoint
	/// for a distance equal to half of the [`LineWidth`].
	///
	/// For [`LineWidth::Thin`], this is equivalent to [`Butt`].
	///
	/// [`Butt`]: CapStyle::Butt
	Projecting = 3,
}

/// Defines how the corners of [`Thick`] lines are drawn.
///
/// [`Thick`]: LineWidth::Thick
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum JoinStyle {
	/// The outer edges of the two lines extend to meet at an angle, if that
	/// angle is at least 11 degrees.
	///
	/// If the angle is less than 11 degrees, this is treated as
	/// [`JoinStyle::Bevel`].
	Miter = 0,

	/// A circular arc with a diameter equal to the [`LineWidth`] is centered on
	/// the joinpoint.
	Round = 1,

	/// [`CapStyle::Butt`] endpoint styles are used, then the triangular notch
	/// is filled.
	Bevel = 2,
}

/// Defines the contents of the source for line, text, and fill [requests].
///
/// [requests]: crate::x11::request
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum FillStyle {
	/// This is the foreground, except for the odd dashes of line requests with
	/// [`LineStyle::DoubleDash`], where it is the background.
	Solid = 0,

	// FIXME (docs): X11 protocol just says 'Tile' as description...
	/// A tiled `FillStyle`.
	Tiled = 1,

	/// The foreground, masked by a stipple pattern.
	Stippled = 2,

	/// Same as [`Stippled`], but foreground everywhere it has a one, and
	/// background everywhere it has a zero.
	OpaqueStippled = 3,
}

/// Defines what pixels are drawn for paths in [`FillPoly` requests].
///
/// [`FillPoly` requests]: crate::x11::request::FillPoly
// Hell if I know what the X11 protocol is talking about for these variants.
// Really technical language. I imagine it's simply not worth documenting.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum FillRule {
	EvenOdd = 0,
	Winding = 1,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum SubwindowMode {
	ClipByChildren = 0,
	IncludeDescendents = 1,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum ArcMode {
	Chord = 0,
	PieSlice = 1,
}

/// This is a type alias for <code>[Option]<[Pixmap]></code>.
///
/// This represents the type used in the [`clip_mask` graphics option].
///
/// [`clip_mask` graphics option]: GraphicsOptions::clip_mask
pub type ClipMask = Option<Pixmap>;

/// A set of options that apply to a [`GraphicsContext`].
///
/// The following table shows each attribute and its default value if it is not
/// explicitly initialized in the [`CreateGraphicsContext` request].
///
/// [`GraphicsContext`]: crate::GraphicsContext
/// [`CreateGraphicsContext` request]: crate::x11::request::CreateGraphicsContext
///
/// |Option                   |Default value                          |
/// |-------------------------|---------------------------------------|
/// |[`function`]             |[`Function::Copy`]                     |
/// |[`plane_mask`]           |`0xffff_ffff`                          |
/// |[`foreground`]           |`0`                                    |
/// |[`background`]           |`1`                                    |
/// |[`line_width`]           |[`LineWidth::Thin`]                    |
/// |[`line_style`]           |[`LineStyle::Solid`]                   |
/// |[`cap_style`]            |[`CapStyle::Butt`]                     |
/// |[`join_style`]           |[`JoinStyle::Miter`]                   |
/// |[`fill_style`]           |[`FillStyle::Solid`]                   |
/// |[`fill_rule`]            |[`FillRule::EvenOdd`]                  |
/// |[`arc_mode`]             |[`ArcMode::PieSlice`]                  |
/// |[`tile`]                 |[Pixmap] filled with the [`foreground`]|
/// |[`stipple`]              |[Pixmap] filled with ones              |
/// |[`tile_stipple_x_origin`]|`0`                                    |
/// |[`tile_stipple_y_origin`]|`0`                                    |
/// |[`font`]                 |Depends on the server                  |
/// |[`subwindow_mode`]       |[`SubwindowMode::ClipByChildren`]      |
/// |[`graphics_exposure`]    |`true`                                 |
/// |[`clip_x_origin`]        |`0`                                    |
/// |[`clip_y_origin`]        |`0`                                    |
/// |[`clip_mask`]            |[`None`]                               |
/// |[`dash_offset`]          |`0`                                    |
/// |[`dashes`]               |`4`                                    |
///
/// [Pixmap]: Pixmap
///
/// [`function`]: GraphicsOptions::function
/// [`plane_mask`]: GraphicsOptions::plane_mask
/// [`foreground`]: GraphicsOptions::foreground
/// [`background`]: GraphicsOptions::background
/// [`line_width`]: GraphicsOptions::line_width
/// [`line_style`]: GraphicsOptions::line_style
/// [`cap_style`]: GraphicsOptions::cap_style
/// [`join_style`]: GraphicsOptions::join_style
/// [`fill_style`]: GraphicsOptions::fill_style
/// [`fill_rule`]: GraphicsOptions::fill_rule
/// [`arc_mode`]: GraphicsOptions::arc_mode
/// [`tile`]: GraphicsOptions::tile
/// [`stipple`]: GraphicsOptions::stipple
/// [`tile_stipple_x_origin`]: GraphicsOptions::tile_stipple_x_origin
/// [`tile_stipple_y_origin`]: GraphicsOptions::tile_stipple_y_origin
/// [`font`]: GraphicsOptions::font
/// [`subwindow_mode`]: GraphicsOptions::subwindow_mode
/// [`graphics_exposure`]: GraphicsOptions::graphics_exposure
/// [`clip_x_origin`]: GraphicsOptions::clip_x_origin
/// [`clip_y_origin`]: GraphicsOptions::clip_y_origin
/// [`clip_mask`]: GraphicsOptions::clip_mask
/// [`dash_offset`]: GraphicsOptions::dash_offset
/// [`dashes`]: GraphicsOptions::dashes
pub struct GraphicsOptions {
	x11_size: usize,

	mask: GraphicsOptionsMask,

	function: Option<__Function>,

	plane_mask: Option<u32>,

	foreground: Option<Pixel>,
	background: Option<Pixel>,

	line_width: Option<__LineWidth>,

	line_style: Option<__LineStyle>,
	cap_style: Option<__CapStyle>,
	join_style: Option<__JoinStyle>,
	fill_style: Option<__FillStyle>,
	fill_rule: Option<__FillRule>,

	tile: Option<Pixmap>,
	stipple: Option<Pixmap>,

	// This represents an `i16` value.
	tile_stipple_x_origin: Option<i32>,
	// This represents an `i16` value.
	tile_stipple_y_origin: Option<i32>,

	font: Option<Font>,

	subwindow_mode: Option<__SubwindowMode>,

	graphics_exposures: Option<__bool>,

	// This represents an `i16` value.
	clip_x_origin: Option<i32>,
	// This represents an `i16` value.
	clip_y_origin: Option<i32>,
	clip_mask: Option<ClipMask>,

	// This represents a `u16` value.
	dash_offset: Option<u32>,
	// This represents a `u8` value.
	dashes: Option<u32>,

	arc_mode: Option<__ArcMode>,
}

impl GraphicsOptions {
	/// Returns a new [`GraphicsOptionsBuilder`] with which a `GraphicsOptions`
	/// set can be constructed.
	#[must_use]
	pub const fn builder() -> GraphicsOptionsBuilder {
		GraphicsOptionsBuilder::new()
	}
}

/// A builder used to construct a new [`GraphicsOptions` set].
///
/// All graphics options start as [`None`], and be configured with the methods
/// on this builder. When the builder is configured, [`build()`] can be used to
/// construct the resulting [`GraphicsOptions`].
///
/// [`build()`]: GraphicsOptionsBuilder::build
/// [`GraphicsOptions` set]: GraphicsOptions
#[derive(Clone, Default, Debug, Hash, PartialEq, Eq)]
pub struct GraphicsOptionsBuilder {
	x11_size: usize,

	mask: GraphicsOptionsMask,

	function: Option<Function>,

	plane_mask: Option<u32>,

	foreground: Option<Pixel>,
	background: Option<Pixel>,

	line_width: Option<LineWidth>,

	line_style: Option<LineStyle>,
	cap_style: Option<CapStyle>,
	join_style: Option<JoinStyle>,
	fill_style: Option<FillStyle>,
	fill_rule: Option<FillRule>,

	tile: Option<Pixmap>,
	stipple: Option<Pixmap>,

	tile_stipple_x_origin: Option<i16>,
	tile_stipple_y_origin: Option<i16>,

	font: Option<Font>,

	subwindow_mode: Option<SubwindowMode>,

	graphics_exposures: Option<bool>,

	clip_x_origin: Option<i16>,
	clip_y_origin: Option<i16>,
	clip_mask: Option<ClipMask>,

	dash_offset: Option<u16>,
	dashes: Option<u8>,

	arc_mode: Option<ArcMode>,
}

impl GraphicsOptionsBuilder {
	/// Creates a new `GraphicsOptionsBuilder`.
	///
	/// All graphics options start as [`None`], and be configured with the other
	/// methods on this builder. When the builder is configured, [`build()`] can
	/// be used to construct the resulting [`GraphicsOptions`].
	///
	/// [`build()`]: GraphicsOptionsBuilder::build
	#[must_use]
	pub const fn new() -> Self {
		Self {
			x11_size: GraphicsOptionsMask::X11_SIZE,

			mask: GraphicsOptionsMask::empty(),

			function: None,

			plane_mask: None,

			foreground: None,
			background: None,

			line_width: None,

			line_style: None,
			cap_style: None,
			join_style: None,
			fill_style: None,
			fill_rule: None,

			tile: None,
			stipple: None,

			tile_stipple_x_origin: None,
			tile_stipple_y_origin: None,

			font: None,

			subwindow_mode: None,

			graphics_exposures: None,

			clip_x_origin: None,
			clip_y_origin: None,
			clip_mask: None,

			dash_offset: None,
			dashes: None,

			arc_mode: None,
		}
	}

	/// Constructs the resulting [`GraphicsOptions` set] with the configured
	/// options.
	///
	/// [`GraphicsOptions` set]: GraphicsOptions
	#[must_use]
	pub fn build(self) -> GraphicsOptions {
		GraphicsOptions {
			x11_size: self.x11_size,

			mask: self.mask,

			function: self.function.map(__Function),

			plane_mask: self.plane_mask,

			foreground: self.foreground,
			background: self.background,

			line_width: self.line_width.map(__LineWidth),

			line_style: self.line_style.map(__LineStyle),
			cap_style: self.cap_style.map(__CapStyle),
			join_style: self.join_style.map(__JoinStyle),
			fill_style: self.fill_style.map(__FillStyle),
			fill_rule: self.fill_rule.map(__FillRule),

			tile: self.tile,
			stipple: self.stipple,

			tile_stipple_x_origin: self.tile_stipple_x_origin.map(Into::into),
			tile_stipple_y_origin: self.tile_stipple_y_origin.map(Into::into),

			font: self.font,

			subwindow_mode: self.subwindow_mode.map(__SubwindowMode),

			graphics_exposures: self.graphics_exposures.map(__bool),

			clip_x_origin: self.clip_x_origin.map(Into::into),
			clip_y_origin: self.clip_y_origin.map(Into::into),
			clip_mask: self.clip_mask,

			dash_offset: self.dash_offset.map(Into::into),
			dashes: self.dashes.map(Into::into),

			arc_mode: self.arc_mode.map(__ArcMode),
		}
	}
}

impl GraphicsOptionsBuilder {
	pub fn function(&mut self, function: Function) -> &mut Self {
		if self.function.is_none() {
			self.x11_size += 4;
		}

		self.function = Some(function);
		self.mask |= GraphicsOptionsMask::FUNCTION;

		self
	}

	pub fn plane_mask(&mut self, plane_mask: u32) -> &mut Self {
		if self.plane_mask.is_none() {
			self.x11_size += 4;
		}

		self.plane_mask = Some(plane_mask);
		self.mask |= GraphicsOptionsMask::PLANE_MASK;

		self
	}

	pub fn foreground(&mut self, foreground: Pixel) -> &mut Self {
		if self.foreground.is_none() {
			self.x11_size += 4;
		}

		self.foreground = Some(foreground);
		self.mask |= GraphicsOptionsMask::FOREGROUND;

		self
	}
	pub fn background(&mut self, background: Pixel) -> &mut Self {
		if self.background.is_none() {
			self.x11_size += 4;
		}

		self.background = Some(background);
		self.mask |= GraphicsOptionsMask::BACKGROUND;

		self
	}

	pub fn line_width(&mut self, line_width: LineWidth) -> &mut Self {
		if self.line_width.is_none() {
			self.x11_size += 4;
		}

		self.line_width = Some(line_width);
		self.mask |= GraphicsOptionsMask::LINE_WIDTH;

		self
	}

	pub fn line_style(&mut self, line_style: LineStyle) -> &mut Self {
		if self.line_style.is_none() {
			self.x11_size += 4;
		}

		self.line_style = Some(line_style);
		self.mask |= GraphicsOptionsMask::LINE_STYLE;

		self
	}
	pub fn cap_style(&mut self, cap_style: CapStyle) -> &mut Self {
		if self.cap_style.is_none() {
			self.x11_size += 4;
		}

		self.cap_style = Some(cap_style);
		self.mask |= GraphicsOptionsMask::CAP_STYLE;

		self
	}
	pub fn join_style(&mut self, join_style: JoinStyle) -> &mut Self {
		if self.join_style.is_none() {
			self.x11_size += 4;
		}

		self.join_style = Some(join_style);
		self.mask |= GraphicsOptionsMask::JOIN_STYLE;

		self
	}
	pub fn fill_style(&mut self, fill_style: FillStyle) -> &mut Self {
		if self.fill_style.is_none() {
			self.x11_size += 4;
		}

		self.fill_style = Some(fill_style);
		self.mask |= GraphicsOptionsMask::FILL_STYLE;

		self
	}
	pub fn fill_rule(&mut self, fill_rule: FillRule) -> &mut Self {
		if self.fill_rule.is_none() {
			self.x11_size += 4;
		}

		self.fill_rule = Some(fill_rule);
		self.mask |= GraphicsOptionsMask::FILL_RULE;

		self
	}

	pub fn tile(&mut self, tile: Pixmap) -> &mut Self {
		if self.tile.is_none() {
			self.x11_size += 4;
		}

		self.tile = Some(tile);
		self.mask |= GraphicsOptionsMask::TILE;

		self
	}
	pub fn stipple(&mut self, stipple: Pixmap) -> &mut Self {
		if self.stipple.is_none() {
			self.x11_size += 4;
		}

		self.stipple = Some(stipple);
		self.mask |= GraphicsOptionsMask::STIPPLE;

		self
	}

	pub fn tile_stipple_x_origin(&mut self, tile_stipple_x_origin: i16) -> &mut Self {
		if self.tile_stipple_x_origin.is_none() {
			self.x11_size += 4;
		}

		self.tile_stipple_x_origin = Some(tile_stipple_x_origin);
		self.mask |= GraphicsOptionsMask::TILE_STIPPLE_X_ORIGIN;

		self
	}
	pub fn tile_stipple_y_origin(&mut self, tile_stipple_y_origin: i16) -> &mut Self {
		if self.tile_stipple_y_origin.is_none() {
			self.x11_size += 4;
		}

		self.tile_stipple_y_origin = Some(tile_stipple_y_origin);
		self.mask |= GraphicsOptionsMask::TILE_STIPPLE_Y_ORIGIN;

		self
	}

	pub fn font(&mut self, font: Font) -> &mut Self {
		if self.font.is_none() {
			self.x11_size += 4;
		}

		self.font = Some(font);
		self.mask |= GraphicsOptionsMask::FONT;

		self
	}

	pub fn subwindow_mode(&mut self, subwindow_mode: SubwindowMode) -> &mut Self {
		if self.subwindow_mode.is_none() {
			self.x11_size += 4;
		}

		self.subwindow_mode = Some(subwindow_mode);
		self.mask |= GraphicsOptionsMask::SUBWINDOW_MODE;

		self
	}

	pub fn graphics_exposures(&mut self, graphics_exposures: bool) -> &mut Self {
		if self.graphics_exposures.is_none() {
			self.x11_size += 4;
		}

		self.graphics_exposures = Some(graphics_exposures);
		self.mask |= GraphicsOptionsMask::GRAPHICS_EXPOSURES;

		self
	}

	pub fn clip_x_origin(&mut self, clip_x_origin: i16) -> &mut Self {
		if self.clip_x_origin.is_none() {
			self.x11_size += 4;
		}

		self.clip_x_origin = Some(clip_x_origin);
		self.mask |= GraphicsOptionsMask::CLIP_X_ORIGIN;

		self
	}
	pub fn clip_y_origin(&mut self, clip_y_origin: i16) -> &mut Self {
		if self.clip_y_origin.is_none() {
			self.x11_size += 4;
		}

		self.clip_y_origin = Some(clip_y_origin);
		self.mask |= GraphicsOptionsMask::CLIP_Y_ORIGIN;

		self
	}
	pub fn clip_mask(&mut self, clip_mask: ClipMask) -> &mut Self {
		if self.clip_mask.is_none() {
			self.x11_size += 4;
		}

		self.clip_mask = Some(clip_mask);
		self.mask |= GraphicsOptionsMask::CLIP_MASK;

		self
	}

	pub fn dash_offset(&mut self, dash_offset: u16) -> &mut Self {
		if self.dash_offset.is_none() {
			self.x11_size += 4;
		}

		self.dash_offset = Some(dash_offset);
		self.mask |= GraphicsOptionsMask::DASH_OFFSET;

		self
	}
	pub fn dashes(&mut self, dashes: u8) -> &mut Self {
		if self.dashes.is_none() {
			self.x11_size += 4;
		}

		self.dashes = Some(dashes);
		self.mask |= GraphicsOptionsMask::DASHES;

		self
	}

	pub fn arc_mode(&mut self, arc_mode: ArcMode) -> &mut Self {
		if self.arc_mode.is_none() {
			self.x11_size += 4;
		}

		self.arc_mode = Some(arc_mode);
		self.mask |= GraphicsOptionsMask::ARC_MODE;

		self
	}
}

impl GraphicsOptions {
	#[must_use]
	pub fn function(&self) -> Option<&Function> {
		self.function.as_ref().map(|__Function(function)| function)
	}

	#[must_use]
	pub const fn plane_mask(&self) -> &Option<u32> {
		&self.plane_mask
	}

	#[must_use]
	pub const fn foreground(&self) -> &Option<Pixel> {
		&self.foreground
	}
	#[must_use]
	pub const fn background(&self) -> &Option<Pixel> {
		&self.background
	}

	#[must_use]
	pub fn line_width(&self) -> Option<&LineWidth> {
		self.line_width
			.as_ref()
			.map(|__LineWidth(line_width)| line_width)
	}

	#[must_use]
	pub fn line_style(&self) -> Option<&LineStyle> {
		self.line_style
			.as_ref()
			.map(|__LineStyle(line_style)| line_style)
	}
	#[must_use]
	pub fn cap_style(&self) -> Option<&CapStyle> {
		self.cap_style
			.as_ref()
			.map(|__CapStyle(cap_style)| cap_style)
	}
	#[must_use]
	pub fn join_style(&self) -> Option<&JoinStyle> {
		self.join_style
			.as_ref()
			.map(|__JoinStyle(join_style)| join_style)
	}
	#[must_use]
	pub fn fill_style(&self) -> Option<&FillStyle> {
		self.fill_style
			.as_ref()
			.map(|__FillStyle(fill_style)| fill_style)
	}
	#[must_use]
	pub fn fill_rule(&self) -> Option<&FillRule> {
		self.fill_rule
			.as_ref()
			.map(|__FillRule(fill_rule)| fill_rule)
	}

	#[must_use]
	pub const fn tile(&self) -> &Option<Pixmap> {
		&self.tile
	}
	#[must_use]
	pub const fn stipple(&self) -> &Option<Pixmap> {
		&self.stipple
	}

	#[must_use]
	pub fn tile_stipple_x_origin(&self) -> Option<i16> {
		self.tile_stipple_x_origin.map(|tile_stipple_x_origin| {
			tile_stipple_x_origin
				.try_into()
				.expect("must fit into i16; represents i16 value")
		})
	}
	#[must_use]
	pub fn tile_stipple_y_origin(&self) -> Option<i16> {
		self.tile_stipple_y_origin.map(|tile_stipple_y_origin| {
			tile_stipple_y_origin
				.try_into()
				.expect("must fit into i16; represents i16 value")
		})
	}

	#[must_use]
	pub const fn font(&self) -> &Option<Font> {
		&self.font
	}

	#[must_use]
	pub fn subwindow_mode(&self) -> Option<&SubwindowMode> {
		self.subwindow_mode
			.as_ref()
			.map(|__SubwindowMode(subwindow_mode)| subwindow_mode)
	}

	#[must_use]
	pub fn graphics_exposure(&self) -> Option<&bool> {
		self.graphics_exposures.as_ref().map(|__bool(bool)| bool)
	}

	#[must_use]
	pub fn clip_x_origin(&self) -> Option<i16> {
		self.clip_x_origin.map(|clip_x_origin| {
			clip_x_origin
				.try_into()
				.expect("must fit into i16; represents i16 value")
		})
	}
	#[must_use]
	pub fn clip_y_origin(&self) -> Option<i16> {
		self.clip_y_origin.map(|clip_y_origin| {
			clip_y_origin
				.try_into()
				.expect("must fit into i16; represents i16 value")
		})
	}
	#[must_use]
	pub const fn clip_mask(&self) -> &Option<ClipMask> {
		&self.clip_mask
	}

	#[must_use]
	pub fn dash_offset(&self) -> Option<u16> {
		self.dash_offset.map(|dash_offset| {
			dash_offset
				.try_into()
				.expect("must fit into u16; represents u16 value")
		})
	}
	#[must_use]
	pub fn dashes(&self) -> Option<u8> {
		self.dashes.map(|dashes| {
			dashes
				.try_into()
				.expect("must fit into u8; represents u8 value")
		})
	}

	#[must_use]
	pub fn arc_mode(&self) -> Option<&ArcMode> {
		self.arc_mode.as_ref().map(|__ArcMode(arc_mode)| arc_mode)
	}
}

// impl XRBK traits for GraphicsOptions {{{

impl X11Size for GraphicsOptions {
	fn x11_size(&self) -> usize {
		self.x11_size
	}
}

impl Readable for GraphicsOptions {
	#[allow(clippy::similar_names, clippy::too_many_lines)]
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		let mask = GraphicsOptionsMask::read_from(buf)?;
		let mut x11_size = mask.x11_size();

		let function = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::FUNCTION),
		)?;

		let plane_mask = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::PLANE_MASK),
		)?;

		let foreground = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::FOREGROUND),
		)?;
		let background = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::BACKGROUND),
		)?;

		let line_width = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::LINE_WIDTH),
		)?;

		let line_style = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::LINE_STYLE),
		)?;
		let cap_style = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::CAP_STYLE),
		)?;
		let join_style = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::JOIN_STYLE),
		)?;
		let fill_style = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::FILL_STYLE),
		)?;
		let fill_rule = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::FILL_RULE),
		)?;

		let tile =
			super::read_set_value(buf, &mut x11_size, mask.contains(GraphicsOptionsMask::TILE))?;
		let stipple = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::STIPPLE),
		)?;

		let tile_stipple_x_origin = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::TILE_STIPPLE_X_ORIGIN),
		)?;
		let tile_stipple_y_origin = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::TILE_STIPPLE_Y_ORIGIN),
		)?;

		let font =
			super::read_set_value(buf, &mut x11_size, mask.contains(GraphicsOptionsMask::FONT))?;

		let subwindow_mode = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::SUBWINDOW_MODE),
		)?;

		let graphics_exposures = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::GRAPHICS_EXPOSURES),
		)?;

		let clip_x_origin = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::CLIP_X_ORIGIN),
		)?;
		let clip_y_origin = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::CLIP_Y_ORIGIN),
		)?;
		let clip_mask = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::CLIP_MASK),
		)?;

		let dash_offset = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::DASH_OFFSET),
		)?;
		let dashes = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::DASHES),
		)?;

		let arc_mode = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::ARC_MODE),
		)?;

		Ok(Self {
			x11_size,

			mask,

			function,

			plane_mask,

			foreground,
			background,

			line_width,

			line_style,
			cap_style,
			join_style,
			fill_style,
			fill_rule,

			tile,
			stipple,

			tile_stipple_x_origin,
			tile_stipple_y_origin,

			font,

			subwindow_mode,

			graphics_exposures,

			clip_x_origin,
			clip_y_origin,
			clip_mask,

			dash_offset,
			dashes,

			arc_mode,
		})
	}
}

impl Writable for GraphicsOptions {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		self.mask.write_to(buf)?;

		if let Some(function) = &self.function {
			function.write_to(buf)?;
		}

		if let Some(plane_mask) = &self.plane_mask {
			plane_mask.write_to(buf)?;
		}

		if let Some(foreground) = &self.foreground {
			foreground.write_to(buf)?;
		}
		if let Some(background) = &self.background {
			background.write_to(buf)?;
		}

		if let Some(line_width) = &self.line_width {
			line_width.write_to(buf)?;
		}

		if let Some(line_style) = &self.line_style {
			line_style.write_to(buf)?;
		}
		if let Some(cap_style) = &self.cap_style {
			cap_style.write_to(buf)?;
		}
		if let Some(join_style) = &self.join_style {
			join_style.write_to(buf)?;
		}
		if let Some(fill_style) = &self.fill_style {
			fill_style.write_to(buf)?;
		}
		if let Some(fill_rule) = &self.fill_rule {
			fill_rule.write_to(buf)?;
		}

		if let Some(tile) = &self.tile {
			tile.write_to(buf)?;
		}
		if let Some(stipple) = &self.stipple {
			stipple.write_to(buf)?;
		}

		if let Some(tile_stipple_x_origin) = &self.tile_stipple_x_origin {
			tile_stipple_x_origin.write_to(buf)?;
		}
		if let Some(tile_stipple_y_origin) = &self.tile_stipple_y_origin {
			tile_stipple_y_origin.write_to(buf)?;
		}

		if let Some(font) = &self.font {
			font.write_to(buf)?;
		}

		if let Some(subwindow_mode) = &self.subwindow_mode {
			subwindow_mode.write_to(buf)?;
		}

		if let Some(graphics_exposures) = &self.graphics_exposures {
			graphics_exposures.write_to(buf)?;
		}

		if let Some(clip_x_origin) = &self.clip_x_origin {
			clip_x_origin.write_to(buf)?;
		}
		if let Some(clip_y_origin) = &self.clip_y_origin {
			clip_y_origin.write_to(buf)?;
		}
		if let Some(clip_mask) = &self.clip_mask {
			clip_mask.write_to(buf)?;
		}

		if let Some(dash_offset) = &self.dash_offset {
			dash_offset.write_to(buf)?;
		}
		if let Some(dashes) = &self.dashes {
			dashes.write_to(buf)?;
		}

		if let Some(arc_mode) = &self.arc_mode {
			arc_mode.write_to(buf)?;
		}

		Ok(())
	}
}

// }}}

// Internal 4-byte representations of types {{{

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __Function(Function);

impl ConstantX11Size for __Function {
	const X11_SIZE: usize = 4;
}

impl X11Size for __Function {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __Function {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_u32() {
			discrim if discrim == 0 => Function::Clear,
			discrim if discrim == 1 => Function::And,
			discrim if discrim == 2 => Function::AndReverse,
			discrim if discrim == 3 => Function::Copy,

			discrim if discrim == 4 => Function::AndInverted,
			discrim if discrim == 5 => Function::NoOp,
			discrim if discrim == 6 => Function::Xor,
			discrim if discrim == 7 => Function::Or,

			discrim if discrim == 8 => Function::Nor,
			discrim if discrim == 9 => Function::Equiv,
			discrim if discrim == 10 => Function::Invert,
			discrim if discrim == 11 => Function::OrReverse,

			discrim if discrim == 12 => Function::CopyInverted,
			discrim if discrim == 13 => Function::OrInverted,
			discrim if discrim == 14 => Function::Nand,
			discrim if discrim == 15 => Function::Set,

			other_discrim => return Err(UnrecognizedDiscriminant(other_discrim as usize)),
		}))
	}
}

impl Writable for __Function {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(function) = self;

		match function {
			Function::Clear => buf.put_u32(0),
			Function::And => buf.put_u32(1),
			Function::AndReverse => buf.put_u32(2),
			Function::Copy => buf.put_u32(3),

			Function::AndInverted => buf.put_u32(4),
			Function::NoOp => buf.put_u32(5),
			Function::Xor => buf.put_u32(6),
			Function::Or => buf.put_u32(7),

			Function::Nor => buf.put_u32(8),
			Function::Equiv => buf.put_u32(9),
			Function::Invert => buf.put_u32(10),
			Function::OrReverse => buf.put_u32(11),

			Function::CopyInverted => buf.put_u32(12),
			Function::OrInverted => buf.put_u32(13),
			Function::Nand => buf.put_u32(14),
			Function::Set => buf.put_u32(15),
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __LineWidth(LineWidth);

impl ConstantX11Size for __LineWidth {
	const X11_SIZE: usize = 4;
}

impl X11Size for __LineWidth {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __LineWidth {
	#[allow(
		clippy::cast_possible_truncation,
		reason = "truncation is intended behavior if the width is too large for a u16 value"
	)]
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_u32() {
			discrim if discrim == 0 => LineWidth::Thin,
			other_width => LineWidth::Thick(other_width as u16),
		}))
	}
}

impl Writable for __LineWidth {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(line_width) = self;

		match line_width {
			LineWidth::Thin => buf.put_u32(0),
			LineWidth::Thick(width) => buf.put_u32(u32::from(*width)),
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __LineStyle(LineStyle);

impl ConstantX11Size for __LineStyle {
	const X11_SIZE: usize = 4;
}

impl X11Size for __LineStyle {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __LineStyle {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_u32() {
			discrim if discrim == 0 => LineStyle::Solid,
			discrim if discrim == 1 => LineStyle::OnOffDash,
			discrim if discrim == 2 => LineStyle::DoubleDash,

			other_discrim => return Err(UnrecognizedDiscriminant(other_discrim as usize)),
		}))
	}
}

impl Writable for __LineStyle {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(line_style) = self;

		match line_style {
			LineStyle::Solid => buf.put_u32(0),
			LineStyle::OnOffDash => buf.put_u32(1),
			LineStyle::DoubleDash => buf.put_u32(2),
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __CapStyle(CapStyle);

impl ConstantX11Size for __CapStyle {
	const X11_SIZE: usize = 4;
}

impl X11Size for __CapStyle {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __CapStyle {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_u32() {
			discrim if discrim == 0 => CapStyle::NotLast,
			discrim if discrim == 1 => CapStyle::Butt,
			discrim if discrim == 2 => CapStyle::Round,
			discrim if discrim == 3 => CapStyle::Projecting,

			other_discrim => return Err(UnrecognizedDiscriminant(other_discrim as usize)),
		}))
	}
}

impl Writable for __CapStyle {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(cap_style) = self;

		match cap_style {
			CapStyle::NotLast => buf.put_u32(0),
			CapStyle::Butt => buf.put_u32(1),
			CapStyle::Round => buf.put_u32(2),
			CapStyle::Projecting => buf.put_u32(3),
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __JoinStyle(JoinStyle);

impl ConstantX11Size for __JoinStyle {
	const X11_SIZE: usize = 4;
}

impl X11Size for __JoinStyle {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __JoinStyle {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_u32() {
			discrim if discrim == 0 => JoinStyle::Miter,
			discrim if discrim == 1 => JoinStyle::Round,
			discrim if discrim == 2 => JoinStyle::Bevel,

			other_discrim => return Err(UnrecognizedDiscriminant(other_discrim as usize)),
		}))
	}
}

impl Writable for __JoinStyle {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(join_style) = self;

		match join_style {
			JoinStyle::Miter => buf.put_u32(0),
			JoinStyle::Round => buf.put_u32(1),
			JoinStyle::Bevel => buf.put_u32(2),
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __FillStyle(FillStyle);

impl ConstantX11Size for __FillStyle {
	const X11_SIZE: usize = 4;
}

impl X11Size for __FillStyle {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __FillStyle {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_u32() {
			discrim if discrim == 0 => FillStyle::Solid,
			discrim if discrim == 1 => FillStyle::Tiled,
			discrim if discrim == 2 => FillStyle::Stippled,
			discrim if discrim == 3 => FillStyle::OpaqueStippled,

			other_discrim => return Err(UnrecognizedDiscriminant(other_discrim as usize)),
		}))
	}
}

impl Writable for __FillStyle {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(fill_style) = self;

		match fill_style {
			FillStyle::Solid => buf.put_u32(0),
			FillStyle::Tiled => buf.put_u32(1),
			FillStyle::Stippled => buf.put_u32(2),
			FillStyle::OpaqueStippled => buf.put_u32(3),
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __FillRule(FillRule);

impl ConstantX11Size for __FillRule {
	const X11_SIZE: usize = 4;
}

impl X11Size for __FillRule {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __FillRule {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_u32() {
			discrim if discrim == 0 => FillRule::EvenOdd,
			discrim if discrim == 1 => FillRule::Winding,

			other_discrim => return Err(UnrecognizedDiscriminant(other_discrim as usize)),
		}))
	}
}

impl Writable for __FillRule {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(fill_rule) = self;

		match fill_rule {
			FillRule::EvenOdd => buf.put_u32(0),
			FillRule::Winding => buf.put_u32(1),
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __SubwindowMode(SubwindowMode);

impl ConstantX11Size for __SubwindowMode {
	const X11_SIZE: usize = 4;
}

impl X11Size for __SubwindowMode {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __SubwindowMode {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_u32() {
			discrim if discrim == 0 => SubwindowMode::ClipByChildren,
			discrim if discrim == 1 => SubwindowMode::IncludeDescendents,

			other_discrim => return Err(UnrecognizedDiscriminant(other_discrim as usize)),
		}))
	}
}

impl Writable for __SubwindowMode {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(subwindow_mode) = self;

		match subwindow_mode {
			SubwindowMode::ClipByChildren => buf.put_u32(0),
			SubwindowMode::IncludeDescendents => buf.put_u32(1),
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __ArcMode(ArcMode);

impl ConstantX11Size for __ArcMode {
	const X11_SIZE: usize = 4;
}

impl X11Size for __ArcMode {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __ArcMode {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_u32() {
			discrim if discrim == 0 => ArcMode::Chord,
			discrim if discrim == 1 => ArcMode::PieSlice,

			other_discrim => return Err(UnrecognizedDiscriminant(other_discrim as usize)),
		}))
	}
}

impl Writable for __ArcMode {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(arc_mode) = self;

		match arc_mode {
			ArcMode::Chord => buf.put_u32(0),
			ArcMode::PieSlice => buf.put_u32(1),
		}

		Ok(())
	}
}

// }}}
