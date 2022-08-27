// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::bitflags;

use crate::protocol::common::masks::{DeviceEventMask, EventMask};
use crate::protocol::common::values::{
	BitGravity, Colormap, Cursor, Pixmap, VisualId, WinGravity, Window,
};

use crate::protocol::wrappers::{Inherit, Relative};

/// The class of a [Window], as defined in the X11 protocol.
pub enum Class {
	InputOutput,
	InputOnly,
}

pub enum CwBackingStore {
	NotUseful,
	WhenMapped,
	Always,
}

// The [`CreateWindow`] request itself:
pub struct CreateWindow<'a> {
	pub depth: u8,
	pub window_id: Window,
	pub parent: Window,
	pub x: i16,
	pub y: i16,
	pub width: u16,
	pub height: u16,
	pub border_width: u16,
	pub class: Inherit<Class>,
	pub visual: VisualId,
	pub value_mask: CwValueMask,
	/// [Values](CwValue) must appear in the following order:
	/// 1. [`CwBackgroundPixmap`]
	/// 2. [`CwBackgroundPixel`]
	/// 3. [`CwBorderPixmap`]
	/// 4. [`CwBorderPixel`]
	/// 5. [`CwBitGravity`]
	/// 6. [`CwWinGravity`]
	/// 7. [`CwBackingStore`]
	/// 8. [`CwBackingPlanes`]
	/// 9. [`CwBackingPixel`]
	/// 10. [`CwOverrideRedirect`]
	/// 11. [`CwSaveUnder`]
	/// 12. [`CwEventMask`]
	/// 13. [`CwDoNotPropagateMask`]
	/// 14. [`CwColormap`]
	/// 15. [`CwCursor`]
	pub values: &'a [Box<dyn CwValue>],
}

bitflags! {
	pub struct CwValueMask: u32 {
	    /// 1. [`CwBackgroundPixmap`]
		const BACKGROUND_PIXMAP = 0x_0000_0001;
		/// 2. [`CwBackgroundPixel`]
		const BACKGROUND_PIXEL = 0x_0000_0002;
		/// 3. [`CwBorderPixmap`]
		const BORDER_PIXMAP = 0x_0000_0004;
		/// 4. [`CwBorderPixel`]
		const BORDER_PIXEL = 0x_0000_0008;
		/// 5. [`CwBitGravity`]
		const BIT_GRAVITY = 0x_0000_0010;
		/// 6. [`CwWinGravity`]
		const WIN_GRAVITY = 0x_0000_0020;
		/// 7. [`CwBackingStore`]
		const BACKING_STORE = 0x_0000_0040;
		/// 8. [`CwBackingPlanes`]
		const BACKING_PLANES = 0x_0000_0080;
		/// 9. [`CwBackingPixel`]
		const BACKING_PIXEL = 0x_0000_0100;
		/// 10. [`CwOverrideRedirect`]
		const OVERRIDE_REDIRECT = 0x_0000_0200;
		/// 11. [`CwSaveUnder`]
		const SAVE_UNDER = 0x_0000_0400;
		/// 12. [`CwEventMask`]
		const EVENT_MASK = 0x_0000_0800;
		/// 13. [`CwDoNotPropagateMask`]
		const DO_NOT_PROPAGATE_MASK = 0x_0000_1000;
		/// 14. [`CwColormap`]
		const COLORMAP = 0x_0000_2000;
		/// 15. [`CwCursor`]
		const CURSOR = 0x_0000_4000;
	}
}

// Type definitions {{{
/// 1. [`Option`]`<`[`Relative`]`<`[`Pixmap`]`>>`
pub type CwBackgroundPixmap = Option<Relative<Pixmap>>;
/// 2. [`u32`]
pub type CwBackgroundPixel = u32;
/// 3. [`Inherit`]`<`[`Pixmap`]`>`
pub type CwBorderPixmap = Inherit<Pixmap>;
/// 4. [`u32`]
pub type CwBorderPixel = u32;
/// 5. [`BitGravity`]
pub type CwBitGravity = BitGravity;
/// 6. [`WinGravity`]
pub type CwWinGravity = WinGravity;
/// 7. [`u32`]
pub type CwBackingPlanes = u32;
/// 8. [`u32`]
pub type CwBackingPixel = u32;
/// 9. [`bool`]
pub type CwOverrideRedirect = bool;
/// 10. [`bool`]
pub type CwSaveUnder = bool;
/// 11. [`EventMask`]
pub type CwEventMask = EventMask;
/// 12. [`DeviceEventMask`]
pub type CwDoNotPropagateMask = DeviceEventMask;
/// 13. [`Inherit`]`<`[`Colormap`]`>`
pub type CwColormap = Inherit<Colormap>;
/// 14. [`Option`]`<`[`Cursor`]`>`
pub type CwCursor = Option<Cursor>;
// }}}

pub trait CwValue {} // {{{

impl CwValue for Option<Relative<u32>> {}
impl CwValue for u32 {}
impl CwValue for Inherit<u32> {}
impl CwValue for BitGravity {}
impl CwValue for WinGravity {}
impl CwValue for CwBackingStore {}
impl CwValue for bool {}
impl CwValue for EventMask {}
impl CwValue for DeviceEventMask {}
impl CwValue for Option<Cursor> {}

// }}}
