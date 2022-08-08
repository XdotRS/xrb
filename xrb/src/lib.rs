// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! An implementation of the
//! [X11 protocol](https://x.org/releases/X11R7.7/doc/xproto/xprotocol.html/).

pub const PROTOCOL_MAJOR_VERSION: u16 = 11;
pub const PROTOCOL_MINOR_VERSION: u16 = 0;

pub type ResourceId = u32;
pub type Window = ResourceId;
pub type Pixmap = ResourceId;
pub type Cursor = ResourceId;
pub type Font = ResourceId;
pub type GContext = u32;
pub type Colormap = ResourceId;
pub type Drawable = ResourceId;
pub type Fontable = ResourceId;
pub type Atom = u32;
pub type VisualId = u32;
pub type Value = u32;
pub type TimeStamp = u32;
pub type KeySym = u32;
pub type KeyCode = u8;
pub type Button = u8;

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

pub trait ToBytes {
    /// Returns a representation of `Self` as bytes.
    fn to_bytes(&self) -> &[u8];
}

/// Defines the ordering of bytes in a connection to the X server. Also known as endianness.
///
/// An ordering of [MostSignificantFirst](ByteOrder::MostSignficantFirst) is represented by the
/// byte value of octal 102 (`0o102u8`).
///
/// An ordering of [LeastSignificantFirst](ByteOrder::LeastSignficantFirst) is represented by the
/// byte value of octal 154 (`0o154u8`).
///
/// The [ByteOrder] is the first byte sent in a [ConnectionInit] request, and all 16-bit and 32-bit
/// quantities sent by the client following this [ByteOrder] must be transmitted with this
/// [ByteOrder]. All 16-bit and 32-bit quantities returned by the server will be transmitted with
/// this [ByteOrder] too.
///
/// The [ByteOrder] sent in the [ConnectionInitRequest] should probably match the system's
/// endianness, so that all 16-bit and 32-bit values can be sent with no conversions. You can get
/// the system's [ByteOrder] with [`ByteOrder::native()`].
pub enum ByteOrder {
    /// Values are transmitted most significant byte first.
    MostSignificantFirst,
    /// Values are transmitted least significant byte first.
    LeastSignificantFirst,
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

impl ToBytes for ByteOrder {
    fn to_bytes(&self) -> &[u8] {
        match self {
            // Octal 102.
            Self::MostSignificantFirst => &[0o102u8],
            // Octal 154.
            Self::LeastSignificantFirst => &[0o154u8],
        }
    }
}

pub struct ConnectionInitRequest<'a> {
    pub byte_order: ByteOrder,
    /// Should always be 11.
    pub protocol_major_version: u16,
    /// Should always be 0.
    pub protocol_minor_version: u16,
    pub auth_protocol_name: &'a str,
    pub auth_data: &'a str,
}

/// The ordering of bits in bitmaps.
pub enum BitmapBitOrder {
    /// Bitmaps are ordered least-signficiant-bit first.
    LeastSignficantFirst,
    /// Bitmaps are ordered most-significant-bit first.
    MostSignficantFirst,
}

pub struct Format {
    pub depth: u8,
    /// Can be `1u8`, `4u8`, `16u8`, `24u8`, or `32u8`.
    pub bits_per_pixel: u8,
    /// Can be `8u8`, `16u8`, or `32u8`.
    pub scanline_pad: u8,
}

pub enum VisualClass {
    StaticGray,
    StaticColor,
    TrueColor,
    GrayScale,
    PseudoColor,
    DirectColor,
}

pub struct VisualType {
    pub visual_id: VisualId,
    pub class: VisualClass,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
}

pub struct Depth<'a> {
    pub depth: u8,
    pub visuals: &'a [VisualType],
}

pub enum BackingStores {
    Never,
    WhenMapped,
    Always,
}

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

pub enum ConnectionInitResult<'a> {
    Failed {
        /// Should always be 11.
        protocol_major_version: u16,
        /// Should always be 0.
        protocol_minor_version: u16,
        reason: &'a str,
    },
    Success {
        /// Should always be 11.
        protocol_major_version: u16,
        /// Should always be 0.
        protocol_minor_version: u16,
        /// Identifies the owner of the X server implementation.
        vendor: &'a str,
        /// The vendor controls the semantics of the release number.
        release_number: u32,
        resource_id_base: u32,
        /// A contiguous set of bits, at least 18, used to allocate [ResourceId]s.
        ///
        /// Clients allocate [ResourceId]s for [Window]s, [Pixmap]s, [Cursor]s, [Font]s,
        /// [GContext]s, and [Colormap]s by choosing a value with only some subset of these bits
        /// set and ORing it with `resource_id_base`. Only values constructed in this way can be
        /// used to name newly created resources over the connection. [ResourceId]s never have the
        /// top three bits set. The client is not restricted to linear or contiguous allocation of
        /// [ResourceId]s. Once a [ResourceId] has been freed, it can be reused. A [ResourceId]
        /// must be unique with respect to the [ResourceId]s of all other resources, not just other
        /// resources of the same type. However, note that the value spaces of [ResourceId]s,
        /// [Atom]s, [VisualId]s, and [KeySym]s are distinguished by context, and, as such, are not
        /// required to be disjoint; for example, a given numeric value might be both a valid
        /// [Window] ID, a valid [Atom], and a valid [KeySym].
        resource_id_mask: u32,
        image_byte_order: ByteOrder,
        /// Can be `8u8`, `16u8`, or `32u8`.
        bitmap_scanline_unit: u8,
        /// Can be `8u8`, `16u8`, or `32u8`.
        bitmap_scanline_pad: u8,
        bitmap_bit_order: BitmapBitOrder,
        pixmap_formats: &'a [Format],
        roots: &'a [Screen<'a>],
        motion_buffer_size: u32,
        /// Specifies the maximum length of a request accepted by the server, in 4-byte units.
        ///
        /// That is, length is the maximum value that can appear in the lenght field of a request.
        /// Reqeusts larger than this maximum generate a `Length` error, and the server will read
        /// and simply discard the entire request. `maximum_request_length` will always be at least
        /// `4096` (that is, requests of length up to and including 16384 bytes will be accepted by
        /// all servers).
        maximum_request_length: u16,
        /// Specifies the smallest keycode value transmitted by the server. Never less than 8.
        min_keycode: KeyCode,
        /// Specifies the biggest keycode value transmitted by the server. Never more than 255.
        max_keycode: KeyCode,
    },
    Authenticate {
        reason: &'a str,
    },
}

pub struct ConnectionInitReply<'a> {
    pub result: ConnectionInitResult<'a>,
}
