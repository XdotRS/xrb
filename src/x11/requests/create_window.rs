// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{Request, WinAttr, WinAttrMask};

use crate::x11::common::values::{VisualId, Window};
use crate::x11::wrappers::Inherit;

use crate::rw::{ReadError, ReadResult, ReadValue, Serialize, WriteResult, WriteValue};

impl Request for CreateWindow {
	fn opcode() -> u8 {
		// The major opcode that refers to a [`CreateWindow`] request is 1.
		1
	}

	fn minor_opcode() -> Option<u8> {
		// This is an X core protocol request: it is not an extension, and so
		// therefore does not use the `minor_opcode`.
		None
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
	/// The mask that specifies which [attributes] are present in the `values`
	/// vector.
	///
	/// [attributes]:WinAttr
	pub value_mask: WinAttrMask,
	/// [Attributes](WinAttr) must appear in the following order, so that they
	/// match the [`WinAttrMask`]:
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
	/// [`BackgroundPixmap`]:WinAttr::BackgroundPixmap
	/// [`BackgroundPixel`]:WinAttr::BackgroundPixel
	/// [`BorderPixmap`]:WinAttr::BorderPixmap
	/// [`BorderPixel`]:WinAttr::BorderPixel
	/// [`BitGravity`]:WinAttr::BitGravity
	/// [`WinGravity`]:WinAttr::WinGravity
	/// [`BackingStore`]:WinAttr::BackingStore
	/// [`BackingPlanes`]:WinAttr::BackingPlanes
	/// [`BackingPixel`]:WinAttr::BackingPixel
	/// [`OverrideRedirect`]:WinAttr::OverrideRedirect
	/// [`SaveUnder`]:WinAttr::SaveUnder
	/// [`EventMask`]:WinAttr::EventMask
	/// [`DoNotPropagateMask`]:WinAttr::DoNotPropagateMask
	/// [`Colormap`]:WinAttr::Colormap
	/// [`Cursor`]:WinAttr::Cursor
	pub values: Vec<WinAttr>,
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
