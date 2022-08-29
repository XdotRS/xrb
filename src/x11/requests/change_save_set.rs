// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::x11::common::values::Window;

use super::Request;
use crate::rw::{ReadError, ReadResult, ReadValue, Serialize, WriteResult, WriteValue};

pub struct ChangeSaveSet {
	pub mode: Mode,
	pub window: Window,
}

impl Request for ChangeSaveSet {
	fn opcode() -> u8 {
		6
	}

	fn minor_opcode() -> Option<u16> {
		None
	}

	fn length(&self) -> u16 {
		2
	}
}

#[derive(Copy, Clone)]
pub enum Mode {
	Insert,
	Delete,
}

impl WriteValue for Mode {
	fn write_1b(self) -> WriteResult<u8> {
		Ok(match self {
			Self::Insert => 0,
			Self::Delete => 1,
		})
	}

	fn write_2b(self) -> WriteResult<u16> {
		Ok(match self {
			Self::Insert => 0,
			Self::Delete => 1,
		})
	}

	fn write_4b(self) -> WriteResult<u32> {
		Ok(match self {
			Self::Insert => 0,
			Self::Delete => 1,
		})
	}
}

impl ReadValue for Mode {
	fn read_1b(byte: u8) -> ReadResult<Self>
	where
		Self: Sized,
	{
		match byte {
			0 => Ok(Self::Insert),
			1 => Ok(Self::Delete),
			_ => Err(ReadError::InvalidData),
		}
	}

	fn read_2b(bytes: u16) -> ReadResult<Self>
	where
		Self: Sized,
	{
		match bytes {
			0 => Ok(Self::Insert),
			1 => Ok(Self::Delete),
			_ => Err(ReadError::InvalidData),
		}
	}

	fn read_4b(bytes: u32) -> ReadResult<Self>
	where
		Self: Sized,
	{
		match bytes {
			0 => Ok(Self::Insert),
			1 => Ok(Self::Delete),
			_ => Err(ReadError::InvalidData),
		}
	}
}

impl Serialize for ChangeSaveSet {
	fn serialize(self) -> WriteResult<Vec<u8>> {
		let mut bytes = vec![];

		// Header {{{

		// Major opcode
		Self::opcode().write_1b_to(&mut bytes)?;
		// `mode`
		self.mode.write_1b_to(&mut bytes)?;
		// Length
		self.length().write_2b_to(&mut bytes)?;

		// }}}

		// `window`
		self.window.write_4b_to(&mut bytes)?;

		Ok(bytes)
	}
}
