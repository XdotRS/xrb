// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::x11::*;
use xrb_proc_macros::{ByteSize, StaticByteSize};

#[derive(StaticByteSize, ByteSize)]
pub enum Attribute {
	BackgroundPixmap(Option<Relatable<Pixmap>>),
	BackgroundPixel(u32),
	BorderPixmap(Inheritable<Pixmap>),
	BorderPixel(u32),
	BitGravity(BitGravity),
	WinGravity(WinGravity),
	BackingStore(BackingStore),
	BackingPlanes(u32),
	BackingPixel(u32),
	OverrideRedirect(bool),
	SaveUnder(bool),
	EventMask(EventMask),
	DoNotPropagateMask(DeviceEventMask),
	Colormap(Inheritable<Colormap>),
	Cursor(Option<Cursor>),
}

#[derive(StaticByteSize, ByteSize)]
pub enum GraphicsContextValue {
	Function(Function), // TODO: 1 byte?
	PlaneMask(u32),
	Foreground(u32),
	Background(u32),
	LineWidth(u16),       // TODO: 2 bytes?
	LineStyle(LineStyle), //  TODO: 1 byte?
	CapStyle(CapStyle),   // TODO: 1 byte?
	JoinStyle(JoinStyle), // TODO: 1 byte?
	FillStyle(FillStyle), // TODO: 1 byte?
	FillRule(FillRule),   // TODO: 1 byte?
	Tile(Pixmap),
	Stipple(Pixmap),
	TileStippleXorigin(u16), // TODO: 2 bytes?
	TileStippleYorigin(u16), // TODO: 2 bytes?
	Font(Font),
	SubwindowMode(SubwindowMode), // TODO: 1 byte?
	GraphicsExposures(bool),      // TODO: 1 byte?
	ClipXorigin(u16),             // TODO: 2 bytes?
	ClipYorigin(u16),             // TODO: 2 bytes?
	ClipMask(Option<Pixmap>),
	DashOffset(u16),  // TODO: 2 bytes?
	Dashes(u8),       // TODO: 1 byte?
	ArcMode(ArcMode), // TODO: 1 byte?
}

#[derive(StaticByteSize, ByteSize)]
pub enum ConfigureWindowValue {
	X(i16),
	Y(i16),
	Width(u16),
	Height(u16),
	BorderWidth(u16),
	Sibling(Window),
	StackMode(StackMode),
}

#[derive(StaticByteSize, ByteSize)]
pub enum Function {
	Clear,
	And,
	AndReverse,
	Copy,
	AndInverted,
	NoOp,
	Xor,
	Or,
	Nor,
	Equiv,
	Invert,
	OrReverse,
	CopyInverted,
	OrInverted,
	Nand,
	Set,
}

impl Default for Function {
	fn default() -> Self {
		Self::NoOp
	}
}

#[derive(StaticByteSize, ByteSize)]
pub enum LineStyle {
	Solid,
	OnOffDash,
	DoubleDash,
}

impl Default for LineStyle {
	fn default() -> Self {
		Self::Solid
	}
}

#[derive(StaticByteSize, ByteSize)]
pub enum CapStyle {
	NotLast,
	Butt,
	Round,
	Projecting,
}

impl Default for CapStyle {
	fn default() -> Self {
		Self::Butt
	}
}

#[derive(StaticByteSize, ByteSize)]
pub enum JoinStyle {
	Miter,
	Round,
	Bevel,
}

impl Default for JoinStyle {
	fn default() -> Self {
		Self::Miter
	}
}

#[derive(StaticByteSize, ByteSize)]
pub enum FillStyle {
	Solid,
	Tiled,
	Stippled,
	OpaqueStippled,
}

impl Default for FillStyle {
	fn default() -> Self {
		Self::Solid
	}
}

#[derive(StaticByteSize, ByteSize)]
pub enum FillRule {
	EvenOdd,
	Winding,
}

impl Default for FillRule {
	fn default() -> Self {
		Self::EvenOdd
	}
}

#[derive(StaticByteSize, ByteSize)]
pub enum SubwindowMode {
	ClipByChildren,
	IncludeInferiors,
}

impl Default for SubwindowMode {
	fn default() -> Self {
		Self::ClipByChildren
	}
}

#[derive(StaticByteSize, ByteSize)]
pub enum ArcMode {
	Chord,
	PieSlice,
}
