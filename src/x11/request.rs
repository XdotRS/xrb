// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Requests: commands to the X server.
//!
//! Requests are messages sent _from_ the client _to_ the X server. They are
//! commands for the X server to carry out. Some requests generate replies
//! that are sent back to the client.

use crate::Window;
use bytes::BufMut;
use xrbk::Buf;

use crate::mask::AttributeMask;

use xrbk_macro::derive_xrb;
extern crate self as xrb;

derive_xrb! {
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct CreateWindow: Request(1) {
		#[metabyte]
		pub depth: u8,

		pub window_id: Window,
		pub parent: Window,
		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,
		pub border_width: u16,
		//pub class: Inheritable<WindowClass, u16>,
		//pub visual: Inheritable<VisualId, u32>,
		pub attribute_mask: AttributeMask,
	}
}
