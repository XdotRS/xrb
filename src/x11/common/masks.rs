// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::bitflags;

use crate::rw::{ReadError, ReadResult, ReadValue, WriteResult, WriteValue};

// [`WriteValue`] and [`ReadValue`] are not implemented for bitmasks, because
// [`bitflags`] already offers a simpler solution: use the generated `bits` and
// `from_bits` methods.

bitflags! {
	/// A bitmask of X core protocol events.
	///
	/// Bitmask value `0xfe000000` must be zero.
	pub struct EventMask: u32 {
		const KEY_PRESS = 0x00000001;
		const KEY_RELEASE = 0x00000002;
		const BUTTON_PRESS = 0x00000004;
		const BUTTON_RELEASE = 0x00000008;
		const ENTER_WINDOW = 0x00000010;
		const LEAVE_WINDOW = 0x00000020;
		const POINTER_MOTION = 0x00000040;
		const POINTER_MOTION_HINT = 0x00000080;
		const BUTTON_1_MOTION = 0x00000100;
		const BUTTON_2_MOTION = 0x00000200;
		const BUTTON_3_MOTION = 0x00000400;
		const BUTTON_4_MOTION = 0x00000800;
		const BUTTON_5_MOTION = 0x00001000;
		const BUTTON_MOTION = 0x00002000;
		const KEYMAP_STATE = 0x00004000;
		const EXPOSURE = 0x00008000;
		const VISIBILITY_CHANGE = 0x00010000;
		const STRUCTURE_NOTIFY = 0x00020000;
		const SUBSTRUCTURE_NOTIFY = 0x00040000;
		const SUBSTRUCTURE_REDIRECT = 0x00080000;
		const FOCUS_CHANGE = 0x00100000;
		const PROPERTY_CHANGE = 0x00400000;
		const COLORMAP_CHANGE = 0x00800000;
		const OWNER_GRAB_BUTTON = 0x01000000;
		// TODO: Should this be a constant? doc comment? plain comment?
		// unused but must be zero = 0xfe000000;
	}

	/// A bitmask of X core protocol events, specifically used in pointer events.
	///
	/// Bitmask value `0xffff8003` must be zero.
	pub struct PointerEventMask: u32 {
		const KEY_PRESS = 0x00000001;
		const KEY_RELEASE = 0x00000002;
		const BUTTON_PRESS = 0x00000004;
		const BUTTON_RELEASE = 0x00000008;
		const ENTER_WINDOW = 0x00000010;
		const LEAVE_WINDOW = 0x00000020;
		const POINTER_MOTION = 0x00000040;
		const POINTER_MOTION_HINT = 0x00000080;
		const BUTTON_1_MOTION = 0x00000100;
		const BUTTON_2_MOTION = 0x00000200;
		const BUTTON_3_MOTION = 0x00000400;
		const BUTTON_4_MOTION = 0x00000800;
		const BUTTON_5_MOTION = 0x00001000;
		const BUTTON_MOTION = 0x00002000;
		const KEYMAP_STATE = 0x00004000;
		const EXPOSURE = 0x00008000;
		const VISIBILITY_CHANGE = 0x00010000;
		const STRUCTURE_NOTIFY = 0x00020000;
		const SUBSTRUCTURE_NOTIFY = 0x00040000;
		const SUBSTRUCTURE_REDIRECT = 0x00080000;
		const FOCUS_CHANGE = 0x00100000;
		const PROPERTY_CHANGE = 0x00400000;
		const COLORMAP_CHANGE = 0x00800000;
		const OWNER_GRAB_BUTTON = 0x01000000;
		// TODO: Should this be a constant? doc comment? plain comment?
		// unused but must be zero = 0xffff8003;
	}

	/// A bitmask of X core protocol events, specifically used in device events.
	///
	/// Bitmask value `0xffffc0b0` must be zero.
	pub struct DeviceEventMask: u32 {
		const KEY_PRESS = 0x00000001;
		const KEY_RELEASE = 0x00000002;
		const BUTTON_PRESS = 0x00000004;
		const BUTTON_RELEASE = 0x00000008;
		const ENTER_WINDOW = 0x00000010;
		const LEAVE_WINDOW = 0x00000020;
		const POINTER_MOTION = 0x00000040;
		const POINTER_MOTION_HINT = 0x00000080;
		const BUTTON_1_MOTION = 0x00000100;
		const BUTTON_2_MOTION = 0x00000200;
		const BUTTON_3_MOTION = 0x00000400;
		const BUTTON_4_MOTION = 0x00000800;
		const BUTTON_5_MOTION = 0x00001000;
		const BUTTON_MOTION = 0x00002000;
		const KEYMAP_STATE = 0x00004000;
		const EXPOSURE = 0x00008000;
		const VISIBILITY_CHANGE = 0x00010000;
		const STRUCTURE_NOTIFY = 0x00020000;
		const SUBSTRUCTURE_NOTIFY = 0x00040000;
		const SUBSTRUCTURE_REDIRECT = 0x00080000;
		const FOCUS_CHANGE = 0x00100000;
		const PROPERTY_CHANGE = 0x00400000;
		const COLORMAP_CHANGE = 0x00800000;
		const OWNER_GRAB_BUTTON = 0x01000000;
		// TODO: Should this be a constant? doc comment? plain comment?
		// unused but must be zero = 0xffffc0b0;
	}

	/// A bitmask of modifier keys and mouse buttons.
	///
	/// Bitmask value `0xe000` must be zero.
	pub struct KeyButtonMask: u16 {
		const SHIFT = 0x0001;
		const LOCK = 0x0002;
		const CONTROL = 0x0004;
		const MOD_1 = 0x0008;
		const MOD_2 = 0x0010;
		const MOD_3 = 0x0020;
		const MOD_4 = 0x0040;
		const MOD_5 = 0x0080;
		const BUTTON_1 = 0x0100;
		const BUTTON_2 = 0x0200;
		const BUTTON_3 = 0x0400;
		const BUTTON_4 = 0x0800;
		const BUTTON_5 = 0x1000;
		// TODO: Should this be a constant? doc comment? plain comment?
		// unused but must be zero = 0xe000;
	}

	/// A bitmask of modifier keys and mouse buttons, specifically used in key events.
	///
	/// Bitmask value `0xff00` must be zero.
	pub struct KeyMask: u16 {
		const SHIFT = 0x0001;
		const LOCK = 0x0002;
		const CONTROL = 0x0004;
		const MOD_1 = 0x0008;
		const MOD_2 = 0x0010;
		const MOD_3 = 0x0020;
		const MOD_4 = 0x0040;
		const MOD_5 = 0x0080;
		const BUTTON_1 = 0x0100;
		const BUTTON_2 = 0x0200;
		const BUTTON_3 = 0x0400;
		const BUTTON_4 = 0x0800;
		const BUTTON_5 = 0x1000;
		// TODO: Should this be a constant? doc comment? plain comment?
		// unused but must be zero = 0xff00;
	}
}

impl WriteValue for EventMask {
	fn write_1b(self) -> WriteResult<u8> {
		self.bits().write_1b()
	}

	fn write_2b(self) -> WriteResult<u16> {
		self.bits().write_2b()
	}

	fn write_4b(self) -> WriteResult<u32> {
		self.bits().write_4b()
	}
}

impl ReadValue for EventMask {
	fn read_1b(byte: u8) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::from_bits(byte as u32).ok_or(ReadError::InvalidData)
	}

	fn read_2b(bytes: u16) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::from_bits(bytes as u32).ok_or(ReadError::InvalidData)
	}

	fn read_4b(bytes: u32) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::from_bits(bytes).ok_or(ReadError::InvalidData)
	}
}

impl WriteValue for DeviceEventMask {
	fn write_1b(self) -> WriteResult<u8> {
		self.bits().write_1b()
	}

	fn write_2b(self) -> WriteResult<u16> {
		self.bits().write_2b()
	}

	fn write_4b(self) -> WriteResult<u32> {
		self.bits().write_4b()
	}
}

impl ReadValue for DeviceEventMask {
	fn read_1b(byte: u8) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::from_bits(byte as u32).ok_or(ReadError::InvalidData)
	}

	fn read_2b(bytes: u16) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::from_bits(bytes as u32).ok_or(ReadError::InvalidData)
	}

	fn read_4b(bytes: u32) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::from_bits(bytes).ok_or(ReadError::InvalidData)
	}
}

impl WriteValue for PointerEventMask {
	fn write_1b(self) -> WriteResult<u8> {
		self.bits().write_1b()
	}

	fn write_2b(self) -> WriteResult<u16> {
		self.bits().write_2b()
	}

	fn write_4b(self) -> WriteResult<u32> {
		self.bits().write_4b()
	}
}

impl ReadValue for PointerEventMask {
	fn read_1b(byte: u8) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::from_bits(byte as u32).ok_or(ReadError::InvalidData)
	}

	fn read_2b(bytes: u16) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::from_bits(bytes as u32).ok_or(ReadError::InvalidData)
	}

	fn read_4b(bytes: u32) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::from_bits(bytes).ok_or(ReadError::InvalidData)
	}
}

impl WriteValue for KeyButtonMask {
	fn write_1b(self) -> WriteResult<u8> {
		self.bits().write_1b()
	}

	fn write_2b(self) -> WriteResult<u16> {
		self.bits().write_2b()
	}

	fn write_4b(self) -> WriteResult<u32> {
		self.bits().write_4b()
	}
}

impl ReadValue for KeyButtonMask {
	fn read_1b(byte: u8) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::from_bits(byte as u16).ok_or(ReadError::InvalidData)
	}

	fn read_2b(bytes: u16) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::from_bits(bytes).ok_or(ReadError::InvalidData)
	}

	fn read_4b(bytes: u32) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::from_bits(bytes as u16).ok_or(ReadError::InvalidData)
	}
}

impl WriteValue for KeyMask {
	fn write_1b(self) -> WriteResult<u8> {
		self.bits().write_1b()
	}

	fn write_2b(self) -> WriteResult<u16> {
		self.bits().write_2b()
	}

	fn write_4b(self) -> WriteResult<u32> {
		self.bits().write_4b()
	}
}

impl ReadValue for KeyMask {
	fn read_1b(byte: u8) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::from_bits(byte as u16).ok_or(ReadError::InvalidData)
	}

	fn read_2b(bytes: u16) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::from_bits(bytes).ok_or(ReadError::InvalidData)
	}

	fn read_4b(bytes: u32) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Self::from_bits(bytes as u16).ok_or(ReadError::InvalidData)
	}
}
