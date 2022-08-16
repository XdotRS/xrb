// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{error, proto::ids::Window as WindowId};

pub trait Error<T = u32>
where
	Self: Sized,
	T: Sized,
{
	fn error_code() -> u8;
	fn sequence_num(&self) -> u16;
	fn minor_opcode(&self) -> u16;
	fn major_opcode(&self) -> u8;
	fn data(&self) -> T;
	fn new(sequence_num: u16, minor_opcode: u16, major_opcode: u8, data: T) -> Self;
}

error! {
	pub struct Request: Error(1);
	pub struct Value: Error(2) -> u32;
	pub struct Window: Error(3) -> WindowId;
	pub struct Pixmap: Error(4) -> u32;
	pub struct Atom: Error(5) -> u32;
	pub struct Cursor: Error(6) -> u32;
	pub struct Font: Error(7) -> u32;
	pub struct Match: Error(8);
	pub struct Drawable: Error(9) -> u32;
	pub struct Access: Error(10);
	pub struct Alloc: Error(11);
	pub struct Colormap: Error(12) -> u32;
	pub struct GContext: Error(13) -> u32;
	pub struct IdChoice: Error(14) -> u32;
	pub struct Name: Error(15);
	pub struct Length: Error(16);
	pub struct Implementation: Error(17);
}
