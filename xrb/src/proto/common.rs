// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{bitmask, Deserialize, Serialize};

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

bitmask! {
    /// A bitmask of X core protocol events.
    ///
    /// Bitmask value `0xfe000000` must be zero.
    pub enum Event -> u32 {
        KeyPress => 0x00000001,
        KeyRelease => 0x00000002,
        ButtonPress => 0x00000004,
        ButtonRelease => 0x00000008,
        EnterWindow => 0x00000010,
        LeaveWindow => 0x00000020,
        PointerMotion => 0x00000040,
        PointerMotionHint => 0x00000080,
        Button1Motion => 0x00000100,
        Button2Motion => 0x00000200,
        Button3Motion => 0x00000400,
        Button4Motion => 0x00000800,
        Button5Motion => 0x00001000,
        ButtonMotion => 0x00002000,
        KeymapState => 0x00004000,
        Exposure => 0x00008000,
        VisibilityChange => 0x00010000,
        StructureNotify => 0x00020000,
        SubstructureNotify => 0x00040000,
        SubstructureRedirect => 0x00080000,
        FocusChange => 0x00100000,
        PropertyChange => 0x00400000,
        ColormapChange => 0x00800000,
        OwnerGrabButton => 0x01000000,
        // unused but must be zero => 0xfe000000
    }

    /// A bitmask of X core protocol events, specifically used in pointer events.
    ///
    /// Bitmask value `0xffff8003` must be zero.
    pub enum PointerEvent -> u32 {
        KeyPress => 0x00000001,
        KeyRelease => 0x00000002,
        ButtonPress => 0x00000004,
        ButtonRelease => 0x00000008,
        EnterWindow => 0x00000010,
        LeaveWindow => 0x00000020,
        PointerMotion => 0x00000040,
        PointerMotionHint => 0x00000080,
        Button1Motion => 0x00000100,
        Button2Motion => 0x00000200,
        Button3Motion => 0x00000400,
        Button4Motion => 0x00000800,
        Button5Motion => 0x00001000,
        ButtonMotion => 0x00002000,
        KeymapState => 0x00004000,
        Exposure => 0x00008000,
        VisibilityChange => 0x00010000,
        StructureNotify => 0x00020000,
        SubstructureNotify => 0x00040000,
        SubstructureRedirect => 0x00080000,
        FocusChange => 0x00100000,
        PropertyChange => 0x00400000,
        ColormapChange => 0x00800000,
        OwnerGrabButton => 0x01000000,
        // unused but must be zero => 0xffff8003
    }

    /// A bitmask of X core protocol events, specifically used in device events.
    ///
    /// Bitmask value `0xffffc0b0` must be zero.
    pub enum DeviceEvent -> u32 {
        KeyPress => 0x00000001,
        KeyRelease => 0x00000002,
        ButtonPress => 0x00000004,
        ButtonRelease => 0x00000008,
        EnterWindow => 0x00000010,
        LeaveWindow => 0x00000020,
        PointerMotion => 0x00000040,
        PointerMotionHint => 0x00000080,
        Button1Motion => 0x00000100,
        Button2Motion => 0x00000200,
        Button3Motion => 0x00000400,
        Button4Motion => 0x00000800,
        Button5Motion => 0x00001000,
        ButtonMotion => 0x00002000,
        KeymapState => 0x00004000,
        Exposure => 0x00008000,
        VisibilityChange => 0x00010000,
        StructureNotify => 0x00020000,
        SubstructureNotify => 0x00040000,
        SubstructureRedirect => 0x00080000,
        FocusChange => 0x00100000,
        PropertyChange => 0x00400000,
        ColormapChange => 0x00800000,
        OwnerGrabButton => 0x01000000,
        // unused but must be zero => 0xffc0b0
    }

    /// A bitmask of modifier keys and mouse buttons.
    ///
    /// Bitmask value `0xe000` must be zero.
    pub enum KeyButtonMask -> u16 {
        Shift => 0x0001,
        Lock => 0x0002,
        Control => 0x0004,
        Mod1 => 0x0008,
        Mod2 => 0x0010,
        Mod3 => 0x0020,
        Mod4 => 0x0040,
        Mod5 => 0x0080,
        Button1 => 0x0100,
        Button2 => 0x0200,
        Button3 => 0x0400,
        Button4 => 0x0800,
        Button5 => 0x1000,
        // unused but must be zero => 0xe000
    }

    /// A bitmask of modifier keys and mouse buttons, specifically used for key events.
    ///
    /// Bitmask value `0xff00` must be zero.
    pub enum KeyMask -> u16 {
        Shift => 0x0001,
        Lock => 0x0002,
        Control => 0x0004,
        Mod1 => 0x0008,
        Mod2 => 0x0010,
        Mod3 => 0x0020,
        Mod4 => 0x0040,
        Mod5 => 0x0080,
        Button1 => 0x0100,
        Button2 => 0x0200,
        Button3 => 0x0400,
        Button4 => 0x0800,
        Button5 => 0x1000,
        // unused but must be zero => 0xff00
    }
}
