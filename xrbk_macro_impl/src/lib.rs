// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The implementation of [XRBK macro].
//!
//! This crate is not for general use.
//!
//! [XRBK macro]: https://github.com/XdotRS/xrb/tree/main/xrbk_macro

// Deny these lints.
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
// Warn for these lints.
#![warn(clippy::use_self)]
#![warn(clippy::pedantic)]
#![warn(clippy::complexity)]
#![warn(clippy::cargo)]
#![warn(clippy::missing_const_for_fn)]
#![warn(rustdoc::broken_intra_doc_links)]
// Allow these lints.
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::module_name_repetitions)]

pub mod definition;
