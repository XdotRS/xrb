// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! # Cornflakes
//!
//! Cornflakes is a utility library developed for [X Rust Bindings]. It provides
//! traits for extending the functionality of [bytes]' [`Buf`] and [`BufMut`]
//! traits with some utilities relevant to XRB, as well as [`ToBytes`] and
//! [`FromBytes`] traits for (de)serialization, and [`ByteSize`] and
//! [`StaticByteSize`] traits for getting the byte size a type is written as.
//!
//! [X Rust Bindings]: https://docs.aquariwm.org/doc/xrb/
//! [bytes]: https://docs.rs/bytes/latest/bytes/
//! [`Buf`]: https://docs.rs/bytes/latest/bytes/trait.Buf.html
//! [`BufMut`]: https://docs.rs/bytes/latest/bytes/trait.BufMut.html

mod implementations;
mod traits;

pub use implementations::*;
pub use traits::*;

pub(crate) type IoResult<T = ()> = Result<T, std::io::Error>;
