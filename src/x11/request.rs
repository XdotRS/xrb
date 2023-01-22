// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Requests] defined in the [core X11 protocol].
//!
//! [Requests] are messages sent from an X client to the X server.
//!
//! [Requests]: crate::message::Request
//! [core X11 protocol]: super

use crate::{
	set::Attributes,
	unit::Px,
	visual::VisualId,
	x11::error,
	CopyableFromParent,
	Drawable,
	Point,
	Rectangle,
	Window,
	WindowClass,
};

use crate::{set::WindowConfig, x11::reply};
use xrbk_macro::{derive_xrb, Readable, Writable, X11Size};

extern crate self as xrb;

/// An [error] generated because of a failed [`CreateWindow` request].
///
/// [error]: crate::message::Error
///
/// [`CreateWindow` request]: CreateWindow
pub enum CreateWindowError {
	/// A [`Colormap` error].
	///
	/// [`Colormap` error]: error::Colormap
	Colormap(error::Colormap),
	/// A [`CursorAppearance` error].
	///
	/// [`CursorAppearance` error]: error::CursorAppearance
	CursorAppearance(error::CursorAppearance),
	/// A [`ResourceIdChoice` error].
	///
	/// [`ResourceIdChoice` error]: error::ResourceIdChoice
	ResourceIdChoice(error::ResourceIdChoice),
	/// A [`Match` error].
	///
	/// [`Match` error]: error::Match
	Match(error::Match),
	/// A [`Pixmap` error].
	///
	/// [`Pixmap` error]: error::Pixmap
	Pixmap(error::Pixmap),
	/// A [`Value` error].
	///
	/// [`Value` error]: error::Value
	Value(error::Value),
	/// A [`Window` error].
	///
	/// [`Window` error]: error::Window
	Window(error::Window),
}

derive_xrb! {
	/// A [request] that creates a new unmapped [window] and assigns the
	/// provided [`Window` ID][window] to it.
	///
	/// [request]: crate::message::Request
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
		/// allocated to this client, a [`ResourceIdChoice` error] is generated.
		///
		/// [window]: Window
		///
		/// [`ResourceIdChoice` error]: error::ResourceIdChoice
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
		pub attributes: Attributes,
	}
}

/// An [error] generated because of a failed [`ChangeWindowAttributes` request].
///
/// [error]: crate::message::Error
///
/// [`ChangeWindowAttributes` request]: ChangeWindowAttributes
pub enum ChangeWindowAttributesError {
	/// An [`Access` error].
	///
	/// [`Access` error]: error::Access
	Access(error::Access),
	/// A [`Colormap` error].
	///
	/// [`Colormap` error]: error::Colormap
	Colormap(error::Colormap),
	/// A [`CursorAppearance` error].
	///
	/// [`CursorAppearance` error]: error::CursorAppearance
	CursorAppearance(error::CursorAppearance),
	/// A [`Match` error].
	///
	/// [`Match` error]: error::Match
	Match(error::Match),
	/// A [`Pixmap` error].
	///
	/// [`Pixmap` error]: error::Pixmap
	Pixmap(error::Pixmap),
	/// A [`Value` error].
	///
	/// [`Value` error]: error::Value
	Value(error::Value),
	/// A [`Window` error].
	///
	/// [`Window` error]: error::Window
	Window(error::Window),
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
	/// [request]: crate::message::Request
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
		pub target: Window,

		/// The [attributes] which are changed.
		///
		/// [attributes]: Attributes
		pub attributes: Attributes,
	}

	/// A [request] that returns the current [attributes] of the [window].
	///
	/// [request]: crate::message::Request
	/// [attributes]: Attributes
	/// [window]: Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct GetWindowAttributes: Request(3, error::Window) -> reply::GetWindowAttributes {
		/// The [window] for which this [request] gets the [attributes].
		///
		/// [request]: crate::message::Request
		/// [attributes]: Attributes
		/// [window]: Window
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
	/// [request]: crate::message::Request
	///
	/// [event]: super::event::Destroy
	///
	/// [`UnmapWindow` request]: UnmapWindow
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct DestroyWindow: Request(4, error::Window) {
		/// The [window] which is the target of the `DestroyWindow` [request].
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		/// [request]: crate::message::Request
		///
		/// [`Window` error]: error::Window
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
	/// [request]: crate::message::Request
	///
	/// [destroys]: DestroyWindow
	///
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
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
		pub target: Window,
	}
}

/// An [error] generated because of a failed [`ChangeSavedWindows` request].
///
/// [error]: crate::message::Error
///
/// [`ChangeSavedWindows` request]: ChangeSavedWindows
pub enum ChangeSavedWindowsError {
	/// A [`Match` error].
	///
	/// [`Match` error]: error::Match
	Match(error::Match),
	/// A [`Value` error].
	///
	/// [`Value` error]: error::Value
	Value(error::Value),
	/// A [`Window` error].
	///
	/// [`Window` error]: error::Window
	Window(error::Window),
}

/// Whether something is added or removed.
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum ChangeMode {
	/// The change is achieved by adding the thing.
	Add,
	/// The change is achieved by removing the thing.
	Remove,
}

derive_xrb! {
	/// A [request] that [adds] or [removes] the specified [window] from the
	/// set of [windows][window] which you have chosen to save.
	///
	/// When a client's resources are destroyed, each of the client's saved
	/// [windows] which are descendents of [windows] created by the client is
	/// [reparented] to the closest ancestor which is not created by the client.
	///
	/// # Errors
	/// The given `window` must not be a [window] created by you, else a
	/// [`Match` error] is generated.
	///
	/// A [`Window` error] is generated if the `window` does not refer to a
	/// defined [window].
	///
	/// A [`Value` error] is generated if the `change_mode` is encoded
	/// incorrectly. It is a bug in X Rust Bindings if that happens.
	///
	/// [window]: Window
	/// [windows]: Window
	/// [request]: crate::message::Request
	///
	/// [adds]: ChangeMode::Add
	/// [removes]: ChangeMode::Remove
	///
	/// [reparented]: ReparentWindow
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ChangeSavedWindows: Request(6, ChangeSavedWindowsError) {
		#[metabyte]
		/// Whether the `window` is added to or removed from your saved
		/// [windows].
		///
		/// [windows]: Window
		pub change_mode: ChangeMode,

		/// The [window] which is added to or removed from your saved
		/// [windows][window].
		///
		/// # Errors
		/// A [`Match` error] is generated if you created this [window].
		///
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [`Match` error]: error::Match
		/// [`Window` error]: error::Window
		pub window: Window,
	}
}

/// An [error] generated because of a failed [`ReparentWindow` request].
///
/// [error]: crate::message::Error
///
/// [`ReparentWindow` request]: ReparentWindow
pub enum ReparentWindowError {
	/// A [`Match` error].
	///
	/// [`Match` error]: error::Match
	Match(error::Match),
	/// A [`Window` error].
	///
	/// [`Window` error]: error::Window
	Window(error::Window),
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
	/// [request]: crate::message::Request
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
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
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
		pub new_parent: Window,

		/// The `target`'s new coordinates relative to its `new_parent`'s
		/// top-left corner.
		pub coords: Point,
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
	/// [request]: crate::message::Request
	///
	/// [`Map` event]: super::event::Map
	/// [`MapWindowRequest` event]: super::event::MapWindowRequest
	///
	/// [`override_redirect` attribute]: Attributes::override_redirect
	///
	/// [`SUBSTRUCTURE_REDIRECT`]: crate::EventMask::SUBSTRUCTURE_REDIRECT
	///
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct MapWindow: Request(8, error::Window) {
		/// The [window] which is the target of the `MapWindow` [request].
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		/// [request]: crate::message::Request
		///
		/// [`Window` error]: error::Window
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
	/// [request]: crate::message::Request
	///
	/// [maps]: MapWindow
	///
	///
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
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
	/// [request]: crate::message::Request
	///
	/// [`Unmap` event]: super::event::Unmap
	///
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct UnmapWindow: Request(10, error::Window) {
		/// The [window] which is the target of the `UnmapWindow` [request].
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		/// [request]: crate::message::Request
		///
		/// [`Window` error]: error::Window
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
	/// [request]: crate::message::Request
	///
	/// [unmaps]: UnmapWindow
	///
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
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
		pub target: Window,
	}
}

/// An [error] generated because of a failed [`ConfigureWindow` request].
///
/// [error]: crate::message::Error
///
/// [`ConfigureWindow` request]: ConfigureWindow
pub enum ConfigureWindowError {
	/// A [`Match` error].
	///
	/// [`Match` error]: error::Match
	Match(error::Match),
	/// A [`Value` error].
	///
	/// [`Value` error]: error::Value
	Value(error::Value),
	/// A [`Window` error].
	///
	/// [`Window` error]: error::Window
	Window(error::Window),
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
	/// [request]: crate::message::Request
	///
	/// [configures]: WindowConfig
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
		/// [request]: crate::message::Request
		///
		/// [`Window` error]: error::Window
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
		pub config: WindowConfig,
	}
}

/// An [error] generated because of a failed [`CirculateWindow` request].
///
/// [error]: crate::message::Error
///
/// [`CirculateWindow` request]: CirculateWindow
pub enum CirculateWindowError {
	/// A [`Value` error].
	///
	/// [`Value` error]: error::Value
	Value(error::Value),
	/// A [`Window` error].
	///
	/// [`Window` error]: error::Window
	Window(error::Window),
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
	/// [request]: crate::message::Request
	///
	/// [circulates]: CirculateDirection
	///
	/// [`SUBSTRUCTURE_REDIRECT`]: crate::EventMask::SUBSTRUCTURE_REDIRECT
	///
	/// [`CirculateWindowRequest` event]: super::event::CirculateWindowRequest
	/// [`Circulate` event]: super::event::Circulate
	///
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
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
		/// [request]: crate::message::Request
		///
		/// [`Window error]: error::Window
		pub target: Window,
	}

	/// A [request] that returns the root [window] and current geometry of the
	/// given [drawable].
	///
	/// It is legal to pass an [`InputOnly`] [window] as a [drawable] to this
	/// [request].
	///
	/// # Errors
	/// A [`Drawable` error] is generated if the `target` does not refer to a
	/// defined [window] nor [pixmap].
	///
	/// [window]: Window
	/// [pixmap]: crate::Pixmap
	/// [drawable]: Drawable
	/// [request]: crate::message::Request
	///
	/// [`InputOnly`]: WindowClass::InputOnly
	///
	/// [`Drawable` error]: error::Drawable
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
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
		/// [request]: crate::message::Request
		///
		/// [`Drawable` error]: Error::Drawable
		pub target: Drawable,
	}
}
