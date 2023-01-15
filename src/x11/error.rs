// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::message::Error;

use xrbk_macro::derive_xrb;
extern crate self as xrb;

derive_xrb! {
	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Request: Error(1) {
		#[sequence]
		pub sequence: u16,
		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Value: Error(2) {
		#[sequence]
		pub sequence: u16,

		#[error_data]
		pub bad_value: [u8; 4],

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Window: Error(3) {
		#[sequence]
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Pixmap: Error(4) {
		#[sequence]
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Atom: Error(5) {
		#[sequence]
		pub sequence: u16,

		#[error_data]
		pub bad_atom_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct CursorAppearance: Error(6) {
		#[sequence]
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Font: Error(7) {
		#[sequence]
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Match: Error(8) {
		#[sequence]
		pub sequence: u16,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Drawable: Error(9) {
		#[sequence]
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Access: Error(10) {
		#[sequence]
		pub sequence: u16,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Alloc: Error(11) {
		#[sequence]
		pub sequence: u16,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Colormap: Error(12) {
		#[sequence]
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct GraphicsContext: Error(13) {
		#[sequence]
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct ResourceIdChoice: Error(14) {
		#[sequence]
		pub sequence: u16,

		#[error_data]
		pub bad_resource_id: u32,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Name: Error(15) {
		#[sequence]
		pub sequence: u16,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Length: Error(16) {
		#[sequence]
		pub sequence: u16,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}

	#[derive(Debug, Hash, Writable, Readable, X11Size)]
	pub struct Implementation: Error(17) {
		#[sequence]
		pub sequence: u16,

		#[minor_opcode]
		pub minor_opcode: u16,
		#[major_opcode]
		pub major_opcode: u8,
		[_; ..],
	}
}
