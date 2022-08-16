// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{bitmask, BitGravity, Deserialize, Serialize, VisualId, WinGravity, Window};

use super::Request;

#[derive(Serialize, Deserialize)]
pub enum WindowClass {
	CopyFromParent,
	InputOutput,
	InputOnly,
}

pub enum Visual {
	CopyFromParent,
	Id(VisualId),
}

impl Serialize for Visual {
	fn write(self, buf: &mut impl bytes::BufMut) {
		match self {
			Self::CopyFromParent => 0u32.write(buf),
			Self::Id(id) => id.write(buf),
		}
	}
}

impl Deserialize for Visual {
	fn read(buf: &mut impl bytes::Buf) -> Self {
		let visual = u32::read(buf);

		match visual {
			0u32 => Self::CopyFromParent,
			_ => Self::Id(visual),
		}
	}
}

bitmask! {
	pub enum CreateWindowValueMask: Bitmask<u32> {
		BackgroundPixmap => 0x00000001,
		BackgroundPixel => 0x00000002,
		BorderPixmap => 0x00000004,
		BorderPixel => 0x00000008,
		BitGravity => 0x00000010,
		WinGravity => 0x00000020,
		BackingStore => 0x00000040,
		BackingPlanes => 0x00000080,
		BackingPixel => 0x00000100,
		OverrideRedirect => 0x00000200,
		SaveUnder => 0x00000400,
		EventMask => 0x00000800,
		DoNotPropagateMask => 0x00001000,
		Colormap => 0x00002000,
		Cursor =>  0x00004000,
	}
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum BackingStore {
	NotUseful,
	WhenMapped,
	Always,
}

pub trait CreateWindowValue {}

impl CreateWindowValue for u32 {}
impl CreateWindowValue for BitGravity {}
impl CreateWindowValue for WinGravity {}
impl CreateWindowValue for bool {}
impl CreateWindowValue for BackingStore {}

pub struct CreateWindow<'a> {
	pub depth: u8,
	pub window_id: Window,
	pub parent: Window,
	pub x: i16,
	pub y: i16,
	pub width: u16,
	pub height: u16,
	pub border_width: u16,
	pub class: WindowClass,
	pub visual: Visual,
	pub mask: CreateWindowValueMask,
	// TODO: Reference (`&'a dyn CreateWindowValue`) or box (`Box<dyn CreateWindowValue`)? Both
	//       have their advantages and disadvantages: references give more of a 'direct access',
	//       but they create a lot of problems with borrowing temporary values. `Box` might be
	//       the best choice?
	pub values: &'a [Box<dyn CreateWindowValue>],
}

impl Request for CreateWindow<'_> {
	fn major_opcode() -> u8 {
		1u8
	}

	fn length(&self) -> u16 {
		(8 + self.values.len()).try_into().unwrap()
	}
}

impl Serialize for CreateWindow<'_> {
	fn write(self, buf: &mut impl bytes::BufMut) {
		// Header //
		Self::major_opcode().write(buf); // request major opcode
		self.depth.write(buf); // metadata byte (second byte of header) is `depth`
		self.length().write(buf);

		// Data //
		self.window_id.write(buf); // window id
		self.parent.write(buf); // parent
		self.x.write(buf); // x
		self.y.write(buf); // y
		self.width.write(buf); // width
		self.height.write(buf); // height
		self.border_width.write(buf); // border width
		self.class.write(buf); // class
		self.visual.write(buf); // visual
		self.mask.write(buf); // mask of the values present in the value list

		// TODO: Serialize heterogeneous `values` list. Should refactor [`crate::serialization`]
		//       first to create _intermediate representations_ that allow for easy conversions.
	}
}
