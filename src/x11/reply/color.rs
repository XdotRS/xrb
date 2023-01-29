// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Replies] defined in the [core X11 protocol] for
//! [requests that relate to colors].
//!
//! [Replies] are messages sent from the X server to an X client in response to
//! a [request].
//!
//! [Replies]: Reply
//! [request]: crate::message::Request
//! [core X11 protocol]: crate::x11
//!
//! [requests that relate to colors]: request::color

extern crate self as xrb;

use derivative::Derivative;

use xrbk_macro::derive_xrb;

use crate::{message::Reply, x11::request, Colormap};

derive_xrb! {
	/// The [reply] to a [`ListInstalledColormaps` request].
	///
	/// [reply]: Reply
	///
	/// [`ListInstalledColormaps` request]: request::ListInstalledColormaps
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct ListInstalledColormaps: Reply for request::ListInstalledColormaps {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		// The length of `colormaps`.
		#[allow(clippy::cast_possible_truncation)]
		let colormaps_len: u16 = colormaps => colormaps.len() as u16,
		[_; 22],

		/// The [colormaps] which are currently installed on the given
		/// `target`'s [screen].
		///
		/// This list is in no particular order.
		///
		/// This list has no indication as to which [colormaps] are contained in
		/// the [screen]'s list of required [colormaps].
		///
		/// [colormaps]: Colormaps
		/// [screen]: crate::visual::Screen
		#[context(colormaps_len => usize::from(*colormaps_len))]
		pub colormaps: Vec<Colormap>,
	}
}
