// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod create_window;

use crate::rw::Deserialize;

pub trait Request<REPLY = ()>: Deserialize {
	fn opcode() -> u8;
	fn length(&self) -> u16;
}
