// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{error, Atom as AtomId, Deserialize, ResId, Serialize};

pub trait Error<T>
where
	Self: Sized + Serialize + Deserialize,
	T: Sized + Serialize + Deserialize,
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
	pub struct Window: Error(3) -> ResId;
	pub struct Pixmap: Error(4) -> ResId;
	pub struct Atom: Error(5) -> AtomId;
	pub struct Cursor: Error(6) -> ResId;
	pub struct Font: Error(7) -> ResId;
	pub struct Match: Error(8);
	pub struct Drawable: Error(9) -> ResId;
	pub struct Access: Error(10);
	pub struct Alloc: Error(11);
	pub struct Colormap: Error(12) -> ResId;
	pub struct GContext: Error(13) -> ResId;
	pub struct IdChoice: Error(14) -> ResId;
	pub struct Name: Error(15);
	pub struct Length: Error(16);
	pub struct Implementation: Error(17);
}
