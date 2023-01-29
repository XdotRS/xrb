// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Requests] defined in the [core X11 protocol] that relate to [windows] and
//! their management.
//!
//! [Requests] are messages sent from an X client to the X server.
//!
//! [windows]: Window
//! [Requests]: Request
//! [core X11 protocol]: crate::x11

extern crate self as xrb;

use xrbk_macro::{derive_xrb, Readable, Writable, X11Size};

use crate::{
	message::Request,
	set::{Attributes, WindowConfig},
	unit::Px,
	visual::VisualId,
	x11::{error, reply},
	Coords,
	CopyableFromParent,
	Drawable,
	Rectangle,
	Window,
	WindowClass,
};

macro_rules! request_error {
	(
		$(#[$meta:meta])*
		$vis:vis enum $Name:ident for $Request:ty {
			$($($Error:ident),+$(,)?)?
		}
	) => {
		#[doc = concat!(
			"An [error](crate::message::Error) generated because of a failed [`",
			stringify!($Request),
			"` request](",
			stringify!($Request),
			")."
		)]
		#[doc = ""]
		$(#[$meta])*
		$vis enum $Name {
			$($(
				#[doc = concat!(
					"A [`",
					stringify!($Error),
					"` error](error::",
					stringify!($Error),
					")."
				)]
				$Error(error::$Error)
			),+)?
		}
	};
}

request_error! {
	pub enum CreateWindowError for CreateWindow {
		Colormap,
		CursorAppearance,
		ResourceIdChoice,
		Match,
		Pixmap,
		Value,
		Window,
	}
}

derive_xrb! {
	/// A [request] that creates a new [window].
	///
	/// [request]: Request
	/// [window]: Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct CreateWindow: Request(1, CreateWindowError) {
		#[metabyte]
		/// The [window]'s depth.
		///
		/// For a `class` of [`CopyFromParent`] or
		/// <code>[Other]\([InputOutput])</code>, [`CopyFromParent`] for `depth`
		/// means that the depth is copied from the `parent`.
		///
		/// # Errors
		/// For a `class` of <code>[Other]\([InputOnly])</code>, however,
		/// [`CopyFromParent`] is *required*, else a [`Match` error] is
		/// generated, and _does not_ mean that the depth is copied from the
		/// `parent`.
		///
		/// [window]: Window
		///
		/// [`CopyFromParent`]: CopyableFromParent::CopyFromParent
		/// [Other]: CopyableFromParent::Other
		/// [InputOutput]: WindowClass::InputOutput
		/// [InputOnly]: WindowClass::InputOnly
		///
		/// [`Match` error]: error::Match
		pub depth: CopyableFromParent<u8>,

		/// The [`Window` ID][window] which is to be assigned to the [window].
		///
		/// # Errors
		/// If the provided [`Window` ID][window] is already used or it is not
		/// allocated to your client, a [`ResourceIdChoice` error] is generated.
		///
		/// [window]: Window
		///
		/// [`ResourceIdChoice` error]: error::ResourceIdChoice
		#[doc(alias = "wid")]
		pub window_id: Window,
		/// The [window] which should be used as the new [window]'s parent.
		///
		/// # Errors
		/// For a `class` of [`InputOutput`], the parent cannot be an
		/// [`InputOnly`] [window], else a [`Match` error] is generated.
		///
		/// [window]: Window
		///
		/// [`InputOutput`]: WindowClass::InputOutput
		/// [`InputOnly`]: WindowClass::InputOnly
		///
		/// [`Match` error]: error::Match
		pub parent: Window,

		/// The coordinates and dimensions of the [window].
		///
		/// The coordinates are of the top-left corner of the [window], and are
		/// relative to the top-left corner of the [window]'s `parent`.
		///
		/// [window]: Window
		pub geometry: Rectangle,
		/// The width of the [window]'s border.
		///
		/// [window]: Window
		pub border_width: Px<u16>,

		/// The [window]'s [class].
		///
		/// [`CopyFromParent`] means the class is taken from the `parent`.
		///
		/// # Errors
		/// For [`InputOutput`], the `visual` type and `depth` must be
		/// combination supported by the [screen], else a [`Match` error] is
		/// generated.
		///
		/// For [`InputOnly`], the `visual` must be one supported by the
		/// [screen], else a [`Match` error] is generated.
		///
		/// [window]: Window
		/// [class]: WindowClass
		/// [screen]: crate::visual::Screen
		///
		/// [`CopyFromParent`]: CopyableFromParent::CopyFromParent
		/// [`InputOutput`]: WindowClass::InputOutput
		/// [`InputOnly`]: WindowClass::InputOnly
		///
		/// [`Match` error]: error::Match
		pub class: CopyableFromParent<WindowClass>,
		/// The visual used by the [window].
		///
		/// See [`VisualType`] for more information.
		///
		/// [window]: Window
		///
		/// [`VisualType`]: crate::visual::VisualType
		pub visual: CopyableFromParent<VisualId>,

		/// Additional [attributes] configured for the [window].
		///
		/// See [`Attributes`] for more information.
		///
		/// [window]: Window
		/// [attributes]: Attributes
		#[doc(alias("values", "value_mask", "value_list", "attribute_mask", "attribute_list"))]
		pub attributes: Attributes,
	}
}

request_error! {
	pub enum ChangeWindowAttributesError for ChangeWindowAttributes {
		Access,
		Colormap,
		CursorAppearance,
		Match,
		Pixmap,
		Value,
		Window,
	}
}

derive_xrb! {
	/// A [request] that configures the [attributes] of a [window].
	///
	/// The [`event_mask`] attribute on the [window] is not shared between
	/// clients: one client modifying the [`event_mask`] only selects interest
	/// in the relevant events for that client. There are three exceptions to
	/// this: only one client at a time may select [`SUBSTRUCTURE_REDIRECT`],
	/// [`RESIZE_REDIRECT`], or [`BUTTON_PRESS`] on the [window].
	///
	/// [request]: Request
	/// [window]: Window
	/// [attributes]: Attributes
	///
	/// [`event_mask`]: Attributes::event_mask
	/// [`SUBSTRUCTURE_REDIRECT`]: crate::EventMask::SUBSTRUCTURE_REDIRECT
	/// [`RESIZE_REDIRECT`]: crate::EventMask::RESIZE_REDIRECT
	/// [`BUTTON_PRESS`]: crate::EventMask::BUTTON_PRESS
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ChangeWindowAttributes: Request(2, ChangeWindowAttributesError) {
		/// The [window] which the `attributes` are changed on.
		///
		/// [window]: Window
		#[doc(alias = "window")]
		pub target: Window,

		/// The [attributes] which are changed.
		///
		/// [attributes]: Attributes
		#[doc(alias("values", "value_mask", "value_list", "attribute_mask", "attribute_list"))]
		pub attributes: Attributes,
	}

	/// A [request] that returns the current [attributes] of the [window].
	///
	/// [request]: Request
	/// [attributes]: Attributes
	/// [window]: Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct GetWindowAttributes: Request(3, error::Window) -> reply::GetWindowAttributes {
		/// The [window] for which this [request] gets the [attributes].
		///
		/// [request]: Request
		/// [attributes]: Attributes
		/// [window]: Window
		#[doc(alias = "window")]
		pub target: Window,
	}

	/// A [request] that destroys the given [window] and all its descendents.
	///
	/// If the `target` [window] is mapped, an [`UnmapWindow` request] is
	/// automatically performed.
	///
	/// The ordering of [`Destroy` events][event] is such that a
	/// [`Destroy` event][event] is generated on every descendent of the target
	/// [window] before being generated on the [window] itself.
	///
	/// This [request] has no effect on root [windows][window].
	///
	/// # Errors
	/// A [`Window` error] is generated if the `target` does not refer to a
	/// defined [window].
	///
	/// [window]: Window
	/// [request]: Request
	///
	/// [event]: crate::x11::event::Destroy
	///
	/// [`UnmapWindow` request]: UnmapWindow
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct DestroyWindow: Request(4, error::Window) {
		/// The [window] which is the target of the `DestroyWindow` [request].
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		/// [request]: Request
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,
	}

	/// A [request] that [destroys] every child of the given [window] in
	/// bottom-to-top stacking order.
	///
	/// A [`DestroyWindow` request][destroys] is performed on each child.
	///
	/// # Errors
	/// A [`Window` error] is generated if the `target` does not refer to a
	/// defined [window].
	///
	/// [window]: Window
	/// [request]: Request
	///
	/// [destroys]: DestroyWindow
	///
	/// [`Window` error]: error::Window
	#[doc(alias = "DestroySubwindows")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct DestroyChildren: Request(5, error::Window) {
		/// The [window] which will have its children [destroyed].
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [destroyed]: DestroyWindow
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,
	}
}

request_error! {
	pub enum ReparentWindowError for ReparentWindow {
		Match,
		Window,
	}
}

derive_xrb! {
	/// A [request] that changes a [window]'s parent to a different one.
	///
	/// If the [window] is mapped, an [`UnmapWindow` request] is first
	/// automatically performed. The [window] is then removed from its current
	/// parent and inserted as a child of the `new_parent`. If the [window] was
	/// mapped originally, then a [`MapWindow` request] is then automatically
	/// performed to map it again.
	///
	/// # Errors
	/// A [`Window` error] is generated if either the `target` or the
	/// `new_parent` do not refer to defined [windows][window].
	///
	/// A [`Match` error] is generated if the `new_parent` is not on the same
	/// [screen] as the old parent.
	///
	/// A [`Match` error] is generated if the `new_parent` is the `target`
	/// [window] itself, or a descendent of the `target` [window].
	///
	/// A [`Match` error] is generated if the `new_parent` is [`InputOnly`] but
	/// the `target` [window] is not.
	///
	/// A [`Match` error] is generated if the `target` [window] has a
	/// [`ParentRelative`] [`background_pixmap`] and the `new_parent` does not
	/// have the same depth as the `target` [window].
	///
	/// [window]: Window
	/// [request]: Request
	/// [screen]: crate::visual::Screen
	///
	/// [`UnmapWindow` request]: UnmapWindow
	/// [`MapWindow` request]: MapWindow
	///
	/// [`InputOnly`]: WindowClass::InputOnly
	/// [`ParentRelative`]: crate::ParentRelatable::ParentRelative
	///
	/// [`background_pixmap`]: Attributes::background_pixmap
	///
	/// [`Match` error]: error::Match
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct ReparentWindow: Request(7, ReparentWindowError) {
		/// The [window] which will be transferred to be a child of the
		/// `new_parent`.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,
		/// The `target`'s new parent [window].
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// A [`Match` error] is generated if this [window] is not on the same
		/// [screen] as the `target` [window].
		///
		/// A [`Match` error] is generated if this [window] is the same [window]
		/// as the `target`, or is a descendent of the `target` [window].
		///
		/// A [`Match` error] is generated if this [window] is [`InputOnly`] but
		/// the `target` [window] is not.
		///
		/// [window]: Window
		/// [screen]: crate::visual::Screen
		///
		/// [`InputOnly`]: WindowClass::InputOnly
		///
		/// [`Match` error]: error::Match
		/// [`Window` error]: error::Window
		#[doc(alias = "parent")]
		pub new_parent: Window,

		/// The `target`'s new coordinates relative to its `new_parent`'s
		/// top-left corner.
		#[doc(alias("x", "y"))]
		pub coords: Coords,
	}

	/// A [request] that maps the given [window].
	///
	/// If the [window]'s [`override_redirect` attribute] is `false` and some
	/// other client has selected [`SUBSTRUCTURE_REDIRECT`] on its parent, then
	/// a [`MapWindowRequest` event] is generated, but the [window] remains
	/// unmapped. Otherwise, the [window] is mapped and a [`Map` event] is
	/// generated.
	///
	/// If the [window] is already mapped, this [request] has no effect.
	///
	/// # Errors
	/// A [`Window` error] is generated if the `target` does not refer to a
	/// defined [window].
	///
	/// [window]: Window
	/// [request]: Request
	///
	/// [`Map` event]: crate::x11::event::Map
	/// [`MapWindowRequest` event]: crate::x11::event::MapWindowRequest
	///
	/// [`override_redirect` attribute]: Attributes::override_redirect
	///
	/// [`SUBSTRUCTURE_REDIRECT`]: crate::EventMask::SUBSTRUCTURE_REDIRECT
	///
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct MapWindow: Request(8, error::Window) {
		/// The [window] which is the target of the `MapWindow` [request].
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		/// [request]: Request
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,
	}

	/// A [request] that [maps] every unmapped child of the given [window] in
	/// top-to-bottom stacking order.
	///
	/// A [`MapWindow` request][maps] is performed on each unmapped child.
	///
	/// # Errors
	/// A [`Window` error] is generated if the `target` does not refer to a
	/// defined [window].
	///
	/// [window]: Window
	/// [request]: Request
	///
	/// [maps]: MapWindow
	///
	///
	/// [`Window` error]: error::Window
	#[doc(alias = "MapSubwindows")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct MapChildren: Request(9, error::Window) {
		/// The [window] which will have its unmapped children [mapped].
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [mapped]: MapWindow
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,
	}

	/// A [request] that unmaps the given [window].
	///
	/// If the [window] is currently mapped, the [window] is unmapped and an
	/// [`Unmap` event] is generated.
	///
	/// If the [window] is already unmapped, this [request] has no effect.
	///
	/// # Errors
	/// A [`Window` error] is generated if the `target` does not refer to a
	/// defined [window].
	///
	/// [window]: Window
	/// [request]: Request
	///
	/// [`Unmap` event]: crate::x11::event::Unmap
	///
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct UnmapWindow: Request(10, error::Window) {
		/// The [window] which is the target of the `UnmapWindow` [request].
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		/// [request]: Request
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,
	}

	/// A [request] that [unmaps] every mapped child of the
	/// given [window] in bottom-to-top stacking order.
	///
	/// An [`UnmapWindow` request][unmaps] is performed on each mapped child.
	///
	/// # Errors
	/// A [`Window` error] is generated if the `target` does not refer to a
	/// defined [window].
	///
	/// [window]: Window
	/// [request]: Request
	///
	/// [unmaps]: UnmapWindow
	///
	/// [`Window` error]: error::Window
	#[doc(alias = "UnmapSubwindows")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct UnmapChildren: Request(11, error::Window) {
		/// The [window] which will have its mapped children [unmapped].
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [unmapped]: UnmapWindow
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,
	}
}

request_error! {
	pub enum ConfigureWindowError for ConfigureWindow {
		Match,
		Value,
		Window,
	}
}

derive_xrb! {
	/// A [request] that [configures] a [window].
	///
	/// See [`WindowConfig`] for more information.
	///
	/// # Errors
	/// A [`Window` error] is generated if `target` does not refer to a defined
	/// [window].
	///
	/// A [`Match` error] is generated if either the [`width`] or [`height`] is
	/// configured to be zero.
	///
	/// A [`Match` error] is generated if the [`border_width`] is configured to
	/// be anything other than zero if the `target` [window] is [`InputOnly`].
	///
	/// A [`Match` error] is generated if [`sibling`] is configured without a
	/// specified [`stack_mode`].
	///
	/// A [`Match` error] is generated if [`sibling`] is specified but that
	/// specified [window] is not actually a sibling of the `target` [window].
	///
	/// [window]: Window
	/// [request]: Request
	///
	/// [configures]: WindowConfig
	///
	/// [`InputOnly`]: WindowClass::InputOnly
	///
	/// [`width`]: WindowConfig::width
	/// [`height`]: WindowConfig::height
	/// [`border_width`]: WindowConfig::border_width
	/// [`sibling`]: WindowConfig::sibling
	/// [`stack_mode`]: WindowConfig::stack_mode
	///
	/// [`Window` error]: error::Window
	/// [`Match` error]: error::Match
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ConfigureWindow: Request(12, ConfigureWindowError) {
		/// The [window] which is the target of the `ConfigureWindow` [request].
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		/// [request]: Request
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,

		/// The changes to the `target` [window]'s [configuration].
		///
		/// See [`WindowConfig`] for more information.
		///
		/// If the `target` [window] is a root [window], this [request] has no
		/// effect.
		///
		/// # Errors
		/// A [`Match` error] is generated if either the [`width`] or the
		/// [`height`] is zero.
		///
		/// A [`Match` error] is generated if the [`border_width`] is set to
		/// zero if the `target` [window] is [`InputOnly`].
		///
		/// A [`Match` error] is generated if [`sibling`] is configured without
		/// a specified [`stack_mode`].
		///
		/// A [`Match` error] is generated if [`sibling`] is specified but that
		/// specified [window] is not actually a sibling of the `target`
		/// [window].
		///
		/// [configuration]: WindowConfig
		/// [window]: Window
		/// [request]: Request
		///
		/// [`width`]: WindowConfig::width
		/// [`height`]: WindowConfig::height
		/// [`border_width`]: WindowConfig::border_width
		/// [`sibling`]: WindowConfig::sibling
		/// [`stack_mode`]: WindowConfig::stack_mode
		///
		/// [`InputOnly`]: WindowClass::InputOnly
		///
		/// [`Match` error]: error::Match
		#[doc(alias("values", "value_mask", "value_list", "window_config"))]
		pub config: WindowConfig,
	}
}

request_error! {
	pub enum CirculateWindowError for CirculateWindow {
		Value,
		Window,
	}
}

/// The direction with which a [window]'s mapped children are circulated in
/// their stacking order.
///
/// This is used in the [`CirculateWindow` request].
///
/// [window]: Window
///
/// [`CirculateWindow` request]: CirculateWindow
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum CirculateDirection {
	/// Raises the lowest mapped child that is occluded by another child, if
	/// any, to the top of the stack.
	RaiseLowest,

	/// Lowers the highest mapped child that occludes another child, if any, to
	/// the bottom of the stack.
	LowerHighest,
}

derive_xrb! {
	/// A [request] that [circulates] the mapped children of the given [window].
	///
	/// If some other client has selected [`SUBSTRUCTURE_REDIRECT`] on the
	/// `target` [window], then a [`CirculateWindowRequest` event] is generated
	/// and no further processing occurs. Otherwise, a [`Circulate` event] is
	/// generated if one of the [window]'s children is actually restacked.
	///
	/// # Errors
	/// A [`Window` error] is generated if the `target` does not refer to a
	/// defined [window].
	///
	/// [window]: Window
	/// [request]: Request
	///
	/// [circulates]: CirculateDirection
	///
	/// [`SUBSTRUCTURE_REDIRECT`]: crate::EventMask::SUBSTRUCTURE_REDIRECT
	///
	/// [`CirculateWindowRequest` event]: crate::x11::event::CirculateWindowRequest
	/// [`Circulate` event]: crate::x11::event::Circulate
	///
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct CirculateWindow: Request(13, CirculateWindowError) {
		#[metabyte]
		/// Which of the [window]'s children might be circulated and in which
		/// direction.
		///
		/// [window]: Window
		pub direction: CirculateDirection,

		/// The [window] which is the target of the `CirculateWindow` [request].
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		/// [request]: Request
		///
		/// [`Window error`]: error::Window
		#[doc(alias = "window")]
		pub target: Window,
	}

	/// A [request] that returns the root [window] and current geometry of the
	/// given [drawable].
	///
	/// It is legal to pass an [`InputOnly`] [window] as a [drawable] to this
	/// [request].
	///
	/// # Replies
	/// This [request] generates a [`GetGeometry` reply].
	///
	/// # Errors
	/// A [`Drawable` error] is generated if the `target` does not refer to a
	/// defined [window] nor [pixmap].
	///
	/// [window]: Window
	/// [pixmap]: crate::Pixmap
	/// [drawable]: Drawable
	/// [request]: Request
	///
	/// [`InputOnly`]: WindowClass::InputOnly
	///
	/// [`GetGeometry` reply]: reply::GetGeometry
	///
	/// [`Drawable` error]: error::Drawable
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct GetGeometry: Request(14, error::Drawable) -> reply::GetGeometry {
		/// The [drawable] for which this [request] gets its geometry.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// [window]: Window
		/// [pixmap]: crate::Pixmap
		/// [drawable]: Drawable
		/// [request]: Request
		///
		/// [`Drawable` error]: error::Drawable
		#[doc(alias = "drawable")]
		pub target: Drawable,
	}

	/// A [request] that returns the root [window], the parent, and the children
	/// of the given [window].
	///
	/// # Replies
	/// This [request] generates a [`QueryWindowTree` reply].
	///
	/// # Errors
	/// A [`Window` error] is generated if `target` does not refer to a defined
	/// [window].
	///
	/// [window]: Window
	/// [request]: Request
	///
	/// [`QueryWindowTree` reply]: reply::QueryWindowTree
	///
	/// [`Window` error]: error::Window
	#[doc(alias("QueryTree", "GetTree", "GetWindowTree"))]
	#[doc(alias("QueryParent", "QueryChildren", "QueryRoot"))]
	#[doc(alias("GetParent", "GetChildren", "GetRoot"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct QueryWindowTree: Request(15, error::Window) -> reply::QueryWindowTree {
		/// The [window] for which this [request] gets its root [window],
		/// parent, and children.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		/// [request]: Request
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,
	}
}
