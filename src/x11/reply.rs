// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Replies] defined in the [core X11 protocol].
//!
//! [Replies] are messages sent from the X server to an X client in response to
//! a [request].
//!
//! [Replies]: crate::message::Reply
//! [request]: crate::message::Request
//! [core X11 protocol]: super

// TODO: should these modules be private and re-exported, or public?
//       or public and also re-exported?

pub use color::*;
pub use font::*;
pub use graphics::*;
pub use input::*;
pub use meta::*;
pub use miscellaneous::*;
pub use window::*;

pub mod color;
pub mod font;
pub mod graphics;
pub mod input;
pub mod meta;
pub mod miscellaneous;
pub mod window;
