// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bytes::BytesMut;

use crate::{
	bitmask, serialization::KnownSize, BitGravity, Colormap, Cursor, Deserialize, Pixmap,
	Serialize, VisualId, WinGravity, Window,
};

use super::Request;

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

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum BackingStore {
	NotUseful,
	WhenMapped,
	Always,
}

impl KnownSize for BackingStore {
	fn size() -> usize {
		1
	}
}

#[derive(Clone, Copy)]
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

impl<T> KnownSize for Inherit<T>
where
	T: KnownSize + Serialize + Deserialize,
{
	fn size() -> usize {
		T::size()
	}
}

#[derive(Clone, Copy)]
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

impl<T> KnownSize for OptionalInherit<T>
where
	T: KnownSize + Serialize + Deserialize,
{
	fn size() -> usize {
		T::size()
	}
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
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

impl CreateWindowValue {
	pub fn size(&self) -> usize {
		match self {
			Self::BackgroundPixmap(_) => <OptionalInherit<Pixmap>>::size(),
			Self::BackgroundPixel(_) => u32::size(),
			Self::BorderPixmap(_) => <Inherit<Pixmap>>::size(),
			Self::BorderPixel(_) => u32::size(),
			Self::BitGravity(_) => BitGravity::size(),
			Self::WinGravity(_) => WinGravity::size(),
			Self::BackingStore(_) => BackingStore::size(),
			Self::BackingPlanes(_) => u32::size(),
			Self::BackingPixel(_) => u32::size(),
			Self::OverrideRedirect(_) => bool::size(),
			Self::SaveUnder(_) => bool::size(),
			Self::EventMask(_) => u32::size(),
			Self::DoNotPropagateMask(_) => u32::size(),
			Self::Colormap(_) => <Inherit<Colormap>>::size(),
			Self::Cursor(_) => <Option<Cursor>>::size(),
		}
	}
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
	pub visual: Visual,
	pub mask: CreateWindowValueMask,
	pub values: &'a [CreateWindowValue],
}

impl Request for CreateWindow<'_> {
	fn major_opcode() -> u8 {
		1u8
	}

	fn length(&self) -> u16 {
		(8 + self.values.len()).try_into().unwrap()
	}
}

impl Serialize for CreateWindow<'_> {
	fn write(self, buf: &mut impl bytes::BufMut) {
		// Header //
		Self::major_opcode().write(buf); // request major opcode
		self.depth.write(buf); // metadata byte (second byte of header) is `depth`
		self.length().write(buf);

		// Data //
		self.window_id.write(buf); // window id
		self.parent.write(buf); // parent
		self.x.write(buf); // x
		self.y.write(buf); // y
		self.width.write(buf); // width
		self.height.write(buf); // height
		self.border_width.write(buf); // border width
		self.class.write(buf); // class
		self.visual.write(buf); // visual
		self.mask.write(buf); // mask of the values present in the value list
		self.values.write(buf); // value list

		// Padding //
		// Since the total length needs to match `length()`, and not all values are 4 bytes, we may
		// need to add unused padding bytes to the end. We calculate the byte length of the value
		// list, as we know all values are a `KnownSize`, and subtract it from the space allocated
		// for the value list, which is `self.values.len() * 4`.
		let bytesize: usize = self.values.iter().map(|value| value.size()).sum();
		let padding = self.values.len() * 4 - bytesize;

		// We can then just put that many empty bytes at the end.
		buf.put_bytes(0u8, padding);
	}
}
