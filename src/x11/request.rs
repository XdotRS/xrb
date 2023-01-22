// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Requests] defined in the [core X11 protocol].
//!
//! [Requests] are messages sent from an X client to the X server.
//!
//! [Requests]: crate::message::Request
//! [core X11 protocol]: super

use crate::{unit::Px, visual::VisualId, CopyableFromParent, Rectangle, Window, WindowClass};

use crate::{set::Attributes, x11::error};
use xrbk_macro::derive_xrb;

extern crate self as xrb;

/// An error generated because of a failed [`CreateWindow` request].
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

/// An error generated because of a failed [`ChangeWindowAttributes` request].
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
	pub struct ChangeWindowAttributes: Request(2) {
		/// The [window] which the `attributes` are changed on.
		///
		/// [window]: Window
		pub window: Window,
		/// The [attributes] which are changed.
		///
		/// [attributes]: Attributes
		pub attributes: Attributes,
	}
}
