// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xrbk_macro::{ConstantX11Size, Readable, Writable, X11Size};

use crate::{common::set::__bool, Font, Pixel, Pixmap};
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
	pub struct GraphicsOptionMask: u32 {
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

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum Function {
	Clear = 0,
	And = 1,
	AndReverse = 2,
	Copy = 3,

	AndInverted = 4,
	NoOp = 5,
	Xor = 6,
	Or = 7,

	Nor = 8,
	Equiv = 9,
	Invert = 10,
	OrReverse = 11,

	CopyInverted = 12,
	OrInverted = 13,
	Nand = 14,
	Set = 15,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum LineStyle {
	Solid = 0,
	OnOffDash = 1,
	DoubleDash = 2,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum CapStyle {
	NotLast = 0,
	Butt = 1,
	Round = 2,
	Projecting = 3,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum JoinStyle {
	Miter = 0,
	Round = 1,
	Bevel = 2,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum FillStyle {
	Solid = 0,
	Tiled = 1,
	Stippled = 2,
	OpaqueStippled = 3,
}

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

pub type ClipMask = Option<Pixmap>;

pub struct GraphicsOptions {
	x11_size: usize,

	mask: GraphicsOptionMask,

	function: Option<__Function>,

	plane_mask: Option<u32>,

	foreground: Option<Pixel>,
	background: Option<Pixel>,

	// This represents a `u16` value.
	line_width: Option<u32>,

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
	#[allow(clippy::similar_names)]
	pub fn new(
		// {{{
		function: Option<Function>,
		plane_mask: Option<u32>,
		foreground: Option<Pixel>,
		background: Option<Pixel>,
		line_width: Option<u16>,
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
		graphics_exposure: Option<bool>,
		clip_x_origin: Option<i16>,
		clip_y_origin: Option<i16>,
		clip_mask: Option<ClipMask>,
		dash_offset: Option<u16>,
		dashes: Option<u8>,
		arc_mode: Option<ArcMode>,
	) -> Self {
		let mut mask = GraphicsOptionMask::empty();
		let mut x11_size = GraphicsOptionMask::X11_SIZE;

		if function.is_some() {
			x11_size += __Function::X11_SIZE;
			mask |= GraphicsOptionMask::FUNCTION;
		}

		if let Some(plane_mask) = &plane_mask {
			x11_size += plane_mask.x11_size();
			mask |= GraphicsOptionMask::PLANE_MASK;
		}

		if let Some(foreground) = &foreground {
			x11_size += foreground.x11_size();
			mask |= GraphicsOptionMask::FOREGROUND;
		}
		if let Some(background) = &background {
			x11_size += background.x11_size();
			mask |= GraphicsOptionMask::BACKGROUND;
		}

		if let Some(line_width) = &line_width {
			x11_size += line_width.x11_size();
			mask |= GraphicsOptionMask::LINE_WIDTH;
		}

		if line_style.is_some() {
			x11_size += __LineStyle::X11_SIZE;
			mask |= GraphicsOptionMask::LINE_STYLE;
		}
		if cap_style.is_some() {
			x11_size += __CapStyle::X11_SIZE;
			mask |= GraphicsOptionMask::CAP_STYLE;
		}
		if join_style.is_some() {
			x11_size += __JoinStyle::X11_SIZE;
			mask |= GraphicsOptionMask::JOIN_STYLE;
		}
		if fill_style.is_some() {
			x11_size += __FillStyle::X11_SIZE;
			mask |= GraphicsOptionMask::FILL_STYLE;
		}
		if fill_rule.is_some() {
			x11_size += __FillRule::X11_SIZE;
			mask |= GraphicsOptionMask::FILL_RULE;
		}

		if let Some(tile) = &tile {
			x11_size += tile.x11_size();
			mask |= GraphicsOptionMask::TILE;
		}
		if let Some(stipple) = &stipple {
			x11_size += stipple.x11_size();
			mask |= GraphicsOptionMask::STIPPLE;
		}

		if tile_stipple_x_origin.is_some() {
			x11_size += i32::X11_SIZE;
			mask |= GraphicsOptionMask::TILE_STIPPLE_X_ORIGIN;
		}
		if tile_stipple_y_origin.is_some() {
			x11_size += i32::X11_SIZE;
			mask |= GraphicsOptionMask::TILE_STIPPLE_Y_ORIGIN;
		}

		if let Some(font) = &font {
			x11_size += font.x11_size();
			mask |= GraphicsOptionMask::FONT;
		}

		if subwindow_mode.is_some() {
			x11_size += __SubwindowMode::X11_SIZE;
			mask |= GraphicsOptionMask::SUBWINDOW_MODE;
		}

		if graphics_exposure.is_some() {
			x11_size += __bool::X11_SIZE;
			mask |= GraphicsOptionMask::GRAPHICS_EXPOSURES;
		}

		if clip_x_origin.is_some() {
			x11_size += i32::X11_SIZE;
			mask |= GraphicsOptionMask::CLIP_X_ORIGIN;
		}
		if clip_y_origin.is_some() {
			x11_size += i32::X11_SIZE;
			mask |= GraphicsOptionMask::CLIP_Y_ORIGIN;
		}
		if let Some(clip_mask) = &clip_mask {
			x11_size += clip_mask.x11_size();
			mask |= GraphicsOptionMask::CLIP_MASK;
		}

		if dash_offset.is_some() {
			x11_size += u32::X11_SIZE;
			mask |= GraphicsOptionMask::DASH_OFFSET;
		}
		if dashes.is_some() {
			x11_size += u32::X11_SIZE;
			mask |= GraphicsOptionMask::DASHES;
		}

		if arc_mode.is_some() {
			x11_size += __ArcMode::X11_SIZE;
			mask |= GraphicsOptionMask::ARC_MODE;
		}

		Self {
			x11_size,

			mask,

			function: function.map(__Function),

			plane_mask,

			foreground,
			background,

			line_width: line_width.map(std::convert::Into::into),

			line_style: line_style.map(__LineStyle),
			cap_style: cap_style.map(__CapStyle),
			join_style: join_style.map(__JoinStyle),
			fill_style: fill_style.map(__FillStyle),
			fill_rule: fill_rule.map(__FillRule),

			tile,
			stipple,

			tile_stipple_x_origin: tile_stipple_x_origin.map(std::convert::Into::into),
			tile_stipple_y_origin: tile_stipple_y_origin.map(std::convert::Into::into),

			font,

			subwindow_mode: subwindow_mode.map(__SubwindowMode),

			graphics_exposures: graphics_exposure.map(__bool),

			clip_x_origin: clip_x_origin.map(std::convert::Into::into),
			clip_y_origin: clip_y_origin.map(std::convert::Into::into),
			clip_mask,

			dash_offset: dash_offset.map(std::convert::Into::into),
			dashes: dashes.map(std::convert::Into::into),

			arc_mode: arc_mode.map(__ArcMode),
		}
	} // }}}

	// option functions {{{

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
	pub fn line_width(&self) -> Option<u16> {
		self.line_width.map(|line_width| {
			line_width
				.try_into()
				.expect("must fit into u16; represents u16 value")
		})
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

	// }}}
}

// impl XRBK traits for GraphicsOptions {{{

impl X11Size for GraphicsOptions {
	fn x11_size(&self) -> usize {
		self.x11_size
	}
}

impl Readable for GraphicsOptions {
	#[allow(clippy::similar_names)]
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		let mask = GraphicsOptionMask::read_from(buf)?;
		let mut x11_size = mask.x11_size();

		let function = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::FUNCTION),
		)?;

		let plane_mask = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::PLANE_MASK),
		)?;

		let foreground = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::FOREGROUND),
		)?;
		let background = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::BACKGROUND),
		)?;

		let line_width = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::LINE_WIDTH),
		)?;

		let line_style = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::LINE_STYLE),
		)?;
		let cap_style = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::CAP_STYLE),
		)?;
		let join_style = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::JOIN_STYLE),
		)?;
		let fill_style = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::FILL_STYLE),
		)?;
		let fill_rule = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::FILL_RULE),
		)?;

		let tile =
			super::read_set_value(buf, &mut x11_size, mask.contains(GraphicsOptionMask::TILE))?;
		let stipple = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::STIPPLE),
		)?;

		let tile_stipple_x_origin = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::TILE_STIPPLE_X_ORIGIN),
		)?;
		let tile_stipple_y_origin = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::TILE_STIPPLE_Y_ORIGIN),
		)?;

		let font =
			super::read_set_value(buf, &mut x11_size, mask.contains(GraphicsOptionMask::FONT))?;

		let subwindow_mode = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::SUBWINDOW_MODE),
		)?;

		let graphics_exposures = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::GRAPHICS_EXPOSURES),
		)?;

		let clip_x_origin = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::CLIP_X_ORIGIN),
		)?;
		let clip_y_origin = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::CLIP_Y_ORIGIN),
		)?;
		let clip_mask = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::CLIP_MASK),
		)?;

		let dash_offset = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::DASH_OFFSET),
		)?;
		let dashes = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::DASHES),
		)?;

		let arc_mode = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(GraphicsOptionMask::ARC_MODE),
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
