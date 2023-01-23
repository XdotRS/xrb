// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Replies] defined in the [core X11 protocol].
//!
//! [Replies] are messages sent from the X server to an X client in response to
//! a [request].
//!
//! [Replies]: crate::message::Reply
//! [request]: crate::message::Request
//! [core X11 protocol]: super

use super::request;
use crate::{
	visual::{Color, VisualId},
	Atom,
	BitGravity,
	Colormap,
	DeviceEventMask,
	EventMask,
	MaintainContents,
	Rectangle,
	Window,
	WindowClass,
	WindowGravity,
};

use crate::unit::Px;
use xrbk_macro::{derive_xrb, Readable, Writable, X11Size};

extern crate self as xrb;

/// The state of the [window] regarding how it is mapped.
///
/// [window]: Window
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum MapState {
	/// The [window] is not mapped.
	///
	/// [window]: Window
	Unmapped,

	/// The [window] is mapped but one of its ancestors is unmapped.
	///
	/// [window]: Window
	Unviewable,

	/// The [window] is mapped and all of its ancestors are mapped.
	///
	/// [window]: Window
	Viewable,
}

derive_xrb! {
	/// The [reply] to a [`GetWindowAttributes` request].
	///
	/// [reply]: crate::message::Reply
	///
	/// [`GetWindowAttributes` request]: request::GetWindowAttributes
	#[derive(Debug, X11Size, Readable, Writable)]
	pub struct GetWindowAttributes: Reply for request::GetWindowAttributes {
		#[sequence]
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: crate::message::Reply
		///
		/// [`Reply::sequence`]: crate::message::Reply::sequence
		pub sequence: u16,

		#[metabyte]
		/// The conditions under which the X server should maintain the obscured
		/// [regions] of the [window].
		///
		/// See [`Attributes::maintain_contents`] for more information.
		///
		/// [regions]: crate::Region
		/// [window]: Window
		///
		/// [`Attributes::maintain_contents`]: crate::set::Attributes::maintain_contents
		pub maintain_contents: MaintainContents,

		/// The visual used by the [window].
		///
		/// See [`VisualType`] for more information.
		///
		/// [window]: Window
		///
		/// [`VisualType`]: crate::visual::VisualType
		pub visual: VisualId,
		/// The [window]'s [class].
		///
		/// [window]: Window
		/// [class]: WindowClass
		pub class: WindowClass,

		/// Defines the [region] of the [window] which is retained when the
		/// [window] is resized.
		///
		/// See [`Attributes::bit_gravity`] for more information.
		///
		/// [region]: crate::Region
		/// [window]: Window
		///
		/// [`Attributes::bit_gravity`]: crate::set::Attributes::bit_gravity
		pub bit_gravity: BitGravity,
		/// Defines how the [window] is repositioned if its parent is resized.
		///
		/// See [`Attributes::window_gravity`] for more information.
		///
		/// [window]: Window
		///
		/// [`Attributes::window_gravity`]: crate::set::Attributes::window_gravity
		pub window_graivty: WindowGravity,

		/// Defines which bit planes of the [window] hold dynamic data which is
		/// maintained for `maintain_contents` and `maintain_windows_under`.
		///
		/// See [`Attributes::maintained_planes`] for more information.
		///
		/// [window]: Window
		///
		/// [`Attributes::maintained_planes`]: crate::set::Attributes::maintained_planes
		pub maintained_planes: u32,
		/// Defines the [color] used for bit planes which are not preserved for
		/// `maintain_contents` and `maintain_windows_under` (see
		/// `maintained_planes`).
		///
		/// See [`Attributes::maintenance_fallback_color`] for more information.
		///
		/// [color]: Color
		///
		/// [`Attributes::maintenance_fallback_color`]: crate::set::Attributes::maintenance_fallback_color
		pub maintenance_fallback_color: Color,
		/// Whether the X server should maintain the contents of [windows] under
		/// this [window].
		///
		/// See [`Attributes::maintain_windows_under`] for more information.
		///
		/// [window]: Window
		///
		/// [`Attributes::maintain_windows_under`]: crate::set::Attributes::maintain_windows_under
		pub maintain_windows_under: bool,

		// TODO
		pub map_installed: bool,
		/// The [window]'s [map state].
		///
		/// See [`MapState`] for more information.
		///
		/// [window]: Window
		/// [map state]: MapState
		pub map_state: MapState,

		/// Whether [`MapWindow`] and [`ConfigureWindow`] requests on the
		/// [window] override a [`SUBSTRUCTURE_REDIRECT`] selection on its
		/// parent.
		///
		/// This is typically used to inform a window manager not to tamper with
		/// the [window].
		///
		/// See [`Attributes::override_redirect`] for more information.
		///
		/// [window]: Window
		///
		/// [`MapWindow`]: request::MapWindow
		/// [`ConfigureWindow`]: request::ConfigureWindow
		///
		/// [`SUBSTRUCTURE_REDIRECT`]: EventMask::SUBSTRUCTURE_REDIRECT
		///
		/// [`Attributes::override_redirect`]: crate::set::Attributes::override_redirect
		pub override_redirect: bool,

		/// The [colormap] which best reflects the true colors of the [window].
		///
		/// See [`Attributes::colormap`] for more information.
		///
		/// [window]: Window
		/// [colormap]: Colormap
		///
		/// [`Attributes::colormap`]: crate::set::Attributes::colormap
		pub colormap: Option<Colormap>,

		/// All of the [events] selected by all clients on the [window].
		///
		/// This is the bitwise OR of every client's [`event_mask`] on the
		/// [window].
		///
		/// [window]: Window
		/// [events]: crate::message::Event
		///
		/// [`event_mask`]: crate::set::Attributes::event_mask
		pub all_event_masks: EventMask,
		/// The [events] selected by you on the [window].
		///
		/// This is your [`event_mask`] on the [window].
		///
		/// [window]: Window
		/// [events]: crate::message::Event
		///
		/// [`event_mask`]: crate::set::Attributes::event_mask
		pub your_event_mask: EventMask,
		/// Defines the [events][event] which should not be propagated to
		/// ancestors of the [window] if no client has selected the [event] on
		/// the [window].
		///
		/// See [`Attributes::do_not_propagate_mask`] for more information.
		///
		/// [event]: crate::message::Event
		/// [window]: Window
		///
		/// [`Attributes::do_not_propagate_mask`]: crate::set::Attributes::do_not_propagate_mask
		pub do_not_propagate_mask: DeviceEventMask,
		[_; ..],
	}

	/// The [reply] to a [`GetGeometry` request].
	///
	/// [reply]: crate::message::Reply
	///
	/// [`GetGeometry` request]: request::GetGeometry
	#[derive(Debug, X11Size, Readable, Writable)]
	pub struct GetGeometry: Reply for request::GetGeometry {
		#[sequence]
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: crate::message::Reply
		///
		/// [`Reply::sequence`]: crate::message::Reply::sequence
		pub sequence: u16,

		#[metabyte]
		/// The number of bits per pixel for the [drawable].
		///
		/// [drawable]: crate::Drawable
		pub depth: u8,

		/// The [drawable]'s root [window].
		///
		/// [window]: Window
		/// [drawable]: crate::Drawable
		pub root: Window,

		/// The [drawable]'s geometry.
		///
		/// For a [pixmap], the `x` and `y` coordinates will always be zero.
		///
		/// For a [window], the coordinates are relative to the top-left corner
		/// of the [window]'s parent.
		///
		/// [window]: Window
		/// [pixmap]: create::Pixmap
		/// [drawable]: crate::Drawable
		pub geometry: Rectangle,
		/// The width of the [drawable]'s border.
		///
		/// For a [pixmap], this will always be zero.
		///
		/// [pixmap]: crate::Pixmap
		/// [drawable]: crate::Drawable
		pub border_width: Px<u16>,
		[_; ..],
	}

	/// The [reply] to a [`QueryWindowTree` request].
	///
	/// [reply]: crate::message::Reply
	///
	/// [`QueryWindowTree` request]: request::QueryWindowTree
	#[derive(Debug, X11Size, Readable, Writable)]
	pub struct QueryWindowTree: Reply for request::QueryWindowTree {
		#[sequence]
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: crate::message::Reply
		///
		/// [`Reply::sequence`]: crate::message::Reply::sequence
		pub sequence: u16,

		/// The `target` [window]'s root [window].
		///
		/// [window]: Window
		pub root: Window,
		/// The `target` [window]'s parent.
		///
		/// [window]: Window
		pub parent: Option<Window>,

		// The length of `children`.
		#[allow(clippy::cast_possible_truncation)]
		let children_len: u16 = children => children.len() as u16,
		[_; 14],

		/// The `target` [window]'s children.
		///
		/// [window]: Window
		#[context(children_len => usize::from(*children_len))]
		pub children: Vec<Window>,
	}

	/// The [reply] to a [`GetAtom` request].
	///
	/// [reply]: crate::message::Reply
	///
	/// [`GetAtom` request]: request::GetAtom
	#[derive(Debug, X11Size, Readable, Writable)]
	pub struct GetAtom: Reply for request::GetAtom {
		#[sequence]
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: crate::message::Reply
		///
		/// [`Reply::sequence`]: crate::message::Reply::sequence
		pub sequence: u16,

		/// The returned [atom].
		///
		/// If `no_creation` was set to `true` and an [atom] by the given `name`
		/// didn't already exist, this will be [`None`].
		///
		/// [atom]: Atom
		pub atom: Option<Atom>,
		[_; ..],
	}
}
