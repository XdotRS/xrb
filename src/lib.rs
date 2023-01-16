// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Enable the `doc_notable_trait` feature, which allows certain traits to be
// designated as notable in documentation, thus bringing the reader's attention
// to them above others.
//
// This is used in traits that provide core functionality for a type where that
// type would not be "complete" without it. We use it for [`Request`] and
// [`Reply`], for example.
#![feature(doc_notable_trait)]
// This is so we can provide a reason when we ignore a particular lint with
// `allow`.
#![feature(lint_reasons)]
#![cfg_attr(feature = "try", feature(try_trait_v2))]
// Deny the following clippy lints to enforce them:
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
// Warn for these lints, rather than denying them.
#![warn(clippy::use_self)]
// Warn for pedantic & cargo lints. They are allowed completely by default.
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
// Continue to allow these though.
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::module_name_repetitions)]
#![warn(missing_docs)]

//! <h1 align="center" style="margin-bottom: 0;">
//!     X Rust Bindings
//! </h1>
//! <p align="center">
//!     <a href="https://github.com/XdotRS/xrb/blob/main/LICENSE">
//!         <img src="https://img.shields.io/crates/l/xrb?style=for-the-badge" /></a>
//!     <a href="https://github.com/XdotRS/xrb/issues">
//!         <img src="https://img.shields.io/github/issues-raw/XdotRS/xrb?style=for-the-badge" /></a>
//!     <a href="https://github.com/orgs/XdotRS/projects/1/views/1">
//!         <img src="https://img.shields.io/badge/todo-project-8860b8?style=for-the-badge" /></a>
//!     <a href="https://github.com/XdotRS/xrb/actions/workflows/ci.yml">
//!         <img src="https://img.shields.io/github/actions/workflow/status/XdotRS/xrb/ci.yml?event=push&branch=main&label=ci&style=for-the-badge" /></a>
//! </p>
//!
//! X Rust Bindings, better known as XRB, is a crate implementing messages,
//! types, data structures, and their serialization/deserialization for the
//! [X Window System protocol v11 (a.k.a. X11)][X11]. It provides a
//! foundation upon which more opinionated APIs and connection handling may
//! be built in order to form an 'X library'. In particular, XRB will server
//! as the foundation for [X.RS] in the future.
//!
//! [X11]: https://x.org/releases/X11R7.7/doc/x11protocol.html
//! [X.RS]: https://github.com/XdotRS/xrs/

/// The major version of the X protocol used in XRB.
///
/// The X protocol major version may increment if breaking changes are
/// introduced; seeing as this has not happened since the 80s, it's probably
/// safe to assume it won't.
pub const PROTOCOL_MAJOR_VERSION: u16 = 11;
/// The minor version of the X protocol used in XRB.
///
/// The X protocol minor version may increment if non-breaking features are
/// added to the X protocol; seeing as this has not happened since the 80s, it's
/// probably safe to assume it won't.
pub const PROTOCOL_MINOR_VERSION: u16 = 0;

mod common;
pub mod connection;
pub mod message;

/// Implementation of the core X11 protocol.
pub mod x11 {
	pub mod event;
	// pub mod request;
	pub mod error;
}

pub use common::*;
