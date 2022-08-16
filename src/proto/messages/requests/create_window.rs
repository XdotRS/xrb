// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{bitmask, BitGravity, VisualId, WinGravity, Window};

use super::Request;

pub enum WindowClass {
	CopyFromParent,
	InputOutput,
	InputOnly,
}

pub enum Visual {
	CopyFromParent,
	Id(VisualId),
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

#[derive(Clone, Copy)]
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
