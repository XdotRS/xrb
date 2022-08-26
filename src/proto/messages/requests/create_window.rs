// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::bitflags;

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

bitflags! {
	pub struct CwValueMask: u32 {
		const BACKGROUND_PIXMAP = 0x00000001;
		const BACKGROUND_PIXEL = 0x00000002;
		const BORDER_PIXMAP = 0x00000004;
		const BORDER_PIXEL = 0x00000008;
		const BIT_GRAVITY = 0x00000010;
		const WIN_GRAVITY = 0x00000020;
		const BACKING_STORE = 0x00000040;
		const BACKING_PLANES = 0x00000080;
		const BACKING_PIXEL = 0x00000100;
		const OVERRIDE_REDIRECT = 0x00000200;
		const SAVE_UNDER = 0x00000400;
		const EVENT_MASK = 0x00000800;
		const DO_NOT_PROPAGATE_MASK = 0x00001000;
		const COLORMAP = 0x00002000;
		const CURSOR = 0x00004000;
	}
}
