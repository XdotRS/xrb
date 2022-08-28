// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::x11::common::values::Window;

use super::Request;
use crate::rw::{Serialize, WriteResult, WriteValue};

pub struct ReparentWindow {
	pub window: Window,
	pub parent: Window,
	pub x: i16,
	pub y: i16,
}

impl Request for ReparentWindow {
	fn opcode() -> u8 {
		7
	}

	fn minor_opcode() -> Option<u16> {
		None
	}

	fn length(&self) -> u16 {
		4
	}
}

impl Serialize for ReparentWindow {
	fn serialize(self) -> WriteResult<Vec<u8>> {
		let mut bytes = vec![];

		// Header {{{

		// Major opcode
		Self::opcode().write_1b_to(&mut bytes)?;
		// Empty byte
		0u8.write_1b_to(&mut bytes)?;
		// Length
		self.length().write_2b_to(&mut bytes)?;

		// }}}

		// `window`
		self.window.write_4b_to(&mut bytes)?;
		// `parent`
		self.parent.write_4b_to(&mut bytes)?;
		// `x`
		self.x.write_2b_to(&mut bytes)?;
		// `y`
		self.y.write_2b_to(&mut bytes)?;

		Ok(bytes)
	}
}
