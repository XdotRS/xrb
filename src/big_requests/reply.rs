// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Replies] defined in the [Big Requests extension].
//!
//! [Replies] are messages sent from the X server to an X client in response to
//! a [request].
//!
//! [Replies]: crate::message::Reply
//! [request]: crate::message::Request
//! [Big Requests extension]: super

extern crate self as xrb;

use crate::{big_requests::request, message::Reply};
use derivative::Derivative;
use xrbk_macro::derive_xrb;

derive_xrb! {
	/// The [reply] to a [`EnableBigRequests` request].
	///
	/// [reply]: Reply
	///
	/// [`EnableBigRequests` request]: request::EnableBigRequests
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct EnableBigRequests: Reply for request::EnableBigRequests {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// The new maximum length for a [request].
		///
		/// This value will always be greater than the one returned in [initial setup].
		///
		/// [initial setup]: crate::connection
		/// [request]: crate::message::Request
		pub maximum_request_length: u32,
		[_; 2],
	}
}
