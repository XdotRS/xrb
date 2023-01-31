// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate self as xrb;

use crate::{message::Reply, x11::request, Dimensions};

use derivative::Derivative;
use xrbk_macro::derive_xrb;

derive_xrb! {
	/// The [reply] to a [`QueryIdealDimensions` request].
	///
	/// [reply]: Reply
	///
	/// [`QueryIdealDimensions` request]: request::QueryIdealDimensions
	#[doc(alias("QueryBestSize"))]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct QueryIdealDimensions: Reply for request::QueryIdealDimensions {
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

		/// The ideal [dimensions], as described in the
		/// [`QueryIdealDimensions` request].
		///
		/// [dimensions]: Dimensions
		///
		/// [`QueryIdealDimensions` request]: request::QueryIdealDimensions
		#[doc(alias("width", "height", "dimensions", "ideal_width", "ideal_height"))]
		pub ideal_dimensions: Dimensions,
		[_; ..],
	}
}
