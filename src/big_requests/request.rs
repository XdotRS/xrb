// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Requests] defined in the [Big Requests extension].
//!
//! [Requests] are messages sent from an X client to the X server.
//!
//! [Requests]: crate::message::Request
//! [Big Requests extension]: super

extern crate self as xrb;

use crate::{big_requests::reply, message::Request};
use derivative::Derivative;
use xrbk_macro::derive_xrb;

derive_xrb! {
	/// A [request] that enables extended-length protocol requests for the requesting client.
	///
	/// # Replies
	/// This [request] generates a [`EnableBigRequests` reply].
	///
	/// # Events and Errors
	/// This request does not generate any [Events] or [Errors].
	///
	/// [request]: Request
	/// [Events]: crate::message::Event
	/// [Errors]: crate::message::Error
	/// [`EnableBigRequests` reply]: reply::EnableBigRequests
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct EnableBigRequests: Request(0 /* TODO: extensions use dynamic major opcodes */) -> reply::EnableBigRequests {}
}
