// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! # X Rust Bindings
//! X Rust Bindings is a Rust library directly implementing the types and protocol messages of the
//! [X11 protocol specification](https://x.org/releases/X11R7.7/doc/xproto/xprotocol.html/). XRB is
//! _not_ a high-level API library, and it does not provide a direct connection to an X server, nor
//! does it do anything else on its own. XRB's development purpose is to provide a foundation for
//! higher-level Rust API wrapper libraries. It is used by [X.RS](https://crates.io/crates/xrs),
//! the official accompanying API library for XRB.
//!
//! To demonstrate the difference between X Rust Bindings and a higher-level API library, here is a
//! comparison between the same protocol message in XRB and its higher-level equivalent in
//! [X.RS](https://crates.io/crates/xrs):
//!
//! ### `ConnectionInit` request in XRB
//! ```rust
//! /// A request to initiate a connection to the X server.
//! pub struct ConnectionInit<'a> {
//!     pub byte_order: ByteOrder,
//!     /// Should always be 11.
//!     pub protocol_major_version: u16,
//!     /// Should always be 0.
//!     pub protocol_minor_version: u16,
//!     pub auth_protocol_name: &'a str,
//!     pub auth_data: &'a str,
//! }
//! ```
//!
//! ### `InitConnection` request in X.RS
//! ```rust
//! /// A request to initiate a connection to the X server.
//! pub struct InitConnection {}
//! ```
//!
// TODO: You can find a glossary for use in writing docs
// [here](https://x.org/releases/X11R7.7/doc/xproto/x11protocol.html#glossary).

/// The major version of the X protocol used in XRB. Should always be 11.
///
/// The X protocol major version may increment if breaking changes are introduced; seeing as this
/// has not happened since the 80s, it's probably safe to assume it won't.
pub const PROTOCOL_MAJOR_VERSION: u16 = 11;
/// The minor version of the X protocol used in XRB. Should always be 0.
///
/// The X protocol minor version may increment if non-breaking features are added to the X
/// protocol; seeing as this has not happened since the 80s, it's probably safe to assume it won't.
pub const PROTOCOL_MINOR_VERSION: u16 = 0;

mod proto;

mod macros;
mod serialization;

pub use proto::bitmasks;
pub use proto::common::*;

pub use serialization::{Deserialize, KnownSize, Serialize};
pub use xrb_proc_macros::{Deserialize, KnownSize, Serialize};

pub mod queries {}

pub mod notifications {}

pub mod requests {}

pub mod replies {}
