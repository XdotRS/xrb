// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use derive_more::{From, Into};
use xrbk::X11Size;
use xrbk_macro::{derive_xrb, Readable, Writable, X11Size};

use crate::{mask::EventMask, BackingStores, Color, Colormap, Keycode, String8, VisualId, Window};

/// Calculates the number of bytes used to reach the next 4-byte boundary.
const fn pad(n: usize) -> usize {
	(4 - (n % 4)) % 4
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
pub enum Endianness {
	BigEndian = 0x42,
	LittleEndian = 0x6c,
}

derive_xrb! {
	#[derive(Debug)]
	pub struct InitConnection {
		// XRBK assumes the endianness is big endian, so we hardcode that in.
		let byte_order: Endianness = Endianness::BigEndian,
		_,

		// XRB is implemented for one specific version of the X11 protocol, so
		// it doesn't make sense to allow any other version to be sent here.
		let protocol_major_version: u16 = crate::PROTOCOL_MAJOR_VERSION,
		let protocol_minor_version: u16 = crate::PROTOCOL_MINOR_VERSION,

		#[allow(clippy::cast_possible_truncation)]
		let auth_protocol_name_len: u16 = auth_protocol_name => auth_protocol_name.len() as u16,
		#[allow(clippy::cast_possible_truncation)]
		let auth_protocol_data_len: u16 = auth_protocol_data => auth_protocol_data.len() as u16,
		[_; 2],

		#[context(auth_protocol_name_len => *auth_protocol_name_len as usize)]
		pub auth_protocol_name: String8,
		[_; ..],

		#[context(auth_protocol_data_len => *auth_protocol_data_len as usize)]
		pub auth_protocol_data: String8,
		[_; ..],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, X11Size, Readable, Writable)]
pub enum ImageEndianness {
	LittleEndian,
	BigEndian,
}

derive_xrb! {
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Format {
		pub depth: u8,
		pub bits_per_pixel: u8,
		pub scanline_pad: u8,
		[_; 5],
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, From, Into)]
	pub struct Millimeters(u16);

	#[derive(Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Screen {
		pub root: Window,
		pub default_colormap: Colormap,

		pub white_pixel: u32,
		pub black_pixel: u32,

		pub current_input_masks: EventMask,

		pub width_px: u16,
		pub height_px: u16,
		pub width_mm: Millimeters,
		pub height_mm: Millimeters,

		pub min_installed_maps: u16,
		pub max_installed_maps: u16,

		pub root_visual: VisualId,
		pub backing_stores: BackingStores,
		pub save_unders: bool,
		pub root_depth: u8,

		#[allow(clippy::cast_possible_truncation)]
		let allowed_depths_len: u8 = allowed_depths => allowed_depths.len() as u8,
		#[context(allowed_depths_len => *allowed_depths_len as usize)]
		pub allowed_depths: Vec<Depth>,
	}

	#[derive(Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Depth {
		pub depth: u8,
		_,

		#[allow(clippy::cast_possible_truncation)]
		let visuals_len: u16 = visuals => visuals.len() as u16,
		[_; 4],

		#[context(visuals_len => *visuals_len as usize)]
		pub visuals: Vec<VisualType>,
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum VisualClass {
		StaticGray,
		GrayScale,
		StaticColor,
		PseudoColor,
		TrueColor,
		DirectColor,
	}

	#[derive(Clone, Eq, PartialEq, Hash, Debug)]
	pub struct VisualType {
		pub visual_id: VisualId,
		pub class: VisualClass,
		pub bits_per_rgb_value: u8,
		pub colormap_entries: u16,
		pub color_mask: Color,
		[_; 4],
	}

	#[derive(Debug)]
	pub enum ConnectionResponse {
		/// There was a failure in attempting the connection.
		Failed(ConnectionFailure),
		/// The connection was successfully established.
		Success(ConnectionSuccess),
		/// The connection was refused because authentication was not successful.
		Authenticate(ConnectionAuthenticationError),
	}
}

pub enum ConnError {
	Failed(ConnectionFailure),
	AuthenticationError(ConnectionAuthenticationError),
}

impl ConnectionResponse {
	// false negative for this lint here, so we allow it
	#[allow(clippy::missing_const_for_fn)]
	pub fn ok(self) -> Result<ConnectionSuccess, ConnError> {
		match self {
			Self::Failed(failure) => Err(ConnError::Failed(failure)),
			Self::Success(success) => Ok(success),
			Self::Authenticate(auth_error) => Err(ConnError::AuthenticationError(auth_error)),
		}
	}
}

derive_xrb! {
	/// There was a failure in attempting the connection.
	#[derive(Debug)]
	pub struct ConnectionFailure {
		#[allow(clippy::cast_possible_truncation)]
		let reason_len: u8 = reason => reason.len() as u8,

		/// The major version of the X11 protocol used by the X server.
		protocol_major_version: u16,
		/// The minor version of the X11 protocol used by the X server.
		protocol_minor_version: u16,

		// Length in 4-byte units of "additional data".
		#[allow(clippy::cast_possible_truncation)]
		let additional_data_len: u16 = reason => {
			let len = reason.len() + pad(reason.len());
			(len / 4) as u16
		},

		/// The reason for the failure.
		#[context(reason_len => *reason_len as usize)]
		reason: String8,
		[_; ..],
	}

	/// The connection was successfully established.
	#[derive(Debug)]
	pub struct ConnectionSuccess {
		_,
		/// The major version of the X11 protocol used by the X server.
		protocol_major_version: u16,
		/// The minor version of the X11 protocol used by the X server.
		protocol_minor_version: u16,

		#[allow(clippy::cast_possible_truncation)]
		let additional_data_len: u16 = pixmap_formats, vendor, roots => {
			let vendor_len = vendor.len() + pad(vendor.len());
			let len = 32 + pixmap_formats.x11_size() + vendor_len + roots.x11_size();

			(len / 4) as u16
		},

		release_number: u32,

		resource_id_base: u32,
		resource_id_mask: u32,

		motion_buffer_size: u32,

		#[allow(clippy::cast_possible_truncation)]
		let vendor_len: u16 = vendor => vendor.len() as u16,

		maximum_request_length: u16,

		#[allow(clippy::cast_possible_truncation)]
		let roots_len: u8 = roots => roots.len() as u8,
		#[allow(clippy::cast_possible_truncation)]
		let pixmap_formats_len: u8 = pixmap_formats => pixmap_formats.len() as u8,

		image_byte_order: ImageEndianness,
		bitmap_format_bit_order: ImageEndianness,
		bitmap_format_scanline_unit: u8,
		bitmap_format_scanline_pad: u8,

		min_keycode: Keycode,
		max_keycode: Keycode,
		[_; 4],

		#[context(vendor_len => *vendor_len as usize)]
		vendor: String8,
		[_; ..],

		#[context(pixmap_formats_len => *pixmap_formats_len as usize)]
		pixmap_formats: Vec<Format>,
		#[context(roots_len => *roots_len as usize)]
		roots: Vec<Screen>,
	}

	/// The connection was refused because authentication was unsuccessful.
	#[derive(Debug)]
	pub struct ConnectionAuthenticationError {
		[_; 5],

		// Length in 4-byte units of "additional data".
		#[allow(clippy::cast_possible_truncation)]
		let additional_data_len: u16 = reason => {
			let len = reason.len() + pad(reason.len());
			(len / 4) as u16
		},

		#[context(additional_data_len => {
			// FIXME: but... how do you separate the reason and padding?
			(*additional_data_len as usize) * 4
		})]
		reason: String8,
		[_; ..],
	}
}

#[cfg(feature = "try")]
mod r#try {
	use super::*;
	use std::ops::{ControlFlow, FromResidual, Try};

	impl FromResidual for ConnectionResponse {
		fn from_residual(residual: <Self as Try>::Residual) -> Self {
			residual
		}
	}

	impl Try for ConnectionResponse {
		type Output = ConnectionSuccess;
		type Residual = Self;

		fn from_output(output: Self::Output) -> Self {
			Self::Success(output)
		}

		fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
			match self {
				Self::Success(success) => ControlFlow::Continue(success),

				Self::Failed(_) | Self::Authenticate(_) => ControlFlow::Break(self),
			}
		}
	}
}
