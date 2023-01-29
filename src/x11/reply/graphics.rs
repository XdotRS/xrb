// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Replies] defined in the [core X11 protocol] for
//! [requests that relate to graphics operations].
//!
//! [Replies] are messages sent from the X server to an X client in response to
//! a [request].
//!
//! [Replies]: crate::message::Reply
//! [request]: crate::message::Request
//! [core X11 protocol]: crate::x11
//!
//! [requests that relate to graphics operations]: request::graphics

extern crate self as xrb;

use derivative::Derivative;

use xrbk::pad;
use xrbk_macro::derive_xrb;

use crate::{visual::VisualId, x11::request};

derive_xrb! {
	/// The [reply] to a [`CaptureImage` request].
	///
	/// [reply]: crate::message::Reply
	///
	/// [`CaptureImage` request]: request::CaptureImage
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct CaptureImage: Reply for request::CaptureImage {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: crate::message::Reply
		///
		/// [`Reply::sequence`]: crate::message::Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// The depth of the `target` [drawable] when it was created.
		///
		/// [drawable]: crate::Drawable
		#[metabyte]
		pub depth: u8,

		/// The visual type of the `target` if it is a [window].
		///
		/// If the `target` is a [pixmap], this is [`None`].
		///
		/// [window]: crate::Window
		/// [pixmap]: crate::Pixmap
		pub visual: Option<VisualId>,
		[_; 20],

		// FIXME: how do we know what is padding and what is data?????
		/// The image's data.
		#[context(self::remaining => remaining)]
		pub data: Vec<u8>,
		[_; data => pad(data)],
	}
}
