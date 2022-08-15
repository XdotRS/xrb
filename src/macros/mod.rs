// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod bitmask;
mod error;
mod event;
mod predefine;
mod reply;
mod request;

/// Generates a doc comment for the given tokens.
///
/// # Example
/// Consider the following syntax:
/// ```rust
/// doc! {
///     "This is a doc comment.",
/// 	const TEXT: &str = "Hello, world!";
/// }
/// ```
/// This will generate:
/// ```rust
/// /// This is a doc comment.
/// const TEXT: &str = "Hello, world!";
/// ```
#[macro_export]
macro_rules! doc {
    ($x:expr, $($t:tt)+) => {
        #[doc = $x]
        $($t)+
    };
}
