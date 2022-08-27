// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[macro_use]
mod wrapper_macro;

use crate::protocol::common::values::{Timestamp, Window};

wrappers! {
	pub enum Inherit<T> {
		Value(T),
		CopyFromParent = 0,
	}

	pub enum Relative<T> {
		Value(T),
		ParentRelative = 1,
	}

	pub enum Specificity<T> {
		Specific(T),
		Any = 0,
	}

	pub enum Time {
		Specific(Timestamp),
		Current = 0,
	}

	pub enum Destination {
		Window(Window),
		PointerWindow = 0,
		InputFocus = 1,
	}

	pub enum Focus {
		Window(Window),
		PointerRoot = 1,
	}
}
