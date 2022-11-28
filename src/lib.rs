// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Enable the `doc_notable_trait` feature, which allows certain traits to be
// designated as notable in documentation, thus bringing the reader's attention
// to them above others.
//
// This is used in traits that provide core functionality for a type where that
// type would not be "complete" without it. We use it for [`Request`] and
// [`Reply`], for example.
#![feature(doc_notable_trait)]
// This is so we can provide a reason when we ignore a particular lint with
// `allow`.
#![feature(lint_reasons)]
// Deny the following clippy lints to enforce them:
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
// Warn for these lints, rather than denying them.
#![warn(clippy::use_self)]
// Warn for pedantic & cargo lints. They are allowed completely by default.
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
// Continue to allow these though.
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::module_name_repetitions)]

//! # X Rust Bindings
//! X Rust Bindings is a Rust library directly implementing the types and
//! protocol messages of the
//! [X11 protocol specification](https://x.org/releases/X11R7.7/doc/xproto/xprotocol.html/).
//! XRB is _not_ a high-level API library, and it does not provide a direct
//! connection to an X server, nor does it do anything else on its own. XRB's
//! development purpose is to provide a foundation for higher-level Rust API
//! wrapper libraries. It is used by [X.RS](https://crates.io/crates/xrs), the
//! official accompanying API library for XRB.

/// The major version of the X protocol used in XRB.
///
/// The X protocol major version may increment if breaking changes are introduced; seeing as this
/// has not happened since the 80s, it's probably safe to assume it won't.
pub const PROTOCOL_MAJOR_VERSION: u16 = 11;
/// The minor version of the X protocol used in XRB.
///
/// The X protocol minor version may increment if non-breaking features are added to the X
/// protocol; seeing as this has not happened since the 80s, it's probably safe to assume it won't.
pub const PROTOCOL_MINOR_VERSION: u16 = 0;

// /// Implementations for the core X11 protocol.
// mod x11;
mod traits;

pub use traits::*;

extern crate self as xrb;

use xrbk_macro::define;

define! {
	pub struct Window(u32);
	pub struct VisualId(u32);

	pub struct GetWindowAttributes: Request(3) -> GetWindowAttributesReply {
		pub window: Window,
	}

	pub enum BackingStore {
		NotUseful,
		WhenMapped,
		Always,
	}

	pub enum WindowClass {
		InputOutput = 1,
		InputOnly,
	}

	pub enum MapState {
		Unmapped,
		Unviewable,
		Viewable,
	}

	pub struct GetWindowAttributesReply: Reply for GetWindowAttributes {
		#[metabyte]
		pub backing_store: BackingStore,
		pub visual_id: VisualId,
		pub class: WindowClass,
		//pub bit_gravity: BitGravity,
		//pub win_gravity: WinGravity,
		pub backing_planes: u32,
		pub backing_pixel: u32,
		pub save_under: bool,
		pub map_is_installed: bool,
		pub map_state: MapState,
		pub override_redirect: bool,
		//pub colormap: Option<Colormap>,
		//pub all_event_masks: EventMask,
		//pub your_event_mask: EventMask,
		//pub do_not_propagate_mask: DeviceEvent,
		[(); 2],
	}
}
