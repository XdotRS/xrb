// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{Deserialize, Serialize};

/// [u32].
pub type ResId = u32;

// [ResId]s: none of these can have the same [ResId] as another _[ResId]_ specifically //
/// [ResId].
pub type Window = ResId;
/// [ResId].
pub type Pixmap = ResId;
/// [ResId].
pub type Cursor = ResId;
/// [ResId].
pub type Font = ResId;
/// [ResId].
pub type GContext = ResId;
/// [ResId].
pub type Colormap = ResId;
/// [ResId].
pub type Drawable = ResId; // TODO: A [Drawable] is specifically a [Window] or [Pixmap] - trait or?
/// [ResId].
pub type Fontable = ResId; // TODO: A [Fontable] is specifically a [Font] or [GContext] - trait or?

// These are unique types, not [ResId]s //
/// [u32].
pub type Atom = u32;
/// [u32].
pub type VisualId = u32;
/// [u32].
pub type Timestamp = u32;

// Keyboard //
/// [u32]. The most significant bit (`0x10000000`) is reserved for vendor-specific [KeySym]s.
pub type KeySym = u32;
/// [u8]. `8` <= `KeyCode` <= `255`.
pub type KeyCode = u8;
/// [u8]. Starts at 1.
pub type Button = u8;

/// (u8, u8).
pub type Char2b = (u8, u8);
/// &[[Char2b]].
pub type String16<'a> = &'a [Char2b];

/// (x: i16, y: i16)
pub type Point = (i16, i16);

/// A rectangle with (`x`,`y`) coordinates and (`width` x `height`) dimensions.
#[derive(Serialize, Deserialize)]
pub struct Rect {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

// TODO: Name? Might be confused with `Arc` in std.
#[derive(Serialize, Deserialize)]
pub struct Arc {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub start_angle: i16,
    pub end_angle: i16,
}

#[derive(Serialize)]
pub enum Protocol {
    Internet,
    DecNet,
    Chaos,
    ServerInterpreted = 5,
    InternetV6,
}

// TODO: Docs.
pub struct Host {
    family: Protocol,
    address: String,
}

// [`Host`] has a unique length of its address, plus padding, so we have to do this manually. //
impl Serialize for Host {
    fn write(self, buf: &mut impl bytes::BufMut) {
        let length = self.address.len() as u16; // length of address
        let address_padding = length % 4; // extra padding to reach a multiple of 4 bytes

        self.family.write(buf); // family
        0u8.write(buf); // padding - unused byte, can be anything
        length.write(buf); // length of address
        buf.put(self.address.as_bytes()); // the address itself
        buf.put_bytes(0u8, address_padding.into()); // extra padding for 4-byte multiple
    }
}

impl Deserialize for Host {
    fn read(buf: &mut impl bytes::Buf) -> Self {
        buf.advance(1);
        let family = Protocol::Internet;

        buf.advance(1); // skip the padding (unused) byte

        let length = u16::read(buf); // read length of address
        let address_padding = length % 4; // extra padding: we need to skip this at the end

        let bytes = buf.copy_to_bytes(length.into()); // read `length` number of bytes for the address
        let address = String::from_utf8(bytes.to_vec()).unwrap();

        buf.advance(address_padding.into());

        Self { family, address }
    }
}

// TODO: Docs
#[derive(Serialize, Deserialize)]
pub enum BitGravity {
    Forget,
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast,
    Static,
}

// TODO: Docs
#[derive(Serialize, Deserialize)]
pub enum WinGravity {
    Unmap,
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast,
    Static,
}
