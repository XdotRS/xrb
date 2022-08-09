// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xrb_derive_macros::{Deserialize, Serialize};

/// The ID of a [Window], [Pixmap], [Cursor], [Font], [GraphicsContext], or [Colormap].
///
/// A [ResourceId] must be unique; that is, none of the previously mentioned resources can have a
/// [ResourceId] the same as any other [ResourceId] for any other resource. [ResourceId]s can,
/// however, be the same as other [`u32`] quantities like [Atom]s, [Keysym]s, etc.
///
/// The top three bits of any [ResourceId] is never set.
pub type ResourceId = u32;
/// A [ResourceId] referring to a particular window.
pub type Window = ResourceId;
/// A [ResourceId] referring to a particular pixmap.
pub type Pixmap = ResourceId;
/// A [ResourceId] referring to a particular cursor.
pub type Cursor = ResourceId;
/// A [ResourceId] referring to a particular font.
pub type Font = ResourceId;
/// A [ResourceId] referring to a partiuclar graphics context.
pub type GraphicsContext = ResourceId;
/// A [ResourceId] referring to a particular colormap.
pub type Colormap = ResourceId;
/// A [ResourceId] referring to either a [Pixmap] or a [Window].
pub type Drawable = ResourceId;
/// A [ResourceId] referring to either a [GraphicsContext] or a [Font].
pub type Fontable = ResourceId;
/// A fixed-length identifier referring to a string of text previously registered with the X
/// server.
///
/// An [Atom]'s purpose is to enable a fixed-length representation of a string; strings can be
/// registered with the X server through an [InternAtomRequest], which returns the stored [Atom]
/// for that string, if it was previously registered, or the new [Atom] that was generated for that
/// string if it is new.
pub type Atom = u32;
pub type VisualId = u32;
/// A value to be included in a `value_list` in certain events.
pub type Value = u32;
/// A timestamp.
pub type Timestamp = u32;
pub type Keysym = u32;
/// A keycode representing a particular key.
pub type Keycode = u8;
/// A code representing a particular button on the mouse.
pub type Button = u8;

/// An enum representing the core X protocol events.
// #[derive(Deserialize, Serialize)]
pub enum Event {
    KeyPress,
    KeyRelease,
    OwnerGrabButton,
    ButtonPress,
    ButtonRelease,
    EnterWindow,
    LeaveWindow,
    PointerMotion,
    PointerMotionHint,
    Button1Motion,
    Button2Motion,
    Button3Motion,
    Button4Motion,
    Button5Motion,
    ButtonMotion,
    Exposure,
    VisibilityChange,
    StructureNotify,
    ResizeRedirect,
    SubstructureNotify,
    SubstructureRedirect,
    FocusChange,
    PropertyChange,
    ColormapChange,
    KeymapState,
}

/// Defines the ordering of bytes in a connection to the X server. Also known as endianness.
///
/// An ordering of [MostSignificantFirst](ByteOrder::MostSignficantFirst) is represented by the
/// byte value of octal 102 (`0o102u8`).
///
/// An ordering of [LeastSignificantFirst](ByteOrder::LeastSignficantFirst) is represented by the
/// byte value of octal 154 (`0o154u8`).
///
/// The [ByteOrder] is the first byte sent in a [ConnectionInitRequest], and all 16-bit and 32-bit
/// quantities sent by the client following this [ByteOrder] must be transmitted with this
/// [ByteOrder]. All 16-bit and 32-bit quantities returned by the server will be transmitted with
/// this [ByteOrder] too.
///
/// The [ByteOrder] sent in the [ConnectionInitRequest] should probably match the system's
/// endianness, so that all 16-bit and 32-bit values can be sent with no conversions. You can get
/// the system's [ByteOrder] with [`ByteOrder::native()`].
// #[derive(Deserialize, Serialize)]
pub enum ByteOrder {
    /// Values are transmitted most significant byte first.
    MostSignificantFirst = 0o102,
    /// Values are transmitted least significant byte first.
    LeastSignificantFirst = 0o154,
}

impl ByteOrder {
    /// Returns the [ByteOrder]/endianness used by the system.
    ///
    /// This is the recommended [ByteOrder] to use: using the `native()` byte order means there is
    /// no endianness conversion cost when sending and receiving 16-bit and 32-bit values to and
    /// from the X server.
    pub fn native() -> Self {
        if cfg!(target_endian = "little") {
            Self::LeastSignificantFirst
        } else {
            Self::MostSignificantFirst
        }
    }
}

/// The ordering of bits in bitmaps.
// #[derive(Deserialize, Serialize)]
pub enum BitmapBitOrder {
    /// Bitmaps are ordered least-signficiant-bit first.
    LeastSignficantFirst,
    /// Bitmaps are ordered most-significant-bit first.
    MostSignficantFirst,
}

#[derive(Deserialize, Serialize)]
pub struct Format {
    pub depth: u8,
    /// Can be `1u8`, `4u8`, `16u8`, `24u8`, or `32u8`.
    pub bits_per_pixel: u8,
    /// Can be `8u8`, `16u8`, or `32u8`.
    pub scanline_pad: u8,
}

// #[derive(Deserialize, Serialize)]
pub enum VisualClass {
    StaticGray,
    StaticColor,
    TrueColor,
    GrayScale,
    PseudoColor,
    DirectColor,
}

// #[derive(Deserialize, Serialize)]
pub struct VisualType {
    pub visual_id: VisualId,
    pub class: VisualClass,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
}

// #[derive(Deserialize, Serialize)]
pub struct Depth<'a> {
    pub depth: u8,
    pub visuals: &'a [VisualType],
}

// #[derive(Deserialize, Serialize)]
pub enum BackingStores {
    Never,
    WhenMapped,
    Always,
}

// #[derive(Derserialize, Serialize)]
pub struct Screen<'a> {
    pub root: Window,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub allowed_depths: &'a [Depth<'a>],
    pub root_depth: u8,
    pub root_visual: VisualId,
    pub default_colormap: Colormap,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub backing_stores: BackingStores,
    pub save_unders: bool,
    pub current_input_masks: &'a [Event],
}
