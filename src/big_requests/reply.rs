// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate self as xrb;

use crate::{big_requests::request, message::Reply};
use derivative::Derivative;
use xrbk_macro::derive_xrb;

derive_xrb! {
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct BigRequestsEnable: Reply for request::BigRequestsEnable {
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		pub maximum_request_length: u32,
		[_; 2],
	}
}
