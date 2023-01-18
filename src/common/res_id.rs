// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use derive_more::{From, Into};
use xrbk_macro::{new, unwrap, ConstantX11Size, Readable, Wrap, Writable, X11Size};

/// A resource ID referring to either a [`Window`] or a [`Pixmap`].
///
/// Both [windows] and [pixmaps] can be used in graphics operations as `source`s
/// and `destination`s. Collectively, they are known as `Drawable`s.
///
/// [`InputOnly`] [windows], however, cannot be used in graphics operations, and
/// so cannot be `Drawable`s.
///
/// [windows]: Window
/// [pixmaps]: Pixmap
/// [`InputOnly`]: crate::WindowClass::InputOnly
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
)]
pub struct Drawable(pub(crate) u32);

impl From<Window> for Drawable {
	fn from(window: Window) -> Self {
		let Window(id) = window;
		Self(id)
	}
}

impl From<Pixmap> for Drawable {
	fn from(pixmap: Pixmap) -> Self {
		let Pixmap(id) = pixmap;
		Self(id)
	}
}

/// A resource ID referring to a particular window resource.
///
/// Every [screen] has a root window which covers the whole screen. Any other
/// windows on that screen are descendents of that root Window.
///
/// This is a resource ID, which means it cannot collide with the ID of any
/// other resource. These are the types considered resources:
/// - [`Colormap`s](Colormap)
/// - [`CursorAppearance`s](CursorAppearance)
/// - [`GraphicsContext`s](GraphicsContext)
/// - [`Pixmap`s](Pixmap)
/// - [`Window`s](Window)
///
/// [screen]: crate::common::visual::Screen
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
)]
pub struct Window(pub(crate) u32);

impl From<Drawable> for Window {
	fn from(drawable: Drawable) -> Self {
		let Drawable(id) = drawable;
		Self(id)
	}
}

/// A resource ID referring to a particular pixmap resource.
///
/// This is a resource ID, which means it cannot collide with the ID of any
/// other resource. These are the types considered resources:
/// - [`Colormap`s](Colormap)
/// - [`CursorAppearance`s](CursorAppearance)
/// - [`GraphicsContext`s](GraphicsContext) ([`Fontable`])
/// - [`Font`s](Font) ([`Fontable`])
/// - [`Pixmap`s](Pixmap) ([`Drawable`])
/// - [`Window`s](Window) ([`Drawable`])
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
)]
pub struct Pixmap(pub(crate) u32);

impl From<Drawable> for Pixmap {
	fn from(drawable: Drawable) -> Self {
		let Drawable(id) = drawable;
		Self(id)
	}
}

/// A resource ID referring to a particular cursor appearance resource.
///
/// This is a resource ID, which means it cannot collide with the ID of any
/// other resource. These are the types considered resources:
/// - [`Colormap`s](Colormap)
/// - [`CursorAppearance`s](CursorAppearance)
/// - [`GraphicsContext`s](GraphicsContext) ([`Fontable`])
/// - [`Font`s](Font) ([`Fontable`])
/// - [`Pixmap`s](Pixmap) ([`Drawable`])
/// - [`Window`s](Window) ([`Drawable`])
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
)]
pub struct CursorAppearance(pub(crate) u32);

/// A resource ID referring to either a [`Font`] or a [`GraphicsContext`].
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
)]
pub struct Fontable(pub(crate) u32);

impl From<Font> for Fontable {
	fn from(font: Font) -> Self {
		let Font(id) = font;
		Self(id)
	}
}

impl From<GraphicsContext> for Fontable {
	fn from(context: GraphicsContext) -> Self {
		let GraphicsContext(id) = context;
		Self(id)
	}
}

/// A resource ID referring to a particular font resource.
///
/// This is a resource ID, which means it cannot collide with the ID of any
/// other resource. These are the types considered resources:
/// - [`Colormap`s](Colormap)
/// - [`CursorAppearance`s](CursorAppearance)
/// - [`GraphicsContext`s](GraphicsContext) ([`Fontable`])
/// - [`Font`s](Font) ([`Fontable`])
/// - [`Pixmap`s](Pixmap) ([`Drawable`])
/// - [`Window`s](Window) ([`Drawable`])
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
)]
pub struct Font(pub(crate) u32);

impl From<Fontable> for Font {
	fn from(fontable: Fontable) -> Self {
		let Fontable(id) = fontable;
		Self(id)
	}
}

/// A resource ID referring to a particular graphics context resource.
///
/// Information relating to graphics output is stored in a graphics
/// context such as foreground pixel, background pixel, line width,
/// clipping region, etc. A graphics context can only be used with
/// [`Drawable`]s that have the same `root` and `depth` as the
/// `GraphicsContext`.
///
/// This is a resource ID, which means it cannot collide with the ID of any
/// other resource. These are the types considered resources:
/// - [`Colormap`s](Colormap)
/// - [`CursorAppearance`s](CursorAppearance)
/// - [`GraphicsContext`s](GraphicsContext) ([`Fontable`])
/// - [`Font`s](Font) ([`Fontable`])
/// - [`Pixmap`s](Pixmap) ([`Drawable`])
/// - [`Window`s](Window) ([`Drawable`])
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
)]
pub struct GraphicsContext(pub(crate) u32);

impl From<Fontable> for GraphicsContext {
	fn from(fontable: Fontable) -> Self {
		let Fontable(id) = fontable;
		Self(id)
	}
}

/// A resource ID referring to a particular colormap resource.
///
/// This is a resource ID, which means it cannot collide with the ID of any
/// other resource. These are the types considered resources:
/// - [`Colormap`s](Colormap)
/// - [`CursorAppearance`s](CursorAppearance)
/// - [`GraphicsContext`s](GraphicsContext) ([`Fontable`])
/// - [`Font`s](Font) ([`Fontable`])
/// - [`Pixmap`s](Pixmap) ([`Drawable`])
/// - [`Window`s](Window) ([`Drawable`])
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Hash,
	Debug,
	From,
	Into,
	// `new` and `unwrap` const fns
	new,
	unwrap,
	// XRBK traits
	X11Size,
	ConstantX11Size,
	Readable,
	Writable,
	Wrap,
)]
pub struct Colormap(pub(crate) u32);
