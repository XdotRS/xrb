// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{
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
	WinGravity,
};
use xrbk::{Buf, BufMut, ReadResult, Readable, Writable, WriteResult, X11Size};

use bitflags::bitflags;
use xrbk_macro::{ConstantX11Size, Readable, Writable, X11Size};

bitflags! {
	/// A mask of [attributes] given for a [window].
	///
	/// The following table shows each attribute, its default value if it is
	/// not explicitly initialized in the [`CreateWindow` request], and the
	/// [window classes] that it can be set with.
	///
	/// [attributes]: Attributes
	/// [window]: Window
	/// [`CreateWindow` request]: crate::x11::request::CreateWindow
	/// [window classes]: crate::WindowClass
	///
	/// |Attribute                |Default value            |Classes                          |
	/// |-------------------------|-------------------------|---------------------------------|
	/// |[`background_pixmap`]    |[`None`]                 |[`InputOutput`] only             |
	/// |[`border_pixmap`]        |[`CopyFromParent`]       |[`InputOutput`] only             |
	/// |[`bit_gravity`]          |[`Forget`]               |[`InputOutput`] only             |
	/// |[`win_gravity`]          |[`NorthWest`]            |[`InputOutput`] and [`InputOnly`]|
	/// |[`backing_store`]        |[`NotUseful`]            |[`InputOutput`] only             |
	/// |[`backing_planes`]       |`0x_ffff_ffff`           |[`InputOutput`] only             |
	/// |[`backing_pixel`]        |`0x_0000_0000`           |[`InputOutput`] only             |
	/// |[`save_under`]           |`false`                  |[`InputOutput`] only             |
	/// |[`event_mask`]           |[`empty()`][event empty] |[`InputOutput`] and [`InputOnly`]|
	/// |[`do_not_propagate_mask`]|[`empty()`][device empty]|[`InputOutput`] and [`InputOnly`]|
	/// |[`override_redirect`]    |`false`                  |[`InputOutput`] and [`InputOnly`]|
	/// |[`colormap`]             |[`CopyFromParent`]       |[`InputOutput`] only             |
	/// |[`cursor_appearance`]    |[`None`]                 |[`InputOutput`] and [`InputOnly`]|
	///
	/// [`background_pixmap`]: Attributes::background_pixmap
	/// [`border_pixmap`]: Attributes::border_pixmap
	/// [`bit_gravity`]: Attributes::bit_gravity
	/// [`win_gravity`]: Attributes::win_gravity
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
	/// [`NorthWest`]: WinGravity::NorthWest
	/// [`NotUseful`]: BackingStore::NotUseful
	/// [event none]: EventMask::empty
	/// [device none]: DeviceEventMask::empty
	///
	/// [`InputOutput`]: crate::WindowClass::InputOutput
	/// [`InputOnly`]: crate::WindowClass::InputOnly
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
		/// See also: [`win_gravity`], [`WinGravity`].
		///
		/// [`win_gravity`]: Attributes::win_gravity
		const WIN_GRAVITY = 0x0000_0020;

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

/// A boolean type wrapping a `u32` value for use in [`Attributes`].
///
/// This is used to easily encode booleans as four bytes. It is not part of the
/// public API.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, ConstantX11Size, X11Size, Readable, Writable)]
struct Bool(u32);

impl From<bool> for Bool {
	fn from(value: bool) -> Self {
		if value {
			Self(1)
		} else {
			Self(0)
		}
	}
}

impl From<Bool> for bool {
	fn from(value: Bool) -> Self {
		value.0 != 0
	}
}

/// A set of attributes for a [window].
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

	background_pixmap: Option<ParentRelatable<Option<Pixmap>>>,
	background_pixel: Option<Pixel>,

	border_pixmap: Option<CopyableFromParent<Pixmap>>,
	border_pixel: Option<Pixel>,

	bit_gravity: Option<BitGravity>,
	win_gravity: Option<WinGravity>,

	backing_store: Option<BackingStore>,
	backing_planes: Option<u32>,
	backing_pixel: Option<Pixel>,

	override_redirect: Option<Bool>,
	save_under: Option<Bool>,

	event_mask: Option<EventMask>,
	do_not_propagate_mask: Option<DeviceEventMask>,

	colormap: Option<CopyableFromParent<Colormap>>,

	#[allow(clippy::option_option)]
	cursor_appearance: Option<Option<CursorAppearance>>,
}

impl Attributes {
	#[must_use]
	#[allow(clippy::too_many_arguments)]
	pub fn new(
		background_pixmap: Option<ParentRelatable<Option<Pixmap>>>,
		background_pixel: Option<Pixel>, border_pixmap: Option<CopyableFromParent<Pixmap>>,
		border_pixel: Option<Pixel>, bit_gravity: Option<BitGravity>,
		win_gravity: Option<WinGravity>, backing_store: Option<BackingStore>,
		backing_planes: Option<u32>, backing_pixel: Option<Pixel>, override_redirect: Option<bool>,
		save_under: Option<bool>, event_mask: Option<EventMask>,
		do_not_propagate_mask: Option<DeviceEventMask>,
		colormap: Option<CopyableFromParent<Colormap>>,
		cursor_appearance: Option<Option<CursorAppearance>>,
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

		if let Some(bit_gravity) = &bit_gravity {
			x11_size += bit_gravity.x11_size();
			mask |= AttributeMask::BIT_GRAVITY;
		}
		if let Some(win_gravity) = &win_gravity {
			x11_size += win_gravity.x11_size();
			mask |= AttributeMask::WIN_GRAVITY;
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

		if let Some(override_redirect) = &override_redirect {
			x11_size += override_redirect.x11_size();
			mask |= AttributeMask::OVERRIDE_REDIRECT;
		}
		if let Some(save_under) = &save_under {
			x11_size += save_under.x11_size();
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

			bit_gravity,
			win_gravity,

			backing_store,
			backing_planes,
			backing_pixel,

			// These booleans are converted into our [`Bool`] type so that they can easily be
			// written as four bytes.
			override_redirect: override_redirect.map(std::convert::Into::into),
			save_under: save_under.map(Into::into),

			event_mask,
			do_not_propagate_mask,

			colormap,

			cursor_appearance,
		}
	}

	#[must_use]
	pub const fn background_pixmap(&self) -> &Option<ParentRelatable<Option<Pixmap>>> {
		&self.background_pixmap
	}
	#[must_use]
	pub const fn background_pixel(&self) -> &Option<Pixel> {
		&self.background_pixel
	}

	#[must_use]
	pub const fn border_pixmap(&self) -> &Option<CopyableFromParent<Pixmap>> {
		&self.border_pixmap
	}
	#[must_use]
	pub const fn border_pixel(&self) -> &Option<Pixel> {
		&self.border_pixel
	}

	#[must_use]
	pub const fn bit_gravity(&self) -> &Option<BitGravity> {
		&self.bit_gravity
	}
	#[must_use]
	pub const fn win_gravity(&self) -> &Option<WinGravity> {
		&self.win_gravity
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
	pub fn override_redirect(&self) -> Option<bool> {
		self.override_redirect.map(Into::into)
	}
	#[must_use]
	pub fn save_under(&self) -> Option<bool> {
		self.save_under.map(Into::into)
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
	pub const fn colormap(&self) -> &Option<CopyableFromParent<Colormap>> {
		&self.colormap
	}

	#[must_use]
	pub const fn cursor_appearance(&self) -> &Option<Option<CursorAppearance>> {
		&self.cursor_appearance
	}
}

impl X11Size for Attributes {
	fn x11_size(&self) -> usize {
		self.x11_size
	}
}

impl Readable for Attributes {
	#[allow(clippy::too_many_lines)]
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		let mut x11_size = 0;
		let mask = AttributeMask::read_from(buf)?;

		let background_pixmap = if mask.contains(AttributeMask::BACKGROUND_PIXMAP) {
			let ret = <ParentRelatable<Option<Pixmap>>>::read_from(buf)?;
			x11_size += ret.x11_size();

			Some(ret)
		} else {
			None
		};
		let background_pixel = if mask.contains(AttributeMask::BACKGROUND_PIXEL) {
			let ret = Pixel::read_from(buf)?;
			x11_size += ret.x11_size();

			Some(ret)
		} else {
			None
		};

		let border_pixmap = if mask.contains(AttributeMask::BORDER_PIXMAP) {
			let ret = <CopyableFromParent<Pixmap>>::read_from(buf)?;
			x11_size += ret.x11_size();

			Some(ret)
		} else {
			None
		};
		let border_pixel = if mask.contains(AttributeMask::BORDER_PIXEL) {
			let ret = Pixel::read_from(buf)?;
			x11_size += ret.x11_size();

			Some(ret)
		} else {
			None
		};

		let bit_gravity = if mask.contains(AttributeMask::BIT_GRAVITY) {
			let ret = BitGravity::read_from(buf)?;
			x11_size += ret.x11_size();

			Some(ret)
		} else {
			None
		};
		let win_gravity = if mask.contains(AttributeMask::WIN_GRAVITY) {
			let ret = WinGravity::read_from(buf)?;
			x11_size += ret.x11_size();

			Some(ret)
		} else {
			None
		};

		let backing_store = if mask.contains(AttributeMask::BACKING_STORE) {
			let ret = BackingStore::read_from(buf)?;
			x11_size += ret.x11_size();

			Some(ret)
		} else {
			None
		};
		let backing_planes = if mask.contains(AttributeMask::BACKING_PLANES) {
			let ret = u32::read_from(buf)?;
			x11_size += ret.x11_size();

			Some(ret)
		} else {
			None
		};
		let backing_pixel = if mask.contains(AttributeMask::BACKING_PIXEL) {
			let ret = Pixel::read_from(buf)?;
			x11_size += ret.x11_size();

			Some(ret)
		} else {
			None
		};

		let override_redirect = if mask.contains(AttributeMask::OVERRIDE_REDIRECT) {
			let ret = Bool::read_from(buf)?;
			x11_size += ret.x11_size();

			Some(ret)
		} else {
			None
		};
		let save_under = if mask.contains(AttributeMask::SAVE_UNDER) {
			let ret = Bool::read_from(buf)?;
			x11_size += ret.x11_size();

			Some(ret)
		} else {
			None
		};

		let event_mask = if mask.contains(AttributeMask::EVENT_MASK) {
			let ret = EventMask::read_from(buf)?;
			x11_size += ret.x11_size();

			Some(ret)
		} else {
			None
		};
		let do_not_propagate_mask = if mask.contains(AttributeMask::DO_NOT_PROPAGATE_MASK) {
			let ret = DeviceEventMask::read_from(buf)?;
			x11_size += ret.x11_size();

			Some(ret)
		} else {
			None
		};

		let colormap = if mask.contains(AttributeMask::COLORMAP) {
			let ret = <CopyableFromParent<Colormap>>::read_from(buf)?;
			x11_size += ret.x11_size();

			Some(ret)
		} else {
			None
		};

		let cursor_appearance = if mask.contains(AttributeMask::CURSOR_APPEARANCE) {
			let ret = <Option<CursorAppearance>>::read_from(buf)?;
			x11_size += ret.x11_size();

			Some(ret)
		} else {
			None
		};

		Ok(Self {
			x11_size,
			mask,

			background_pixmap,
			background_pixel,

			border_pixmap,
			border_pixel,

			bit_gravity,
			win_gravity,

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
		if let Some(win_gravity) = &self.win_gravity {
			win_gravity.write_to(buf)?;
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
