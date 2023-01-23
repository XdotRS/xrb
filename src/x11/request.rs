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
	message::Event,
	set::{Attributes, WindowConfig},
	unit::Px,
	visual::VisualId,
	x11::{error, reply},
	Any,
	Atom,
	CopyableFromParent,
	CurrentableTime,
	CursorAppearance,
	CursorEventMask,
	DestinationWindow,
	Drawable,
	EventMask,
	GrabMode,
	Point,
	Rectangle,
	String8,
	Window,
	WindowClass,
};
use xrbk::{
	Buf,
	BufMut,
	ConstantX11Size,
	ReadError,
	ReadError::UnrecognizedDiscriminant,
	ReadResult,
	ReadableWithContext,
	Wrap,
	Writable,
	WriteResult,
	X11Size,
};

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
		#[doc(alias = "values")]
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
		#[doc(alias = "values")]
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
		#[doc(alias = "x", alias = "y")]
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
		#[doc(alias = "values")]
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
	/// [pixmap]: crate::Pixmap
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
		/// [pixmap]: crate::Pixmap
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
	#[doc(alias = "QueryTree")]
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
	#[doc(alias = "InternAtom")]
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
	/// If the `change_mode` is [`Prepend`] or [`Append`], the `type` and
	/// `format` must match that of the existing property's value, else a
	/// [`Match` error] is generated.
	///
	/// [window]: Window
	/// [request]: crate::message::Request
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
		/// [`Prepend`]: ModifyPropertyMode::Prepend
		/// [`Append`]: ModifyPropertyMode::Append
		///
		/// [`Match` error]: error::Match
		#[doc(alias = "mode")]
		pub change_mode: ModifyPropertyMode,

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
	/// If the given `time` is earlier than time of the previous owner change or
	/// is later than the X server's [current time], this [request] has no
	/// effect.
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

		/// The time at which this change is recorded to occur at.
		///
		/// If this time is earlier than the server's current 'last-change' time
		/// for the selection's owner, or this time is later than the server's
		/// current time, this [request] has no effect.
		///
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

		/// The time at which this conversion is recorded as having taken place.
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
	/// Active [grabs] are ignored for this [request].
	///
	/// # Errors
	/// A [`Window` error] is generated if the `destination` is [`DestinationWindow::Other`] and the
	/// specified [window] is not defined.
	///
	/// [window]: Window
	/// [event]: Event
	/// [request]: crate::message::Request
	/// [grabs]: GrabMode
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
		/// grabbing client.
		///
		/// [events]: Event
		pub event_mask: CursorEventMask,

		/// The [grab mode] applied to the cursor.
		///
		/// For [`GrabMode::Asynchronous`], cursor [event] processing continues
		/// as normal.
		///
		/// For [`GrabMode::Synchronous`], cursor [event] processing appears to
		/// freeze - cursor [events][event] generated during this time are not
		/// lost: they are queued to be processed later. The freeze ends when
		/// either the grabbing client sends an [`AllowEvents` request], or when
		/// the cursor grab is released.
		///
		/// [event]: Event
		/// [grab mode]: GrabMode
		///
		/// [`AllowEvents` request]: AllowEvents
		#[doc(alias = "pointer_mode")]
		pub cursor_grab_mode: GrabMode,
		/// The [grab mode] applied to the keyboard.
		///
		/// For [`GrabMode::Asynchronous`], keyboard [event] processing
		/// continues as normal.
		///
		/// For [`GrabMode::Synchronous`], keyboard [event] processing appears
		/// to freeze - keyboard [events][event] generated during this time are
		/// not lost: they are queued to be processed later. The freeze ends
		/// when either the grabbing client sends an [`AllowEvents` request], or
		/// when the keyboard grab is released.
		///
		/// [event]: Event
		/// [grab mode]: GrabMode
		///
		/// [`AllowEvents` request]: AllowEvents
		#[doc(alias = "keyboard_mode")]
		pub keyboard_grab_mode: GrabMode,

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

		/// The time at which this grab is recorded as having initiated.
		pub time: CurrentableTime,
	}

	/// A [request] that ends a cursor grab if this client has it actively
	/// grabbed.
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
		/// The time at which the grab is recorded as having ceased.
		pub time: CurrentableTime,
	}
}
