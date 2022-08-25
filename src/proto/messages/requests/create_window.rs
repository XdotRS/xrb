// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::bitmask;

use crate::proto::ids::VisualId;
use crate::proto::ids::Window;

pub struct CreateWindow<'a> {
	pub depth: u8,
	/// The [ResId](crate::proto::ids::ResId) for the created window to use.
	pub window_id: Window,
	/// The parent of which the created window shall be a child of.
	pub parent: Window,
	pub x: i16,
	pub y: i16,
	pub width: u16,
	pub height: u16,
	pub border_width: u16,
	pub class: CwClass,
	pub visual: CwVisualId,
	pub value_mask: CwValueMask,
	pub values: &'a [Box<dyn CwValue>],
}

pub enum CwClass {
	CopyFromParent,
	InputOutput,
	InputOnly,
}

/// A [VisualId] value that can be specified to inherit from the parent.
pub enum CwVisualId {
	CopyFromParent,
	VisualId(VisualId),
}

// TODO: Implement for possible values
pub trait CwValue {}

// TODO: Replace with `bitflags` crate
bitmask! {
	pub enum CwValueMask: Bitmask<u32> {
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
		Cursor => 0x00004000,
	}
}
