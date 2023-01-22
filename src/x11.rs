// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Messages defined in the core X11 protocol: [requests], [replies], [events],
//! and [errors].
//!
//! [requests]: x11::request
//! [replies]: x11::reply
//! [events]: x11::event
//! [errors]: x11::error

pub mod error;
pub mod event;
// pub mod reply;
pub mod request;
