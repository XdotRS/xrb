// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod atoms;

use xrb_proc_macros::{ByteSize, StaticByteSize};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, StaticByteSize, ByteSize)]
pub struct VisualId(u32);

impl VisualId {
	/// Creates a new [`VisualId`] with the given `id`.
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self(id)
	}

	/// Creates a new [`VisualId`] with an `id` of `0`.
	#[must_use]
	pub const fn empty() -> Self {
		Self(0)
	}

	/// Gets the `id` of the [`VisualId`].
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
	/// Creates a new [`Window`] with the given `id`.
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self { id }
	}

	/// Creates a new [`Window`] with an ID of `0`.
	#[must_use]
	pub const fn empty() -> Self {
		Self { id: 0 }
	}
}

impl Pixmap {
	/// Creates a new [`Pixmap`] with the given `id`.
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self { id }
	}

	/// Creates a new [`Pixmap`] with an ID of `0`.
	#[must_use]
	pub const fn empty() -> Self {
		Self { id: 0 }
	}
}

impl Cursor {
	/// Creates a new [`Cursor`] with the given `id`.
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self { id }
	}

	/// Creates a new [`Cursor`] with an ID of `0`.
	#[must_use]
	pub const fn empty() -> Self {
		Self { id: 0 }
	}
}

impl Font {
	/// Creates a new [`Font`] with the given `id`.
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self { id }
	}

	/// Creates a new [`Font`] with an ID of `0`.
	#[must_use]
	pub const fn empty() -> Self {
		Self { id: 0 }
	}
}

impl GraphicsContext {
	/// Creates a new [`GraphicsContext`] with the given `id`.
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self { id }
	}

	/// Creates a new [`GraphicsContext`] with an ID of `0`.
	#[must_use]
	pub const fn empty() -> Self {
		Self { id: 0 }
	}
}

impl Colormap {
	/// Creates a new [`Colormap`] with the given `id`.
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self { id }
	}

	/// Creates a new [`Colormap`] with an ID of `0`.
	#[must_use]
	pub const fn empty() -> Self {
		Self { id: 0 }
	}
}

fn _assert_object_safety(_res_id: &dyn ResId) {}
