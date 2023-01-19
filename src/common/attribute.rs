// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{
	util,
	BackingStore,
	BitGravity,
	Colormap,
	CopyableFromParent,
	CursorAppearance,
	DeviceEventMask,
	EventMask,
	ParentRelatable,
	Pixel,
	Pixmap,
	WindowGravity,
};
use xrbk::{
	Buf,
	BufMut,
	ConstantX11Size,
	ReadError,
	ReadResult,
	Readable,
	Writable,
	WriteResult,
	X11Size,
};

use bitflags::bitflags;
use xrbk_macro::{ConstantX11Size, Readable, Writable, X11Size};

bitflags! {
	/// A mask of [attributes] given for a [window].
	///
	/// For more information, and for the attributes themselves, please see [`Attributes`].
	///
	/// [attributes]: Attributes
	/// [window]: crate::Window
	#[derive(Default, X11Size, Readable, ConstantX11Size, Writable)]
	pub struct AttributeMask: u32 {
		/// See also: [`background_pixmap`], <code>[ParentRelatable]<[Option]<[Pixmap]>></code>.
		///
		/// [`background_pixmap`]: Attributes::background_pixmap
		const BACKGROUND_PIXMAP = 0x0000_0001;
		/// See also: [`background_pixel`], [`Pixel`].
		///
		/// [`background_pixel`]: Attributes::background_pixel
		const BACKGROUND_PIXEL = 0x0000_0002;

		/// See also: [`border_pixmap`], <code>[CopyableFromParent]<[Pixmap]></code>.
		///
		/// [`border_pixmap`]: Attributes::border_pixmap
		const BORDER_PIXMAP = 0x0000_0004;
		/// See also: [`border_pixel`], [`Pixel`].
		///
		/// [`border_pixel`]: Attributes::border_pixel
		const BORDER_PIXEL = 0x0000_0008;

		/// See also: [`bit_gravity`], [`BitGravity`].
		///
		/// [`bit_gravity`]: Attributes::bit_gravity
		const BIT_GRAVITY = 0x0000_0010;
		/// See also: [`window_gravity`], [`WindowGravity`].
		///
		/// [`window_gravity`]: Attributes::window_gravity
		const WINDOW_GRAVITY = 0x0000_0020;

		/// See also: [`backing_store`], [`BackingStore`].
		///
		/// [`backing_store`]: Attributes::backing_store
		const BACKING_STORE = 0x0000_0040;
		/// See also: [`backing_planes`], [`u32`].
		///
		/// [`backing_planes`]: Attributes::backing_planes
		const BACKING_PLANES = 0x0000_0080;
		/// See also: [`backing_pixel`], [`Pixel`].
		///
		/// [`backing_pixel`]: Attributes::backing_pixel
		const BACKING_PIXEL = 0x0000_0100;

		/// See also: [`override_redirect`], [`bool`].
		///
		/// [`override_redirect`]: Attributes::OverrideRedirect
		const OVERRIDE_REDIRECT = 0x0000_0200;
		/// See also: [`save_under`], [`bool`].
		///
		/// [`save_under`]: Attributes::save_under
		const SAVE_UNDER = 0x0000_0400;

		/// See also: [`event_mask`], [`EventMask`].
		///
		/// [`event_mask`]: Attributes::event_mask
		const EVENT_MASK = 0x0000_0800;
		/// See also: [`do_not_propagate_mask`], [`DeviceEventMask`].
		///
		/// [`do_not_propagate_mask`]: Attributes::do_not_propagate_mask
		const DO_NOT_PROPAGATE_MASK = 0x0000_1000;

		/// See also: [`colormap`], <code>[CopyableFromParent]<[Colormap]></code>.
		///
		/// [`colormap`]: Attributes::colormap
		const COLORMAP = 0x0000_2000;

		/// See also: [`cursor_appearance`], [`CursorAppearance`].
		///
		/// [`cursor_appearance`]: Attributes::cursor_appearance
		const CURSOR_APPEARANCE = 0x0000_4000;
	}
}

/// This is a type alias for <code>[ParentRelatable]<[Option]<[Pixmap]>></code>.
///
/// This represents the type used in [`background_pixmap` attributes].
///
/// [`background_pixmap` attributes]: Attributes::background_pixmap
pub type BackgroundPixmap = ParentRelatable<Option<Pixmap>>;
/// This is a type alias for <code>[CopyableFromParent]<[Pixmap]></code>.
///
/// This represents the type used in [`border_pixmap` attributes].
///
/// [`border_pixmap` attributes]: Attributes::border_pixmap
pub type BorderPixmap = CopyableFromParent<Pixmap>;
/// This is a type alias for <code>[Option]<[CursorAppearance]></code>.
///
/// This represents the type used in
/// [`cursor_appearance` attributes][attribute].
///
/// This type alias exists because there can be confusion where the
/// [`cursor_appearance` attribute][attribute] may be missing:
/// <code>[Option]<[Option]<[CursorAppearance]>></code>. The outer `Option`
/// refers to whether this attribute is specified, whereas the inner `Option`
/// refers to whether the cursor has an appearance.
///
/// [attribute]: Attributes::cursor_appearance
pub type CursorAppearanceAttribute = Option<CursorAppearance>;
/// This is a type alias for <code>[CopyableFromParent]<[Colormap]></code>.
///
/// This represents the type used in [`colormap` attributes].
///
/// [`colormap` attributes]: Attributes::colormap
pub type ColormapAttribute = CopyableFromParent<Colormap>;

/// A set of attributes for a [window].
///
/// The following table shows each attribute, its default value if it is
/// not explicitly initialized in the [`CreateWindow` request], and the
/// [window classes] that it can be set with.
///
/// [window]: Window
/// [`CreateWindow` request]: crate::x11::request::CreateWindow
/// [window classes]: crate::WindowClass
///
/// |Attribute                |Default value     |Classes                      |
/// |-------------------------|------------------|-----------------------------|
/// |[`background_pixmap`]    |[`None`]          |[`InputOutput`] only         |
/// |[`border_pixmap`]        |[`CopyFromParent`]|[`InputOutput`] only         |
/// |[`bit_gravity`]          |[`Forget`]        |[`InputOutput`] only         |
/// |[`window_gravity`]       |[`NorthWest`] |[`InputOutput`] and [`InputOnly`]|
/// |[`backing_store`]        |[`NotUseful`]     |[`InputOutput`] only         |
/// |[`backing_planes`]       |`0x_ffff_ffff`    |[`InputOutput`] only         |
/// |[`backing_pixel`]        |`0x_0000_0000`    |[`InputOutput`] only         |
/// |[`save_under`]           |`false`           |[`InputOutput`] only         |
/// |[`event_mask`]           |[`empty()`][e]|[`InputOutput`] and [`InputOnly`]|
/// |[`do_not_propagate_mask`]|[`empty()`][d]|[`InputOutput`] and [`InputOnly`]|
/// |[`override_redirect`]    |`false`       |[`InputOutput`] and [`InputOnly`]|
/// |[`colormap`]             |[`CopyFromParent`]|[`InputOutput`] only         |
/// |[`cursor_appearance`]    |[`None`]      |[`InputOutput`] and [`InputOnly`]|
///
/// [`background_pixmap`]: Attributes::background_pixmap
/// [`border_pixmap`]: Attributes::border_pixmap
/// [`bit_gravity`]: Attributes::bit_gravity
/// [`window_gravity`]: Attributes::window_gravity
/// [`backing_store`]: Attributes::backing_store
/// [`backing_planes`]: Attributes::backing_planes
/// [`backing_pixel`]: Attributes::backing_pixel
/// [`save_under`]: Attributes::save_under
/// [`event_mask`]: Attributes::event_mask
/// [`do_not_propagate_mask`]: Attributes::do_not_propagate_mask
/// [`override_redirect`]: Attributes::override_redirect
/// [`colormap`]: Attributes::colormap
/// [`cursor_appearance`]: Attributes::cursor_appearance
///
/// [`CopyFromParent`]: CopyableFromParent::CopyFromParent
/// [`Forget`]: BitGravity::Forget
/// [`NorthWest`]: WindowGravity::NorthWest
/// [`NotUseful`]: BackingStore::NotUseful
/// [e]: EventMask::empty
/// [d]: DeviceEventMask::empty
///
/// [`InputOutput`]: crate::WindowClass::InputOutput
/// [`InputOnly`]: crate::WindowClass::InputOnly
///
/// [window]: Window
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Attributes {
	/// Total [`X11Size`] of these `Attributes`.
	///
	/// This is cached so that it doesn't have to be recalculated each time -
	/// `Attributes` is immutable.
	///
	/// This field is not part of the X11 format for this struct.
	x11_size: usize,

	mask: AttributeMask,

	background_pixmap: Option<BackgroundPixmap>,
	background_pixel: Option<Pixel>,

	border_pixmap: Option<BorderPixmap>,
	border_pixel: Option<Pixel>,

	bit_gravity: Option<__BitGravity>,
	window_gravity: Option<__WindowGravity>,

	backing_store: Option<BackingStore>,
	backing_planes: Option<u32>,
	backing_pixel: Option<Pixel>,

	override_redirect: Option<__bool>,
	save_under: Option<__bool>,

	event_mask: Option<EventMask>,
	do_not_propagate_mask: Option<DeviceEventMask>,

	colormap: Option<ColormapAttribute>,

	#[allow(clippy::option_option)]
	cursor_appearance: Option<CursorAppearanceAttribute>,
}

impl Attributes {
	#[must_use]
	// FIXME: ask in Rust community about alternatives to using so many arguments -
	//        should this be an `AttributesBuilder` of some sort?
	pub fn new(
		background_pixmap: Option<BackgroundPixmap>, background_pixel: Option<Pixel>,
		border_pixmap: Option<BorderPixmap>, border_pixel: Option<Pixel>,
		bit_gravity: Option<BitGravity>, window_gravity: Option<WindowGravity>,
		backing_store: Option<BackingStore>, backing_planes: Option<u32>,
		backing_pixel: Option<Pixel>, override_redirect: Option<bool>, save_under: Option<bool>,
		event_mask: Option<EventMask>, do_not_propagate_mask: Option<DeviceEventMask>,
		colormap: Option<ColormapAttribute>, cursor_appearance: Option<CursorAppearanceAttribute>,
	) -> Self {
		let mut x11_size = 0;
		let mut mask = AttributeMask::empty();

		if let Some(background_pixmap) = &background_pixmap {
			x11_size += background_pixmap.x11_size();
			mask |= AttributeMask::BACKGROUND_PIXMAP;
		}
		if let Some(background_pixel) = &background_pixel {
			x11_size += background_pixel.x11_size();
			mask |= AttributeMask::BACKGROUND_PIXEL;
		}

		if let Some(border_pixmap) = &border_pixmap {
			x11_size += border_pixmap.x11_size();
			mask |= AttributeMask::BORDER_PIXMAP;
		}
		if let Some(border_pixel) = &border_pixel {
			x11_size += border_pixel.x11_size();
			mask |= AttributeMask::BORDER_PIXEL;
		}

		if bit_gravity.is_some() {
			x11_size += __BitGravity::X11_SIZE;
			mask |= AttributeMask::BIT_GRAVITY;
		}
		if window_gravity.is_some() {
			x11_size += __WindowGravity::X11_SIZE;
			mask |= AttributeMask::WINDOW_GRAVITY;
		}

		if let Some(backing_store) = &backing_store {
			x11_size += backing_store.x11_size();
			mask |= AttributeMask::BACKING_STORE;
		}
		if let Some(backing_planes) = &backing_planes {
			x11_size += backing_planes.x11_size();
			mask |= AttributeMask::BACKING_PLANES;
		}
		if let Some(backing_pixel) = &backing_pixel {
			x11_size += backing_pixel.x11_size();
			mask |= AttributeMask::BACKING_PIXEL;
		}

		if override_redirect.is_some() {
			x11_size += __bool::X11_SIZE;
			mask |= AttributeMask::OVERRIDE_REDIRECT;
		}
		if save_under.is_some() {
			x11_size += __bool::X11_SIZE;
			mask |= AttributeMask::SAVE_UNDER;
		}

		if let Some(event_mask) = &event_mask {
			x11_size += event_mask.x11_size();
			mask |= AttributeMask::EVENT_MASK;
		}
		if let Some(do_not_propagate_mask) = &do_not_propagate_mask {
			x11_size += do_not_propagate_mask.x11_size();
			mask |= AttributeMask::DO_NOT_PROPAGATE_MASK;
		}

		if let Some(colormap) = &colormap {
			x11_size += colormap.x11_size();
			mask |= AttributeMask::COLORMAP;
		}

		if let Some(cursor_appearance) = &cursor_appearance {
			x11_size += cursor_appearance.x11_size();
			mask |= AttributeMask::CURSOR_APPEARANCE;
		}

		Self {
			x11_size,
			mask,

			background_pixmap,
			background_pixel,

			border_pixmap,
			border_pixel,

			/// These gravities are converted into our [`__BitGravity`] and
			/// [`__WindowGravity`] types respectively so that they can be
			/// easily written as four bytes.
			bit_gravity: bit_gravity.map(__BitGravity),
			window_gravity: window_gravity.map(__WindowGravity),

			backing_store,
			backing_planes,
			backing_pixel,

			// These booleans are converted into our [`__bool`] type so that they can easily be
			// written as four bytes.
			override_redirect: override_redirect.map(__bool),
			save_under: save_under.map(__bool),

			event_mask,
			do_not_propagate_mask,

			colormap,

			cursor_appearance,
		}
	}

	#[must_use]
	pub const fn background_pixmap(&self) -> &Option<BackgroundPixmap> {
		&self.background_pixmap
	}
	#[must_use]
	pub const fn background_pixel(&self) -> &Option<Pixel> {
		&self.background_pixel
	}

	#[must_use]
	pub const fn border_pixmap(&self) -> &Option<BorderPixmap> {
		&self.border_pixmap
	}
	#[must_use]
	pub const fn border_pixel(&self) -> &Option<Pixel> {
		&self.border_pixel
	}

	#[must_use]
	pub fn bit_gravity(&self) -> Option<&BitGravity> {
		self.bit_gravity
			.as_ref()
			.map(|__BitGravity(gravity)| gravity)
	}
	#[must_use]
	pub fn window_gravity(&self) -> Option<&WindowGravity> {
		self.window_gravity
			.as_ref()
			.map(|__WindowGravity(gravity)| gravity)
	}

	#[must_use]
	pub const fn backing_store(&self) -> &Option<BackingStore> {
		&self.backing_store
	}
	#[must_use]
	pub const fn backing_planes(&self) -> &Option<u32> {
		&self.backing_planes
	}
	#[must_use]
	pub const fn backing_pixel(&self) -> &Option<Pixel> {
		&self.backing_pixel
	}

	#[must_use]
	pub fn override_redirect(&self) -> Option<&bool> {
		self.override_redirect.as_ref().map(|__bool(bool)| bool)
	}
	#[must_use]
	pub fn save_under(&self) -> Option<&bool> {
		self.save_under.as_ref().map(|__bool(bool)| bool)
	}

	#[must_use]
	pub const fn event_mask(&self) -> &Option<EventMask> {
		&self.event_mask
	}
	#[must_use]
	pub const fn do_not_propagate_mask(&self) -> &Option<DeviceEventMask> {
		&self.do_not_propagate_mask
	}

	#[must_use]
	pub const fn colormap(&self) -> &Option<ColormapAttribute> {
		&self.colormap
	}

	#[must_use]
	pub const fn cursor_appearance(&self) -> &Option<CursorAppearanceAttribute> {
		&self.cursor_appearance
	}
}

impl X11Size for Attributes {
	fn x11_size(&self) -> usize {
		self.x11_size
	}
}

impl Readable for Attributes {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		let mut x11_size = 0;
		let mask = AttributeMask::read_from(buf)?;

		let background_pixmap = util::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributeMask::BACKGROUND_PIXMAP),
		)?;
		let background_pixel = util::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributeMask::BACKGROUND_PIXEL),
		)?;

		let border_pixmap = util::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributeMask::BORDER_PIXMAP),
		)?;
		let border_pixel = util::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributeMask::BORDER_PIXEL),
		)?;

		let bit_gravity = util::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributeMask::BIT_GRAVITY),
		)?;
		let window_gravity = util::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributeMask::WINDOW_GRAVITY),
		)?;

		let backing_store = util::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributeMask::BACKING_STORE),
		)?;
		let backing_planes = util::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributeMask::BACKING_PLANES),
		)?;
		let backing_pixel = util::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributeMask::BACKING_PIXEL),
		)?;

		let override_redirect = util::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributeMask::OVERRIDE_REDIRECT),
		)?;
		let save_under =
			util::read_set_value(buf, &mut x11_size, mask.contains(AttributeMask::SAVE_UNDER))?;

		let event_mask =
			util::read_set_value(buf, &mut x11_size, mask.contains(AttributeMask::EVENT_MASK))?;
		let do_not_propagate_mask = util::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributeMask::DO_NOT_PROPAGATE_MASK),
		)?;

		let colormap =
			util::read_set_value(buf, &mut x11_size, mask.contains(AttributeMask::COLORMAP))?;

		let cursor_appearance = util::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributeMask::CURSOR_APPEARANCE),
		)?;

		Ok(Self {
			x11_size,
			mask,

			background_pixmap,
			background_pixel,

			border_pixmap,
			border_pixel,

			bit_gravity,
			window_gravity,

			backing_store,
			backing_planes,
			backing_pixel,

			override_redirect,
			save_under,

			event_mask,
			do_not_propagate_mask,

			colormap,

			cursor_appearance,
		})
	}
}

impl Writable for Attributes {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		self.mask.write_to(buf)?;

		if let Some(background_pixmap) = &self.background_pixmap {
			background_pixmap.write_to(buf)?;
		}
		if let Some(background_pixel) = &self.background_pixel {
			background_pixel.write_to(buf)?;
		}

		if let Some(border_pixmap) = &self.border_pixmap {
			border_pixmap.write_to(buf)?;
		}
		if let Some(border_pixel) = &self.border_pixel {
			border_pixel.write_to(buf)?;
		}

		if let Some(bit_gravity) = &self.bit_gravity {
			bit_gravity.write_to(buf)?;
		}
		if let Some(window_gravity) = &self.window_gravity {
			window_gravity.write_to(buf)?;
		}

		if let Some(backing_store) = &self.backing_store {
			backing_store.write_to(buf)?;
		}
		if let Some(backing_planes) = &self.backing_planes {
			backing_planes.write_to(buf)?;
		}
		if let Some(backing_pixel) = &self.backing_pixel {
			backing_pixel.write_to(buf)?;
		}

		if let Some(override_redirect) = &self.override_redirect {
			override_redirect.write_to(buf)?;
		}
		if let Some(save_under) = &self.save_under {
			save_under.write_to(buf)?;
		}

		if let Some(event_mask) = &self.event_mask {
			event_mask.write_to(buf)?;
		}
		if let Some(do_not_propagate_mask) = &self.do_not_propagate_mask {
			do_not_propagate_mask.write_to(buf)?;
		}

		if let Some(colormap) = &self.colormap {
			colormap.write_to(buf)?;
		}

		if let Some(cursor_appearance) = &self.cursor_appearance {
			cursor_appearance.write_to(buf)?;
		}

		Ok(())
	}
}

// Internal 4-byte representations of types {{{

/// Wraps a `bool` but writes it as four bytes in the X11 format.
///
/// This is not part of the public API.
#[allow(
	non_camel_case_types,
	reason = "This is an internal representation of a `bool`. Its naming scheme is `__` to \
	          indicate that it is internal, and `bool` to indicate its wrapped type."
)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __bool(bool);

impl ConstantX11Size for __bool {
	const X11_SIZE: usize = 4;
}

impl X11Size for __bool {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __bool {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(buf.get_u32() != 0))
	}
}

impl Writable for __bool {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(bool) = self;

		if *bool {
			buf.put_u32(1);
		} else {
			buf.put_u32(0);
		}

		Ok(())
	}
}

/// A type wrapping a [`BitGravity`] to represent [bit gravities] in
/// [`Attributes`] as four bytes.
///
/// [bit gravities]: BitGravity
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __BitGravity(BitGravity);

impl ConstantX11Size for __BitGravity {
	const X11_SIZE: usize = 4;
}

impl X11Size for __BitGravity {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __BitGravity {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self> {
		Ok(Self(match buf.get_u32() {
			discrim if discrim == 0 => BitGravity::Forget,
			discrim if discrim == 1 => BitGravity::Static,
			discrim if discrim == 2 => BitGravity::NorthWest,
			discrim if discrim == 3 => BitGravity::North,
			discrim if discrim == 4 => BitGravity::NorthEast,
			discrim if discrim == 5 => BitGravity::West,
			discrim if discrim == 6 => BitGravity::Center,
			discrim if discrim == 7 => BitGravity::East,
			discrim if discrim == 8 => BitGravity::SouthWest,
			discrim if discrim == 9 => BitGravity::South,
			discrim if discrim == 10 => BitGravity::SouthEast,

			other_discrim => {
				return Err(ReadError::UnrecognizedDiscriminant(other_discrim as usize))
			},
		}))
	}
}

impl Writable for __BitGravity {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(bit_gravity) = self;

		match bit_gravity {
			BitGravity::Forget => buf.put_u32(0),
			BitGravity::Static => buf.put_u32(1),
			BitGravity::NorthWest => buf.put_u32(2),
			BitGravity::North => buf.put_u32(3),
			BitGravity::NorthEast => buf.put_u32(4),
			BitGravity::West => buf.put_u32(5),
			BitGravity::Center => buf.put_u32(6),
			BitGravity::East => buf.put_u32(7),
			BitGravity::SouthWest => buf.put_u32(8),
			BitGravity::South => buf.put_u32(9),
			BitGravity::SouthEast => buf.put_u32(10),
		}

		Ok(())
	}
}

/// A type wrapping a [`WindowGravity`] to represent [window gravities] in
/// [`Attributes`] as four bytes.
///
/// [window gravities]: WindowGravity
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __WindowGravity(WindowGravity);

impl ConstantX11Size for __WindowGravity {
	const X11_SIZE: usize = 4;
}

impl X11Size for __WindowGravity {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __WindowGravity {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self> {
		Ok(Self(match buf.get_u32() {
			discrim if discrim == 0 => WindowGravity::Unmap,
			discrim if discrim == 1 => WindowGravity::Static,
			discrim if discrim == 2 => WindowGravity::NorthWest,
			discrim if discrim == 3 => WindowGravity::North,
			discrim if discrim == 4 => WindowGravity::NorthEast,
			discrim if discrim == 5 => WindowGravity::West,
			discrim if discrim == 6 => WindowGravity::Center,
			discrim if discrim == 7 => WindowGravity::East,
			discrim if discrim == 8 => WindowGravity::SouthWest,
			discrim if discrim == 9 => WindowGravity::South,
			discrim if discrim == 10 => WindowGravity::SouthEast,

			other_discrim => {
				return Err(ReadError::UnrecognizedDiscriminant(other_discrim as usize))
			},
		}))
	}
}

impl Writable for __WindowGravity {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(window_gravity) = self;

		match window_gravity {
			WindowGravity::Unmap => buf.put_u32(0),
			WindowGravity::Static => buf.put_u32(1),
			WindowGravity::NorthWest => buf.put_u32(2),
			WindowGravity::North => buf.put_u32(3),
			WindowGravity::NorthEast => buf.put_u32(4),
			WindowGravity::West => buf.put_u32(5),
			WindowGravity::Center => buf.put_u32(6),
			WindowGravity::East => buf.put_u32(7),
			WindowGravity::SouthWest => buf.put_u32(8),
			WindowGravity::South => buf.put_u32(9),
			WindowGravity::SouthEast => buf.put_u32(10),
		}

		Ok(())
	}
}

// }}}
