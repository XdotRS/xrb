// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::__bool;
use crate::{
	visual::ColorId,
	BitGravity,
	Colormap,
	CopyableFromParent,
	CursorAppearance,
	DeviceEventMask,
	EventMask,
	MaintainContents,
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
/// [window]: crate::Window
/// [`CreateWindow` request]: crate::x11::request::CreateWindow
/// [window classes]: crate::WindowClass
///
/// |Attribute                 |Default value      |Classes                    |
/// |--------------------------|-------------------|---------------------------|
/// |[`background_pixmap`]     |[`None`]           |[`InputOutput`] only       |
/// |[`background_color`]      |_N/A_              |[`InputOutput`] only       |
/// |[`border_pixmap`]         |[`CopyFromParent`] |[`InputOutput`] only       |
/// |[`border_color`]          |_N/A_              |[`InputOutput`] only       |
/// |[`bit_gravity`]           |[`Forget`]         |[`InputOutput`] only       |
/// |[`window_gravity`]        |[`NorthWest`]|[`InputOutput`] and [`InputOnly`]|
/// |[`maintain_contents`]     |[`NotUseful`]      |[`InputOutput`] only       |
/// |[`maintained_planes`]     |`0x_ffff_ffff`     |[`InputOutput`] only       |
/// |[`maintenance_fallback_color`]|[`ColorId::ZERO`]|[`InputOutput`] only     |
/// |[`override_redirect`]     |`false`      |[`InputOutput`] and [`InputOnly`]|
/// |[`maintain_windows_under`]|`false`            |[`InputOutput`] only       |
/// |[`event_mask`]           |[`empty()`][e]|[`InputOutput`] and [`InputOnly`]|
/// |[`do_not_propagate_mask`]|[`empty()`][d]|[`InputOutput`] and [`InputOnly`]|
/// |[`colormap`]              |[`CopyFromParent`] |[`InputOutput`] only       |
/// |[`cursor_appearance`]     |[`None`]     |[`InputOutput`] and [`InputOnly`]|
///
/// [`background_pixmap`]: Attributes::background_pixmap
/// [`background_color`]: Attributes::background_color
/// [`border_pixmap`]: Attributes::border_pixmap
/// [`border_color`]: Attributes::border_color
/// [`bit_gravity`]: Attributes::bit_gravity
/// [`window_gravity`]: Attributes::window_gravity
/// [`maintain_contents`]: Attributes::maintain_contents
/// [`maintained_planes`]: Attributes::maintained_planes
/// [`maintenance_fallback_color`]: Attributes::maintenance_fallback_color
/// [`maintain_windows_under`]: Attributes::maintain_windows_under
/// [`event_mask`]: Attributes::event_mask
/// [`do_not_propagate_mask`]: Attributes::do_not_propagate_mask
/// [`override_redirect`]: Attributes::override_redirect
/// [`colormap`]: Attributes::colormap
/// [`cursor_appearance`]: Attributes::cursor_appearance
///
/// [`CopyFromParent`]: CopyableFromParent::CopyFromParent
/// [`Forget`]: BitGravity::Forget
/// [`NorthWest`]: WindowGravity::NorthWest
/// [`NotUseful`]: MaintainContents::NotUseful
/// [e]: EventMask::empty
/// [d]: DeviceEventMask::empty
///
/// [`InputOutput`]: crate::WindowClass::InputOutput
/// [`InputOnly`]: crate::WindowClass::InputOnly
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
	background_color: Option<ColorId>,

	border_pixmap: Option<BorderPixmap>,
	border_color: Option<ColorId>,

	bit_gravity: Option<__BitGravity>,
	window_gravity: Option<__WindowGravity>,

	maintain_contents: Option<MaintainContents>,
	maintained_planes: Option<u32>,
	maintenance_fallback_color: Option<ColorId>,

	override_redirect: Option<__bool>,
	maintain_windows_under: Option<__bool>,

	event_mask: Option<EventMask>,
	do_not_propagate_mask: Option<DeviceEventMask>,

	colormap: Option<ColormapAttribute>,

	#[allow(clippy::option_option)]
	cursor_appearance: Option<CursorAppearanceAttribute>,
}

impl Attributes {
	/// Returns a new [`AttributesBuilder`] with which an `Attributes` set can
	/// be created.
	#[must_use]
	pub const fn builder() -> AttributesBuilder {
		AttributesBuilder::new()
	}
}

/// A builder used to construct a new [`Attributes` set].
///
/// All attributes start as [`None`], and can be configured with methods on this
/// builder. When the builder is configured, [`build()`] can be
/// used to construct the resulting [`Attributes`].
///
/// [`build()`]: AttributesBuilder::build
/// [`Attributes` set]: Attributes
#[derive(Clone, Default, Debug, Hash, PartialEq, Eq)]
pub struct AttributesBuilder {
	x11_size: usize,

	mask: AttributesMask,

	background_pixmap: Option<BackgroundPixmap>,
	background_color: Option<ColorId>,

	border_pixmap: Option<BorderPixmap>,
	border_color: Option<ColorId>,

	bit_gravity: Option<BitGravity>,
	window_gravity: Option<WindowGravity>,

	maintain_contents: Option<MaintainContents>,
	maintained_planes: Option<u32>,
	maintenance_fallback_color: Option<ColorId>,

	override_redirect: Option<bool>,
	maintain_windows_under: Option<bool>,

	event_mask: Option<EventMask>,
	do_not_propagate_mask: Option<DeviceEventMask>,

	colormap: Option<ColormapAttribute>,

	cursor_appearance: Option<CursorAppearanceAttribute>,
}

impl AttributesBuilder {
	/// Creates a new `AttributesBuilder`.
	///
	/// All attributes start as [`None`], and can be configured with the other
	/// methods on this builder. When the builder is configured, [`build()`] can
	/// be used to build the resulting [`Attributes`].
	///
	/// [`build()`]: AttributesBuilder::build
	#[must_use]
	pub const fn new() -> Self {
		Self {
			x11_size: AttributesMask::X11_SIZE,

			mask: AttributesMask::empty(),

			background_pixmap: None,
			background_color: None,

			border_pixmap: None,
			border_color: None,

			bit_gravity: None,
			window_gravity: None,

			maintain_contents: None,
			maintained_planes: None,
			maintenance_fallback_color: None,

			override_redirect: None,
			maintain_windows_under: None,

			event_mask: None,
			do_not_propagate_mask: None,

			colormap: None,

			cursor_appearance: None,
		}
	}

	/// Constructs the resulting [`Attributes` set] with the configured
	/// attributes.
	///
	/// [`Attributes` set]: Attributes
	#[must_use]
	pub fn build(self) -> Attributes {
		Attributes {
			x11_size: self.x11_size,

			mask: self.mask,

			background_pixmap: self.background_pixmap,
			background_color: self.background_color,

			border_pixmap: self.border_pixmap,
			border_color: self.border_color,

			bit_gravity: self.bit_gravity.map(__BitGravity),
			window_gravity: self.window_gravity.map(__WindowGravity),

			maintain_contents: self.maintain_contents,
			maintained_planes: self.maintained_planes,
			maintenance_fallback_color: self.maintenance_fallback_color,

			override_redirect: self.override_redirect.map(__bool),
			maintain_windows_under: self.maintain_windows_under.map(__bool),

			event_mask: self.event_mask,
			do_not_propagate_mask: self.do_not_propagate_mask,

			colormap: self.colormap,

			cursor_appearance: self.cursor_appearance,
		}
	}
}

impl AttributesBuilder {
	/// Configures the [pixmap] used as the [window]'s background.
	///
	/// See [`Attributes::background_pixmap`] for more information.
	///
	/// # Errors
	/// If [`ParentRelative`] is specified, the [window] must have the same
	/// depth as the parent, else a [`Match` error] is generated.
	///
	/// [`ParentRelative`]: ParentRelatable::ParentRelative
	///
	/// [pixmap]: Pixmap
	/// [window]: crate::Window
	///
	/// [`Match` error]: crate::x11::error::Match
	pub fn background_pixmap(&mut self, background_pixmap: BackgroundPixmap) -> &mut Self {
		if self.background_pixmap.is_none() {
			self.x11_size += 4;
		}

		self.background_pixmap = Some(background_pixmap);
		self.mask |= AttributesMask::BACKGROUND_PIXMAP;

		self
	}
	/// Configures the solid color used as the [window]'s background.
	///
	/// If this is configured, it takes precedence over [`background_pixmap`].
	///
	/// See [`Attributes::background_color`] for more information.
	///
	/// [window]: crate::Window
	/// [`background_pixmap`]: AttributesBuilder::background_pixmap
	pub fn background_color(&mut self, background_color: ColorId) -> &mut Self {
		if self.background_color.is_none() {
			self.x11_size += 4;
		}

		self.background_color = Some(background_color);
		self.mask |= AttributesMask::BACKGROUND_COLOR;

		self
	}

	/// Configures the [pixmap] used for the [window]'s border.
	///
	/// See [`Attributes::border_pixmap`] for more information.
	///
	/// # Errors
	/// The [pixmap] and the [window] must have the same root window and the
	/// same depth, else a [`Match` error] is generated.
	///
	/// If [`CopyFromParent`] is specified, the [window] must have the same
	/// depth as its parent, else a [`Match` error] is generated.
	///
	/// [pixmap]: Pixmap
	/// [window]: crate::Window
	///
	/// [`CopyFromParent`]: CopyableFromParent::CopyFromParent
	/// [`Match` error]: crate::x11::error::Match
	pub fn border_pixmap(&mut self, border_pixmap: BorderPixmap) -> &mut Self {
		if self.border_pixmap.is_none() {
			self.x11_size += 4;
		}

		self.border_pixmap = Some(border_pixmap);
		self.mask |= AttributesMask::BORDER_PIXMAP;

		self
	}
	/// Configures the solid color used for the [window]'s border.
	///
	/// If this is configured, it takes precedence over the [`border_pixmap`].
	///
	/// See [`Attributes::border_color`] for more information.
	///
	/// [window]: crate::Window
	///
	/// [`border_pixmap`]: AttributesBuilder::border_pixmap
	pub fn border_color(&mut self, border_color: ColorId) -> &mut Self {
		if self.border_color.is_none() {
			self.x11_size += 4;
		}

		self.border_color = Some(border_color);
		self.mask |= AttributesMask::BORDER_COLOR;

		self
	}

	/// Configures the [region] of the [window] which should be retained if the
	/// [window] is resized.
	///
	/// See [`Attributes::bit_gravity`] for more information.
	///
	/// [region]: crate::Region
	/// [window]: crate::Window
	pub fn bit_gravity(&mut self, bit_gravity: BitGravity) -> &mut Self {
		if self.bit_gravity.is_none() {
			self.x11_size += 4;
		}

		self.bit_gravity = Some(bit_gravity);
		self.mask |= AttributesMask::BIT_GRAVITY;

		self
	}
	/// Configures how the [window] should be repositioned if its parent is
	/// resized.
	///
	/// See [`Attributes::window_gravity`] for more information.
	///
	/// [window]: crate::Window
	pub fn window_gravity(&mut self, window_gravity: WindowGravity) -> &mut Self {
		if self.window_gravity.is_none() {
			self.x11_size += 4;
		}

		self.window_gravity = Some(window_gravity);
		self.mask |= AttributesMask::WINDOW_GRAVITY;

		self
	}

	/// Configures the conditions under which the X server should maintain the
	/// contents of obscured [regions] of the [window].
	///
	/// See [`Attributes::maintain_contents`] for more information.
	///
	/// [regions]: crate::Region
	/// [window]: crate::Window
	pub fn maintain_contents(&mut self, maintain_contents: MaintainContents) -> &mut Self {
		if self.maintain_contents.is_none() {
			self.x11_size += 4;
		}

		self.maintain_contents = Some(maintain_contents);
		self.mask |= AttributesMask::MAINTAIN_CONTENTS;

		self
	}
	/// Configures which bit planes of the [window] are maintained for
	/// [`maintain_contents`] and [`maintain_windows_under`].
	///
	/// See [`Attributes::maintain_contents`] for more information.
	///
	/// [window]: crate::Window
	///
	/// [`maintain_contents`]: Attributes::maintain_contents
	/// [`maintain_windows_under`]: Attributes::maintain_windows_under
	pub fn maintained_planes(&mut self, maintained_planes: u32) -> &mut Self {
		if self.maintained_planes.is_none() {
			self.x11_size += 4;
		}

		self.maintained_planes = Some(maintained_planes);
		self.mask |= AttributesMask::MAINTAINED_PLANES;

		self
	}
	/// Configures the color used for  bit planes which are not preserved for
	/// [`maintain_contents`] and [`maintain_windows_under`].
	///
	/// See [`Attributes::maintenance_fallback_color`] for more information.
	///
	/// [`maintain_contents`]: Attributes::maintain_contents
	/// [`maintain_windows_under`]: Attributes::maintain_windows_under
	pub fn maintenance_fallback_color(&mut self, maintenance_fallback_color: ColorId) -> &mut Self {
		if self.maintenance_fallback_color.is_none() {
			self.x11_size += 4;
		}

		self.maintenance_fallback_color = Some(maintenance_fallback_color);
		self.mask |= AttributesMask::MAINTENANCE_FALLBACK_COLOR;

		self
	}

	/// Configures whether [`MapWindow`] and [`ConfigureWindow`] requests on the
	/// [window] should override a [`SUBSTRUCTURE_REDIRECT`] selection on its
	/// parent.
	///
	/// See [`Attributes::override_redirect`] for more information.
	///
	/// [window]: crate::Window
	///
	/// [`MapWindow`]: crate::x11::request::MapWindow
	/// [`ConfigureWindow`]: crate::x11::request::ConfigureWindow
	///
	/// [`SUBSTRUCTURE_REDIRECT`]: EventMask::SUBSTRUCTURE_REDIRECT
	pub fn override_redirect(&mut self, override_redirect: bool) -> &mut Self {
		if self.override_redirect.is_none() {
			self.x11_size += 4;
		}

		self.override_redirect = Some(override_redirect);
		self.mask |= AttributesMask::OVERRIDE_REDIRECT;

		self
	}
	/// Configures whether the X server should maintain the contents of
	/// [windows][window] under this [window].
	///
	/// See [`Attributes::maintain_windows_under`] for more information.
	///
	/// [window]: crate::Window
	pub fn maintain_windows_under(&mut self, maintain_windows_under: bool) -> &mut Self {
		if self.maintain_windows_under.is_none() {
			self.x11_size += 4;
		}

		self.maintain_windows_under = Some(maintain_windows_under);
		self.mask |= AttributesMask::MAINTAIN_WINDOWS_UNDER;

		self
	}

	/// Configures which [events] the client wishes to select interest in for
	/// the [window] (or, for some [events], descendents of the [window]).
	///
	/// See [`Attributes::event_mask`] for more information.
	///
	/// [events]: crate::message::Event
	/// [window]: crate::Window
	pub fn event_mask(&mut self, event_mask: EventMask) -> &mut Self {
		if self.event_mask.is_none() {
			self.x11_size += 4;
		}

		self.event_mask = Some(event_mask);
		self.mask |= AttributesMask::EVENT_MASK;

		self
	}
	/// Configures which [events][event] should not be propagated to ancestors
	/// of the [window] when no client has selected the [event] on the [window].
	///
	/// See [`Attributes::do_not_propagate_mask`] for more information.
	///
	/// [event]: crate::message::Event
	/// [window]: crate::Window
	pub fn do_not_propagate_mask(&mut self, do_not_propagate_mask: DeviceEventMask) -> &mut Self {
		if self.do_not_propagate_mask.is_none() {
			self.x11_size += 4;
		}

		self.do_not_propagate_mask = Some(do_not_propagate_mask);
		self.mask |= AttributesMask::DO_NOT_PROPAGATE_MASK;

		self
	}

	/// Configures which [colormap] is specified as best reflecting the true
	/// colors of the [window].
	///
	/// See [`Attributes::colormap`] for more information.
	///
	/// # Errors
	/// The [colormap] must have the same [visual type] and root window as the
	/// [window], else a [`Match` error] is generated.
	///
	/// If [`CopyFromParent`] is specified, the [window] must have the same
	/// [visual type] as its parent, else a [`Match` error] is generated. The
	/// parent must not have a [`colormap` attribute] of [`None`], else a
	/// [`Match` error] is generated.
	///
	/// [colormap]: Colormap
	/// [visual type]: crate::visual::VisualType
	/// [window]: crate::Window
	///
	/// [`colormap` attribute]: Attributes::colormap
	/// [`Match` error]: crate::x11::error::Match
	pub fn colormap(&mut self, colormap: ColormapAttribute) -> &mut Self {
		if self.colormap.is_none() {
			self.x11_size += 4;
		}

		self.colormap = Some(colormap);
		self.mask |= AttributesMask::COLORMAP;

		self
	}

	/// Configures the [appearance of the cursor] used when the cursor is within
	/// the [window].
	///
	/// [appearance of the cursor]: CursorAppearance
	/// [window]: crate::Window
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
	/// The [pixmap] used as the [window]'s background.
	///
	/// If a background of [`ParentRelative`] is specified, then the
	/// `background_pixmap` of the [window]'s parent is used. Changes to the
	/// parent's `background_pixmap` will be reflected for this [window]'s
	/// background.
	///
	/// A `background_pixmap` of [`None`] means the [window] has a transparent
	/// background.
	///
	/// If [`background_color`] is configured, it overrides the value of the
	/// `background_pixmap`.
	///
	/// [pixmap]: Pixmap
	/// [window]: crate::Window
	///
	/// [`ParentRelative`]: ParentRelatable::ParentRelative
	/// [`background_color`]: Attributes::background_color
	#[must_use]
	#[allow(
		clippy::missing_const_for_fn,
		reason = "const is omitted for API uniformity with the other methods and sets"
	)]
	pub fn background_pixmap(&self) -> Option<&BackgroundPixmap> {
		self.background_pixmap.as_ref()
	}
	/// The solid color used as the [window]'s background.
	///
	/// A [pixmap] of undefined size filled with this color is used.
	///
	/// If this is configured, it takes precedence over the
	/// [`background_pixmap`].
	///
	/// [window]: crate::Window
	/// [pixmap]: Pixmap
	///
	/// [`background_pixmap`]: Attributes::background_pixmap
	#[must_use]
	#[allow(
		clippy::missing_const_for_fn,
		reason = "const is omitted for API uniformity with the other methods and sets"
	)]
	pub fn background_color(&self) -> Option<&ColorId> {
		self.background_color.as_ref()
	}

	/// The [pixmap] used as the [window]'s border.
	///
	/// If a `border_pixmap` of [`CopyFromParent`] is specified, then the
	/// `border_pixmap` of the [window]'s parent is copied, but subsequent
	/// changes to the parent's border will not affect this [window]'s border.
	///
	/// [pixmap]: Pixmap
	/// [window]: crate::Window
	#[must_use]
	#[allow(
		clippy::missing_const_for_fn,
		reason = "const is omitted for API uniformity with the other methods and sets"
	)]
	pub fn border_pixmap(&self) -> Option<&BorderPixmap> {
		self.border_pixmap.as_ref()
	}
	/// The solid color used as the [window]'s border.
	///
	/// A [pixmap] of undefined size filled with this color is used.
	///
	/// If this is configured, it takes precedence over the
	/// [`border_pixmap`].
	///
	/// [window]: crate::Window
	/// [pixmap]: Pixmap
	///
	/// [`border_pixmap`]: Attributes::border_pixmap
	#[must_use]
	#[allow(
		clippy::missing_const_for_fn,
		reason = "const is omitted for API uniformity with the other methods and sets"
	)]
	pub fn border_color(&self) -> Option<&ColorId> {
		self.border_color.as_ref()
	}

	/// Defines the [region] of the [window] which should be retained if the
	/// [window] is resized.
	///
	/// See [`BitGravity`] for more information.
	///
	/// [region]: crate::Region
	/// [window]: crate::Window
	#[must_use]
	#[allow(
		clippy::missing_const_for_fn,
		reason = "const is omitted for API uniformity with the other methods and sets"
	)]
	pub fn bit_gravity(&self) -> Option<&BitGravity> {
		self.bit_gravity
			.as_ref()
			.map(|__BitGravity(gravity)| gravity)
	}
	/// Defines how the [window] should be repositioned if its parent is
	/// resized.
	///
	/// See [`WindowGravity`] for more information.
	///
	/// [window]: crate::Window
	#[must_use]
	#[allow(
		clippy::missing_const_for_fn,
		reason = "const is omitted for API uniformity with the other methods and sets"
	)]
	pub fn window_gravity(&self) -> Option<&WindowGravity> {
		self.window_gravity
			.as_ref()
			.map(|__WindowGravity(gravity)| gravity)
	}

	/// The conditions under which the X server should maintain the contents of
	/// obscured [regions] of the [window].
	///
	/// See [`MaintainContents`] for more information.
	///
	/// [regions]: crate::Region
	/// [window]: crate::Window
	#[must_use]
	#[allow(
		clippy::missing_const_for_fn,
		reason = "const is omitted for API uniformity with the other methods and sets"
	)]
	pub fn maintain_contents(&self) -> Option<&MaintainContents> {
		self.maintain_contents.as_ref()
	}
	/// Which bit planes of the [window] hold dynamic data which must be
	/// maintained for [`maintain_contents`] and [`maintain_windows_under`].
	///
	/// See [`MaintainContents`] for more information.
	///
	/// [window]: crate::Window
	/// [`maintain_contents`]: Attributes::maintain_contents
	/// [`maintain_windows_under`]: Attributes::maintain_windows_under
	#[must_use]
	#[allow(
		clippy::missing_const_for_fn,
		reason = "const is omitted for API uniformity with the other methods and sets"
	)]
	pub fn maintained_planes(&self) -> Option<&u32> {
		self.maintained_planes.as_ref()
	}
	/// The color to use for bit planes which are not preserved for
	/// [`maintain_contents`] and [`maintain_windows_under`] (see
	/// [`maintained_planes`]).
	///
	/// See [`MaintainContents`] for more information.
	///
	/// [`maintain_contents`]: Attributes::maintain_contents
	/// [`maintained_planes`]: Attributes::maintained_planes
	/// [`maintain_windows_under`]: Attributes::maintain_windows_under
	#[must_use]
	#[allow(
		clippy::missing_const_for_fn,
		reason = "const is omitted for API uniformity with the other methods and sets"
	)]
	pub fn maintenance_fallback_color(&self) -> Option<&ColorId> {
		self.maintenance_fallback_color.as_ref()
	}

	/// Whether [`MapWindow`] and [`ConfigureWindow`] requests on this [window]
	/// should override a [`SUBSTRUCTURE_REDIRECT`] selection on its parent.
	///
	/// This is typically used to inform a window manager not to tamper with the
	/// [window].
	///
	/// [window]: crate::Window
	///
	/// [`MapWindow`]: crate::x11::request::MapWindow
	/// [`ConfigureWindow`]: crate::x11::request::ConfigureWindow
	///
	/// [`SUBSTRUCTURE_REDIRECT`]: EventMask::SUBSTRUCTURE_REDIRECT
	#[must_use]
	pub fn override_redirect(&self) -> Option<&bool> {
		self.override_redirect.as_ref().map(|__bool(bool)| bool)
	}
	/// Whether the X server should maintain the contents of [windows][window]
	/// under this [window].
	///
	/// See [`MaintainContents`] for more information.
	///
	/// [window]: crate::Window
	#[must_use]
	pub fn maintain_windows_under(&self) -> Option<&bool> {
		self.maintain_windows_under
			.as_ref()
			.map(|__bool(bool)| bool)
	}

	/// Defines which [events] the client wishes to select interest in for this
	/// [window] (or, for some [events], descendents of this [window]).
	///
	/// See [`EventMask`] for more information.
	///
	/// [events]: crate::message::Event
	/// [window]: crate::Window
	#[must_use]
	#[allow(
		clippy::missing_const_for_fn,
		reason = "const is omitted for API uniformity with the other methods and sets"
	)]
	pub fn event_mask(&self) -> Option<&EventMask> {
		self.event_mask.as_ref()
	}
	/// Defines which [events][event] should not be propagated to ancestors of
	/// this [window] when no client has selected the [event] on this [window].
	///
	/// [event]: crate::message::Event
	/// [window]: crate::Window
	#[must_use]
	#[allow(
		clippy::missing_const_for_fn,
		reason = "const is omitted for API uniformity with the other methods and sets"
	)]
	pub fn do_not_propagate_mask(&self) -> Option<&DeviceEventMask> {
		self.do_not_propagate_mask.as_ref()
	}

	/// Specifies the [colormap] which best reflects the true colors of this
	/// [window].
	///
	/// If [`CopyFromParent`] is specified, the [window]'s parent's [colormap]
	/// is copied. Subsequent changes to the parent's `colormap` attribute to
	/// not affect this [window].
	///
	/// [colormap]: Colormap
	/// [window]: crate::Window
	///
	/// [`CopyFromParent`]: CopyableFromParent::CopyFromParent
	/// [visual type]: crate::visual::VisualType
	///
	/// [`Match` error]: crate::x11::error::Match
	#[must_use]
	#[allow(
		clippy::missing_const_for_fn,
		reason = "const is omitted for API uniformity with the other methods and sets"
	)]
	pub fn colormap(&self) -> Option<&ColormapAttribute> {
		self.colormap.as_ref()
	}

	/// The [appearance of the cursor] used when the cursor is within this
	/// [window].
	///
	/// If [`None`] is specified, the `cursor_appearance` of the [window]'s
	/// parent is used. Any change in the parent's `cursor_appearance` will
	/// affect the appearance of the cursor within this [window] too.
	///
	/// [window]: crate::Window
	///
	/// [appearance of the cursor]: CursorAppearance
	#[must_use]
	#[allow(
		clippy::missing_const_for_fn,
		reason = "const is omitted for API uniformity with the other methods and sets"
	)]
	pub fn cursor_appearance(&self) -> Option<&CursorAppearanceAttribute> {
		self.cursor_appearance.as_ref()
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
		/// Whether the [`background_pixmap` attribute] is configured.
		///
		/// See [`Attributes::background_pixmap`] for more information.
		///
		/// [`background_pixmap` attribute]: Attributes::background_pixmap
		const BACKGROUND_PIXMAP = 0x0000_0001;
		/// Whether the [`background_color` attribute] is configured.
		///
		/// See [`Attributes::background_color`] for more information.
		///
		/// [`background_color` attribute]: Attributes::background_color
		const BACKGROUND_COLOR = 0x0000_0002;

		/// Whether the [`border_pixmap` attribute] is configured.
		///
		/// See [`Attributes::border_pixmap`] for more information.
		///
		/// [`border_pixmap` attribute]: Attributes::border_pixmap
		const BORDER_PIXMAP = 0x0000_0004;
		/// Whether the [`border_color` attribute] is configured.
		///
		/// See [`Attributes::border_color`] for more information.
		///
		/// [`border_color` attribute]: Attributes::border_color
		const BORDER_COLOR = 0x0000_0008;

		/// Whether the [`bit_gravity` attribute] is configured.
		///
		/// See [`Attributes::bit_gravity`] for more information.
		///
		/// [`bit_gravity` attribute]: Attributes::bit_gravity
		const BIT_GRAVITY = 0x0000_0010;
		/// Whether the [`window_gravity` attribute] is configured.
		///
		/// See [`Attributes::window_gravity`] for more information.
		///
		/// [`window_gravity` attribute]: Attributes::window_gravity
		const WINDOW_GRAVITY = 0x0000_0020;

		/// Whether the [`maintain_contents` attribute] is configured.
		///
		/// See [`Attributes::maintain_contents`] for more information.
		///
		/// [`maintain_contents` attribute]: Attributes::maintain_contents
		const MAINTAIN_CONTENTS = 0x0000_0040;
		/// Whether the [`maintained_planes` attribute] is configured.
		///
		/// See [`Attributes::maintained_planes`] for more information.
		///
		/// [`maintained_planes` attribute]: Attributes::maintained_planes
		const MAINTAINED_PLANES = 0x0000_0080;
		/// Whether the [`maintenance_fallback_color` attribute] is configured.
		///
		/// See [`Attributes::maintenance_fallback_color`] for more information.
		///
		/// [`maintenance_fallback_color` attribute]: Attributes::maintenance_fallback_color
		const MAINTENANCE_FALLBACK_COLOR = 0x0000_0100;

		/// Whether the [`override_redirect` attribute] is configured.
		///
		/// See [`Attributes::override_redirect`] for more information.
		///
		/// [`override_redirect` attribute]: Attributes::override_redirect
		const OVERRIDE_REDIRECT = 0x0000_0200;
		/// Whether the [`maintain_windows_under` attribute] is configured.
		///
		/// See [`Attributes::maintain_windows_under`] for more information.
		///
		/// [`maintain_windows_under` attribute]: Attributes::maintain_windows_under
		const MAINTAIN_WINDOWS_UNDER = 0x0000_0400;

		/// Whether the [`event_mask` attribute] is configured.
		///
		/// See [`Attributes::event_mask`] for more information.
		///
		/// [`event_mask` attribute]: Attributes::event_mask
		const EVENT_MASK = 0x0000_0800;
		/// Whether the [`do_not_propagate_mask` attribute] is configured.
		///
		/// See [`Attributes::do_not_propagate_mask`] for more information.
		///
		/// [`do_not_propagate_mask` attribute]: Attributes::do_not_propagate_mask
		const DO_NOT_PROPAGATE_MASK = 0x0000_1000;

		/// Whether the [`colormap` attribute] is configured.
		///
		/// See [`Attributes::colormap`] for more information.
		///
		/// [`colormap` attribute]: Attributes::colormap
		const COLORMAP = 0x0000_2000;

		/// Whether the [`cursor_appearance` attribute] is configured.
		///
		/// See [`Attributes::cursor_appearance`] for more information.
		///
		/// [`cursor_appearance` attribute]: Attributes::cursor_appearance
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
		let background_color = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::BACKGROUND_COLOR),
		)?;

		let border_pixmap = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::BORDER_PIXMAP),
		)?;
		let border_color = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::BORDER_COLOR),
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

		let maintain_contents = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::MAINTAIN_CONTENTS),
		)?;
		let maintained_planes = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::MAINTAINED_PLANES),
		)?;
		let maintenance_fallback_color = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::MAINTENANCE_FALLBACK_COLOR),
		)?;

		let override_redirect = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::OVERRIDE_REDIRECT),
		)?;
		let maintain_windows_under = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(AttributesMask::MAINTAIN_WINDOWS_UNDER),
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
			background_color,

			border_pixmap,
			border_color,

			bit_gravity,
			window_gravity,

			maintain_contents,
			maintained_planes,
			maintenance_fallback_color,

			override_redirect,
			maintain_windows_under,

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
		if let Some(background_color) = &self.background_color {
			background_color.write_to(buf)?;
		}

		if let Some(border_pixmap) = &self.border_pixmap {
			border_pixmap.write_to(buf)?;
		}
		if let Some(border_color) = &self.border_color {
			border_color.write_to(buf)?;
		}

		if let Some(bit_gravity) = &self.bit_gravity {
			bit_gravity.write_to(buf)?;
		}
		if let Some(window_gravity) = &self.window_gravity {
			window_gravity.write_to(buf)?;
		}

		if let Some(maintain_contents) = &self.maintain_contents {
			maintain_contents.write_to(buf)?;
		}
		if let Some(maintained_planes) = &self.maintained_planes {
			maintained_planes.write_to(buf)?;
		}
		if let Some(maintenance_fallback_color) = &self.maintenance_fallback_color {
			maintenance_fallback_color.write_to(buf)?;
		}

		if let Some(override_redirect) = &self.override_redirect {
			override_redirect.write_to(buf)?;
		}
		if let Some(maintain_windows_under) = &self.maintain_windows_under {
			maintain_windows_under.write_to(buf)?;
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
