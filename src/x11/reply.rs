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

use crate::{
	unit::Px,
	visual::{Color, VisualId},
	x11::request::{self, DataFormat, DataList},
	Atom,
	BitGravity,
	Colormap,
	DeviceEventMask,
	EventMask,
	GrabStatus,
	MaintainContents,
	Rectangle,
	String8,
	Window,
	WindowClass,
	WindowGravity,
};
use derivative::Derivative;

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
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetWindowAttributes: Reply for request::GetWindowAttributes {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: crate::message::Reply
		///
		/// [`Reply::sequence`]: crate::message::Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// The conditions under which the X server should maintain the obscured
		/// [regions] of the [window].
		///
		/// See [`Attributes::maintain_contents`] for more information.
		///
		/// [regions]: crate::Region
		/// [window]: Window
		///
		/// [`Attributes::maintain_contents`]: crate::set::Attributes::maintain_contents
		#[doc(alias = "backing_store")]
		#[metabyte]
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
		#[doc(alias = "win_gravity")]
		pub window_graivty: WindowGravity,

		/// Defines which bit planes of the [window] hold dynamic data which is
		/// maintained for `maintain_contents` and `maintain_windows_under`.
		///
		/// See [`Attributes::maintained_planes`] for more information.
		///
		/// [window]: Window
		///
		/// [`Attributes::maintained_planes`]: crate::set::Attributes::maintained_planes
		#[doc(alias = "backing_planes")]
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
		#[doc(alias = "backing_pixel")]
		pub maintenance_fallback_color: Color,
		/// Whether the X server should maintain the contents of [windows] under
		/// this [window].
		///
		/// See [`Attributes::maintain_windows_under`] for more information.
		///
		/// [window]: Window
		///
		/// [`Attributes::maintain_windows_under`]: crate::set::Attributes::maintain_windows_under
		#[doc(alias = "save_under")]
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
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetGeometry: Reply for request::GetGeometry {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: crate::message::Reply
		///
		/// [`Reply::sequence`]: crate::message::Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// The number of bits per pixel for the [drawable].
		///
		/// [drawable]: crate::Drawable
		#[metabyte]
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
		#[doc(alias = "x", alias = "y", alias = "width", alias = "height")]
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
	#[doc(alias = "QueryTree")]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct QueryWindowTree: Reply for request::QueryWindowTree {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: crate::message::Reply
		///
		/// [`Reply::sequence`]: crate::message::Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
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
	#[doc(alias = "InternAtom")]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetAtom: Reply for request::GetAtom {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: crate::message::Reply
		///
		/// [`Reply::sequence`]: crate::message::Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
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

	/// The [reply] to a [`GetAtomName` request].
	///
	/// [reply]: crate::message
	///
	/// [`GetAtomName` request]: request::GetAtomName
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetAtomName: Reply for request::GetAtomName {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: crate::message::Reply
		///
		/// [`Reply::sequence`]: crate::message::Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		// The length of `name`.
		#[allow(clippy::cast_possible_truncation)]
		let name_len: u16 = name => name.len() as u16,
		[_; 22],

		/// The name of the [atom].
		///
		/// [atom]: Atom
		#[context(name_len => usize::from(*name_len))]
		pub name: String8,
		[_; ..],
	}

	/// The [reply] to a [`GetProperty` request].
	///
	/// [reply]: crate::message::Reply
	///
	/// [`GetProperty` request]: request::GetProperty
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetProperty: Reply for request::GetProperty {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: crate::message::Reply
		///
		/// [`Reply::sequence`]: crate::message::Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// Whether the `value` is empty ([`None`]), or made up of `i8` values,
		/// `i16` values, or `i32` values.
		#[metabyte]
		pub format: Option<DataFormat>,

		/// The actual type of the property.
		pub r#type: Option<Atom>,
		/// The number of bytes remaining in the `property`'s data.
		///
		/// If the specified `property` does not exist for the `target`
		/// [window], this is zero.
		///
		/// If the specified `property` exists but its `type` does not match the
		/// specified type, this is the size of the property's data in bytes.
		///
		/// If the specified `property` exists and the type is either [`Any`] or
		/// matches the actual `type` of the property, this is the number of
		/// bytes remaining in the `property`'s data after the end of the
		/// returned `value`.
		///
		/// [window]: Window
		///
		/// [`Any`]: crate::Any::Any
		#[doc(alias = "bytes_after")]
		pub bytes_remaining: u32,

		// The length of `value`.
		#[allow(clippy::cast_possible_truncation)]
		let value_len: u32 = value => value.len() as u32,
		[_; 12],

		/// The property's value.
		///
		/// If `format` is [`None`], this will be [`DataList::I8`], but with an
		/// empty list.
		#[context(format, value_len => (format.unwrap_or(DataFormat::I8), *value_len))]
		pub value: DataList,
	}

	/// The [reply] for a [`ListProperties` request].
	///
	/// [reply]: crate::message::Reply
	///
	/// [`ListProperties` request]: request::ListProperties
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct ListProperties: Reply for request::ListProperties {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: crate::message::Reply
		///
		/// [`Reply::sequence`]: crate::message::Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		// The length of `properties`.
		#[allow(clippy::cast_possible_truncation)]
		let properties_len: u16 = properties => properties.len() as u16,
		[_; 22],

		/// The properties defined for the given [window].
		///
		/// [window]: Window
		#[doc(alias = "atoms")]
		#[context(properties_len => usize::from(*properties_len))]
		pub properties: Vec<Atom>,
	}

	/// The [reply] to a [`GetSelectionOwner` request].
	///
	/// [reply]: crate::message::Reply
	///
	/// [`GetSelectionOwner` request]: request::GetSelectionOwner
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetSelectionOwner: Reply for request::GetSelectionOwner {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: crate::message::Reply
		///
		/// [`Reply::sequence`]: crate::message::Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// The owner of the given `selection`.
		///
		/// If this is [`None`], then the selection has no owner.
		pub owner: Option<Window>,
		[_; ..],
	}

	/// The [reply] to a [`GrabCursor` request].
	///
	/// [reply]: crate::message::Reply
	///
	/// [`GrabCursor` request]: request::GrabCursor
	#[doc(alias = "GrabPointer")]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GrabCursor: Reply for request::GrabCursor {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: crate::message::Reply
		///
		/// [`Reply::sequence`]: crate::message::Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// The status of the attempted grab.
		///
		/// See [`GrabStatus`] for more information.
		#[doc(alias = "status")]
		#[metabyte]
		pub grab_status: GrabStatus,

		[_; ..],
	}
}
