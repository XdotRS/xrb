// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod change_save_set;
pub mod change_window_attributes;
pub mod create_window;
pub mod destroy_subwindows;
pub mod destroy_window;
pub mod get_window_attributes;
pub mod map_subwindows;
pub mod map_window;
pub mod reparent_window;
pub mod unmap_subwindows;
pub mod unmap_window;
// pub mod configure_window;
pub mod circulate_window;
pub mod get_geometry;
pub mod query_tree;

use bitflags::bitflags;

use crate::x11::common::masks::{DeviceEventMask, EventMask};
use crate::x11::common::values::{BitGravity, Colormap, Cursor, Pixmap, WinGravity};

use crate::x11::wrappers::{Inherit, Relative};

use crate::rw::Serialize;
use crate::values;

use create_window::BackingStore;

/// A request is a message sent from an X client to the X server.
///
/// Since an X client will never receive an actual request message,
/// deserialization is not implemented for requests for the sake of simplicity.
pub trait Request<REPLY = ()>: Serialize {
	/// The major opcode that uniquely identifies this request or extension.
	///
	/// X core protocol requests have unique major opcodes, but each extension
	/// is only assigned one major opcode. Extensions are assigned major opcodes
	/// from 127 through to 255.
	fn opcode() -> u8;

	/// The minor opcode that uniquely identifies this request within its
	/// extension.
	///
	/// As each extension is only assigned one major opcode, the minor opcode
	/// can be used to distinguish different requests contained within an
	/// extension.
	///
	/// [`None`] means that either this request is not from an extension, or the
	/// extension does not make use of the minor opcode, likely because it only
	/// has one request.
	///
	/// [`Some`] means that there is indeed a minor opcode associated with this
	/// request. This request is therefore from an extension.
	fn minor_opcode() -> Option<u16>;

	/// The length of this request, including the header, in 4-byte units.
	///
	/// Every request contains a header whcih is 4 bytes long. This header is
	/// included in the length, so the minimum length is 1 unit (4 bytes). The
	/// length represents the _exact_ length of the request: padding bytes may
	/// need to be added to the end of the request to ensure its length is
	/// brought up to a multiple of 4, if it is not already.
	fn length(&self) -> u16;
}

values! {
	/// Window attributes that can be configured in various requests.
	///
	/// Attributes given in `values` vectors MUST be in the order given in this
	/// enum, so that they match the order of the [`WinAttrMask`].
	pub enum WinAttr<WinAttrMask> {
		BackgroundPixmap(Option<Relative<Pixmap>>): BACKGROUND_PIXMAP,
		BackgroundPixel(u32): BACKGROUND_PIXEL,
		BorderPixmap(Inherit<Pixmap>): BORDER_PIXMAP,
		BorderPixel(u32): BORDER_PIXEL,
		BitGravity(BitGravity): BIT_GRAVITY,
		WinGravity(WinGravity): WIN_GRAVITY,
		BackingStore(BackingStore): BACKING_STORE,
		BackingPlanes(u32): BACKING_PLANES,
		BackingPixel(u32): BACKING_PIXEL,
		OverrideRedirect(bool): OVERRIDE_REDIRECT,
		SaveUnder(bool): SAVE_UNDER,
		EventMask(EventMask): EVENT_MASK,
		DoNotPropagateMask(DeviceEventMask): DO_NOT_PROPAGATE_MASK,
		Colormap(Inherit<Colormap>): COLORMAP,
		Cursor(Option<Cursor>): CURSOR,
	}
}

bitflags! {
	/// A mask of [window attributes] that can be used in various requests.
	///
	/// [window attributes]:WinAttr
	pub struct WinAttrMask: u32 {
		/// The [`BackgroundPixmap`] [attribute](WinAttr).
		///
		/// [`BackgroundPixmap`]:WinAttr::BackgroundPixmap
		const BACKGROUND_PIXMAP = 0x_0000_0001;
		/// The [`BackgroundPixel] [CreateWindow] request [value](WinAttr).
		///
		/// [`BackgroundPixel`]:WinAttr::BackgroundPixel
		const BACKGROUND_PIXEL = 0x_0000_0002;
		/// The [`BorderPixmap`] [attribute](WinAttr).
		///
		/// [`BorderPixmap`]:WinAttr:BorderPixmap
		const BORDER_PIXMAP = 0x_0000_0004;
		/// The [`BorderPixel`] [attribute](WinAttr).
		///
		/// [`BorderPixel`]:WinAttr::BorderPixel
		const BORDER_PIXEL = 0x_0000_0008;
		/// The [`BitGravity`] [attribute](WinAttr).
		///
		/// [`BitGravity`]:WinAttr::BitGravity
		const BIT_GRAVITY = 0x_0000_0010;
		/// The [`WinGravity`] [attribute](WinAttr).
		///
		/// [`WinGravity`]:WinAttr::WinGravity
		const WIN_GRAVITY = 0x_0000_0020;
		/// The [`BackingStore`] [attribute](WinAttr).
		///
		/// [`BackingStore`]:WinAttr::BackingStore
		const BACKING_STORE = 0x_0000_0040;
		/// The [`BackingPlanes`] [attribute](WinAttr).
		///
		/// [`BackingPlanes`]:WinAttr::BackingPlanes
		const BACKING_PLANES = 0x_0000_0080;
		/// The [`BackingPixel`] [attribute](WinAttr).
		///
		/// [`BackingPixel`]:WinAttr::BackingPixel
		const BACKING_PIXEL = 0x_0000_0100;
		/// The [`OverrideRedirect`] [attribute](WinAttr).
		///
		/// [`OverrideRedirect`]:WinAttr::OverrideRedirect
		const OVERRIDE_REDIRECT = 0x_0000_0200;
		/// The [`SaveUnder`] [attribute](WinAttr).
		///
		/// [`SaveUnder`]:WinAttr::SaveUnder
		const SAVE_UNDER = 0x_0000_0400;
		/// The [`EventMask`] [attribute](WinAttr).
		///
		/// [`EventMask`]:WinAttr::EventMask
		const EVENT_MASK = 0x_0000_0800;
		/// The [`DoNotPropagateMask`] [attribute](WinAttr).
		///
		/// [`DoNotPropagateMask`]:WinAttr::DoNotPropagateMask
		const DO_NOT_PROPAGATE_MASK = 0x_0000_1000;
		/// The [`Colormap`] [attribute](WinAttr).
		///
		/// [`Colormap`]:WinAttr::Colormap
		const COLORMAP = 0x_0000_2000;
		/// The [`Cursor`] [attribute](WinAttr).
		///
		/// [`Cursor`]:WinAttr::Cursor
		const CURSOR = 0x_0000_4000;
	}
}
