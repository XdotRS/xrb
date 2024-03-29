// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Messages to initialize a connection with an X server.

use xrbk::X11Size;
use xrbk_macro::{derive_xrb, Readable, Writable, X11Size};

use crate::{
	visual::{Format, Screen},
	Keycode,
	String8,
};

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
	#[derive(Debug, X11Size, Readable, Writable)]
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
	#[derive(Debug, X11Size, Readable, Writable)]
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
	#[derive(Debug, X11Size, Readable, Writable)]
	pub struct ConnectionFailure {
		#[allow(clippy::cast_possible_truncation)]
		let reason_len: u8 = reason => reason.len() as u8,

		/// The major version of the X11 protocol used by the X server.
		pub protocol_major_version: u16,
		/// The minor version of the X11 protocol used by the X server.
		pub protocol_minor_version: u16,

		// Length in 4-byte units of "additional data".
		#[allow(clippy::cast_possible_truncation)]
		let additional_data_len: u16 = reason => {
			let len = reason.len() + pad(reason.len());
			(len / 4) as u16
		},

		/// The reason for the failure.
		#[context(reason_len => *reason_len as usize)]
		pub reason: String8,
		[_; ..],
	}

	/// The connection was successfully established.
	#[derive(Debug, X11Size, Readable, Writable)]
	pub struct ConnectionSuccess {
		_,
		/// The major version of the X11 protocol used by the X server.
		pub protocol_major_version: u16,
		/// The minor version of the X11 protocol used by the X server.
		pub protocol_minor_version: u16,

		#[allow(clippy::cast_possible_truncation)]
		let additional_data_len: u16 = pixmap_formats, vendor, roots => {
			let vendor_len = vendor.len() + pad(vendor.len());
			let len = 32 + pixmap_formats.x11_size() + vendor_len + roots.x11_size();

			(len / 4) as u16
		},

		pub release_number: u32,

		pub resource_id_base: u32,
		pub resource_id_mask: u32,

		pub motion_buffer_size: u32,

		#[allow(clippy::cast_possible_truncation)]
		let vendor_len: u16 = vendor => vendor.len() as u16,

		pub maximum_request_length: u16,

		#[allow(clippy::cast_possible_truncation)]
		let roots_len: u8 = roots => roots.len() as u8,
		#[allow(clippy::cast_possible_truncation)]
		let pixmap_formats_len: u8 = pixmap_formats => pixmap_formats.len() as u8,

		pub image_byte_order: ImageEndianness,
		pub bitmap_format_bit_order: ImageEndianness,
		pub bitmap_format_scanline_unit: u8,
		pub bitmap_format_scanline_padding: u8,

		pub min_keycode: Keycode,
		pub max_keycode: Keycode,
		[_; 4],

		#[context(vendor_len => *vendor_len as usize)]
		pub vendor: String8,
		[_; ..],

		#[context(pixmap_formats_len => *pixmap_formats_len as usize)]
		pub pixmap_formats: Vec<Format>,
		#[context(roots_len => *roots_len as usize)]
		pub roots: Vec<Screen>,
	}

	/// The connection was refused because authentication was unsuccessful.
	#[derive(Debug, X11Size, Readable, Writable)]
	pub struct ConnectionAuthenticationError {
		[_; 5],

		#[allow(clippy::cast_possible_truncation)]
		/// Length in 4-byte units of "additional data".
		let additional_data_len: u16 = reason => {
			let len = reason.len() + pad(reason.len());
			(len / 4) as u16
		},

		#[context(additional_data_len => {
			// FIXME: but... how do you separate the reason and padding?
			(*additional_data_len as usize) * 4
		})]
		pub reason: String8,
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
