// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bytes::BytesMut;

use crate::{
	bitmask,
	bitmasks::{DeviceEvent, Event},
	BitGravity, Colormap, Cursor, Deserialize, Pixmap, Serialize, VisualId, WinGravity, Window,
};

pub trait Request {
	/// The major opcode is a unique identifier for this request.
	///
	/// Major opcodes 128 through 255 are reserved for extensions. An extension may contain
	/// multiple requests, and as such will represent multiple requests with the same major opcode.
	/// An extension may choose to encode an additional _minor opcode_ in the
	/// [`metadata()`](Request::metadata) byte.
	fn major_opcode() -> u8;

	/// Metadata encoded in the second byte of the request.
	///
	/// This may be the minor opcode for extensions, though it is up to the individual extension
	/// as to where it wishes to place minor opcodes, if at all. If this metadata byte is unused,
	/// it is not guaranteed to be zero. The metadata byte may also be used for any purpose
	/// relevant to the request, as defined by the request itself.
	fn metadata(&self) -> u8 {
		0u8
	}

	/// The length of the request in units of four bytes.
	///
	/// Includes the length of the header, which is one unit of 4 bytes that contains the
	/// [`major_opcode()`](Request::major_opcode), [`metadata()`](Request::metadata) byte, and
	/// these two [`length()`](Request::length) bytes, as well as any additional data associated
	/// with the request.
	///
	/// The `length()` must be equal to the minimum length required to contain the request. That
	/// is, if the length of the request is not an exact multiple of 4 bytes, it should be rounded
	/// up to the nearest 4-byte unit by including however many padding bytes is necessary. Any
	/// unused padding bytes are not guaranteed to be zero; they may be set to anything.
	fn length(&self) -> u16;
}

// TODO: Do a few more requests to see how much they all have in common as far as types go. Should
//       probably split this module into separate submodules for every request given how much is
//       associated with just one... Also need to improve on the representation of bitmasks in
//       these requests, currently they are just the raw numbers. They can't be the bitmask enums
//       directly because obviously multiple can be set... do they even need to be represented?
//       Maybe lists of values can be converted to the appropriate bitmask number on the fly. And
//       maybe some useful macros can be made here, problem is just that these requests are a lot
//       more complicated than just errors, and change drastically between different types of
//       request.

#[derive(Serialize, Deserialize)]
pub enum WindowClass {
	CopyFromParent,
	InputOutput,
	InputOnly,
}

pub enum Visual {
	CopyFromParent,
	Id(VisualId),
}

impl Serialize for Visual {
	fn write(self, buf: &mut impl bytes::BufMut) {
		match self {
			Self::CopyFromParent => 0u32.write(buf),
			Self::Id(id) => id.write(buf),
		}
	}
}

impl Deserialize for Visual {
	fn read(buf: &mut impl bytes::Buf) -> Self {
		let visual = u32::read(buf);

		match visual {
			0u32 => Self::CopyFromParent,
			_ => Self::Id(visual),
		}
	}
}

bitmask! {
	pub enum CreateWindowValueMask: Bitmask<u32> {
		BackgroundPixmap => 0x00000001,
		BackgroundPixel => 0x00000002,
		BorderPixmap => 0x00000004,
		BorderPixel => 0x00000008,
		BitGravity => 0x00000010,
		WinGravity => 0x00000020,
		BackingStore => 0x00000040,
		BackingPlanes => 0x00000080,
		BackingPixel => 0x00000100,
		OverrideRedirect => 0x00000200,
		SaveUnder => 0x00000400,
		EventMask => 0x00000800,
		DoNotPropagateMask => 0x00001000,
		Colormap => 0x00002000,
		Cursor =>  0x00004000,
	}
}

#[derive(Serialize, Deserialize)]
pub enum BackingStore {
	NotUseful,
	WhenMapped,
	Always,
}

pub enum Inherit<T>
where
	T: Serialize + Deserialize,
{
	Parent,
	Own(T),
}

impl<T> Serialize for Inherit<T>
where
	T: Serialize + Deserialize,
{
	fn write(self, buf: &mut impl bytes::BufMut) {
		match self {
			Self::Parent => 0u32.write(buf),
			Self::Own(inheritance) => inheritance.write(buf),
		}
	}
}

impl<T> Deserialize for Inherit<T>
where
	T: Serialize + Deserialize,
{
	fn read(buf: &mut impl bytes::Buf) -> Self {
		let inheritance = u32::read(buf);

		match inheritance {
			0u32 => Self::Parent,
			_ => {
				let temp = &mut BytesMut::new();
				inheritance.write(temp);

				Self::Own(T::read(temp))
			}
		}
	}
}

pub enum OptionalInherit<T>
where
	T: Serialize + Deserialize,
{
	None,
	Parent,
	Own(T),
}

impl<T> Serialize for OptionalInherit<T>
where
	T: Serialize + Deserialize,
{
	fn write(self, buf: &mut impl bytes::BufMut) {
		match self {
			Self::None => 0u32.write(buf),
			Self::Parent => 1u32.write(buf),
			Self::Own(inheritance) => inheritance.write(buf),
		}
	}
}

impl<T> Deserialize for OptionalInherit<T>
where
	T: Serialize + Deserialize,
{
	fn read(buf: &mut impl bytes::Buf) -> Self {
		let inheritance = u32::read(buf);

		match inheritance {
			0u32 => Self::None,
			1u32 => Self::Parent,
			_ => {
				let temp = &mut BytesMut::new();
				inheritance.write(temp);

				Self::Own(T::read(temp))
			}
		}
	}
}

pub enum CreateWindowValue {
	BackgroundPixmap(OptionalInherit<Pixmap>),
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
	EventMask(u32),          // bitmask
	DoNotPropagateMask(u32), // bitmask
	Colormap(Inherit<Colormap>),
	Cursor(Option<Cursor>),
}

impl Serialize for CreateWindowValue {
	fn write(self, buf: &mut impl bytes::BufMut) {
		match self {
			Self::BackgroundPixmap(pixmap) => pixmap.write(buf),
			Self::BackgroundPixel(pixel) => pixel.write(buf),
			Self::BorderPixmap(pixmap) => pixmap.write(buf),
			Self::BorderPixel(pixel) => pixel.write(buf),
			Self::BitGravity(bit_gravity) => bit_gravity.write(buf),
			Self::WinGravity(win_gravity) => win_gravity.write(buf),
			Self::BackingStore(store) => store.write(buf),
			Self::BackingPlanes(planes) => planes.write(buf),
			Self::BackingPixel(pixel) => pixel.write(buf),
			Self::OverrideRedirect(override_redirect) => override_redirect.write(buf),
			Self::SaveUnder(save_under) => save_under.write(buf),
			Self::EventMask(event_mask) => event_mask.write(buf),
			Self::DoNotPropagateMask(device_event) => device_event.write(buf),
			Self::Colormap(colormap) => colormap.write(buf),
			Self::Cursor(cursor) => cursor.write(buf),
		}
	}
}

pub struct CreateWindow<'a> {
	pub depth: u8,
	pub window_id: Window,
	pub parent: Window,
	pub x: i16,
	pub y: i16,
	pub width: u16,
	pub height: u16,
	pub border_width: u16,
	pub class: WindowClass,
	pub visual_id: Visual,
	pub mask: CreateWindowValueMask,
	pub values: &'a [CreateWindowValue],
}

impl Request for CreateWindow<'_> {
	fn major_opcode() -> u8 {
		1u8
	}

	fn metadata(&self) -> u8 {
		self.depth
	}

	fn length(&self) -> u16 {
		(8 + self.values.len()).try_into().unwrap()
	}
}
