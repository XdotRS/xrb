// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate self as xrb;

use xrbk_macro::define;
use crate::String8;

// See https://x.org/releases/X11R7.7/doc/xproto/x11protocol.html#Encoding::Connection_Setup

define! {
    pub enum Endianness {
        BigEndian = 0x42,
        LittleEndian = 0x6C,
    }

    pub struct InitConnection {
        pub byte_order: Endianness,

        _,

        pub protocol_major_version: u16,
        pub protocol_minor_version: u16,

        let auth_protocol_name_len: u16 = auth_protocol_name => auth_protocol_name.len() as u16,
        let auth_protocol_data_len: u16 = auth_protocol_data => auth_protocol_data.len() as u16,

        [_; 2],

        #[context(auth_protocol_name_len => *auth_protocol_name_len as usize)]
        pub auth_protocol_name: String8,
        // [_; ..],
        #[context(auth_protocol_data_len => *auth_protocol_data_len as usize)]
        pub auth_protocol_data: String8,
        // [_; ..],
    }

    pub enum ImageByteOrder {
        LittleEndian,
        BigEndian,
    }

    pub enum FormatBitOrder {
        LittleEndian,
        BigEndian,
    }

    pub struct Format {
        pub depth: u8,
        pub bits_per_pixel: u8,
        pub scanline_pad: u8,
        [_; 5],
    }

    pub struct Screen {
        pub root: Window,
        pub default_colormap: Colormap,
        pub white_pixel: u32,
        pub black_pixel: u32,
        pub current_input_masks: EventMask,
        pub width_in_pixels: u16,
        pub height_in_pixels: u16,
        pub width_in_millimeters: u16,
        pub height_in_millimeters: u16,
        pub min_installed_maps: u16,
        pub max_installed_maps: u16,
        pub root_visual: VisualId,
        pub backing_stores: BackingStores,
        pub save_under: bool,
        pub root_depth: u8,
        let allowed_depths_len: u8 = allowed_depths => allowed_depths.len() as u8,
        #[context(allowed_depths_len => *allowed_depths_len as usize)]
        pub allowed_depths: Vec<Depth>,
    }

    pub struct Depth {
        pub depth: u8,
        _,
        let visuals_len: u16 = visuals => visuals.len() as u16,
        [_; 4],
        #[context(visuals_len => *visuals_len as usize)]
        pub visuals: Vec<VisualType>,
    }

    pub struct Color(u32, u32, u32);

    pub enum VisualClass {
        StaticGray,
        Grayscale,
        StaticColor,
        PseudoColor,
        TrueColor,
        DirectColor,
    }

    pub struct VisualType {
        pub visual_id: VisualId,
        pub class: VisualClass,
        pub bits_per_rgb_value: u8,
        pub colormap_entries: u16,
        pub color_mask: Color,
        [_; 4],
    }

    pub enum InitConnectionResponse {
        Failure {
            let reason_len: u8 = reason => reason.len() as u8,
            protocol_major_version: u16,
            protocol_minor_version: u16,
            let length: u16 = 0, // TODO
            #[context(reason_len => *reason_len as usize)]
            reason: String8,
            // [_; ..],
        },
        Success {
            _,
            protocol_major_version: u16,
            protocol_minor_version: u16,
            let length: u16 = 0, // TODO
            release_number: u32,
            resource_id_base: u32,
            resource_id_mask: u32,
            motion_buffer_size: u32,
            let vendor_len: u16 = vendor => vendor.len() as u16,
            maximum_request_length: u16,
            let roots_len: u8 = roots => roots.len() as u8,
            let pixmap_formats_len: u8 = pixmap_formats => pixmap_formats.len() as u8,
            image_byte_order: ImageByteOrder,
            bitmap_format_bit_order: FormatBitOrder,
            bitmap_format_scanline_unit: u8,
            bitmap_format_scanline_pad: u8,
            min_keycode: Keycode,
            max_keycode: Keycode,
            [_; 4],
            #[context(vendor_len => *vendor_len as usize)]
            vendor: String8,
            // [_; ..],
            #[context(pixmap_formats_len => *pixmap_formats_len as usize)]
            pixmap_formats: Vec<Format>,
            #[context(pixmap_formats_len => *pixmap_formats_len as usize)]
            roots: Vec<Screen>,
        },
        AuthenticationRequired {
            [_; 5],
            let length: u16 = 0, // TODO
            pub reason: String8,
            // [_; ..],
        },
    }
}
