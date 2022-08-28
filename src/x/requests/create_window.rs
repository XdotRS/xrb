// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::bitflags;

use crate::rw::{
	Deserialize, ReadError, ReadResult, ReadValue, Serialize, WriteResult, WriteValue,
};
use bytes::{Buf, BufMut, BytesMut};

use crate::x::common::masks::{DeviceEventMask, EventMask};
use crate::x::common::values::{
	BitGravity, Colormap, Cursor, Pixmap, VisualId, WinGravity, Window,
};

use crate::x::wrappers::{Inherit, Relative};

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

/// Values that can be configured in the [`CreateWindow`] request's `values` array.
///
/// Values given in the `values` array MUST be in the order they are given in
/// this enum, so that they match the order of the [`CwValueMask`].
pub enum CwValue {
	BackgroundPixmap(Option<Relative<Pixmap>>),
	BackgroundPixel(u32),
	BorderPixmap(Inherit<Pixmap>),
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
	Colormap(Inherit<Colormap>),
	Cursor(Option<Cursor>),
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

		1_u8.write_1b_to(&mut bytes)?;
		self.depth.write_1b_to(&mut bytes)?;
		(self.values.len() + 8).write_2b_to(&mut bytes)?;
		self.window_id.write_4b_to(&mut bytes)?;
		self.parent.write_4b_to(&mut bytes)?;
		self.x.write_2b_to(&mut bytes)?;
		self.y.write_2b_to(&mut bytes)?;
		self.width.write_2b_to(&mut bytes)?;
		self.height.write_2b_to(&mut bytes)?;
		self.border_width.write_2b_to(&mut bytes)?;
		self.class.write_2b_to(&mut bytes)?;
		self.visual.write_4b_to(&mut bytes)?;
		self.value_mask.bits().write_4b_to(&mut bytes)?;

		for value in self.values {
			// value.write_4b_to(&mut bytes);
		}

		Ok(bytes)
	}
}

impl Deserialize for CreateWindow {
	fn deserialize(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		buf.advance(1); // skip the major opcode

		let depth = u8::read_1b_from(buf)?;
		let length = usize::read_2b_from(buf)?;
		let window_id = Window::read_4b_from(buf)?;
		let parent = Window::read_4b_from(buf)?;
		let x = i16::read_2b_from(buf)?;
		let y = i16::read_2b_from(buf)?;
		let width = u16::read_2b_from(buf)?;
		let height = u16::read_2b_from(buf)?;
		let border_width = u16::read_2b_from(buf)?;
		let class = <Inherit<Class>>::read_2b_from(buf)?;
		let visual = <Inherit<VisualId>>::read_4b_from(buf)?;

		let value_mask = CwValueMask::from_bits(u32::read_4b_from(buf)?).unwrap();
		// TODO: need to deserialize the Vec<CwValue> based on the value mask...
		//       let values = (0..(length - 8)).map(|_| CwValue::read_4b_from(buf)?).collect();
		let values: Vec<CwValue> = vec![];

		Ok(Self {
			depth,
			window_id,
			parent,
			x,
			y,
			width,
			height,
			border_width,
			class,
			visual,
			value_mask,
			values,
		})
	}
}

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

impl CwValue {
	pub fn mask(&self) -> CwValueMask {
		match self {
			Self::BackgroundPixmap(_) => CwValueMask::BACKGROUND_PIXMAP,
			Self::BackgroundPixel(_) => CwValueMask::BACKGROUND_PIXEL,
			Self::BorderPixmap(_) => CwValueMask::BORDER_PIXMAP,
			Self::BorderPixel(_) => CwValueMask::BORDER_PIXEL,
			Self::BitGravity(_) => CwValueMask::BIT_GRAVITY,
			Self::WinGravity(_) => CwValueMask::WIN_GRAVITY,
			Self::BackingStore(_) => CwValueMask::BACKING_STORE,
			Self::BackingPlanes(_) => CwValueMask::BACKING_PLANES,
			Self::BackingPixel(_) => CwValueMask::BACKING_PIXEL,
			Self::OverrideRedirect(_) => CwValueMask::OVERRIDE_REDIRECT,
			Self::SaveUnder(_) => CwValueMask::SAVE_UNDER,
			Self::EventMask(_) => CwValueMask::EVENT_MASK,
			Self::DoNotPropagateMask(_) => CwValueMask::DO_NOT_PROPAGATE_MASK,
			Self::Colormap(_) => CwValueMask::COLORMAP,
			Self::Cursor(_) => CwValueMask::CURSOR,
		}
	}
}
