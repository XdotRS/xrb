// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::Request;
use crate::values;

use bitflags::bitflags;

use crate::rw::{ReadError, ReadResult, ReadValue, Serialize, WriteResult, WriteValue};

use crate::x11::common::masks::{DeviceEventMask, EventMask};
use crate::x11::common::values::{
	BitGravity, Colormap, Cursor, Pixmap, VisualId, WinGravity, Window,
};

use crate::x11::wrappers::{Inherit, Relative};

impl Request for CreateWindow {
	fn opcode() -> u8 {
		// The major opcode that refers to a [`CreateWindow`] request is 1.
		1
	}

	fn length(&self) -> u16 {
		// Since the length is measured in groups of 4 bytes, the rest of the
		// request is 32 bytes long, and each value is 4 bytes, the length is
		// simply the number of values plus 32/4 (len + 8):
		self.values.len() as u16 + 8
	}
}

// The [`CreateWindow`] request itself:
pub struct CreateWindow {
	pub depth: u8,
	/// The window resource ID that has been allocated for this window.
	pub window_id: Window,
	/// The parent window which this window will be created as a child of.
	///
	/// To create a top-level window, set the parent to the root window.
	pub parent: Window,
	/// The x-coordinate of the top-left corner of the window, relative to its parent.
	pub x: i16,
	/// The y-coordinate of the top-left corner of the window, relative to its parent.
	pub y: i16,
	/// The width of the window.
	pub width: u16,
	/// The height of the window.
	pub height: u16,
	/// The width of the window's border.
	pub border_width: u16,
	/// The window's [window class](Class).
	pub class: Inherit<Class>,
	/// The window's [visual](VisualId).
	pub visual: Inherit<VisualId>,
	/// The mask that specifies which [values] are present in the `values` array.
	///
	/// [values]:CwValue
	pub value_mask: CwValueMask,
	/// [Values](CwValue) must appear in the following order, so that they match
	/// the [`CwValueMask`]:
	/// 1. [`BackgroundPixmap`]`(`[`Option`]`<`[`Relative`]`<`[`Pixmap`]`>>)`
	/// 2. [`BackgroundPixel`]`(`[`u32`]`)`
	/// 3. [`BorderPixmap`]`(`[`Inherit`]`<`[`Pixmap`]`>)`
	/// 4. [`BorderPixel`]`(`[`u32`]`)`
	/// 5. [`BitGravity`]`(`[`BitGravity`](BitGravity)`)`
	/// 6. [`WinGravity`]`(`[`WinGravity`](WinGravity)`)`
	/// 7. [`BackingStore`]`(`[`BackingStore`](BackingStore)`)`
	/// 8. [`BackingPlanes`]`(`[`u32`]`)`
	/// 9. [`BackingPixel`]`(`[`u32`]`)`
	/// 10. [`OverrideRedirect`]`(`[`bool`]`)`
	/// 11. [`SaveUnder`]`(`[`bool`]`)`
	/// 12. [`EventMask`]`(`[`EventMask`](EventMask)`)`
	/// 13. [`DoNotPropagateMask`]`(`[`DeviceEventMask`]`)`
	/// 14. [`Colormap`]`(`[`Inherit`]`<`[`Colormap`](Colormap)`>)`
	/// 15. [`Cursor`]`(`[`Option`]`<`[`Cursor`](Cursor)`>)`
	///
	/// [`BackgroundPixmap`]:CwValue::BackgroundPixmap
	/// [`BackgroundPixel`]:CwValue::BackgroundPixel
	/// [`BorderPixmap`]:CwValue::BorderPixmap
	/// [`BorderPixel`]:CwValue::BorderPixel
	/// [`BitGravity`]:CwValue::BitGravity
	/// [`WinGravity`]:CwValue::WinGravity
	/// [`BackingStore`]:CwValue::BackingStore
	/// [`BackingPlanes`]:CwValue::BackingPlanes
	/// [`BackingPixel`]:CwValue::BackingPixel
	/// [`OverrideRedirect`]:CwValue::OverrideRedirect
	/// [`SaveUnder`]:CwValue::SaveUnder
	/// [`EventMask`]:CwValue::EventMask
	/// [`DoNotPropagateMask`]:CwValue::DoNotPropagateMask
	/// [`Colormap`]:CwValue::Colormap
	/// [`Cursor`]:CwValue::Cursor
	pub values: Vec<CwValue>,
}

// The `values!` macro will automatically implement a `mask` method for the
// enum, as well as an implementation for [`WriteValue`].
values! {
	/// Values that can be configured in the [`CreateWindow`] request's `values` array.
	///
	/// Values given in the `values` array MUST be in the order they are given in
	/// this enum, so that they match the order of the [`CwValueMask`].
	pub enum CwValue<CwValueMask> {
		BackgroundPixmap(Option<Relative<Pixmap>>): BACKGROUND_PIXMAP,
		BackgroundPixel(u32): BACKGROUND_PIXEL,
		BorderPixmap(Inherit<Pixmap>): BORDER_PIXMAP,
		BorderPixel(u32): BORDER_PIXEL,
		BitGravity(BitGravity): BIT_GRAVITY,
		WinGravity(WinGravity): WIN_GRAVITY,
		BackingStore(BackingStore): BACKING_STORE,
		BackingPlanes(u32): BACKING_PLANES,
		BackingPixel(u32): BACKING_PIXEL,
		OverrideRedirect(bool): OVERRIDE_REDIRECT,
		SaveUnder(bool): SAVE_UNDER,
		EventMask(EventMask): EVENT_MASK,
		DoNotPropagateMask(DeviceEventMask): DO_NOT_PROPAGATE_MASK,
		Colormap(Inherit<Colormap>): COLORMAP,
		Cursor(Option<Cursor>): CURSOR,
	}
}

/// The class of a [Window], as defined in the X11 protocol.
pub enum Class {
	InputOutput,
	InputOnly,
}

pub enum BackingStore {
	NotUseful,
	WhenMapped,
	Always,
}

bitflags! {
	/// A mask of possible values that can be provided in the [CreateWindow] request `values`
	/// array.
	pub struct CwValueMask: u32 {
		/// The [`BackgroundPixmap`] [CreateWindow] request [value](CwValue).
		///
		/// [`BackgroundPixmap`]:CwValue::BackgroundPixmap
		const BACKGROUND_PIXMAP = 0x_0000_0001;
		/// The [`BackgroundPixel] [CreateWindow] request [value](CwValue).
		///
		/// [`BackgroundPixel`]:CwValue::BackgroundPixel
		const BACKGROUND_PIXEL = 0x_0000_0002;
		/// The [`BorderPixmap`] [CreateWindow] request [value](CwValue).
		const BORDER_PIXMAP = 0x_0000_0004;
		/// The [`BorderPixel`] [CreateWindow] request [value](CwValue).
		///
		/// [`BorderPixel`]:CwValue::BorderPixel
		const BORDER_PIXEL = 0x_0000_0008;
		/// The [`BitGravity`] [CreateWindow] request [value](CwValue).
		///
		/// [`BitGravity`]:CwValue::BitGravity
		const BIT_GRAVITY = 0x_0000_0010;
		/// The [`WinGravity`] [CreateWindow] request [value](CwValue).
		///
		/// [`WinGravity`]:CwValue::WinGravity
		const WIN_GRAVITY = 0x_0000_0020;
		/// The [`BackingStore`] [CreateWindow] request [value](CwValue).
		///
		/// [`BackingStore`]:CwValue::BackingStore
		const BACKING_STORE = 0x_0000_0040;
		/// The [`BackingPlanes`] [CreateWindow] request [value](CwValue).
		///
		/// [`BackingPlanes`]:CwValue::BackingPlanes
		const BACKING_PLANES = 0x_0000_0080;
		/// The [`BackingPixel`] [CreateWindow] request [value](CwValue).
		///
		/// [`BackingPixel`]:CwValue::BackingPixel
		const BACKING_PIXEL = 0x_0000_0100;
		/// The [`OverrideRedirect`] [CreateWindow] request [value](CwValue).
		///
		/// [`OverrideRedirect`]:CwValue::OverrideRedirect
		const OVERRIDE_REDIRECT = 0x_0000_0200;
		/// The [`SaveUnder`] [CreateWindow] request [value](CwValue).
		///
		/// [`SaveUnder`]:CwValue::SaveUnder
		const SAVE_UNDER = 0x_0000_0400;
		/// The [`EventMask`] [CreateWindow] request [value](CwValue).
		///
		/// [`EventMask`]:CwValue::EventMask
		const EVENT_MASK = 0x_0000_0800;
		/// The [`DoNotPropagateMask`] [CreateWindow] request [value](CwValue).
		///
		/// [`DoNotPropagateMask`]:CwValue::DoNotPropagateMask
		const DO_NOT_PROPAGATE_MASK = 0x_0000_1000;
		/// The [`Colormap`] [CreateWindow] request [value](CwValue).
		///
		/// [`Colormap`]:CwValue::Colormap
		const COLORMAP = 0x_0000_2000;
		/// The [`Cursor`] [CreateWindow] request [value](CwValue).
		///
		/// [`Cursor`]:CwValue::Cursor
		const CURSOR = 0x_0000_4000;
	}
}

impl Serialize for CreateWindow {
	fn serialize(self) -> WriteResult<Vec<u8>> {
		let mut bytes = vec![];

		// Header {{{

		// Major opcode
		Self::opcode().write_1b_to(&mut bytes)?;
		// `depth`
		self.depth.write_1b_to(&mut bytes)?;
		// Length
		self.length().write_2b_to(&mut bytes)?;

		// }}}

		// `window_id`
		self.window_id.write_4b_to(&mut bytes)?;
		// `parent`
		self.parent.write_4b_to(&mut bytes)?;
		// `x`
		self.x.write_2b_to(&mut bytes)?;
		// `y`
		self.y.write_2b_to(&mut bytes)?;
		// `width`
		self.width.write_2b_to(&mut bytes)?;
		// `height`
		self.height.write_2b_to(&mut bytes)?;
		// `border_width`
		self.border_width.write_2b_to(&mut bytes)?;
		// `class`
		self.class.write_2b_to(&mut bytes)?;
		// `visual`
		self.visual.write_4b_to(&mut bytes)?;

		// Value list {{{

		// `value_mask`
		self.value_mask.bits().write_4b_to(&mut bytes)?;

		// `values`
		for value in self.values {
			value.write_4b_to(&mut bytes)?;
		}

		// }}}

		Ok(bytes)
	}
}

// We don't need to implement [`Deserialize`] for [`CreateWindow`] because
// requests will never be received by an XRB client; they are only ever sent.
//
// That's good, because I'd get a headache if I were to write the code for
// interpreting the value mask so that the values vector could be correctly
// deserialized right now.

// Serialization/deserialization for `BackingStore` {{{
impl WriteValue for BackingStore {
	fn write_1b(self) -> WriteResult<u8> {
		Ok(match self {
			Self::NotUseful => 0,
			Self::WhenMapped => 1,
			Self::Always => 2,
		})
	}

	fn write_2b(self) -> WriteResult<u16> {
		Ok(match self {
			Self::NotUseful => 0,
			Self::WhenMapped => 1,
			Self::Always => 2,
		})
	}

	fn write_4b(self) -> WriteResult<u32> {
		Ok(match self {
			Self::NotUseful => 0,
			Self::WhenMapped => 1,
			Self::Always => 2,
		})
	}
}

impl ReadValue for BackingStore {
	fn read_1b(byte: u8) -> ReadResult<Self>
	where
		Self: Sized,
	{
		match byte {
			0 => Ok(Self::NotUseful),
			1 => Ok(Self::WhenMapped),
			2 => Ok(Self::Always),
			_ => Err(ReadError::InvalidData),
		}
	}

	fn read_2b(bytes: u16) -> ReadResult<Self>
	where
		Self: Sized,
	{
		match bytes {
			0 => Ok(Self::NotUseful),
			1 => Ok(Self::WhenMapped),
			2 => Ok(Self::Always),
			_ => Err(ReadError::InvalidData),
		}
	}

	fn read_4b(bytes: u32) -> ReadResult<Self>
	where
		Self: Sized,
	{
		match bytes {
			0 => Ok(Self::NotUseful),
			1 => Ok(Self::WhenMapped),
			2 => Ok(Self::Always),
			_ => Err(ReadError::InvalidData),
		}
	}
}
// }}}

// Serialization/deserialization for `Class` {{{
impl WriteValue for Class {
	fn write_1b(self) -> WriteResult<u8> {
		Ok(match self {
			Self::InputOutput => 1,
			Self::InputOnly => 2,
		})
	}

	fn write_2b(self) -> WriteResult<u16> {
		Ok(match self {
			Self::InputOutput => 1,
			Self::InputOnly => 2,
		})
	}

	fn write_4b(self) -> WriteResult<u32> {
		Ok(match self {
			Self::InputOutput => 1,
			Self::InputOnly => 2,
		})
	}
}

impl ReadValue for Class {
	fn read_1b(byte: u8) -> ReadResult<Self>
	where
		Self: Sized,
	{
		match byte {
			1 => Ok(Self::InputOutput),
			2 => Ok(Self::InputOnly),
			_ => Err(ReadError::InvalidData),
		}
	}

	fn read_2b(bytes: u16) -> ReadResult<Self>
	where
		Self: Sized,
	{
		match bytes {
			1 => Ok(Self::InputOutput),
			2 => Ok(Self::InputOnly),
			_ => Err(ReadError::InvalidData),
		}
	}

	fn read_4b(bytes: u32) -> ReadResult<Self>
	where
		Self: Sized,
	{
		match bytes {
			1 => Ok(Self::InputOutput),
			2 => Ok(Self::InputOnly),
			_ => Err(ReadError::InvalidData),
		}
	}
}
// }}}
