// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// TODO
//
// Traits:
// - Reader -- Provides utilities for reading data from a buffer of bytes.
// - Writer -- Provides utilities for writing data to a buffer of bytes.
//
// - ReadBytes -- Reads a type from bytes with no extra information required.
// - ReadSized -- Reads a type from bytes with a specific number of bytes.
// - ReadList -- Reads a list of values from bytes with a given length for the
//   list.
//
// - WriteBytes -- Writes a type as bytes with no extra information required.
// - WriteSized -- Writes a type as bytes with a specific number of bytes.
//
// - ByteCount -- Returns the number of bytes that a type will be written as.

#![feature(doc_notable_trait)]
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

mod rw;
mod util;
