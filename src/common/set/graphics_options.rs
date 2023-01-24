// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xrbk_macro::{ConstantX11Size, Readable, Writable, X11Size};

use crate::{
	set::{__Px, __bool, __u8},
	unit::Px,
	visual::ColorId,
	Font,
	Pixmap,
};
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

/// Given a source and destination pixel, represents a bitwise operation applied
/// to the source and destination to determine the resultant pixel.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum Function {
	/// The resultant pixel is bitwise zero; that is, it has a `0` for each bit.
	Clear,
	/// The resultant pixel is the bitwise AND of the source and the
	/// destination: `source & destination`.
	And,
	/// The resultant pixel is the bitwise AND of the source and the NOT of
	/// the destination: `source & (!destination)`.
	AndReverse,
	/// The resultant pixel is simply the source pixel: `source`.
	Copy,

	/// The resultant pixel is the bitwise AND of the NOT of the source and
	/// the destination: `(!source) & destination`.
	AndInverted,
	/// The resultant pixel is simply the destination pixel: `destination`.
	NoOp,
	/// The resultant pixel is the bitwise XOR of the source and the
	/// destination: `source ^ destination`.
	Xor,
	/// The resultant pixel is the bitwise OR of the source and the destination
	/// `source | destination`.
	Or = 7,

	/// The resultant pixel is the bitwise AND of the NOT of the source and the
	/// NOT of the destination: `(!source) & (!destination)`.
	Nor,
	/// The resultant pixel is the bitwise XOR of the NOT of the source and the
	/// destination: `(!source) ^ destination`.
	Equiv,
	/// The resultant pixel is the bitwise NOT of the destination:
	/// `!destination`.
	Invert,
	/// The resultant pixel is the bitwise OR of the source and the NOT of the
	/// destination: `source | (!destination)`.
	OrReverse,

	/// The resultant pixel is the bitwise NOT of the source: `!source`.
	CopyInverted,
	/// The resultant pixel is the bitwise OR of the NOT of the source and the
	/// destination: `(!source) | destination`.
	OrInverted,
	/// The resultant pixel is the bitwise OR of the NOT of the source and the
	/// NOT of the destination: `(!source) | (!destination)`.
	Nand,
	/// The resultant pixel is bitwise one; that is, it has a `1` for each bit.
	Set,
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
	Solid,

	/// Only the even dashes are drawn.
	///
	/// The [`CapStyle`] applies to all internal ends of the individual dashes,
	/// with the exception of [`CapStyle::NotLast`], which is treated as
	/// [`CapStyle::Butt`].
	OnOffDash,

	/// The full path of the line is drawn, but the even dashes are [filled
	/// differently] than the odd dashes.
	///
	/// [`CapStyle::Butt`] is used where even and odd dashes meet.
	///
	/// [filled differently]: FillStyle
	DoubleDash,
}

/// Defines how the endpoints of a path are drawn.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum CapStyle {
	/// Equivalent to [`Butt`], except in the case of [`LineWidth::Thin`], where
	/// the final endpoint is not drawn.
	///
	/// [`Butt`]: CapStyle::Butt
	NotLast,

	/// The end of the path is square, perpendicular to the slope of the line,
	/// with no projection beyond the endpoint.
	Butt,

	/// The end of the path is a circular arc with a diameter equal to the
	/// [`LineWidth`], centered on the endpoint.
	///
	/// For [`LineWidth::Thin`], this is equivalent to [`Butt`].
	///
	/// [`Butt`]: CapStyle::Butt
	Round,

	/// The end of the path is square, but the path projects beyond the endpoint
	/// for a distance equal to half of the [`LineWidth`].
	///
	/// For [`LineWidth::Thin`], this is equivalent to [`Butt`].
	///
	/// [`Butt`]: CapStyle::Butt
	Projecting,
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
	Miter,

	/// A circular arc with a diameter equal to the [`LineWidth`] is centered on
	/// the joinpoint.
	Round,

	/// [`CapStyle::Butt`] endpoint styles are used, then the triangular notch
	/// is filled.
	Bevel,
}

/// Defines the contents of the source for line, text, and fill [requests].
///
/// [requests]: crate::x11::request
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum FillStyle {
	/// This is the [foreground color], except for the odd dashes of line
	/// requests with [`LineStyle::DoubleDash`], where it is the [background
	/// color].
	///
	/// [foreground color]: GraphicsOptions::foreground_color
	/// [background color]: GraphicsOptions::background_color
	Solid,

	// FIXME (docs): X11 protocol just says 'Tile' as description...
	/// A tiled `FillStyle`.
	Tiled,

	/// The foreground, masked by a stipple pattern.
	Stippled,

	/// Same as [`Stippled`], but the [foreground color] everywhere it has a
	/// one, and the [background color] everywhere it has a zero.
	///
	/// [foreground color]: GraphicsOptions::foreground_color
	/// [background color]: GraphicsOptions::background_color
	OpaqueStippled,
}

/// Defines what pixels are drawn for paths in [`FillPoly` requests].
///
/// [`FillPoly` requests]: crate::x11::request::FillPoly
// Hell if I know what the X11 protocol is talking about for these variants.
// Really technical language. I imagine it's simply not worth documenting.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum FillRule {
	EvenOdd,
	Winding,
}

/// Whether a source or destination [window] is clipped by its descendents.
///
/// [window]: crate::Window
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum ChildMode {
	/// Both source and destination [windows] are additionally clipped by all
	/// viewable [`InputOutput`] children.
	///
	/// [windows]: crate::Window
	/// [`InputOutput`]: crate::WindowClass::InputOutput
	ClipByChildren,

	/// Neither the source nor the destination [window] is clipped by their
	/// descendents.
	IncludeDescendents,
}

/// Controls filling in the [`PolyFillArc` request].
///
/// [`PolyFillArc` request]: crate::x11::request::PolyFillArc
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum ArcMode {
	/// Fills the shape created by tracing the arc and joining its endpoints in
	/// a straight line.
	Chord,

	/// Fills the shape created by tracing the arc and joining each of its
	/// endpoints to its center point.
	PieSlice,
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
/// |Option                   |Default value                                |
/// |-------------------------|---------------------------------------------|
/// |[`function`]             |[`Function::Copy`]                           |
/// |[`plane_mask`]           |`0xffff_ffff`                                |
/// |[`foreground_color`]     |[`ColorId::ZERO`]                            |
/// |[`background_color`]     |[`ColorId::ONE`]                             |
/// |[`line_width`]           |[`LineWidth::Thin`]                          |
/// |[`line_style`]           |[`LineStyle::Solid`]                         |
/// |[`cap_style`]            |[`CapStyle::Butt`]                           |
/// |[`join_style`]           |[`JoinStyle::Miter`]                         |
/// |[`fill_style`]           |[`FillStyle::Solid`]                         |
/// |[`fill_rule`]            |[`FillRule::EvenOdd`]                        |
/// |[`arc_mode`]             |[`ArcMode::PieSlice`]                        |
/// |[`tile`]                 |[Pixmap] filled with the [`foreground_color`]|
/// |[`stipple`]              |[Pixmap] filled with ones                    |
/// |[`tile_stipple_x`]       |`0`                                          |
/// |[`tile_stipple_y`]       |`0`                                          |
/// |[`font`]                 |Depends on the server                        |
/// |[`child_mode`]           |[`ChildMode::ClipByChildren`]                |
/// |[`graphics_exposure`]    |`true`                                       |
/// |[`clip_x`]               |`0`                                          |
/// |[`clip_y`]               |`0`                                          |
/// |[`clip_mask`]            |[`None`]                                     |
/// |[`dash_offset`]          |`0`                                          |
/// |[`dashes`]               |`4`                                          |
///
/// [Pixmap]: Pixmap
///
/// [`function`]: GraphicsOptions::function
/// [`plane_mask`]: GraphicsOptions::plane_mask
/// [`foreground_color`]: GraphicsOptions::foreground_color
/// [`background_color`]: GraphicsOptions::background_color
/// [`line_width`]: GraphicsOptions::line_width
/// [`line_style`]: GraphicsOptions::line_style
/// [`cap_style`]: GraphicsOptions::cap_style
/// [`join_style`]: GraphicsOptions::join_style
/// [`fill_style`]: GraphicsOptions::fill_style
/// [`fill_rule`]: GraphicsOptions::fill_rule
/// [`arc_mode`]: GraphicsOptions::arc_mode
/// [`tile`]: GraphicsOptions::tile
/// [`stipple`]: GraphicsOptions::stipple
/// [`tile_stipple_x`]: GraphicsOptions::tile_stipple_x
/// [`tile_stipple_y`]: GraphicsOptions::tile_stipple_y
/// [`font`]: GraphicsOptions::font
/// [`child_mode`]: GraphicsOptions::child_mode
/// [`graphics_exposure`]: GraphicsOptions::graphics_exposure
/// [`clip_x`]: GraphicsOptions::clip_x
/// [`clip_y`]: GraphicsOptions::clip_y
/// [`clip_mask`]: GraphicsOptions::clip_mask
/// [`dash_offset`]: GraphicsOptions::dash_offset
/// [`dashes`]: GraphicsOptions::dashes
pub struct GraphicsOptions {
	x11_size: usize,

	mask: GraphicsOptionsMask,

	function: Option<__Function>,

	plane_mask: Option<u32>,

	foreground_color: Option<ColorId>,
	background_color: Option<ColorId>,

	line_width: Option<__LineWidth>,

	line_style: Option<__LineStyle>,
	cap_style: Option<__CapStyle>,
	join_style: Option<__JoinStyle>,
	fill_style: Option<__FillStyle>,
	fill_rule: Option<__FillRule>,

	tile: Option<Pixmap>,
	stipple: Option<Pixmap>,

	tile_stipple_x: Option<__Px<i16>>,
	tile_stipple_y: Option<__Px<i16>>,

	font: Option<Font>,

	child_mode: Option<__ChildMode>,

	graphics_exposures: Option<__bool>,

	clip_x: Option<__Px<i16>>,
	clip_y: Option<__Px<i16>>,
	clip_mask: Option<ClipMask>,

	dash_offset: Option<__Px<u16>>,
	dashes: Option<__u8>,

	arc_mode: Option<__ArcMode>,
}

impl GraphicsOptions {
	/// Returns a new [`GraphicsOptionsBuilder`] with which a `GraphicsOptions`
	/// set can be created.
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

	foreground_color: Option<ColorId>,
	background_color: Option<ColorId>,

	line_width: Option<LineWidth>,

	line_style: Option<LineStyle>,
	cap_style: Option<CapStyle>,
	join_style: Option<JoinStyle>,
	fill_style: Option<FillStyle>,
	fill_rule: Option<FillRule>,

	tile: Option<Pixmap>,
	stipple: Option<Pixmap>,

	tile_stipple_x: Option<Px<i16>>,
	tile_stipple_y: Option<Px<i16>>,

	font: Option<Font>,

	child_mode: Option<ChildMode>,

	graphics_exposures: Option<bool>,

	clip_x: Option<Px<i16>>,
	clip_y: Option<Px<i16>>,
	clip_mask: Option<ClipMask>,

	dash_offset: Option<Px<u16>>,
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

			foreground_color: None,
			background_color: None,

			line_width: None,

			line_style: None,
			cap_style: None,
			join_style: None,
			fill_style: None,
			fill_rule: None,

			tile: None,
			stipple: None,

			tile_stipple_x: None,
			tile_stipple_y: None,

			font: None,

			child_mode: None,

			graphics_exposures: None,

			clip_x: None,
			clip_y: None,
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

			foreground_color: self.foreground_color,
			background_color: self.background_color,

			line_width: self.line_width.map(__LineWidth),

			line_style: self.line_style.map(__LineStyle),
			cap_style: self.cap_style.map(__CapStyle),
			join_style: self.join_style.map(__JoinStyle),
			fill_style: self.fill_style.map(__FillStyle),
			fill_rule: self.fill_rule.map(__FillRule),

			tile: self.tile,
			stipple: self.stipple,

			tile_stipple_x: self.tile_stipple_x.map(__Px),
			tile_stipple_y: self.tile_stipple_y.map(__Px),

			font: self.font,

			child_mode: self.child_mode.map(__ChildMode),

			graphics_exposures: self.graphics_exposures.map(__bool),

			clip_x: self.clip_x.map(__Px),
			clip_y: self.clip_y.map(__Px),
			clip_mask: self.clip_mask,

			dash_offset: self.dash_offset.map(__Px),
			dashes: self.dashes.map(__u8),

			arc_mode: self.arc_mode.map(__ArcMode),
		}
	}
}

impl GraphicsOptionsBuilder {
	/// Configures the bitwise operation used to determine the resultant pixels
	/// in a graphics operation.
	///
	/// See [`GraphicsOptions::function`] for more information.
	pub fn function(&mut self, function: Function) -> &mut Self {
		if self.function.is_none() {
			self.x11_size += 4;
		}

		self.function = Some(function);
		self.mask |= GraphicsOptionsMask::FUNCTION;

		self
	}

	/// Configures the mask of bit planes through which a graphics operation is
	/// applied.
	///
	/// See [`GraphicsOptions::plane_mask`] for more information.
	pub fn plane_mask(&mut self, plane_mask: u32) -> &mut Self {
		if self.plane_mask.is_none() {
			self.x11_size += 4;
		}

		self.plane_mask = Some(plane_mask);
		self.mask |= GraphicsOptionsMask::PLANE_MASK;

		self
	}

	/// Configures the [foreground color] which is used in graphics operations.
	///
	/// See [`GraphicsOptions::foreground_color`] for more information.
	///
	/// [foreground color]: GraphicsOptions::foreground_color
	pub fn foreground_color(&mut self, foreground_color: ColorId) -> &mut Self {
		if self.foreground_color.is_none() {
			self.x11_size += 4;
		}

		self.foreground_color = Some(foreground_color);
		self.mask |= GraphicsOptionsMask::FOREGROUND_COLOR;

		self
	}
	/// Configures the [background color] which is used in graphics operations.
	///
	/// See [`GraphicsOptions::background_color`] for more information.
	///
	/// [background color]: GraphicsOptions::background_color
	pub fn background_color(&mut self, background_color: ColorId) -> &mut Self {
		if self.background_color.is_none() {
			self.x11_size += 4;
		}

		self.background_color = Some(background_color);
		self.mask |= GraphicsOptionsMask::BACKGROUND_COLOR;

		self
	}

	/// Configures the [width of lines] drawn with graphics operations.
	///
	/// See [`GraphicsOptions::line_width`] for more information.
	///
	/// [width of lines]: GraphicsOptions::line_width
	pub fn line_width(&mut self, line_width: LineWidth) -> &mut Self {
		if self.line_width.is_none() {
			self.x11_size += 4;
		}

		self.line_width = Some(line_width);
		self.mask |= GraphicsOptionsMask::LINE_WIDTH;

		self
	}

	/// Configures the [line style] used in graphics operations.
	///
	/// See [`GraphicsOptions::line_style`] for more information.
	///
	/// [line style]: GraphicsOptions::line_style
	pub fn line_style(&mut self, line_style: LineStyle) -> &mut Self {
		if self.line_style.is_none() {
			self.x11_size += 4;
		}

		self.line_style = Some(line_style);
		self.mask |= GraphicsOptionsMask::LINE_STYLE;

		self
	}
	/// Configures the [cap style] used in graphics operations.
	///
	/// See [`GraphicsOptions::cap_style`] for more information.
	///
	/// [cap style]: GraphicsOptions::cap_style
	pub fn cap_style(&mut self, cap_style: CapStyle) -> &mut Self {
		if self.cap_style.is_none() {
			self.x11_size += 4;
		}

		self.cap_style = Some(cap_style);
		self.mask |= GraphicsOptionsMask::CAP_STYLE;

		self
	}
	/// Configures the [join style] used in graphics operations.
	///
	/// See [`GraphicsOptions::join_style`] for more information.
	///
	/// [join style]: GraphicsOptions::join_style
	pub fn join_style(&mut self, join_style: JoinStyle) -> &mut Self {
		if self.join_style.is_none() {
			self.x11_size += 4;
		}

		self.join_style = Some(join_style);
		self.mask |= GraphicsOptionsMask::JOIN_STYLE;

		self
	}
	/// Configures the [fill style] used in graphics operations.
	///
	/// See [`GraphicsOptions::fill_style`] for more information.
	///
	/// [fill style]: GraphicsOptions::fill_style
	pub fn fill_style(&mut self, fill_style: FillStyle) -> &mut Self {
		if self.fill_style.is_none() {
			self.x11_size += 4;
		}

		self.fill_style = Some(fill_style);
		self.mask |= GraphicsOptionsMask::FILL_STYLE;

		self
	}
	/// Configures the [fill rule] used in graphics operations.
	///
	/// See [`GraphicsOptions::fill_rule`] for more information.
	///
	/// [fill rule]: GraphicsOptions::fill_rule
	pub fn fill_rule(&mut self, fill_rule: FillRule) -> &mut Self {
		if self.fill_rule.is_none() {
			self.x11_size += 4;
		}

		self.fill_rule = Some(fill_rule);
		self.mask |= GraphicsOptionsMask::FILL_RULE;

		self
	}

	/// Configures the [tile] [`Pixmap`] used in graphics operations.
	///
	/// See [`GraphicsOptions::tile`] for more information.
	///
	/// [tile]: GraphicsOptions::tile
	pub fn tile(&mut self, tile: Pixmap) -> &mut Self {
		if self.tile.is_none() {
			self.x11_size += 4;
		}

		self.tile = Some(tile);
		self.mask |= GraphicsOptionsMask::TILE;

		self
	}
	/// Configures the [stipple] [`Pixmap`] used in graphics operations.
	///
	/// See [`GraphicsOptions::stipple`] for more information.
	///
	/// [stipple]: GraphicsOptions::stipple
	pub fn stipple(&mut self, stipple: Pixmap) -> &mut Self {
		if self.stipple.is_none() {
			self.x11_size += 4;
		}

		self.stipple = Some(stipple);
		self.mask |= GraphicsOptionsMask::STIPPLE;

		self
	}

	/// Configures the [x coordinate of the top-left corner][x] of the [tile] or
	/// [stipple] [`Pixmap`] used in graphics operations.
	///
	/// See [`GraphicsOptions::tile_stipple_x`] for more information.
	///
	/// [x]: GraphicsOptions::tile_stipple_x
	/// [tile]: GraphicsOptions::tile
	/// [stipple]: GraphicsOptions::stipple
	pub fn tile_stipple_x(&mut self, tile_stipple_x: Px<i16>) -> &mut Self {
		if self.tile_stipple_x.is_none() {
			self.x11_size += 4;
		}

		self.tile_stipple_x = Some(tile_stipple_x);
		self.mask |= GraphicsOptionsMask::TILE_STIPPLE_X;

		self
	}
	/// Configures the [y coordinate of the top-left corner][y] of the [tile] or
	/// [stipple] [`Pixmap`] used in graphics operations.
	///
	/// See [`GraphicsOptions::tile_stipple_y`] for more information.
	///
	/// [y]: GraphicsOptions::tile_stipple_y
	/// [tile]: GraphicsOptions::tile
	/// [stipple]: GraphicsOptions::stipple
	pub fn tile_stipple_y(&mut self, tile_stipple_y: Px<i16>) -> &mut Self {
		if self.tile_stipple_y.is_none() {
			self.x11_size += 4;
		}

		self.tile_stipple_y = Some(tile_stipple_y);
		self.mask |= GraphicsOptionsMask::TILE_STIPPLE_Y;

		self
	}

	/// Configures the [font] used for graphics operations involving text.
	///
	/// See [`GraphicsOptions::font`] for more information.
	///
	/// [font]: GraphicsOptions::font
	pub fn font(&mut self, font: Font) -> &mut Self {
		if self.font.is_none() {
			self.x11_size += 4;
		}

		self.font = Some(font);
		self.mask |= GraphicsOptionsMask::FONT;

		self
	}

	/// Configures whether descendent [windows] are included or masked out when
	/// considering graphics operations.
	///
	/// See [`GraphicsOptions::child_mode`] for more information.
	///
	/// [windows]: crate::Window
	pub fn child_mode(&mut self, child_mode: ChildMode) -> &mut Self {
		if self.child_mode.is_none() {
			self.x11_size += 4;
		}

		self.child_mode = Some(child_mode);
		self.mask |= GraphicsOptionsMask::CHILD_MODE;

		self
	}

	/// Configures whether [`GraphicsExposure` events] are generated when using
	/// graphics operations.
	///
	/// See [`GraphicsOptions::graphics_exposure`] for more information.
	///
	/// [`GraphicsExposure` events]: crate::x11::event::GraphicsExposure
	pub fn graphics_exposure(&mut self, graphics_exposure: bool) -> &mut Self {
		if self.graphics_exposures.is_none() {
			self.x11_size += 4;
		}

		self.graphics_exposures = Some(graphics_exposure);
		self.mask |= GraphicsOptionsMask::GRAPHICS_EXPOSURE;

		self
	}

	/// Configures the [x coordinate of the top-left corner][x] of the
	/// [`clip_mask`].
	///
	/// See [`GraphicsOptions::clip_x`] for more information.
	///
	/// [x]: GraphicsOptions::clip_x
	pub fn clip_x(&mut self, clip_x: Px<i16>) -> &mut Self {
		if self.clip_x.is_none() {
			self.x11_size += 4;
		}

		self.clip_x = Some(clip_x);
		self.mask |= GraphicsOptionsMask::CLIP_X;

		self
	}
	/// Configures the [y coordinate of the top-left corner][y] of the
	/// [`clip_mask`].
	///
	/// See [`GraphicsOptions::clip_y`] for more information.
	///
	/// [y]: GraphicsOptions::clip_y
	pub fn clip_y(&mut self, clip_y: Px<i16>) -> &mut Self {
		if self.clip_y.is_none() {
			self.x11_size += 4;
		}

		self.clip_y = Some(clip_y);
		self.mask |= GraphicsOptionsMask::CLIP_Y;

		self
	}
	/// Configures the [`clip_mask`] used in graphics operations.
	///
	/// See [`GraphicsOptions::clip_mask`] for more information.
	///
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	pub fn clip_mask(&mut self, clip_mask: ClipMask) -> &mut Self {
		if self.clip_mask.is_none() {
			self.x11_size += 4;
		}

		self.clip_mask = Some(clip_mask);
		self.mask |= GraphicsOptionsMask::CLIP_MASK;

		self
	}

	/// Configures the [`dash_offset`] used in graphics operations.
	///
	/// See [`GraphicsOptions::dash_offset`] for more information.
	///
	/// [`dash_offset`]: GraphicsOptions::dash_offset
	pub fn dash_offset(&mut self, dash_offset: Px<u16>) -> &mut Self {
		if self.dash_offset.is_none() {
			self.x11_size += 4;
		}

		self.dash_offset = Some(dash_offset);
		self.mask |= GraphicsOptionsMask::DASH_OFFSET;

		self
	}
	/// Configures the length of [`dashes`] used in graphics operations.
	///
	/// See [`GraphicsOptions::dashes`] for more information.
	///
	/// [`dashes`]: GraphicsOptions::dashes
	pub fn dashes(&mut self, dashes: u8) -> &mut Self {
		if self.dashes.is_none() {
			self.x11_size += 4;
		}

		self.dashes = Some(dashes);
		self.mask |= GraphicsOptionsMask::DASHES;

		self
	}

	/// Configures the [mode used to draw arcs] in a [`PolyFillArc` request].
	///
	/// See [`GraphicsOptions::arc_mode`] for more information.
	///
	/// [mode used to draw arcs]: ArcMode
	/// [`PolyFillArc` request]: crate::x11::request::PolyFillArc
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
	/// The bitwise operation applied to the source pixel and the destination
	/// pixel [colors][color] to determine the resultant pixel [color].
	///
	/// See [`Function`] for information on each function and its meaning.
	///
	/// [color]: ColorId
	#[must_use]
	pub fn function(&self) -> Option<&Function> {
		self.function.as_ref().map(|__Function(function)| function)
	}

	/// The mask applied to the graphics operation's effects on bit planes.
	///
	/// The [`function`] is only applied to colors masked by this mask.
	///
	/// [`function`]: GraphicsOptions::function
	#[must_use]
	pub const fn plane_mask(&self) -> Option<&u32> {
		self.plane_mask.as_ref()
	}

	/// The foreground color used in graphics operations.
	#[must_use]
	pub const fn foreground_color(&self) -> Option<&ColorId> {
		self.foreground_color.as_ref()
	}
	/// The background color used in graphics operations.
	#[must_use]
	pub const fn background_color(&self) -> Option<&ColorId> {
		self.background_color.as_ref()
	}

	/// The [`LineWidth`] used when drawing lines.
	///
	/// [`LineWidth::Thin`] means the thinnest possible [`LineWidth`] which can
	/// be displayed (think lines used in wireframes).
	#[must_use]
	pub fn line_width(&self) -> Option<&LineWidth> {
		self.line_width
			.as_ref()
			.map(|__LineWidth(line_width)| line_width)
	}

	/// The sections of a line which are drawn.
	///
	/// See [`LineStyle`] for more information.
	#[must_use]
	pub fn line_style(&self) -> Option<&LineStyle> {
		self.line_style
			.as_ref()
			.map(|__LineStyle(line_style)| line_style)
	}
	/// Defines how the endpoints of a line are drawn.
	///
	/// See [`CapStyle`] for more information.
	#[must_use]
	pub fn cap_style(&self) -> Option<&CapStyle> {
		self.cap_style
			.as_ref()
			.map(|__CapStyle(cap_style)| cap_style)
	}
	/// Defines how the corners of [`LineWidth::Thick`] lines are drawn.
	///
	/// See [`JoinStyle`] for more information.
	#[must_use]
	pub fn join_style(&self) -> Option<&JoinStyle> {
		self.join_style
			.as_ref()
			.map(|__JoinStyle(join_style)| join_style)
	}
	/// Defines the contents of the source for line, text, and fill requests.
	///
	/// See [`FillStyle`] for more information.
	#[must_use]
	pub fn fill_style(&self) -> Option<&FillStyle> {
		self.fill_style
			.as_ref()
			.map(|__FillStyle(fill_style)| fill_style)
	}
	/// Defines which pixels are drawn for paths in [`FillPoly` requests].
	///
	/// See [`FillRule`] for more information.
	///
	/// [`FillPoly` requests]: crate::x11::request::FillPoly
	#[must_use]
	pub fn fill_rule(&self) -> Option<&FillRule> {
		self.fill_rule
			.as_ref()
			.map(|__FillRule(fill_rule)| fill_rule)
	}

	/// The [pixmap] which is tiled in graphics operations.
	///
	/// [pixmap]: Pixmap
	#[must_use]
	pub const fn tile(&self) -> Option<&Pixmap> {
		self.tile.as_ref()
	}
	/// The [pixmap] which is stippled in graphics operations.
	///
	/// [pixmap]: Pixmap
	#[must_use]
	pub const fn stipple(&self) -> Option<&Pixmap> {
		self.stipple.as_ref()
	}

	/// The x coordinate of the top-left corner of the [tile] or [stipple]
	/// [`Pixmap`], relative to the [drawable]'s top-left corner.
	///
	/// [tile]: GraphicsOptions::tile
	/// [stipple]: GraphicsOptions::stipple
	///
	/// [drawable]: crate::Drawable
	#[must_use]
	pub fn tile_stipple_x(&self) -> Option<&Px<i16>> {
		self.tile_stipple_x.as_ref().map(|__Px(x)| x)
	}
	/// The y coordinate of the top-left corner of the [tile] or [stipple]
	/// [`Pixmap`], relative to the [drawable]'s top-left corner.
	///
	/// [tile]: GraphicsOptions::tile
	/// [stipple]: GraphicsOptions::stipple
	///
	/// [drawable]: crate::Drawable
	#[must_use]
	pub fn tile_stipple_y(&self) -> Option<&Px<i16>> {
		self.tile_stipple_y.as_ref().map(|__Px(y)| y)
	}

	/// The [font] used for text.
	///
	/// [font]: Font
	#[must_use]
	pub const fn font(&self) -> Option<&Font> {
		self.font.as_ref()
	}

	/// Whether descendent [windows] are included or masked out when applying
	/// graphics operations.
	///
	/// See [`ChildMode`] for more information.
	///
	/// [windows]: crate::Window
	#[must_use]
	pub fn child_mode(&self) -> Option<&ChildMode> {
		self.child_mode
			.as_ref()
			.map(|__ChildMode(child_mode)| child_mode)
	}

	/// Whether [`GraphicsExposure` events] are generated.
	///
	/// [`GraphicsExposure` events]: crate::x11::event::GraphicsExposure
	#[must_use]
	pub fn graphics_exposure(&self) -> Option<&bool> {
		self.graphics_exposures.as_ref().map(|__bool(bool)| bool)
	}

	/// The x coordinate of the top-left corner of the [`clip_mask`], relative
	/// to the [drawable]'s top-left corner.
	///
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	/// [drawable]: crate::Drawable
	#[must_use]
	pub fn clip_x(&self) -> Option<&Px<i16>> {
		self.clip_x.as_ref().map(|__Px(x)| x)
	}
	/// The y coordinate of the top-left corner of the [`clip_mask`], relative
	/// to the [drawable]'s top-left corner.
	///
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	/// [drawable]: crate::Drawable
	#[must_use]
	pub fn clip_y(&self) -> Option<&Px<i16>> {
		self.clip_y.as_ref().map(|__Px(y)| y)
	}
	/// A mask applied to the [drawable] when using graphics operations.
	///
	/// Graphics operations will only have an effect where this mask is set.
	///
	/// [drawable]: crate::Drawable
	#[must_use]
	pub const fn clip_mask(&self) -> Option<&ClipMask> {
		self.clip_mask.as_ref()
	}

	/// The offset from the endpoints or joinpoints of a line from which dashes
	/// are drawn.
	#[must_use]
	pub fn dash_offset(&self) -> Option<&Px<u16>> {
		self.dash_offset
			.as_ref()
			.map(|__Px(dash_offset)| dash_offset)
	}
	/// Specifies the length of dashes used in [`LineStyle::DoubleDash`] and
	/// [`LineStyle::OnOffDash`] lines.
	#[must_use]
	pub fn dashes(&self) -> Option<&u8> {
		self.dashes.as_ref().map(|__u8(dashes)| dashes)
	}

	/// Specified the mode with which [arcs] are drawn in
	/// [`PolyFillArc` requests].
	///
	/// See [`ArcMode`] for more information.
	///
	/// [arcs]: crate::Arc
	/// [`PolyFillArc` requests]: crate::x11::request::PolyFillArc
	#[must_use]
	pub fn arc_mode(&self) -> Option<&ArcMode> {
		self.arc_mode.as_ref().map(|__ArcMode(arc_mode)| arc_mode)
	}
}

bitflags! {
	/// A mask of configured options for a [`GraphicsContext`].
	///
	/// This mask is used in the [`GraphicsOptions` set], as well as in the
	/// [`CopyGraphicsContext` request] to specify which options are copied.
	///
	/// [`GraphicsContext`]: crate::GraphicsContext
	/// [`CopyGraphicsContext` request]: crate::x11::request::CopyGraphicsContext
	/// [`GraphicsOptions` set]: GraphicsOptions
	#[derive(Default, X11Size, Readable, ConstantX11Size, Writable)]
	pub struct GraphicsOptionsMask: u32 {
		/// Whether the [function] applied to determine the resultant pixel in a
		/// graphics [request] is configured in the [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::function`] for more information.
		///
		/// [function]: GraphicsOptions::function
		/// [`GraphicsContext`]: crate::GraphicsContext
		/// [request]: crate::message::Request
		const FUNCTION = 0x0000_0001;

		/// Whether the [plane mask] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::plane_mask`] for more information.
		///
		/// [plane mask]: GraphicsOptions::plane_mask
		/// [`GraphicsContext`]: crate::GraphicsContext
		const PLANE_MASK = 0x0000_0002;

		/// Whether the [foreground color] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::foreground_color`] for more information.
		///
		/// [foreground color]: GraphicsOptions::foreground_color
		/// [`GraphicsContext`]: crate::GraphicsContext
		const FOREGROUND_COLOR = 0x0000_0004;
		/// Whether the [background color] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::background_color`] for more information.
		///
		/// [background color]: GraphicsOptions::background_color
		/// [`GraphicsContext`]: crate::GraphicsContext
		const BACKGROUND_COLOR = 0x0000_0008;

		/// Whether the [width of drawn lines][width] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::line_width`] for more information.
		///
		/// [width]: GraphicsOptions::line_width
		/// [`GraphicsContext`]: crate::GraphicsContext
		const LINE_WIDTH = 0x0000_0010;

		/// Whether the [line style] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::line_style`] for more information.
		///
		/// [line style]: GraphicsOptions::line_style
		/// [`GraphicsContext`]: crate::GraphicsContext
		const LINE_STYLE = 0x0000_0020;
		/// Whether the [cap style] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::cap_style`] for more information.
		///
		/// [cap style]: GraphicsOptions::cap_style
		/// [`GraphicsContext`]: crate::GraphicsContext
		const CAP_STYLE = 0x0000_0040;
		/// Whether the [join style] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::join_style`] for more information.
		///
		/// [join style]: GraphicsOptions::join_style
		/// [`GraphicsContext`]: crate::GraphicsContext
		const JOIN_STYLE = 0x0000_0080;
		/// Whether the [fill style] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::fill_style`] for more information.
		///
		/// [fill style]: GraphicsOptions::fill_style
		/// [`GraphicsContext`]: crate::GraphicsContext
		const FILL_STYLE = 0x0000_0100;
		/// Whether the [fill rule] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::fill_rule`] for more information.
		///
		/// [fill rule]: GraphicsOptions::fill_rule
		/// [`GraphicsContext`]: crate::GraphicsContext
		const FILL_RULE = 0x0000_0200;

		/// Whether the [pixmap] which is [tiled] in a graphics operation is
		/// configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::tile`] for more information.
		///
		/// [pixmap]: Pixmap
		/// [tiled]: GraphicsOptions::tile
		/// [`GraphicsContext`]: crate::GraphicsContext
		const TILE = 0x0000_0400;
		/// Whether the [pixmap] which is [stippled] in a graphics operation is
		/// configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::stipple`] for more information.
		///
		/// [pixmap]: Pixmap
		/// [stippled]: GraphicsOptions::stipple
		/// [`GraphicsContext`]: crate::GraphicsContext
		const STIPPLE = 0x0000_0800;

		/// Whether the [x coordinate of the top-left corner][x] of the [tiled]
		/// or [stippled] [`Pixmap`] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::tile_stipple_x`] for more information.
		///
		/// [x]: GraphicsOptions::tile_stipple_x
		/// [tiled]: GraphicsOptions::tile
		/// [stippled]: GraphicsOptions::stipple
		/// [`GraphicsContext`]: crate::GraphicsContext
		const TILE_STIPPLE_X = 0x0000_1000;
		/// Whether the [y coordinate of the top-left corner][y] of the [tiled]
		/// or [stippled] [`Pixmap`] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::tile_stipple_y`] for more information.
		///
		/// [y]: GraphicsOptions::tile_stipple_y
		/// [tiled]: GraphicsOptions::tile
		/// [stippled]: GraphicsOptions::stipple
		/// [`GraphicsContext`]: crate::GraphicsContext
		const TILE_STIPPLE_Y = 0x0000_2000;

		/// Whether the [font] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::font`] for more information.
		///
		/// [font]: GraphicsOptions::font
		/// [`GraphicsContext`]: crate::GraphicsContext
		const FONT = 0x0000_4000;

		/// Whether the[descendent windows being clipped when applying graphics options][child]
		/// is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::child_mode`] for more information.
		///
		/// [child]: GraphicsOptions::child_mode
		/// [`GraphicsContext`]: crate::GraphicsContext
		const CHILD_MODE = 0x0000_8000;

		/// Whether [`GraphicsExposure` events] are configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::graphics_exposure`] for more information.
		///
		/// [`GraphicsExposure` events]: crate::x11::event::GraphicsExposure
		/// [`GraphicsContext`]: crate::GraphicsContext
		const GRAPHICS_EXPOSURE = 0x0001_0000;

		/// Whether the [x coordinate of the top left corner][x] of the
		/// [`clip_mask`] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::clip_x`] for more information.
		///
		/// [x]: GraphicsOptions::clip_x
		/// [`clip_mask`]: GraphicsOptions::clip_mask
		/// [`GraphicsContext`]: crate::GraphicsContext
		const CLIP_X = 0x0002_0000;
		/// Whether the [y coordinate of the top left corner][y] of the
		/// [`clip_mask`] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::clip_y`] for more information.
		///
		/// [y]: GraphicsOptions::clip_y
		/// [`clip_mask`]: GraphicsOptions::clip_mask
		/// [`GraphicsContext`]: crate::GraphicsContext
		const CLIP_Y = 0x0004_0000;
		/// Whether the [`clip_mask`] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::clip_mask`] for more information.
		///
		/// [`clip_mask`]: GraphicsOptions::clip_mask
		/// [`GraphicsContext`]: crate::GraphicsContext
		const CLIP_MASK = 0x0008_0000;

		/// Whether the [dash offset] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::dash_offset`] for more information.
		///
		/// [dash offset]: GraphicsOptions::dash_offset
		/// [`GraphicsContext`]: crate::GraphicsContext
		const DASH_OFFSET = 0x0010_0000;
		/// Whether the [dashes] are configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::dashes`] for more information.
		///
		/// [dashes]: GraphicsOptions::dashes
		/// [`GraphicsContext`]: crate::GraphicsContext
		const DASHES = 0x0020_0000;

		/// Whether the [arc mode] is configured in a [`GraphicsContext`].
		///
		/// See [`GraphicsOptions::arc_mode`] for more information.
		///
		/// [arc mode]: GraphicsOptions::arc_mode
		/// [`GraphicsContext`]: crate::GraphicsContext
		const ARC_MODE = 0x0040_0000;
	}
}

// impl XRBK traits for GraphicsOptions {{{

impl X11Size for GraphicsOptions {
	fn x11_size(&self) -> usize {
		self.x11_size
	}
}

impl Readable for GraphicsOptions {
	#[allow(clippy::too_many_lines)]
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

		let foreground_color = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::FOREGROUND_COLOR),
		)?;
		let background_color = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::BACKGROUND_COLOR),
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

		let tile_stipple_x = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::TILE_STIPPLE_X),
		)?;
		let tile_stipple_y = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::TILE_STIPPLE_Y),
		)?;

		let font =
			super::read_set_value(buf, &mut x11_size, mask.contains(GraphicsOptionsMask::FONT))?;

		let child_mode = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::CHILD_MODE),
		)?;

		let graphics_exposures = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::GRAPHICS_EXPOSURE),
		)?;

		let clip_x = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::CLIP_X),
		)?;
		let clip_y = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionsMask::CLIP_Y),
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

			foreground_color,
			background_color,

			line_width,

			line_style,
			cap_style,
			join_style,
			fill_style,
			fill_rule,

			tile,
			stipple,

			tile_stipple_x,
			tile_stipple_y,

			font,

			child_mode,

			graphics_exposures,

			clip_x,
			clip_y,
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

		if let Some(foreground_color) = &self.foreground_color {
			foreground_color.write_to(buf)?;
		}
		if let Some(background_color) = &self.background_color {
			background_color.write_to(buf)?;
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

		if let Some(tile_stipple_x) = &self.tile_stipple_x {
			tile_stipple_x.write_to(buf)?;
		}
		if let Some(tile_stipple_y) = &self.tile_stipple_y {
			tile_stipple_y.write_to(buf)?;
		}

		if let Some(font) = &self.font {
			font.write_to(buf)?;
		}

		if let Some(child_mode) = &self.child_mode {
			child_mode.write_to(buf)?;
		}

		if let Some(graphics_exposures) = &self.graphics_exposures {
			graphics_exposures.write_to(buf)?;
		}

		if let Some(clip_x) = &self.clip_x {
			clip_x.write_to(buf)?;
		}
		if let Some(clip_y) = &self.clip_y {
			clip_y.write_to(buf)?;
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
		reason = "truncation is intended behavior"
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
struct __ChildMode(ChildMode);

impl ConstantX11Size for __ChildMode {
	const X11_SIZE: usize = 4;
}

impl X11Size for __ChildMode {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __ChildMode {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_u32() {
			discrim if discrim == 0 => ChildMode::ClipByChildren,
			discrim if discrim == 1 => ChildMode::IncludeDescendents,

			other_discrim => return Err(UnrecognizedDiscriminant(other_discrim as usize)),
		}))
	}
}

impl Writable for __ChildMode {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(child_mode) = self;

		match child_mode {
			ChildMode::ClipByChildren => buf.put_u32(0),
			ChildMode::IncludeDescendents => buf.put_u32(1),
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
