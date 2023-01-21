// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::__bool;
use crate::{
	visual::Pixel,
	BackingStore,
	BitGravity,
	Colormap,
	CopyableFromParent,
	CursorAppearance,
	DeviceEventMask,
	EventMask,
	ParentRelatable,
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
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Attributes {
	/// Total [`X11Size`] of these `Attributes`.
	///
	/// This is cached so that it doesn't have to be recalculated each time -
	/// `Attributes` is immutable.
	///
	/// This field is not part of the X11 format for this struct.
	x11_size: usize,

	mask: AttributesMask,

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
	pub const fn builder() -> AttributesBuilder {
		AttributesBuilder::new()
	}
}

#[derive(Clone, Default, Debug, Hash, PartialEq, Eq)]
pub struct AttributesBuilder {
	x11_size: usize,

	mask: AttributesMask,

	background_pixmap: Option<BackgroundPixmap>,
	background_pixel: Option<Pixel>,

	border_pixmap: Option<BorderPixmap>,
	border_pixel: Option<Pixel>,

	bit_gravity: Option<BitGravity>,
	window_gravity: Option<WindowGravity>,

	backing_store: Option<BackingStore>,
	backing_planes: Option<u32>,
	backing_pixel: Option<Pixel>,

	override_redirect: Option<bool>,
	save_under: Option<bool>,

	event_mask: Option<EventMask>,
	do_not_propagate_mask: Option<DeviceEventMask>,

	colormap: Option<ColormapAttribute>,

	cursor_appearance: Option<CursorAppearanceAttribute>,
}

impl AttributesBuilder {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			x11_size: AttributesMask::X11_SIZE,

			mask: AttributesMask::empty(),

			background_pixmap: None,
			background_pixel: None,

			border_pixmap: None,
			border_pixel: None,

			bit_gravity: None,
			window_gravity: None,

			backing_store: None,
			backing_planes: None,
			backing_pixel: None,

			override_redirect: None,
			save_under: None,

			event_mask: None,
			do_not_propagate_mask: None,

			colormap: None,

			cursor_appearance: None,
		}
	}

	#[must_use]
	pub fn build(self) -> Attributes {
		Attributes {
			x11_size: self.x11_size,

			mask: self.mask,

			background_pixmap: self.background_pixmap,
			background_pixel: self.background_pixel,

			border_pixmap: self.border_pixmap,
			border_pixel: self.border_pixel,

			bit_gravity: self.bit_gravity.map(__BitGravity),
			window_gravity: self.window_gravity.map(__WindowGravity),

			backing_store: self.backing_store,
			backing_planes: self.backing_planes,
			backing_pixel: self.backing_pixel,

			override_redirect: self.override_redirect.map(__bool),
			save_under: self.save_under.map(__bool),

			event_mask: self.event_mask,
			do_not_propagate_mask: self.do_not_propagate_mask,

			colormap: self.colormap,

			cursor_appearance: self.cursor_appearance,
		}
	}
}

impl AttributesBuilder {
	pub fn background_pixmap(&mut self, background_pixmap: BackgroundPixmap) -> &mut Self {
		if self.background_pixmap.is_none() {
			self.x11_size += 4;
		}

		self.background_pixmap = Some(background_pixmap);
		self.mask |= AttributesMask::BACKGROUND_PIXMAP;

		self
	}
	pub fn background_pixel(&mut self, background_pixel: Pixel) -> &mut Self {
		if self.background_pixel.is_none() {
			self.x11_size += 4;
		}

		self.background_pixel = Some(background_pixel);
		self.mask |= AttributesMask::BACKGROUND_PIXEL;

		self
	}

	pub fn border_pixmap(&mut self, border_pixmap: BorderPixmap) -> &mut Self {
		if self.border_pixmap.is_none() {
			self.x11_size += 4;
		}

		self.border_pixmap = Some(border_pixmap);
		self.mask |= AttributesMask::BORDER_PIXMAP;

		self
	}
	pub fn border_pixel(&mut self, border_pixel: Pixel) -> &mut Self {
		if self.border_pixel.is_none() {
			self.x11_size += 4;
		}

		self.border_pixel = Some(border_pixel);
		self.mask |= AttributesMask::BORDER_PIXEL;

		self
	}

	pub fn bit_gravity(&mut self, bit_gravity: BitGravity) -> &mut Self {
		if self.bit_gravity.is_none() {
			self.x11_size += 4;
		}

		self.bit_gravity = Some(bit_gravity);
		self.mask |= AttributesMask::BIT_GRAVITY;

		self
	}
	pub fn window_gravity(&mut self, window_gravity: WindowGravity) -> &mut Self {
		if self.window_gravity.is_none() {
			self.x11_size += 4;
		}

		self.window_gravity = Some(window_gravity);
		self.mask |= AttributesMask::WINDOW_GRAVITY;

		self
	}

	pub fn backing_store(&mut self, backing_store: BackingStore) -> &mut Self {
		if self.backing_store.is_none() {
			self.x11_size += 4;
		}

		self.backing_store = Some(backing_store);
		self.mask |= AttributesMask::BACKING_STORE;

		self
	}
	pub fn backing_planes(&mut self, backing_planes: u32) -> &mut Self {
		if self.backing_planes.is_none() {
			self.x11_size += 4;
		}

		self.backing_planes = Some(backing_planes);
		self.mask |= AttributesMask::BACKING_PLANES;

		self
	}
	pub fn backing_pixel(&mut self, backing_pixel: Pixel) -> &mut Self {
		if self.backing_pixel.is_none() {
			self.x11_size += 4;
		}

		self.backing_pixel = Some(backing_pixel);
		self.mask |= AttributesMask::BACKING_PIXEL;

		self
	}

	pub fn override_redirect(&mut self, override_redirect: bool) -> &mut Self {
		if self.override_redirect.is_none() {
			self.x11_size += 4;
		}

		self.override_redirect = Some(override_redirect);
		self.mask |= AttributesMask::OVERRIDE_REDIRECT;

		self
	}
	pub fn save_under(&mut self, save_under: bool) -> &mut Self {
		if self.save_under.is_none() {
			self.x11_size += 4;
		}

		self.save_under = Some(save_under);
		self.mask |= AttributesMask::SAVE_UNDER;

		self
	}

	pub fn event_mask(&mut self, event_mask: EventMask) -> &mut Self {
		if self.event_mask.is_none() {
			self.x11_size += 4;
		}

		self.event_mask = Some(event_mask);
		self.mask |= AttributesMask::EVENT_MASK;

		self
	}
	pub fn do_not_propagate_mask(&mut self, do_not_propagate_mask: DeviceEventMask) -> &mut Self {
		if self.do_not_propagate_mask.is_none() {
			self.x11_size += 4;
		}

		self.do_not_propagate_mask = Some(do_not_propagate_mask);
		self.mask |= AttributesMask::DO_NOT_PROPAGATE_MASK;

		self
	}

	pub fn colormap(&mut self, colormap: ColormapAttribute) -> &mut Self {
		if self.colormap.is_none() {
			self.x11_size += 4;
		}

		self.colormap = Some(colormap);
		self.mask |= AttributesMask::COLORMAP;

		self
	}

	pub fn cursor_appearance(&mut self, cursor_appearance: CursorAppearanceAttribute) -> &mut Self {
		if self.cursor_appearance.is_none() {
			self.x11_size += 4;
		}

		self.cursor_appearance = Some(cursor_appearance);
		self.mask |= AttributesMask::CURSOR_APPEARANCE;

		self
	}
}

impl Attributes {
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

bitflags! {
	/// A mask of [attributes] given for a [window].
	///
	/// For more information, and for the attributes themselves, please see [`Attributes`].
	///
	/// [attributes]: Attributes
	/// [window]: crate::Window
	#[derive(Default, X11Size, Readable, ConstantX11Size, Writable)]
	pub struct AttributesMask: u32 {
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
		let mask = AttributesMask::read_from(buf)?;
		let mut x11_size = mask.x11_size();

		let background_pixmap = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::BACKGROUND_PIXMAP),
		)?;
		let background_pixel = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::BACKGROUND_PIXEL),
		)?;

		let border_pixmap = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::BORDER_PIXMAP),
		)?;
		let border_pixel = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::BORDER_PIXEL),
		)?;

		let bit_gravity = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::BIT_GRAVITY),
		)?;
		let window_gravity = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::WINDOW_GRAVITY),
		)?;

		let backing_store = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::BACKING_STORE),
		)?;
		let backing_planes = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::BACKING_PLANES),
		)?;
		let backing_pixel = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::BACKING_PIXEL),
		)?;

		let override_redirect = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::OVERRIDE_REDIRECT),
		)?;
		let save_under = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::SAVE_UNDER),
		)?;

		let event_mask = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::EVENT_MASK),
		)?;
		let do_not_propagate_mask = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::DO_NOT_PROPAGATE_MASK),
		)?;

		let colormap =
			super::read_set_value(buf, &mut x11_size, mask.contains(AttributesMask::COLORMAP))?;

		let cursor_appearance = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::CURSOR_APPEARANCE),
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
