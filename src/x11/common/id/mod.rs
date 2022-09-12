// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod atoms;

use xrb_proc_macros::{ByteSize, StaticByteSize};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub struct VisualId(u32);

impl VisualId {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self(id)
	}

	#[must_use]
	pub const fn id(&self) -> u32 {
		self.0
	}
}

/// An ID for resources that is unique among other resources.
///
/// A _resource_ is a:
/// - [`Window`]; or a
/// - [`Pixmap`]; or a
/// - [`Cursor`]; or a
/// - [`Font`]; or a
/// - [`GraphicsContext`]; or a
/// - [`Colormap`].
///
/// A resource ID must only be unique among other resources. For example, let's
/// say that a [`Window`] has a resource ID of `7` - this means that no other
/// resource, whether it's a [`Window`] or another resource like a [`Font`], is
/// allowed to share that resource ID of 7. An ID that is _not_ a resource (e.g.
/// [`Atom`]), however, may use the same ID as a resource.
#[doc(notable_trait)]
pub trait ResId {
	/// The resource ID for this resource.
	fn res_id(&self) -> u32;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub struct Window {
	id: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub struct Pixmap {
	id: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub struct Cursor {
	id: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub struct Font {
	id: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub struct GraphicsContext {
	id: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub struct Colormap {
	id: u32,
}

pub trait Drawable {}
pub trait Fontable {}

impl Drawable for Window {}
impl Drawable for Pixmap {}

impl Fontable for Font {}
impl Fontable for GraphicsContext {}

impl ResId for Window {
	fn res_id(&self) -> u32 {
		self.id
	}
}

impl ResId for Pixmap {
	fn res_id(&self) -> u32 {
		self.id
	}
}

impl ResId for Cursor {
	fn res_id(&self) -> u32 {
		self.id
	}
}

impl ResId for Font {
	fn res_id(&self) -> u32 {
		self.id
	}
}

impl ResId for GraphicsContext {
	fn res_id(&self) -> u32 {
		self.id
	}
}

impl ResId for Colormap {
	fn res_id(&self) -> u32 {
		self.id
	}
}

impl Window {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self { id }
	}
}

impl Pixmap {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self { id }
	}
}

impl Cursor {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self { id }
	}
}

impl Font {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self { id }
	}
}

impl GraphicsContext {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self { id }
	}
}

impl Colormap {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self { id }
	}
}

fn _assert_object_safety(_res_id: &dyn ResId) {}
