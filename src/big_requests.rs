// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Messages defined in the [Big Requests extension].
//!
//! The [Big Requests extension] enables extended length field for large
//! requests.
//!
//! [Requests]: crate::message::Request
//! [Replies]: crate::message::Reply
//! [Big Requests extension]: https://www.x.org/releases/X11R7.7/doc/bigreqsproto/bigreq.html

/// This extension's internal name.
/// This is used to retrieve the major opcode for this extension.
pub const EXTENSION_NAME: &'static str = "BIG-REQUESTS";

pub mod reply;
pub mod request;
