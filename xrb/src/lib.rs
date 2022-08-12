// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! An implementation of the
//! [X11 protocol](https://x.org/releases/X11R7.7/doc/xproto/xprotocol.html/).

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

pub mod atoms;
pub mod errors;

mod common;

mod macros;
mod serialization;

pub use common::*;

pub use serialization::{Deserialize, Serialize};
pub use xrb_derive_macros::{Deserialize, Serialize};

pub mod queries {}

pub mod notifications {}

pub mod requests {}

pub mod replies {}
