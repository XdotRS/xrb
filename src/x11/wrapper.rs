// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::x11::common::Timestamp;

pub enum Inheritable<T> {
	CopyFromParent,
	Specific(T),
}

pub enum Relatable<T> {
	ParentRelative,
	Specific(T),
}

pub enum Any<T> {
	Any,
	Specific(T),
}

pub enum Time {
	CurrentTime,
	Specific(Timestamp),
}

pub enum InputFocus<T> {
	PointerRoot,
	Specific(T),
}

// Would this be better as a `Parent` unit struct and a ttype alias for
// `Option<InputFocus<Parent>>`?
pub enum RevertTo {
	None,
	PointerRoot,
	Parent,
}
