// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Requests] defined in the [core X11 protocol].
//!
//! [Requests] are messages sent from an X client to the X server.
//!
//! [Requests]: crate::message::Request
//! [core X11 protocol]: super

extern crate self as xrb;

use xrbk::{
	Buf,
	BufMut,
	ConstantX11Size,
	ReadError,
	ReadError::UnrecognizedDiscriminant,
	ReadResult,
	Readable,
	ReadableWithContext,
	Wrap,
	Writable,
	WriteResult,
	X11Size,
};
use xrbk_macro::{derive_xrb, Readable, Writable, X11Size};

use crate::{
	message::Event,
	set::{Attributes, GraphicsOptions, WindowConfig},
	unit::Px,
	visual::VisualId,
	x11::{error, reply},
	Any,
	AnyModifierKeyMask,
	Atom,
	Button,
	Char16,
	Coords,
	CopyableFromParent,
	CurrentableTime,
	CursorAppearance,
	CursorEventMask,
	DestinationWindow,
	Drawable,
	EventMask,
	FocusWindow,
	Font,
	Fontable,
	FreezeMode,
	GraphicsContext,
	Keycode,
	LengthString8,
	Pixmap,
	Rectangle,
	String16,
	String8,
	Window,
	WindowClass,
};

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
	/// [`SUBSTRUCTURE_REDIRECT`]: EventMask::SUBSTRUCTURE_REDIRECT
	/// [`RESIZE_REDIRECT`]: EventMask::RESIZE_REDIRECT
	/// [`BUTTON_PRESS`]: EventMask::BUTTON_PRESS
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
	/// [request]: crate::message::Request
	///
	/// [destroys]: DestroyWindow
	///
	/// [`Window` error]: error::Window
	#[doc(alias = "DestroySubwindows")]
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
		#[doc(alias = "window")]
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
pub enum AddOrRemove {
	/// The thing is added.
	Add,
	/// The thing is removed.
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
	/// [adds]: AddOrRemove::Add
	/// [removes]: AddOrRemove::Remove
	///
	/// [reparented]: ReparentWindow
	#[doc(alias = "ChangeSaveSet")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ChangeSavedWindows: Request(6, ChangeSavedWindowsError) {
		#[metabyte]
		/// Whether the `window` is added to or removed from your saved
		/// [windows].
		///
		/// [windows]: Window
		#[doc(alias = "mode")]
		pub change_mode: AddOrRemove,

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
	/// [request]: crate::message::Request
	///
	/// [`Map` event]: super::event::Map
	/// [`MapWindowRequest` event]: super::event::MapWindowRequest
	///
	/// [`override_redirect` attribute]: Attributes::override_redirect
	///
	/// [`SUBSTRUCTURE_REDIRECT`]: EventMask::SUBSTRUCTURE_REDIRECT
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
	/// [request]: crate::message::Request
	///
	/// [maps]: MapWindow
	///
	///
	/// [`Window` error]: error::Window
	#[doc(alias = "MapSubwindows")]
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
	/// [request]: crate::message::Request
	///
	/// [unmaps]: UnmapWindow
	///
	/// [`Window` error]: error::Window
	#[doc(alias = "UnmapSubwindows")]
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
		#[doc(alias = "window")]
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
	/// [`SUBSTRUCTURE_REDIRECT`]: EventMask::SUBSTRUCTURE_REDIRECT
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
	/// [pixmap]: Pixmap
	/// [drawable]: Drawable
	/// [request]: crate::message::Request
	///
	/// [`InputOnly`]: WindowClass::InputOnly
	///
	/// [`GetGeometry` reply]: reply::GetGeometry
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
		/// [pixmap]: Pixmap
		/// [drawable]: Drawable
		/// [request]: crate::message::Request
		///
		/// [`Drawable` error]: Error::Drawable
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
	/// [request]: crate::message::Request
	///
	/// [`QueryWindowTree` reply]: reply::QueryWindowTree
	///
	/// [`Window` error]: error::Window
	#[doc(alias("QueryTree", "GetTree", "GetWindowTree"))]
	#[doc(alias("QueryParent", "QueryChildren", "QueryRoot"))]
	#[doc(alias("GetParent", "GetChildren", "GetRoot"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct QueryWindowTree: Request(15, error::Window) -> reply::QueryWindowTree {
		/// The [window] for which this [request] gets its root [window],
		/// parent, and children.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		/// [request]: crate::message::Request
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,
	}

	/// A [request] that returns the [atom] with the given `name`.
	///
	/// If `no_creation` is false and an [atom] by the specified `name` does not
	/// already exist, a new [atom] will be created and then returned. If an
	/// [atom] by the specified `name` already exists, that [atom] will be
	/// returned.
	///
	/// # Replies
	/// This [request] generates a [`GetAtom` reply].
	///
	/// [atom]: Atom
	/// [request]: crate::message::Request
	///
	/// [`GetAtom` reply]: reply::GetAtom
	#[doc(alias("InternAtom", "CreateAtom"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct GetAtom: Request(16, error::Value) -> reply::GetAtom {
		#[metabyte]
		/// Whether the X server should avoid creating a new [atom] for an
		/// unrecognised `name`.
		///
		/// If this is `true`, the X server won't create a new [atom] for a
		/// `name` which doesn't already refer to an [atom]. If it is `false`,
		/// the X server will create a new [atom] for the given `name`.
		///
		/// [atom]: Atom
		#[doc(alias = "only_if_exists")]
		pub no_creation: bool,

		// Encodes the length of `name`.
		#[allow(clippy::cast_possible_truncation)]
		let name_len: u16 = name => name.len() as u16,
		[_; 2],

		/// The name of the [atom] to either create or retrieve.
		///
		/// If an [atom] by this name does not already exist and `no_creation`
		/// is `false`, a new [atom] with this name will be created and
		/// returned.
		///
		/// If an [atom] by this name already exists, that [atom] will be
		/// returned.
		///
		/// [atom]: Atom
		#[context(name_len => usize::from(*name_len))]
		pub name: String8,
		[_; ..],
	}

	/// A [request] that returns the name of the given [atom].
	///
	/// # Replies
	/// This [request] generates a [`GetAtomName` reply].
	///
	/// # Errors
	/// An [`Atom` error] is generated if the `target` does not refer to a
	/// defined [atom].
	///
	/// [atom]: Atom
	/// [request]: crate::message::Request
	///
	/// [`GetAtomName` reply]: reply::GetAtomName
	///
	/// [`Atom` error]: error::Atom
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct GetAtomName: Request(17, error::Atom) -> reply::GetAtomName {
		/// The [atom] for which this [request] gets its name.
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		/// [request]: crate::message::Request
		///
		/// [`Atom` error]: error::Atom
		#[doc(alias = "atom")]
		pub target: Atom,
	}
}

/// An [error] generated because a [`ModifyProperty` request] failed.
///
/// [error]: crate::message::Error
///
/// [`ModifyProperty` request]: ModifyProperty
#[doc(alias = "ChangePropertyError")]
pub enum ModifyPropertyError {
	/// An [`Atom` error].
	///
	/// [`Atom` error]: error::Atom
	Atom(error::Atom),
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

/// Whether a property is [replaced], [prepended] to a [window]'s list of
/// properties, or [appended] to the [window]'s list of properties.
///
/// [replaced]: ModifyPropertyMode::Replace
/// [prepended]: ModifyPropertyMode::Prepend
/// [appended]: ModifyPropertyMode::Append
///
/// [window]: Window
#[doc(alias = "ChangePropertyMode")]
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum ModifyPropertyMode {
	/// The property replaces an existing property; the previous value is
	/// discarded.
	Replace,

	/// The property is prepended to the list of properties.
	Prepend,

	/// The property is appended to the list of properties.
	Append,
}

/// Whether a [`DataList`] is formatted as a list of `i8` values, `i16` values,
/// or `i32` values.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum DataFormat {
	/// The list is formatted as `i8` values.
	I8 = 8,
	/// The list is formatted as `i16` values.
	I16 = 16,
	/// The list is formatted as `i32` values.
	I32 = 32,
}

impl ConstantX11Size for DataFormat {
	const X11_SIZE: usize = 1;
}

impl Wrap for DataFormat {
	type Integer = u8;
}

impl TryFrom<u8> for DataFormat {
	type Error = ReadError;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			i8 if i8 == 8 => Ok(Self::I8),
			i16 if i16 == 16 => Ok(Self::I16),
			i32 if i32 == 32 => Ok(Self::I32),

			other => Err(UnrecognizedDiscriminant(usize::from(other))),
		}
	}
}

impl From<DataFormat> for u8 {
	fn from(format: DataFormat) -> Self {
		match format {
			DataFormat::I8 => 8,
			DataFormat::I16 => 16,
			DataFormat::I32 => 32,
		}
	}
}

/// A list of either `i8` values, `i16` values, or `i32` values.
///
/// This represents uninterpreted 'raw' data.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum DataList {
	/// A list of `i8` values.
	I8(Vec<i8>),
	/// A list of `i16` values.
	I16(Vec<i16>),
	/// A list of `i32` values.
	I32(Vec<i32>),
}

impl DataList {
	/// The length of the data.
	///
	/// This is how many values there are - not the number of bytes.
	#[must_use]
	pub fn len(&self) -> usize {
		match self {
			Self::I8(list) => list.len(),
			Self::I16(list) => list.len(),
			Self::I32(list) => list.len(),
		}
	}

	/// Whether the `DataList` is empty.
	#[must_use]
	pub fn is_empty(&self) -> bool {
		match self {
			Self::I8(list) => list.is_empty(),
			Self::I16(list) => list.is_empty(),
			Self::I32(list) => list.is_empty(),
		}
	}
}

impl X11Size for DataList {
	fn x11_size(&self) -> usize {
		match self {
			Self::I8(list) => list.x11_size(),
			Self::I16(list) => list.x11_size(),
			Self::I32(list) => list.x11_size(),
		}
	}
}

impl ReadableWithContext for DataList {
	type Context = (DataFormat, u32);

	fn read_with(buf: &mut impl Buf, (format, length): &(DataFormat, u32)) -> ReadResult<Self>
	where
		Self: Sized,
	{
		let length = &(*length as usize);

		Ok(match format {
			DataFormat::I8 => Self::I8(<Vec<i8>>::read_with(buf, length)?),
			DataFormat::I16 => Self::I16(<Vec<i16>>::read_with(buf, length)?),
			DataFormat::I32 => Self::I32(<Vec<i32>>::read_with(buf, length)?),
		})
	}
}

impl Writable for DataList {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		match self {
			Self::I8(list) => list.write_to(buf)?,
			Self::I16(list) => list.write_to(buf)?,
			Self::I32(list) => list.write_to(buf)?,
		}

		Ok(())
	}
}

derive_xrb! {
	/// A [request] that modifies the given `property` for the [window].
	///
	/// A [`Property` event] is generated on the `target` [window].
	///
	/// # Errors
	/// A [`Window` error] is generated if `target` does not refer to a defined
	/// [window].
	///
	/// An [`Atom` error] is generated if either `property` or `type` do not
	/// refer to defined [windows][window].
	///
	/// If the `modify_mode` is [`Prepend`] or [`Append`], the `type` and
	/// `format` must match that of the existing property's value, else a
	/// [`Match` error] is generated.
	///
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [`Prepend`]: ModifyPropertyMode::Prepend
	/// [`Append`]: ModifyPropertyMode::Append
	///
	/// [`Property` event]: super::event::Property
	///
	/// [`Window` error]: error::Window
	/// [`Atom` error]: error::Atom
	/// [`Match` error]: error::Match
	#[doc(alias = "ChangeProperty")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ModifyProperty: Request(18, ModifyPropertyError) {
		#[metabyte]
		/// The way in which the property is modified.
		///
		/// If the mode is [`Replace`], the previous property value is
		/// discarded.
		///
		/// If the mode is [`Prepend`], the data is prepended to the existing
		/// data. If the mode is [`Append`], the data is appended to the
		/// existing data.
		///
		/// # Errors
		/// If the mode is [`Prepend`] or [`Append`], the `type` and `format`
		/// must match that of the existing property's value, else a
		/// [`Match` error] is generated.
		///
		/// [window]: Window
		///
		/// [`Replace`]: ModifyPropertyMode::Replace
		/// [`Prepend`]: ModifyPropertyMode::Prepend
		/// [`Append`]: ModifyPropertyMode::Append
		///
		/// [`Match` error]: error::Match
		#[doc(alias = "mode")]
		pub modify_mode: ModifyPropertyMode,

		/// The [window] which the `property` is modified for.
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

		/// The property which is modified.
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		///
		/// [`Atom` error]: error::Atom
		pub property: Atom,
		/// The type of the property's data.
		///
		/// For example, if the property is of type [`Window`], then this would
		/// be [`atom::WINDOW`].
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		/// [`atom::WINDOW`]: crate::atom::WINDOW
		///
		/// [`Atom` error]: error::Atom
		pub r#type: Atom,

		// Whether the `data` is formatted as `i8` values, `i16` values, or
		// `i32` values.
		let format: DataFormat = data => match data {
			DataList::I8(_) => DataFormat::I8,
			DataList::I16(_) => DataFormat::I16,
			DataList::I32(_) => DataFormat::I32,
		},
		[_; 3],

		// The length of `data` in number of values (i.e., an `i32` value is
		// counted as one, rather than the number of bytes).
		#[allow(clippy::cast_possible_truncation)]
		let data_len: u32 = data => data.len() as u32,

		/// The property's value.
		///
		/// See [`DataList`] for information on the format of this data.
		#[context(format, data_len => (*format, *data_len))]
		pub data: DataList,
	}
}

/// An [error] generated because of a failed [`DeleteProperty` request].
///
/// [error]: crate::message::Error
///
/// [`DeleteProperty` request]: DeleteProperty
pub enum DeletePropertyError {
	/// An [`Atom` error].
	///
	/// [`Atom` error]: error::Atom
	Atom(error::Atom),
	/// A [`Window` error].
	///
	/// [`Window` error]: error::Window
	Window(error::Window),
}

derive_xrb! {
	/// A [request] that removes the given `property` from a [window].
	///
	/// If the `property` does not exist on the [window], this [request] has no
	/// effect. Otherwise, a [`Property` event] is generated on the [window].
	///
	/// # Errors
	/// A [`Window` error] is generated if the `target` does not refer to a
	/// defined [window].
	///
	/// An [`Atom` error] is generated if the `property` does not refer to a
	/// defined [atom].
	///
	/// [window]: Window
	/// [atom]: Atom
	/// [request]: crate::message::Request
	///
	/// [`Property` event]: super::event::Property
	///
	/// [`Window` error]: error::Window
	/// [`Atom` error]: error::Atom
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct DeleteProperty: Request(19, DeletePropertyError) {
		/// The [window] for which this [request] removes the `property`.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		/// [request]: crate::message::Request
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,

		/// The property which is to be removed from the `target` [window].
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		/// [window]: Window
		///
		/// [`Atom` error]: error::Atom
		pub property: Atom,
	}
}

/// An [error] generated because of a failed [`GetProperty` request].
///
/// [error]: crate::message::Error
///
/// [`GetProperty` request]: GetProperty
pub enum GetPropertyError {
	/// An [`Atom` error].
	///
	/// [`Atom` error]: error::Atom
	Atom(error::Atom),
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
	/// A [request] that gets the value of the given `property` on the given
	/// [window].
	///
	/// # Replies
	/// This [request] generates a [`GetProperty` reply].
	///
	/// # Errors
	/// A [`Window` error] is generated if `target` does not refer to a defined
	/// [window].
	///
	/// An [`Atom` error] is generated if either `property` or `type` do not
	/// refer to defined [atoms].
	///
	/// [window]: Window
	/// [request]: crate::message::Request
	/// [atoms]: Atom
	///
	/// [`GetProperty` reply]: reply::GetProperty
	///
	/// [`Window` error]: error::Window
	/// [`Atom` error]: error::Atom
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct GetProperty: Request(20, GetPropertyError) -> reply::GetProperty {
		/// Whether the `property` should be deleted from the `target` [window].
		///
		/// If the `type` matches the `property`'s actual type (or is [`Any`]),
		/// the property is removed from the [window]. Otherwise, this is
		/// ignored.
		///
		/// [window]: Window
		///
		/// [`Any`]: Any::Any
		#[metabyte]
		pub delete: bool,

		/// The [window] on which the requested `property` is found.
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

		/// The property for which this [request] gets its value.
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		/// [request]: crate::message::Request
		pub property: Atom,
		/// The property type to filter the [window]'s properties by.
		///
		/// This specifies that specifically a `property` of this type is
		/// requested. If the type does not match, the value is not provided in
		/// [the reply].
		///
		/// [window]: Window
		///
		/// [the reply]: reply::GetProperty
		pub r#type: Any<Atom>,

		/// The offset of the value of the `property` that is requested in
		/// 4-byte units.
		///
		/// This offset is multiplied by 4 when applied to the start of the
		/// `property`'s data.
		#[doc(alias = "long_offset")]
		pub offset: u32,
		/// The length of the value of the `property` that is requested in
		/// 4-byte units.
		///
		/// This length is multiplied by 4 and added to the `offset` to find the
		/// endpoint of the value that is requested.
		#[doc(alias = "long_length")]
		pub length: u32,
	}

	/// A [request] that returns the list of properties defined for the given
	/// [window].
	///
	/// # Replies
	/// This [request] generates a [`ListProperties` reply].
	///
	/// # Errors
	/// A [`Window` error] is generated if `target` does not refer to a defined
	/// [window].
	///
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [`ListProperties` reply]: reply::ListProperties
	///
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ListProperties: Request(21, error::Window) -> reply::ListProperties {
		/// The [window] for which this [request] returns its properties.
		///
		/// # Errors
		/// A [`Window` error] is returned if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		/// [request]: crate::message::Request
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,
	}
}

/// An [error] generated because of a failed [`SetSelectionOwner` request].
///
/// [error]: crate::message::Error
///
/// [`SetSelectionOwner` request]: SetSelectionOwner
pub enum SetSelectionOwnerError {
	/// An [`Atom` error].
	///
	/// [`Atom` error]: error::Atom
	Atom(error::Atom),
	/// A [`Window` error].
	///
	/// [`Window` error]: error::Window
	Window(error::Window),
}

derive_xrb! {
	/// A [request] that changes the owner of the given selection.
	///
	/// If the `new_owner` is different to the previous owner of the selection,
	/// and the previous owner was not [`None`], then a [`SelectionClear` event]
	/// is sent to the previous owner.
	///
	/// If the given `time` is earlier than the [time] of the previous owner
	/// change or is later than the X server's [current time], this [request]
	/// has no effect.
	///
	/// # Errors
	/// A [`Window` error] is generated if `owner` is [`Some`] but does not
	/// refer to a defined [window].
	///
	/// An [`Atom` error] is generated if `selection` does not refer to a
	/// defined [atom].
	///
	/// [window]: Window
	/// [atom]: Atom
	/// [time]: CurrentableTime
	/// [request]: crate::message::Request
	///
	/// [current time]: CurrentableTime::CurrentTime
	///
	/// [`SelectionClear` event]: super::event::SelectionClear
	///
	/// [`Window` error]: error::Window
	/// [`Atom` error]: error::Atom
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct SetSelectionOwner: Request(22, SetSelectionOwnerError) {
		/// Sets the new owner of the `selection`.
		///
		/// [`None`] specifies that the `selection` is to have no owner.
		///
		/// # Errors
		/// A [`Window` error] is generated if this is [`Some`] but does not
		/// refer to a defined [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "owner")]
		pub new_owner: Option<Window>,
		/// The selection for which this [request] changes its owner.
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		/// [request]: crate::message::Request
		///
		/// [`Atom` error]: error::Atom
		pub selection: Atom,

		/// The [time] at which this change is recorded to occur at.
		///
		/// If this [time] is earlier than the server's current 'last-change'
		/// [time] for the selection's owner, or this [time] is later than the
		/// server's [current time], this [request] has no effect.
		///
		/// [time]: CurrentableTime
		/// [current time]: CurrentableTime::CurrentTime
		/// [request]: crate::message::Request
		pub time: CurrentableTime,
	}

	/// A [request] that returns the owner of a given selection.
	///
	/// # Replies
	/// This [request] generates a [`GetSelectionOwner` reply].
	///
	/// # Errors
	/// An [`Atom` error] is generated if `target` does not refer to a defined
	/// [atom].
	///
	/// [atom]: Atom
	/// [request]: crate::message::Request
	///
	/// [`GetSelectionOwner` reply]: reply::GetSelectionOwner
	///
	/// [`Atom` error]: error::Atom
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct GetSelectionOwner: Request(23) -> reply::GetSelectionOwner {
		/// The selection for which this [request] returns its owner.
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		/// [request]: crate::message::Request
		///
		/// [`Atom` error]: error::Atom
		pub target: Atom,
	}
}

/// An [error] generated because of a failed [`ConvertSelection` request].
///
/// [error]: crate::message::Error
///
/// [`ConvertSelection` request]: ConvertSelection
pub enum ConvertSelectionError {
	/// An [`Atom` error].
	///
	/// [`Atom` error]: error::Atom
	Atom(error::Atom),
	/// A [`Window` error].
	///
	/// [`Window` error]: error::Window
	Window(error::Window),
}

derive_xrb! {
	/// A [request] that asks the given selection's owner to convert it to the
	/// given `target_type`.
	///
	/// # Errors
	/// A [`Window` error] is generated if `requester` does not refer to a
	/// defined [window].
	///
	/// An [`Atom` error] is generated if any `selection`, `target_type`, or
	/// `property` do not refer to defined [atoms].
	///
	/// [window]: Window
	/// [atoms]: Atom
	/// [request]: crate::message::Request
	///
	/// [`Window` error]: error::Window
	/// [`Atom` error]: error::Atom
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ConvertSelection: Request(24, ConvertSelectionError) {
		/// Your [window] which is requesting this conversion.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		pub requester: Window,

		/// The selection which this [request] asks to be converted.
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		/// [request]: crate::message::Request
		///
		/// [`Atom` error]: error::Atom
		pub selection: Atom,

		/// The type which the selection should be converted into.
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		///
		/// [`Atom` error]: error::Atom
		pub target_type: Atom,
		pub property: Option<Atom>,

		/// The [time] at which this conversion is recorded as having taken
		/// place.
		///
		/// [time]: CurrentableTime
		pub time: CurrentableTime,
	}
}

/// An [error] generated because of a failed [`SendEvent` request].
///
/// [error]: crate::message::Error
///
/// [`SendEvent` request]: SendEvent
pub enum SendEventError {
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
	/// A [request] that sends the given [event] to the given [window].
	///
	/// If the `event_mask` is empty, the [event] is sent to the client that
	/// created the [window] - if that client no longer exists, the [event] is
	/// not sent.
	///
	/// If `propagate` is `false`, the [event] is sent to every client selecting
	/// any of the [events][event] indicated in the `event_mask`.
	///
	/// If `propagate` is `true` and no clients have selected any of the
	/// [events][event] indicated in the `event_mask` on the [window], the
	/// [event] is sent to the closest ancestor [window] of the [window] which
	/// some client has selected at least one of the indicated [events][event]
	/// for (provided no [windows][window] between the original destination and
	/// the closest ancestor have that [event] in their
	/// [`do_not_propagate_mask`]). The [event] is sent to every client
	/// selecting any of the [events][event] indicated in the `event_mask` on
	/// the final destination.
	///
	/// Active grabs are ignored for this [request].
	///
	/// # Errors
	/// A [`Window` error] is generated if the `destination` is [`DestinationWindow::Other`] and the
	/// specified [window] is not defined.
	///
	/// [window]: Window
	/// [event]: Event
	/// [request]: crate::message::Request
	///
	/// [`do_not_propagate_mask`]: Attributes::do_not_propagate_mask
	///
	/// [`Window` error]: error::Window
	// FIXME: this requires that the event is absolutely 32 bytes, which is
	//        currently not bounded.
	//
	// This feature would be nice for this:
	// <https://github.com/rust-lang/rust/issues/92827>
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct SendEvent<E: Event>: Request(25, SendEventError) {
		/// Whether the `event` should be propagated to the closest appropriate
		/// ancestor, if necessary.
		///
		/// That is, whether the `event` should be propagated to the closest
		/// ancestor of the `destination` [window] which some client has
		/// selected any of the [events] indicated in the `event_mask` on if no
		/// clients have selected any of the [events] in the `event_mask` on the
		/// `destination` [window].
		///
		/// [window]: Window
		/// [events]: Event
		#[metabyte]
		pub propagate: bool,

		/// The destination [window] for the `event`.
		///
		/// [window]: Window
		pub destination: DestinationWindow,

		/// The mask of [events][event] which should be selected for the [event]
		/// to be sent to the selecting clients.
		///
		/// [event]: Event
		pub event_mask: EventMask,

		/// The [event] that is sent.
		///
		/// [event]: Event
		pub event: E,
	}
}

/// An [error] generated because of a failed [`GrabCursor` request].
///
/// [error]: crate::message::Error
///
/// [`GrabCursor` request]: GrabCursor
#[doc(alias = "GrabPointerError")]
pub enum GrabCursorError {
	/// A [`CursorAppearance` error].
	///
	/// [`CursorAppearance` error]: error::CursorAppearance
	#[doc(alias = "Cursor")]
	CursorAppearance(error::CursorAppearance),
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
	/// A [request] to actively grab control of the cursor.
	///
	/// This [request] generates [`EnterWindow`] and [`LeaveWindow`] events.
	///
	/// # Replies
	/// This [request] generates a [`GrabCursor` reply].
	///
	/// # Errors
	/// A [`Window` error] is generated if either the `grab_window` or the
	/// `confine_to` [window] do not refer to defined [windows][window].
	///
	/// A [`CursorAppearance` error] is generated if the `cursor_appearance` is
	/// [`Some`] and does not refer to a defined [cursor appearance].
	///
	/// [cursor appearance]: CursorAppearance
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [`EnterWindow`]: super::event::EnterWindow
	/// [`LeaveWindow`]: super::event::LeaveWindow
	/// [`GrabCursor` reply]: reply::GrabCursor
	///
	/// [`Window` error]: error::Window
	/// [`CursorAppearance` error]: error::CursorAppearance
	#[doc(alias = "GrabPointer")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct GrabCursor: Request(26, GrabCursorError) -> reply::GrabCursor {
		/// Whether cursor [events] which would normally be reported to this
		/// client are reported normally.
		///
		/// [events]: Event
		#[metabyte]
		pub owner_events: bool,

		/// The [window] on which the cursor is grabbed.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		pub grab_window: Window,

		/// A mask of the cursor [events] which are to be reported to the
		/// your client.
		///
		/// [events]: Event
		pub event_mask: CursorEventMask,

		/// The [freeze mode] applied to the cursor.
		///
		/// For [`FreezeMode::Unfrozen`], cursor [event] processing continues
		/// as normal.
		///
		/// For [`FreezeMode::Frozen`], cursor [event] processing appears to
		/// freeze - cursor [events][event] generated during this time are not
		/// lost: they are queued to be processed later. The freeze ends when
		/// either the grabbing client sends an [`AllowEvents` request], or when
		/// the cursor grab is released.
		///
		/// [event]: Event
		/// [freeze mode]: FreezeMode
		///
		/// [`AllowEvents` request]: AllowEvents
		#[doc(alias("pointer_mode", "cursor_mode"))]
		pub cursor_freeze: FreezeMode,
		/// The [freeze mode] applied to the keyboard.
		///
		/// For [`FreezeMode::Unfrozen`], keyboard [event] processing
		/// continues as normal.
		///
		/// For [`FreezeMode::Frozen`], keyboard [event] processing appears
		/// to freeze - keyboard [events][event] generated during this time are
		/// not lost: they are queued to be processed later. The freeze ends
		/// when either the grabbing client sends an [`AllowEvents` request], or
		/// when the keyboard grab is released.
		///
		/// [event]: Event
		/// [freeze mode]: FreezeMode
		///
		/// [`AllowEvents` request]: AllowEvents
		#[doc(alias = "keyboard_mode")]
		pub keyboard_freeze: FreezeMode,

		/// Optionally confines the cursor to the given [window].
		///
		/// This [window] does not need to have any relation to the
		/// `grab_window`.
		///
		/// The cursor will be warped to the closest edge of this [window] if it
		/// is not already within it. Subsequent changes to the configuration of
		/// the [window] which cause the cursor to be outside of the [window]
		/// will also trigger the cursor to be warped to the [window] again.
		///
		/// # Errors
		/// A [`Window` error] is generated if this is [`Some`] and does not
		/// refer to a defined [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		pub confine_to: Option<Window>,

		/// Optionally overrides the [appearance of the cursor], no matter which
		/// [window] it is within, for the duration of the grab.
		///
		/// # Errors
		/// A [`CursorAppearance` error] is generated if this does not refer to
		/// a defined [cursor appearance].
		///
		/// [cursor appearance]: CursorAppearance
		/// [appearance of the cursor]: CursorAppearance
		/// [window]: Window
		///
		/// [`CursorAppearance` error]: error::CursorAppearance
		#[doc(alias = "cursor")]
		pub cursor_appearance: Option<CursorAppearance>,

		/// The [time] at which this grab is recorded as having been initiated.
		///
		/// [time]: CurrentableTime
		pub time: CurrentableTime,
	}

	/// A [request] that ends an active cursor grab by your client.
	///
	/// Any queued [events] are released.
	///
	/// This [request] generates [`EnterWindow`] and [`LeaveWindow`] events.
	///
	/// [request]: crate::message::Request
	/// [events]: Event
	///
	/// [`EnterWindow`]: super::event::EnterWindow
	/// [`LeaveWindow`]: super::event::LeaveWindow
	#[doc(alias = "UngrabPointer")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct UngrabCursor: Request(27) {
		/// The [time] at which the grab is recorded as having been released.
		///
		/// [time]: CurrentableTime
		pub time: CurrentableTime,
	}
}

/// An [error] generated because of a failed [`GrabButton` request].
///
/// [error]: crate::message::Error
///
/// [`GrabButton` request]: GrabButton
pub enum GrabButtonError {
	/// An [`Access` error].
	///
	/// [`Access` error]: error::Access
	Access(error::Access),
	/// A [`CursorAppearance` error].
	///
	/// [`CursorAppearance` error]: error::CursorAppearance
	CursorAppearance(error::CursorAppearance),
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
	/// A [request] that establishes a passive cursor grab for a given `button`
	/// and `modifiers` combination.
	///
	/// If the following conditions are true, the grab is converted into an
	/// active cursor grab (as described in the [`GrabCursor` request]):
	/// - the cursor is not already actively grabbed; and
	/// - the specified `button` and specified `modifiers` are held; and
	/// - the cursor is within the `grab_window`; and
	/// - if the `confine_to` [window] is specified, it is viewable; and
	/// - a passive grab for the same `button` and `modifiers` combination does
	///   not exist for any ancestor of the `grab_window`.
	///
	/// # Errors
	/// A [`Window` error] is generated if either the `grab_window` or the
	/// `confine_to` [window] do not refer to defined [windows][window].
	///
	/// A [`CursorAppearance` error] is generated if the `cursor_appearance` is
	/// [`Some`] and does not refer to a defined [cursor appearance].
	///
	/// An [`Access` error] is generated if some other client has already sent a
	/// `GrabButton` [request] with the same `button` and `modifiers`
	/// combination on the same `grab_window`.
	///
	/// [cursor appearance]: CursorAppearance
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [`GrabCursor` request]: GrabCursor
	///
	/// [`Access` error]: error::Access
	/// [`Window` error]: error::Window
	/// [`CursorAppearance` error]: error::CursorAppearance
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct GrabButton: Request(28, GrabButtonError) {
		/// Whether cursor [events] which would normally be reported to this
		/// client are reported normally.
		///
		/// [events]: Event
		#[metabyte]
		pub owner_events: bool,

		/// The [window] on which the `button` is grabbed.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		pub grab_window: Window,

		/// A mask of the cursor [events] which are to be reported to the
		/// grabbing client.
		///
		/// [events]: Event
		pub event_mask: CursorEventMask,

		/// The [freeze mode] applied to the cursor.
		///
		/// For [`FreezeMode::Unfrozen`], cursor [event] processing continues
		/// as normal.
		///
		/// For [`FreezeMode::Frozen`], cursor [event] processing appears to
		/// freeze - cursor [events][event] generated during this time are not
		/// lost: they are queued to be processed later. The freeze ends when
		/// either the grabbing client sends an [`AllowEvents` request], or when
		/// the cursor grab is released.
		///
		/// [event]: Event
		/// [freeze mode]: FreezeMode
		///
		/// [`AllowEvents` request]: AllowEvents
		#[doc(alias("pointer_mode", "cursor_mode"))]
		pub cursor_freeze: FreezeMode,
		/// The [freeze mode] applied to the keyboard.
		///
		/// For [`FreezeMode::Unfrozen`], keyboard [event] processing
		/// continues as normal.
		///
		/// For [`FreezeMode::Frozen`], keyboard [event] processing appears
		/// to freeze - keyboard [events][event] generated during this time are
		/// not lost: they are queued to be processed later. The freeze ends
		/// when either the grabbing client sends an [`AllowEvents` request], or
		/// when the keyboard grab is released.
		///
		/// [event]: Event
		/// [freeze mode]: FreezeMode
		///
		/// [`AllowEvents` request]: AllowEvents
		#[doc(alias = "keyboard_mode")]
		pub keyboard_freeze: FreezeMode,

		/// Optionally confines the cursor to the given [window].
		///
		/// This [window] does not need to have any relation to the
		/// `grab_window`.
		///
		/// The cursor will be warped to the closest edge of this [window] if it
		/// is not already within it. Subsequent changes to the configuration of
		/// the [window] which cause the cursor to be outside of the [window]
		/// will also trigger the cursor to be warped to the [window] again.
		///
		/// # Errors
		/// A [`Window` error] is generated if this is [`Some`] and does not
		/// refer to a defined [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		pub confine_to: Option<Window>,

		/// Optionally overrides the [appearance of the cursor], no matter which
		/// [window] it is within, for the duration of the grab.
		///
		/// # Errors
		/// A [`CursorAppearance` error] is generated if this does not refer to
		/// a defined [cursor appearance].
		///
		/// [cursor appearance]: CursorAppearance
		/// [appearance of the cursor]: CursorAppearance
		/// [window]: Window
		///
		/// [`CursorAppearance` error]: error::CursorAppearance
		pub cursor_appearance: Option<CursorAppearance>,

		/// The [button] for which this grab is established.
		///
		/// [`Any`] means that the grab is effectively established for all
		/// possible [buttons][button].
		///
		/// When this button and the given `modifiers`,
		///
		/// [button]: Button
		///
		/// [`Any`]: Any::Any
		pub button: Any<Button>,
		_,

		/// The combination of modifiers which must be held for a press of the
		/// `button` to activate the active cursor grab.
		///
		/// [`ANY_MODIFIER`] means _any_ modifiers: that includes no modifiers
		/// at all.
		///
		/// [`ANY_MODIFIER`]: AnyModifierKeyMask::ANY_MODIFIER
		pub modifiers: AnyModifierKeyMask,
	}
}

/// An [error] generated because of a failed [`UngrabButton` request].
///
/// [error]: crate::message::Error
///
/// [`UngrabButton` request]: UngrabButton
pub enum UngrabButtonError {
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
	/// A [request] which releases a [passive button grab] by this client on the
	/// specified `grab_window` if the grab was established by your client.
	///
	/// # Errors
	/// A [`Window` error] is generated if `grab_window` does not refer to a
	/// defined [window].
	///
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [passive button grab]: GrabButton
	///
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct UngrabButton: Request(29, UngrabButtonError) {
		/// The [button] which the [passive button grab] was established for.
		///
		/// [`Any`] matches any `button` specified in the [passive button grab].
		/// It is equivalent to sending this `UngrabButton` [request] for all
		/// possible [buttons][button].
		///
		/// [button]: Button
		/// [request]: crate::message::Request
		///
		/// [passive button grab]: GrabButton
		///
		/// [`Any`]: Any::Any
		#[metabyte]
		pub button: Any<Button>,

		/// The [window] on which the [passive button grab] was established.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [passive button grab]: GrabButton
		///
		/// [`Window` error]: error::Window
		pub grab_window: Window,

		/// The modifier combination specified by the [passive button grab].
		///
		/// [`ANY_MODIFIER`] matches any `modifiers` specified in the
		/// [passive button grab] (including no modifiers). It is equivalent to
		/// sending this `UngrabButton` [request] for all possible `modifiers`
		/// combinations.
		///
		/// [request]: crate::message::Request
		///
		/// [passive button grab]: GrabButton
		///
		/// [`ANY_MODIFIER`]: AnyModifierKeyMask::ANY_MODIFIER
		pub modifiers: AnyModifierKeyMask,
		[_; 2],
	}
}

/// An [error] generated because of a failed [`ChangeActiveCursorGrab` request].
///
/// [error]: crate::message::Error
///
/// [`ChangeActiveCursorGrab` request]: ChangeActiveCursorGrab
#[doc(alias = "ChangeActivePointerGrabError")]
pub enum ChangeActiveCursorGrabError {
	/// A [`CursorAppearance` error].
	///
	/// [`CursorAppearance` error]: error::CursorAppearance
	CursorAppearance(error::CursorAppearance),
	/// A [`Value` error].
	///
	/// [`Value` error]: error::Value
	Value(error::Value),
}

derive_xrb! {
	/// A [request] which modifies the `event_mask` or `cursor_appearance` of an
	/// [active cursor grab].
	///
	/// # Errors
	/// A [`CursorAppearance` error] is generated if `cursor_appearance` does
	/// not refer to a defined [cursor appearance].
	///
	/// [cursor appearance]: CursorAppearance
	/// [request]: crate::message::Request
	///
	/// [active cursor grab]: GrabCursor
	///
	/// [`CursorAppearance` error]: error::CursorAppearance
	#[doc(alias = "ChangeActivePointerGrab")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ChangeActiveCursorGrab: Request(30, ChangeActiveCursorGrabError) {
		/// Optionally overrides the [appearance of the cursor], no matter which
		/// [window] it is within, for the duration of the grab.
		///
		/// This replaces the previously specified `cursor_appearance` for the
		/// grab - [`None`] means that the `cursor_appearance` is no longer
		/// overridden.
		///
		/// # Errors
		/// A [`CursorAppearance` error] is generated if this does not refer to
		/// a defined [cursor appearance].
		///
		/// [cursor appearance]: CursorAppearance
		/// [appearance of the cursor]: CursorAppearance
		/// [window]: Window
		///
		/// [`CursorAppearance` error]: error::CursorAppearance
		#[doc(alias = "cursor")]
		pub cursor_appearance: Option<CursorAppearance>,

		/// The [time] at which this change is recorded as having taken place.
		///
		/// This must be later than the [time] of the last cursor grab, and
		/// equal to or earlier than the X server's [current time].
		///
		/// [time]: CurrentableTime
		/// [current time]: CurrentableTime::CurrentTime
		pub time: CurrentableTime,

		/// A mask of the cursor [events] which are to be reported to the
		/// your client.
		///
		/// [events]: Event
		pub event_mask: CursorEventMask,
		[_; 2],
	}
}

/// An [error] generated because of a failed [`GrabKeyboard` request].
///
/// [error]: crate::message::Error
///
/// [`GrabKeyboard` request]: GrabKeyboard
pub enum GrabKeyboardError {
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
	/// A [request] to actively grab control of the keyboard.
	///
	/// This [request] generates [`Focus`] and [`Unfocus`] events.
	///
	/// # Replies
	/// This [request] generates a [`GrabKeyboard` reply].
	///
	/// # Errors
	/// A [`Window` error] is generated if the `grab_window` does not refer to a
	/// defined [window].
	///
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [`Focus`]: super::event::Focus
	/// [`Unfocus`]: super::event::Unfocus
	///
	/// [`GrabKeyboard` reply]: reply::GrabKeyboard
	///
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct GrabKeyboard: Request(31, GrabKeyboardError) -> reply::GrabKeyboard {
		/// Whether key [events] which would normally be reported to this client
		/// are reported normally.
		///
		/// Both [`KeyPress`] and [`KeyRelease`] events are always reported, no
		/// matter what events you have selected.
		///
		/// [events]: Event
		///
		/// [`KeyPress`]: super::event::KeyPress
		/// [`KeyRelease`]: super::event::KeyRelease
		#[metabyte]
		pub owner_events: bool,

		/// The [window] on which the keyboard is grabbed.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		pub grab_window: Window,

		/// The [time] at which this grab is recorded as having been initiated.
		///
		/// [time]: CurrentableTime
		pub time: CurrentableTime,

		/// The [freeze mode] applied to the cursor.
		///
		/// For [`FreezeMode::Unfrozen`], cursor [event] processing continues
		/// as normal.
		///
		/// For [`FreezeMode::Frozen`], cursor [event] processing appears to
		/// freeze - cursor [events][event] generated during this time are not
		/// lost: they are queued to be processed later. The freeze ends when
		/// either the grabbing client sends an [`AllowEvents` request], or when
		/// the cursor grab is released.
		///
		/// [event]: Event
		/// [freeze mode]: FreezeMode
		///
		/// [`AllowEvents` request]: AllowEvents
		#[doc(alias("pointer_mode", "cursor_mode"))]
		pub cursor_freeze: FreezeMode,
		/// The [freeze mode] applied to the keyboard.
		///
		/// For [`FreezeMode::Unfrozen`], keyboard [event] processing
		/// continues as normal.
		///
		/// For [`FreezeMode::Frozen`], keyboard [event] processing appears
		/// to freeze - keyboard [events][event] generated during this time are
		/// not lost: they are queued to be processed later. The freeze ends
		/// when either the grabbing client sends an [`AllowEvents` request], or
		/// when the keyboard grab is released.
		///
		/// [event]: Event
		/// [freeze mode]: FreezeMode
		///
		/// [`AllowEvents` request]: AllowEvents
		#[doc(alias = "keyboard_mode")]
		pub keyboard_freeze: FreezeMode,
		[_; 2],
	}

	/// A [request] that ends an active keyboard grab by your client.
	///
	/// Any queued [events] are released.
	///
	/// This [request] generates [`Focus`] and [`Unfocus`] events.
	///
	/// [request]: crate::message::Request
	/// [events]: Event
	///
	/// [`Focus`]: super::event::Focus
	/// [`Unfocus`]: super::event::Unfocus
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct UngrabKeyboard: Request(32) {
		/// The [time] at which the grab is recorded as having been released.
		///
		/// [time]: CurrentableTime
		pub time: CurrentableTime,
	}
}

/// An [error] generated because of a failed [`GrabKey` request].
///
/// [error]: crate::message::Error
///
/// [`GrabKey` request]: GrabKey
pub enum GrabKeyError {
	/// An [`Access` error].
	///
	/// [`Access` error]: error::Access
	Access(error::Access),
	/// A [`CursorAppearance` error].
	///
	/// [`CursorAppearance` error]: error::CursorAppearance
	CursorAppearance(error::CursorAppearance),
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
	/// A [request] that establishes a passive key grab for a particular `key`
	/// and `modifiers` combination.
	///
	/// If the following conditions are true, the grab is converted into an
	/// active keyboard grab (as described in the [`GrabKeyboard` request]):
	/// - the keyboard is not already actively grabbed; and
	/// - the specified `key` and specified `modifiers` are held; and
	/// - either the `grab_window` is an ancestor, or is, the currently focused
	///   [window], or the `grab_window` is a descendent of the currently
	///   focused [window] and contains the cursor; and
	/// - a passive grab for the same `key` and `modifiers` combination does
	///   not exist for any ancestor of the `grab_window`.
	///
	/// # Errors
	/// A [`Window` error] is generated if the `grab_window` does not refer to a
	/// defined [window].
	///
	/// An [`Access` error] is generated if some other client has already sent a
	/// `GrabKey` [request] with the same `key` and `modifiers` combination on
	/// the same `grab_window`.
	///
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [`GrabKeyboard` request]: GrabKeyboard
	///
	/// [`Access` error]: error::Access
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct GrabKey: Request(33, GrabKeyError) {
		/// Whether key [events] which would normally be reported to this client
		/// are reported normally.
		///
		/// Both [`KeyPress`] and [`KeyRelease`] events are always reported, no
		/// matter what events you have selected.
		///
		/// [events]: Event
		///
		/// [`KeyPress`]: super::event::KeyPress
		/// [`KeyRelease`]: super::event::KeyRelease
		#[metabyte]
		pub owner_events: bool,

		/// The [window] on which the `key` is grabbed.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		pub grab_window: Window,

		/// The combination of modifiers which must be held for a press of the
		/// `key` to activate the active key grab.
		///
		/// [`ANY_MODIFIER`] means _any_ modifiers: that includes no modifiers
		/// at all.
		///
		/// [`ANY_MODIFIER`]: AnyModifierKeyMask::ANY_MODIFIER
		pub modifiers: AnyModifierKeyMask,
		/// The key for which this grab is established.
		///
		/// [`Any`] means that the grab is effectively established for all
		/// possible keys.
		///
		/// When this key and the given `modifiers`,
		///
		/// [button]: Button
		///
		/// [`Any`]: Any::Any
		pub key: Any<Keycode>,

		/// The [freeze mode] applied to the cursor.
		///
		/// For [`FreezeMode::Unfrozen`], cursor [event] processing continues
		/// as normal.
		///
		/// For [`FreezeMode::Frozen`], cursor [event] processing appears to
		/// freeze - cursor [events][event] generated during this time are not
		/// lost: they are queued to be processed later. The freeze ends when
		/// either the grabbing client sends an [`AllowEvents` request], or when
		/// the cursor grab is released.
		///
		/// [event]: Event
		/// [freeze mode]: FreezeMode
		///
		/// [`AllowEvents` request]: AllowEvents
		#[doc(alias("pointer_mode", "cursor_mode"))]
		pub cursor_freeze: FreezeMode,
		/// The [freeze mode] applied to the keyboard.
		///
		/// For [`FreezeMode::Unfrozen`], keyboard [event] processing
		/// continues as normal.
		///
		/// For [`FreezeMode::Frozen`], keyboard [event] processing appears
		/// to freeze - keyboard [events][event] generated during this time are
		/// not lost: they are queued to be processed later. The freeze ends
		/// when either the grabbing client sends an [`AllowEvents` request], or
		/// when the keyboard grab is released.
		///
		/// [event]: Event
		/// [freeze mode]: FreezeMode
		///
		/// [`AllowEvents` request]: AllowEvents
		#[doc(alias = "keyboard_mode")]
		pub keyboard_freeze: FreezeMode,
		[_; 3],
	}
}

/// An [error] generated because of a failed [`UngrabKey` request].
///
/// [error]: crate::message::Error
///
/// [`UngrabKey` request]: UngrabKey
pub enum UngrabKeyError {
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
	/// A [request] which releases a [passive key grab] on the specified
	/// `grab_window` if the grab was established by your client.
	///
	/// # Errors
	/// A [`Window` error] is generated if `grab_window` does not refer to a
	/// defined [window].
	///
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [passive key grab]: GrabKey
	///
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct UngrabKey: Request(34, UngrabKeyError) {
		/// The key which the [passive key grab] was established for.
		///
		/// [`Any`] matches any `key` specified in the [passive key grab].
		/// It is equivalent to sending this `UngrabKey` [request] for all
		/// possible keys.
		///
		/// [request]: crate::message::Request
		///
		/// [passive key grab]: GrabKey
		///
		/// [`Any`]: Any::Any
		#[metabyte]
		pub key: Any<Keycode>,

		/// The [window] on which the [passive key grab] was established.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [passive key grab]: GrabKey
		///
		/// [`Window` error]: error::Window
		pub grab_window: Window,

		/// The modifier combination specified by the [passive key grab].
		///
		/// [`ANY_MODIFIER`] matches any `modifiers` specified in the
		/// [passive key grab] (including no modifiers). It is equivalent to
		/// sending this `UngrabKey` [request] for all possible `modifiers`
		/// combinations.
		///
		/// [request]: crate::message::Request
		///
		/// [passive key grab]: GrabKey
		///
		/// [`ANY_MODIFIER`]: AnyModifierKeyMask::ANY_MODIFIER
		pub modifiers: AnyModifierKeyMask,
		[_; 2],
	}
}

/// Specifies the conditions under which queued events should be released for an
/// [`AllowEvents` request].
///
/// [`AllowEvents` request]: AllowEvents
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum AllowEventsMode {
	/// Unfreezes the cursor if it is frozen and you have active grab on the
	/// cursor.
	UnfreezeCursor,
	/// Unfreezes the cursor, but freezes it again after the next
	/// [`ButtonPress`] or [`ButtonRelease`].
	///
	/// Your client must have an active grab on the cursor.
	///
	/// The cursor is frozen again specifically after the next [`ButtonPress`]
	/// [`ButtonRelease`] event reported to your client which does not cause
	/// grab to be released.
	///
	/// [`ButtonPress`]: super::event::ButtonPress
	/// [`ButtonRelease`]: super::event::ButtonRelease
	RefreezeCursor,
	/// If the cursor is frozen as a result of the activation of a passive grab
	/// or [`RefreezeCursor`] mode from your client, the grab is released and
	/// the [event] is completely reprocessed.
	///
	/// [`RefreezeCursor`]: AllowEventsMode::RefreezeCursor
	///
	/// [event]: Event
	ReplayCursor,

	/// Unfreezes the keyboard if it is frozen and you have an active grab on
	/// the keyboard.
	UnfreezeKeyboard,
	/// Unfreezes the keyboard, but freezes it again after the next
	/// [`KeyPress`] or [`KeyPress`].
	///
	/// Your client must have an active grab on the keyboard.
	///
	/// The keyboard is frozen again specifically after the next [`KeyPress`]
	/// [`KeyRelease`] event reported to your client which does not cause
	/// grab to be released.
	///
	/// [`KeyPress`]: super::event::KeyPress
	/// [`KeyRelease`]: super::event::KeyRelease
	RefreezeKeyboard,
	/// If the keyboard is frozen as a result of the activation of a passive
	/// grab or [`RefreezeKeyboard`] mode from your client, the grab is released
	/// and the [event] is completely reprocessed.
	///
	/// [`RefreezeKeyboard`]: AllowEventsMode::RefreezeKeyboard
	///
	/// [event]: Event
	ReplayKeyboard,

	/// If both the cursor and the keyboard are frozen by your client, both are
	/// unfrozen.
	UnfreezeBoth,
	/// If both the cursor and the keyboard are frozen by your client, both are
	/// unfrozen but are both frozen again on the next button or key press or
	/// release event.
	///
	/// Any [`ButtonPress`], [`ButtonRelease`], [`KeyPress`], or [`KeyRelease`]
	/// event reported to your client will unfreeze both the cursor and the
	/// keyboard.
	///
	/// [`ButtonPress`]: super::event::ButtonPress
	/// [`ButtonRelease`]: super::event::ButtonRelease
	///
	/// [`KeyPress`]: super::event::KeyPress
	/// [`KeyRelease`]: super::event::KeyRelease
	RefreezeBoth,
}

derive_xrb! {
	/// A [request] that releases some queued events if your client has caused a
	/// device to be [frozen].
	///
	/// [frozen]: FreezeMode::Frozen
	/// [request]: crate::message::Request
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct AllowEvents: Request(35, error::Value) {
		/// The conditions under which the queued [events] are released.
		///
		/// [events]: Event
		#[metabyte]
		pub mode: AllowEventsMode,

		/// The [time] at which this `AllowEvents` [request] is recorded as
		/// having taken place.
		///
		/// This [request] has no effect if this time is earlier than the time
		/// of your most recent active grab or later than the X server's
		/// [current time].
		///
		/// [request]: crate::message::Request
		/// [time]: CurrentableTime
		/// [current time]: CurrentableTime::CurrentTime
		pub time: CurrentableTime,
	}

	/// A [request] that freezes processing of [requests][request] and
	/// connection closes on all other clients' connections.
	///
	/// [request]: crate::message::Request
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct GrabServer: Request(36);

	/// A [request] that unfreezes processing of [requests][request] and
	/// connection closes on all other clients' connections.
	///
	/// [request]: crate::message::Request
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct UngrabServer: Request(37);

	/// A [request] that gets the current location of the cursor.
	///
	/// # Errors
	/// A [`Window` error] is generated if the `target` does not refer to a
	/// defined [window].
	///
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [`Window` error]: error::Window
	#[doc(alias("QueryPointer, QueryCursor, GetCursorPos, GetCursorLocation"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct QueryCursorLocation: Request(38, error::Window) -> reply::QueryCursorLocation {
		/// Specifies a [window] to receive relative coordinates of the cursor
		/// in relation to, if the cursor is on the same screen.
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
	}

	/// A [request] that returns the recorded cursor motion between the given
	/// `start` and `end` times.
	///
	/// The `start` and `end` times are inclusive.
	///
	/// # Errors
	/// A [`Window` error] is generated if `target` does not refer to a defined
	/// [window].
	///
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [`Window` error]: error::Window
	#[doc(alias = "GetMotionEvents")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct GetMotionHistory: Request(39, error::Window) -> reply::GetMotionHistory {
		/// The [window] for which the motion history is returned.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		pub target: Window,

		/// The start of the time period for which motion events are returned.
		///
		/// This is inclusive.
		pub start: CurrentableTime,
		/// The end of the time period for which motion events are returned.
		///
		/// This is inclusive.
		pub end: CurrentableTime,
	}

	/// A [request] that converts coordinates relative to the given `original`
	/// [window] to `output_coords` relative to the given `output` [window].
	///
	/// # Errors
	/// A [`Window` error] is generated if either `original` or `output` do not
	/// refer to defined [windows][window].
	///
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [`Window` error]: error::Window
	#[doc(alias = "TranslateCoordinates")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ConvertCoordinates: Request(40, error::Window) -> reply::ConvertCoordinates {
		/// The [window] which the `original_coords` are relative to.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		#[doc(alias("src_window", "source", "input"))]
		pub original: Window,
		/// The [window] which the `output_coords` will be relative to.
		///
		/// The `original_coords` are converted to coordinates relative to the
		/// top-left corner of this [window].
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		#[doc(alias("dst_window", "destination"))]
		pub output: Window,

		/// The coordinates, relative to the `original` [window]'s top-left
		/// corner, which will be converted.
		///
		/// These coordinates will be converted such that the `output_coords`
		/// are relative to the `output` [window].
		///
		/// [window]: Window
		pub original_coords: Coords,
	}
}

/// Represents dimensions within the `source` [window] of a
/// [`WarpCursor` request].
///
/// [window]: Window
///
/// [`WarpCursor` request]: WarpCursor
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum WarpSourceDimension {
	/// Set the `source_width` to the width of the `source` [window] minus the x
	/// coordinate or the `source_height` to the height of the `source` [window]
	/// minus the y coordinate.
	///
	/// [window]: Window
	FillRemaining,
	/// This specific width or height.
	Other(u16),
}

impl ConstantX11Size for WarpSourceDimension {
	const X11_SIZE: usize = 2;
}

impl X11Size for WarpSourceDimension {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for WarpSourceDimension {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(match buf.get_u16() {
			zero if zero == 0 => Self::FillRemaining,
			other => Self::Other(other),
		})
	}
}

impl Writable for WarpSourceDimension {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		match self {
			Self::FillRemaining => buf.put_u16(0),
			Self::Other(other) => other.write_to(buf)?,
		}

		Ok(())
	}
}

derive_xrb! {
	/// A [request] that instantly moves the cursor to a new location.
	///
	/// # Errors
	/// A [`Window` error] is generated if either the `source` or the `destination` are [`Some`] and
	/// do not refer to defined [windows].
	///
	/// [windows]: Window
	/// [request]: crate::message::Request
	///
	/// [`Window` error]: error::Window
	#[doc(alias = "WarpPointer")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct WarpCursor: Request(41, error::Window) {
		/// The [window] which the cursor is being warped from.
		///
		/// # Errors
		/// A [`Window` error] is generated if this is [`Some`] but does not
		/// refer to a defined [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		#[doc(alias("src", "src_window"))]
		pub source: Option<Window>,
		/// The [window] which the cursor is being warped to.
		///
		/// If this is [`None`], the cursor is simply offset by the `coords`. If
		/// this is [`Some`], the cursor is set to the `coords` relative to this
		/// [window].
		///
		/// # Errors
		/// A [`Window` error] is generated if this is [`SOme`] but does not
		/// refer to a defined [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		#[doc(alias("dst", "dst_window"))]
		pub destination: Option<Window>,

		/// The coordinates of the top-left corner of the rectangular area
		/// within the `source` [window] which the cursor must be within for it
		/// to be warped.
		///
		/// [window]: Window
		#[doc(alias("src_coords", "src_x", "src_y", "source_x", "source_y"))]
		pub source_coords: Coords,
		/// The width of the rectangular area within the `source` [window] which
		/// the cursor must be within for it to be warped.
		///
		/// [window]: Window
		#[doc(alias = "src_width")]
		pub source_width: WarpSourceDimension,
		/// The height of the rectangular area within the `source` [window]
		/// which the cursor must be within for it to be warped.
		///
		/// [window]: Window
		#[doc(alias = "src_height")]
		pub source_height: WarpSourceDimension,

		/// The coordinates applied to the cursor.
		///
		/// If `destination` is [`None`], the cursor is offset by these
		/// coordinates. Otherwise, the cursor is moved to these coordinates
		/// relative to the `destination` [window].
		///
		/// [window]: Window
		#[doc(alias("dst_x", "dst_y", "dst_coords", "destination_coords"))]
		pub coords: Coords,
	}
}

/// An [error] generated because of a failed [`SetFocus` request].
///
/// [error]: crate::message::Error
///
/// [`SetFocus` request]: SetFocus
#[doc(alias = "SetInputFocusError")]
pub enum SetFocusError {
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
	Window(error::Value),
}

/// What the focus should revert to if the focused [window] becomes unviewable.
///
/// This is used in the [`SetFocus` request].
///
/// [window]: Window
///
/// [`SetFocus` request]: SetFocus
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum RevertFocus {
	/// Revert the focus to no [window].
	///
	/// It is recommended to use [`CursorRoot`] in place of this, because at
	/// least the root [window] will have focus with [`CursorRoot`].
	///
	/// [`CursorRoot`]: RevertFocus::CursorRoot
	///
	/// [window]: Window
	None,

	/// Revert the focus to the root [window] which the cursor is on at the
	/// time.
	///
	/// [window]: Window
	CursorRoot,
	/// Revert the focus to the parent of the [window] which the cursor is in at
	/// the time.
	Parent,
}

derive_xrb! {
	/// A [request] which changes the current focus.
	///
	/// This [request] generates [`Focus`] and [`Unfocus`] events.
	///
	/// # Errors
	/// A [`Match` error] is generated of the specified `new_focus` is not
	/// viewable at the time of the [request].
	///
	/// A [`Window` error] is generated if `new_focus` is [`FocusWindow::Other`]
	/// and does not refer to a defined [window].
	///
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [`Focus`]: super::event::Focus
	/// [`Unfocus`]: super::event::Unfocus
	///
	/// [`Match` error]: error::Match
	/// [`Window` error]: error::Window
	#[doc(alias("SetInputFocus", "Focus", "FocusWindow"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct SetFocus: Request(42, SetFocusError) {
		/// What the focus should revert to if the focused [window] becomes
		/// unviewable.
		///
		/// [window]: Window
		#[metabyte]
		pub revert_to: RevertFocus,

		/// The new focus.
		///
		/// # Errors
		/// A [`Window` error] is generated if this is [`FocusWindow::Other`]
		/// but does not refer to a defined [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "focus")]
		pub new_focus: FocusWindow,

		/// The [time] at which the focus is recorded as having changed.
		pub time: CurrentableTime,
	}

	/// A [request] that returns the current focus.
	///
	/// [request]: crate::message::Request
	#[doc(alias = "GetInputFocus")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct GetFocus: Request(43) -> reply::GetFocus;

	/// A [request] that returns a bit vector of the currently held keys on the
	/// keyboard.
	///
	/// [request]: crate::message::Request
	#[doc(alias = "QueryKeymap")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct QueryKeyboard: Request(44) -> reply::QueryKeyboard;
}

/// An [error] generated because of a failed [`AssignFont` request].
///
/// [`AssignFont` request]: AssignFont
pub enum AssignFontError {
	/// A [`ResourceIdChoice` error].
	///
	/// [`ResourceIdChoice` error]: error::ResourceIdChoice
	ResourceIdChoice(error::ResourceIdChoice),
	/// A [`Name` error].
	///
	/// [`Name` error]: error::Name
	Name(error::Name),
}

derive_xrb! {
	/// A [request] that associates the font by the given `name` with the given
	/// `font_id`.
	///
	/// [request]: crate::message::Request
	#[doc(alias("OpenFont", "CreateFont", "LoadFont", "AddFont"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct AssignFont: Request(45, AssignFontError) {
		/// The [`Font` ID] to associate with the font specified by `name`.
		///
		/// [`Font` ID]: Font
		pub font_id: Font,

		// The length of `name`.
		#[allow(clippy::cast_possible_truncation)]
		let name_len: u16 = name => name.len() as u16,
		[_; 2],

		/// A pattern match against the name of the font.
		///
		/// The name uses ISO Latin-1 encoding.
		///
		/// The character `?` matches against any single character (equivalent
		/// to `.` in regular expressions) and `*` matches against any number of
		/// characters (like `.*` in regular expressions).
		#[context(name_len => usize::from(*name_len))]
		pub name: String8,
		[_; ..],
	}

	/// A [request] that removes the association between a given [`Font` ID] and
	/// the font it is associated with.
	///
	/// [request]: crate::message::Request
	/// [`Font` ID]: Font
	#[doc(alias("CloseFont", "DeleteFont", "UnloadFont", "RemoveFont"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct UnassignFont: Request(46) {
		/// The [`Font` ID] which is having its association with a font removed.
		///
		/// [`Font` ID]: Font
		pub target: Font,
	}

	/// A [request] that returns information about the given `target`
	/// font.
	///
	/// # Replies
	/// This [request] generates a [`QueryFont` reply].
	///
	/// # Errors
	/// A [`Font` error] is generated if the `target` does not refer to a
	/// defined [`Font`] nor [`GraphicsContext`].
	///
	/// [request]: crate::message::Request
	///
	/// [`GraphicsContext`]: GraphicsContext
	///
	/// [`QueryFont` reply]: reply::QueryFont
	///
	/// [`Font` error]: error::Font
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct QueryFont: Request(47, error::Font) -> reply::QueryFont {
		/// The font which this [request] returns information about.
		///
		/// # Errors
		/// A [`Font` error] is generated if this does not refer to a defined
		/// [`Font`] nor [`GraphicsContext`].
		///
		/// [request]: crate::message::Request
		///
		/// [`GraphicsContext`]: GraphicsContext
		///
		/// [`Font` error]: error::Font
		pub target: Fontable,
	}
}

/// A private function used in [`QueryTextExtents`] to determine padding.
#[inline]
const fn query_text_extents_padding(odd_length: bool) -> usize {
	if odd_length {
		2 // Char16::X11_SIZE
	} else {
		0
	}
}

derive_xrb! {
	/// A [request] that returns the extents of the given `text` when displayed
	/// with the given `font`.
	///
	/// If the font has no specified `fallback_character`, undefined characters
	/// in the `text` are ignored.
	///
	/// # Replies
	/// This [request] generates a [`QueryTextExtents` reply].
	///
	/// # Errors
	/// A [`Font` error] is generated if `font` does not refer to a defined
	/// [`Font`] nor [`GraphicsContext`].
	///
	/// [request]: crate::message::Request
	///
	/// [`GraphicsContext`]: GraphicsContext
	///
	/// [`QueryTextExtents` reply]: reply::QueryTextExtents
	///
	/// [`Font` error]: error::Font
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct QueryTextExtents: Request(48, error::Font) -> reply::QueryTextExtents {
		// Whether `text` is of odd length. Is it is, it has 2 bytes of padding
		// following it.
		#[metabyte]
		let odd_length: bool = text => text.len() % 2 != 0,

		/// The font used in the `text`.
		///
		/// # Errors
		/// A [`Font` error] is generated if this does not refer to a defined
		/// [`Font`] nor [`GraphicsContext`].
		///
		/// [`GraphicsContext`]: GraphicsContext
		///
		/// [`Font` error]: error::Font
		pub font: Fontable,

		/// The text for which this [request] gets the extents when displayed
		/// with `font`.
		///
		/// [request]: crate::message::Request
		#[context(self::length, odd_length => {
			// The length is in units of 4 bytes - we subtract the header, which is one of those
			// units, then multiply by 4 to get the total number of bytes.
			let length = usize::from(length - 1) * 4;
			// We then remove the size of the previous field, `font`, to get the bytes from the
			// start of `text` onwards.
			let length = length - Fontable::X11_SIZE;
			// We then remove the padding at the end, which can be determined from `odd_length`.
			let length = length - query_text_extents_padding(*odd_length);

			// We then divide the length, which is the number of bytes, by the number of bytes
			// per character.
			length / Char16::X11_SIZE
		})]
		pub text: String16,
		[_; odd_length => query_text_extents_padding(*odd_length)]
	}

	/// A [request] that lists the names of available fonts (as controlled by
	/// the [font search path]).
	///
	/// # Replies
	/// This [request] generates a [`ListFonts` reply].
	///
	/// [request]: crate::message::Request
	///
	/// [font search path]: SetFontSearchDirectories
	///
	/// [`ListFonts` reply]: reply::ListFonts
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ListFonts: Request(49) -> reply::ListFonts {
		/// The maximum number of names that will appear in the returned font
		/// `names`.
		#[doc(alias("max_names", "max_names_len"))]
		pub max_names_count: u16,

		#[allow(clippy::cast_possible_truncation)]
		let pattern_len: u16 = pattern => pattern.len() as u16,
		/// A pattern match against the font names.
		///
		/// The case (uppercase or lowercase) of the pattern does not matter:
		/// font names are converted to lowercase, as is the pattern.
		///
		/// Font names use ISO Latin-1 encoding.
		///
		/// The character `?` matches against any single character (equivalent
		/// to `.` in regular expressions) and `*` matches against any number of
		/// characters (like `.*` in regular expressions).
		#[context(pattern_len => usize::from(*pattern_len))]
		pub pattern: String8,
		[_; ..],
	}

	/// A [request] that lists available fonts (as controlled by the
	/// [font search path]) with information about them.
	///
	/// The information returned for each font almost entirely matches that
	/// returned in a [`QueryFont` reply].
	///
	/// # Replies
	/// This [request] generates [`ListFontsWithInfo` replies].
	///
	/// [request]: crate::message::Request
	///
	/// [font search path]: SetFontSearchDirectories
	///
	/// [`ListFontsWithInfo` replies]: reply::ListFontsWithInfo
	/// [`QueryFont` reply]: reply::QueryFont
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ListFontsWithInfo: Request(50) -> reply::ListFontsWithInfo {
		/// The maximum number of [`FontWithInfo` replies] that will be returned.
		///
		/// [`FontWithInfo` replies]: reply::FontWithInfo
		#[doc(alias("max_names", "max_names_len"))]
		pub max_fonts_count: u16,

		#[allow(clippy::cast_possible_truncation)]
		let pattern_len: u16 = pattern => pattern.len() as u16,
		/// A pattern match against the font names.
		///
		/// The case (uppercase or lowercase) of the pattern does not matter:
		/// font names are converted to lowercase, as is the pattern.
		///
		/// Font names use ISO Latin-1 encoding.
		///
		/// The character `?` matches against any single character (equivalent
		/// to `.` in regular expressions) and `*` matches against any number of
		/// characters (like `.*` in regular expressions).
		#[context(pattern_len => usize::from(*pattern_len))]
		pub pattern: String8,
		[_; ..],
	}

	/// A [request] that defines the directories which are searched for
	/// available fonts.
	///
	/// # Errors
	/// A [`Value` error] is generated if the operating system rejects the given
	/// paths for whatever reason.
	///
	/// [request]: crate::message::Request
	///
	/// [`Value` error]: error::Value
	#[doc(alias = "SetFontPath")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct SetFontSearchDirectories: Request(51, error::Value) {
		// The length of `directories`.
		#[allow(clippy::cast_possible_truncation)]
		let directories_len: u16 = directories => directories.len() as u16,
		[_; 2],

		/// The directories to be searched in the order listed.
		///
		/// Specifying an empty list here restores the default font search path
		/// defined for the X server.
		#[doc(alias = "path")]
		#[context(directories_len => usize::from(*directories_len))]
		pub directories: Vec<LengthString8>,
		[_; ..],
	}

	/// A [request] that returns the current directories which are searched to
	/// find available fonts.
	///
	/// See also: [`SetFontSearchDirectories`].
	///
	/// [request]: crate::message::Request
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct GetFontSearchDirectories: Request(52) -> reply::GetFontSearchDirectories;
}

/// An [error] generated because of a failed [`CreatePixmap` request].
///
/// [error]: crate::message::Error
///
/// [`CreatePixmap` request]: CreatePixmap
pub enum CreatePixmapError {
	/// A [`Drawable` error].
	///
	/// [`Drawable` error]: error::Drawable
	Drawable(error::Drawable),
	/// A [`ResourceIdChoice` error].
	///
	/// [`ResourceIdChoice` error]: error::ResourceIdChoice
	ResourceIdChoice(error::ResourceIdChoice),
	/// A [`Value` error].
	///
	/// [`Value` error]: error::Value
	Value(error::Value),
}

derive_xrb! {
	/// A [request] that creates a new [pixmap] and assigns the provided
	/// [`Pixmap` ID][pixmap] to it.
	///
	/// The initial contents of the [pixmap] are undefined.
	///
	/// # Errors
	/// A [`Value` error] is generated if `depth` is not a depth supported by
	/// the `drawable`'s root [window].
	///
	/// A [`ResourceIdChoice` error] is generated if `pixmap_id` specifies an ID
	/// already used for another resource, or an ID not allocated to your
	/// client.
	///
	/// A [`Drawable` error] is generated if `drawable` does not refer to a
	/// defined [window] nor [pixmap].
	///
	/// [window]: Window
	/// [pixmap]: Pixmap
	/// [request]: crate::message::Request
	///
	/// [`Drawable` error]: error::Drawable
	/// [`ResourceIdChoice` error]: error::ResourceIdChoice
	/// [`Value` error]: error::Value
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct CreatePixmap: Request(53, CreatePixmapError) {
		/// The depth of the [pixmap].
		///
		/// # Errors
		/// A [`Value` error] is generated if this depth is not supported by the
		/// root [window] of the `drawable`.
		///
		/// [pixmap]: Pixmap
		/// [window]: Window
		///
		/// [`Value` error]: error::Value
		#[metabyte]
		pub depth: u8,

		/// The [`Pixmap` ID][pixmap] which is to be assigned to the [pixmap].
		///
		/// # Errors
		/// A [`ResourceIdChoice` error] is generated if this resource ID is
		/// already used or if it isn't allocated to your client.
		///
		/// [pixmap]: Pixmap
		///
		/// [`ResourceIdChoice` error]: error::ResourceIdChoice
		#[doc(alias = "pid")]
		pub pixmap_id: Pixmap,
		// TODO: what is this for??
		/// It is legal to use an [`InputOnly`] [window] as a [drawable] in this
		/// [request].
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// [window]: Window
		/// [pixmap]: Pixmap
		/// [drawable]: Drawable
		/// [request]: crate::message::Request
		///
		/// [`InputOnly`]: WindowClass::InputOnly
		///
		/// [`Drawable` error]: error::Drawable
		pub drawable: Drawable,

		/// The width of the [pixmap].
		///
		/// [pixmap]: Pixmap
		pub width: Px<u16>,
		/// The height of the [pixmap].
		///
		/// [pixmap]: Pixmap
		pub height: Px<u16>,
	}

	/// A [request] that removes the association between a given
	/// [`Pixmap` ID][pixmap] and the [pixmap] it is associated with.
	///
	/// The stored [pixmap] will be freed when it is no longer referenced by any
	/// other resource.
	///
	/// # Errors
	/// A [`Pixmap` error] is generated if `target` does not refer to a defined
	/// [pixmap].
	///
	/// [pixmap]: Pixmap
	/// [request]: crate::message::Request
	///
	/// [`Pixmap` error]: error::Pixmap
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct FreePixmap: Request(54, error::Pixmap) {
		/// The [pixmap] which is to have its association with its ID removed.
		///
		/// # Errors
		/// A [`Pixmap` error] is generated if this does not refer to a defined
		/// [pixmap].
		///
		/// [pixmap]: Pixmap
		///
		/// [`Pixmap` error]: error::Pixmap
		#[doc(alias = "pixmap")]
		pub target: Pixmap,
	}
}

/// An [error] generated because of a failed [`CreateGraphicsContext` request].
///
/// [error]: crate::message::Error
///
/// [`CreateGraphicsContext` request]: CreateGraphicsContext
pub enum CreateGraphicsContextError {
	/// A [`Drawable` error].
	///
	/// [`Drawable` error]: error::Drawable
	Drawable(error::Drawable),
	/// A [`Font` error].
	///
	/// [`Font` error]: error::Font
	Font(error::Font),
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
}

derive_xrb! {
	/// A [request] that creates a new [`GraphicsContext`] and assigns the
	/// provided [`GraphicsContext` ID] to it.
	///
	/// # Errors
	/// A [`ResourceIdChoice` error] is generated if `graphics_context_id`
	/// specifies an ID already used for another resource, or an ID which is not
	/// allocated to your client.
	///
	/// A [`Drawable` error] is generated if `source` does not refer to a
	/// defined [window] nor [pixmap].
	///
	/// [pixmap]: Pixmap
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [`GraphicsContext` ID]: GraphicsContext
	///
	/// [`ResourceIdChoice` error]: error::ResourceIdChoice
	/// [`Drawable` error]: error::Drawable
	#[doc(alias("CreateGc", "CreateGC", "CreateGcontext", "CreateGContext"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct CreateGraphicsContext: Request(55, CreateGraphicsContextError) {
		/// The [`GraphicsContext` ID] which is to be assigned to the
		/// [`GraphicsContext`].
		///
		/// # Errors
		/// A [`ResourceIdChoice` error] is generated if this resource ID is
		/// already used or if it isn't allocated to your client.
		///
		/// [`GraphicsContext` ID]: GraphicsContext
		///
		/// [`ResourceIdChoice` error]: error::ResourceIdChoice
		pub graphics_context_id: GraphicsContext,

		/// The [drawable] used as the source in graphics operations.
		///
		/// See also: [`Function`].
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// [drawable]: Drawable
		/// [window]: Window
		/// [pixmap]: Pixmap
		///
		/// [`Function`]: crate::set::Function
		///
		/// [`Drawable` error]: error::Drawable
		pub source: Drawable,

		/// The [graphics options] used in graphics operations when this
		/// [`GraphicsContext`] is provided.
		///
		/// [graphics options]: GraphicsOptions
		pub graphics_options: GraphicsOptions,
	}
}

/// An [error] generated because of a failed [`ChangeGraphicsOptions` request].
///
/// [error]: crate::message::Error
///
/// [`ChangeGraphicsOptions` request]: ChangeGraphicsOptions
#[doc(alias("ChangeGcError", "ChangeGCError", "ChangeGraphicsContextError"))]
pub enum ChangeGraphicsOptionsError {
	/// A [`Font` error].
	///
	/// [`Font` error]: error::Font
	Font(error::Font),
	/// A [`GraphicsContext` error].
	///
	/// [`GraphicsContext` error]: error::GraphicsContext
	GraphicsContext(error::GraphicsContext),
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
}

derive_xrb! {
	/// A [request] that changes the [graphics options] configured in a
	/// [`GraphicsContext`].
	///
	/// Changing the [`clip_mask`] overrides any [`SetClipRectangles` request]
	/// on the [`GraphicsContext`].
	///
	/// Changing [`dash_offset`] or [`dashes`] overrides any
	/// [`SetDashes` request] on the [`GraphicsContext`].
	///
	/// # Errors
	/// A [`GraphicsContext` error] is generated if `target` does not refer to a
	/// defined [`GraphicsContext`].
	///
	/// [request]: crate::message::Request
	///
	/// [graphics options]: GraphicsOptions
	///
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	/// [`dashes`]: GraphicsOptions::dashes
	///
	/// [`SetClipRectangles` request]: SetClipRectangles
	/// [`SetDashes` request]: SetDashes
	///
	/// [`GraphicsContext` error]: error::GraphicsContext
	#[doc(alias("ChangeGc", "ChangeGC", "ChangeGraphicsContext"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ChangeGraphicsOptions: Request(56, ChangeGraphicsOptionsError) {
		/// The [`GraphicsContext`] for which this [request] changes its
		/// [graphics options].
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// [graphics options]: GraphicsOptions
		/// [request]: crate::message::Request
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		pub target: GraphicsContext,

		/// The changes which are made to the `target`'s [graphics options].
		///
		/// [graphics options]: GraphicsOptions
		pub changed_options: GraphicsOptions,
	}
}
