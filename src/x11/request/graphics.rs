// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Requests] defined in the [core X11 protocol] that relate to graphics
//! operations.
//!
//! [Requests] are messages sent from an X client to the X server.
//!
//! [Requests]: crate::message::Request
//! [core X11 protocol]: crate::x11

extern crate self as xrb;

use xrbk::{pad, ConstantX11Size};
use xrbk_macro::{derive_xrb, ConstantX11Size, Readable, Writable, X11Size};

use crate::{
	set::{GraphicsOptions, GraphicsOptionsMask},
	unit::Px,
	x11::error,
	Arc,
	Coords,
	Dimensions,
	Drawable,
	GraphicsContext,
	Pixmap,
	Rectangle,
	Window,
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
	pub enum CreatePixmapError for CreatePixmap {
		Drawable,
		ResourceIdChoice,
		Value,
	}
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
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
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
		/// [`InputOnly`]: crate::WindowClass::InputOnly
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
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
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

request_error! {
	pub enum CreateGraphicsContextError for GraphicsContext {
		Drawable,
		Font,
		ResourceIdChoice,
		Match,
		Pixmap,
		Value,
	}
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
	/// A [`Drawable` error] is generated if `drawable` does not refer to a
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
		#[doc(alias("cid", "gid", "gcid", "context_id"))]
		pub graphics_context_id: GraphicsContext,

		/// > *<sup>TODO</sup>*\
		/// > ***We don't yet understand what this field is for. If you have any
		/// > ideas, please feel free to open an issue or a discussion on the
		/// > [GitHub repo]!***
		/// >
		/// > [GitHub repo]: https://github.com/XdotRS/xrb/
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// [drawable]: Drawable
		/// [window]: Window
		/// [pixmap]: Pixmap
		///
		/// [`Drawable` error]: error::Drawable
		pub drawable: Drawable,

		/// The [graphics options] used in graphics operations when this
		/// [`GraphicsContext`] is provided.
		///
		/// These [graphics options] may be later configured through the
		/// [`SetDashes` request], [`SetClipRectangles` request], and the
		/// [`ChangeGraphicsOptions` request].
		///
		/// [graphics options]: GraphicsOptions
		///
		/// [`SetDashes` request]: SetDashes
		/// [`SetClipRectangles` request]: SetClipRectangles
		/// [`ChangeGraphicsOptions` request]: ChangeGraphicsOptions
		#[doc(alias("values", "value_mask", "value_list"))]
		#[doc(alias("options", "option_mask", "option_list"))]
		#[doc(alias("graphics_option_mask", "graphics_option_list"))]
		pub graphics_options: GraphicsOptions,
	}
}

request_error! {
	pub enum ChangeGraphicsOptionsError for ChangeGraphicsOptions {
		Font,
		GraphicsContext,
		Match,
		Pixmap,
		Value,
	}
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
	/// The [`SetDashes` request] allows an alternating pattern of dashes to be
	/// configured, while configuring [`dashes`] with this [request] does not.
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
	/// [`dash_offset`]: GraphicsOptions::dash_offset
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
		#[doc(alias("gc", "graphics_context", "context"))]
		pub target: GraphicsContext,

		/// The changes which are made to the `target`'s [graphics options].
		///
		/// [graphics options]: GraphicsOptions
		#[doc(alias("values", "value_mask", "value_list"))]
		#[doc(alias("options", "option_mask", "option_list"))]
		#[doc(alias("graphics_option_mask", "graphics_option_list"))]
		pub changed_options: GraphicsOptions,
	}
}

request_error! {
	pub enum CopyGraphicsOptionsError for CopyGraphicsOptions {
		GraphicsContext,
		Match,
		Value,
	}
}

derive_xrb! {
	/// A [request] that copies the specified [graphics options] from the
	/// `source` [`GraphicsContext`] into the `destination` [`GraphicsContext`].
	///
	/// # Errors
	/// A [`GraphicsContext` error] is generated if either the `source` or the
	/// `destination` do not refer to defined [`GraphicsContext`s].
	///
	/// A [`Match` error] is generated if the `source` and the `destination` do
	/// not have the same root [window] and depth.
	///
	/// [graphics options]: GraphicsOptions
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [`GraphicsContext`s]: GraphicsContext
	///
	/// [`GraphicsContext` error]: error::GraphicsContext
	/// [`Match` error]: error::Match
	#[doc(alias("CopyGc", "CopyGC", "CopyGraphicsContext", "CopyGcontext"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct CopyGraphicsOptions: Request(57, CopyGraphicsOptionsError) {
		/// The [`GraphicsContext`] from which the [options] specified in
		/// `options_mask` are copied.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// A [`Match` error] is generated if this does not have the same root
		/// [window] and depth as the `destination`.
		///
		/// [options]: GraphicsOptions
		/// [window]: Window
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		/// [`Match` error]: error::Match
		#[doc(alias("src_gc", "source_graphics_context", "src"))]
		pub source: GraphicsContext,
		/// The [`GraphicsContext`] into which the [options] specified in
		/// `options_mask` are copied from the `source`.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// A [`Match` error] is generated if this does not have the same root
		/// [window] and depth as the `source`.
		///
		/// [options]: GraphicsOptions
		/// [window]: Window
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		/// [`Match` error]: error::Match
		#[doc(alias("dst_gc", "destination_graphics_context", "dst"))]
		pub destination: GraphicsContext,

		/// A mask that specifies which options are copied from the `source`
		/// into the `destination`.
		#[doc(alias("value_mask"))]
		pub options_mask: GraphicsOptionsMask,
	}
}

request_error! {
	pub enum SetDashesError for SetDashes {
		GraphicsContext,
		Value,
	}
}

derive_xrb! {
	/// A [request] that sets the [`dash_offset`] and the pattern of dashes on a
	/// [`GraphicsContext`].
	///
	/// Configuring [`dashes`] or [`dash_offset`] with a
	/// [`ChangeGraphicsOptions` request] overrides the effects of this
	/// [request].
	///
	/// Configuring [`dashes`] with a [`ChangeGraphicsOptions` request] does not
	/// allow an alternating pattern of dashes to be specified; this [request]
	/// does.
	///
	/// # Errors
	/// A [`GraphicsContext` error] is generated if `target` does not refer to a
	/// defined [`GraphicsContext`].
	///
	/// [request]: crate::message::Request
	///
	/// [`dashes`]: GraphicsOptions::dashes
	/// [`dash_offset`]: GraphicsOptions::dash_offset
	///
	/// [`ChangeGraphicsOptions` request]: ChangeGraphicsOptions
	///
	/// [`GraphicsContext` error]: error::GraphicsContext
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct SetDashes: Request(58, SetDashesError) {
		/// The [`GraphicsContext`] on which this [request] configures its
		/// dashes.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// [request]: crate::message::Request
		pub target: GraphicsContext,

		/// The offset from the endpoints or joinpoints of a dashed line before
		/// the dashes are drawn.
		pub dash_offset: Px<u16>,

		// The length of `dashes`.
		#[allow(clippy::cast_possible_truncation)]
		let dashes_len: u16 = dashes => dashes.len() as u16,
		/// The pattern of dashes used when drawing dashed lines.
		///
		/// Each element represents the length of a dash in the pattern,
		/// measured in pixels. A `dashes` list of odd length is appended to
		/// itself to produce a list of even length.
		#[context(dashes_len => usize::from(*dashes_len))]
		pub dashes: Vec<Px<u8>>,
		[_; dashes => pad(dashes)],
	}
}

request_error! {
	pub enum SetClipRectanglesError for SetClipRectangles {
		GraphicsContext,
		Match,
		Value,
	}
}

/// Specifies the ordering of [rectangles] given by `clip_rectangles` in a
/// [`SetClipRectangles` request].
///
/// [rectangles]: Rectangle
///
/// [`SetClipRectangles` request]: SetClipRectangles
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum ClipRectanglesOrdering {
	/// No particular order is specified.
	///
	/// The [rectangles] given by `clip_rectangles` in a
	/// [`SetClipRectangles` request] are given in no particular order.
	///
	/// [rectangles]: Rectangle
	///
	/// [`SetClipRectangles` request]: SetClipRectangles
	Unsorted,

	/// [Rectangles][rectangles] are ordered by their y coordinate.
	///
	/// The [rectangles] given by `clip_rectangles` in a
	/// [`SetClipRectangles` request] are sorted by their y coordinate from low
	/// to high.
	///
	/// The ordering among [rectangles] with equal y coordinates is not
	/// specified.
	///
	/// [rectangles]: Rectangle
	///
	/// [`SetClipRectangles` request]: SetClipRectangles
	SortedByY,

	/// [Rectangles][rectangles] are ordered primarily by their y coordinate,
	/// and secondarily by their x coordinate.
	///
	/// The [rectangles] given by `clip_rectangles` in a
	/// [`SetClipRectangles` request] are sorted by their y coordinate from low
	/// to high, and those of equal y are sorted by their x coordinate from low
	/// to high.
	///
	/// [rectangles]: Rectangle
	///
	/// [`SetClipRectangles` request]: SetClipRectangles
	SortedByYx,

	/// [Rectangles][rectangles] are ordered primarily by their y coordinate,
	/// secondarily by their x coordinate, and each one which intersects a given
	/// y coordinate has an equal y coordinate and height.
	///
	/// The [rectangles] given by `clip_rectangles` in a
	/// [`SetClipRectangles` request] are sorted by their y coordinate from low
	/// to high, those of equal y are sorted by their x coordinate from low to
	/// high, and every [rectangle][rectangles] which intersects a given y
	/// coordinate is guaranteed to have the same y coordinate and height as
	/// every other intersecting [rectangle][rectangles].
	///
	/// [rectangles]: Rectangle
	///
	/// [`SetClipRectangles` request]: SetClipRectangles
	BandedByYx,
}

derive_xrb! {
	/// A [request] that configures the clip mask of a [`GraphicsContext`] using
	/// a list of [rectangles].
	///
	/// This [request] also sets the [`clip_x`] and [`clip_y`] of the clip mask.
	/// The coordinates used in the [rectangles] are relative to the [`clip_x`]
	/// and [`clip_y`].
	///
	/// # Errors
	/// A [`GraphicsContext` error] is generated if `target` does not refer to a
	/// defined [`GraphicsContext`].
	///
	/// [rectangles]: Rectangle
	/// [request]: crate::message::Request
	///
	/// [`clip_x`]: GraphicsOptions::clip_x
	/// [`clip_y`]: GraphicsOptions::clip_y
	///
	/// [`GraphicsContext` error]: error::GraphicsContext
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct SetClipRectangles: Request(59, SetClipRectanglesError) {
		/// Specifies the ordering of [rectangles] within `clip_rectangles`.
		///
		/// See [`ClipRectanglesOrdering`] for more information.
		///
		/// [rectangles]: Rectangle
		#[metabyte]
		pub ordering: ClipRectanglesOrdering,

		/// The [`GraphicsContext`] on which this [request] configures its clip
		/// mask.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// [request]: crate::message::Request
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		pub target: GraphicsContext,

		/// The x coordinate of the top-left corner of the clip mask.
		///
		/// This is relative to the top-left corner of the destination
		/// [drawable] used in a particular graphics operation.
		///
		/// The coordinates used in the [rectangles] in `clip_rectangles` are
		/// relative to this x coordinate.
		///
		/// [drawable]: Drawable
		/// [rectangles]: Rectangle
		pub clip_x: Px<i16>,
		/// The y coordinate of the top-left corner of the clip mask.
		///
		/// This is relative to the top-left corner of the destination
		/// [drawable] used in a particular graphics operation.
		///
		/// The coordinates used in the [rectangles] in `clip_rectangles` are
		/// relative to this y coordinate.
		///
		/// [drawable]: Drawable
		/// [rectangles]: Rectangle
		pub clip_y: Px<i16>,

		/// A list of non-overlapping [rectangles] that are used to mask the
		/// effects of a graphics operation.
		///
		/// These [rectangles] specify the areas within which the effects of a
		/// graphics operation are applied.
		///
		/// If this list is empty, graphics operations will have no graphical
		/// effect.
		///
		/// [rectangles]: Rectangle
		#[context(self::remaining => remaining / Rectangle::X11_SIZE)]
		pub clip_rectangles: Vec<Rectangle>,
	}

	/// A [request] that deletes the given [`GraphicsContext`].
	///
	/// The association between the [`GraphicsContext` ID] and the
	/// [`GraphicsContext`] itself is removed in the process.
	///
	/// # Errors
	/// A [`GraphicsContext` error] is generated if `target` does not refer to a
	/// defined [`GraphicsContext`].
	///
	/// [request]: crate::message::Request
	///
	/// [`GraphicsContext` ID]: GraphicsContext
	///
	/// [`GraphicsContext` error]: error::GraphicsContext
	#[doc(alias("FreeGc", "FreeGC", "FreeGcontext", "FreeGraphicsContext"))]
	#[doc(alias("DestroyGc", "DestroyGC", "DestroyGcontext"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct DestroyGraphicsContext: Request(60, error::GraphicsContext) {
		/// The [`GraphicsContext`] which is to be deleted.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		#[doc(alias("gc", "graphics_context", "context", "gcontext"))]
		pub target: GraphicsContext,
	}
}

request_error! {
	pub enum ClearAreaError for ClearArea {
		Match,
		Value,
		Window,
	}
}

derive_xrb! {
	/// A [request] that clears a particular area of a [window].
	///
	/// If the [window] has a defined background ([`background_pixmap`] or
	/// [`background_color`], the `area` is replaced by that background.
	/// Otherwise, in the background is [`None`], the contents are not changed.
	///
	/// # Errors
	/// A [`Window` error] is generated if `target` does not refer to a defined
	/// [window].
	///
	/// A [`Match` error] is generated if the `target` is an [`InputOnly`]
	/// [window].
	///
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [`background_pixmap`]: crate::set::Attributes::background_pixmap
	/// [`background_color`]: crate::set::Attributes::background_color
	///
	/// [`InputOnly`]: crate::WindowClass::InputOnly
	///
	/// [`Window` error]: error::Window
	/// [`Match` error]: error::Match
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct ClearArea: Request(61, ClearAreaError) {
		/// Whether [`GraphicsExposure` events] should be generated for regions
		/// of the `area` which are visible or maintained.
		///
		/// [`GraphicsExposure` events]: crate::x11::event::GraphicsExposure
		#[metabyte]
		pub graphics_exposure: bool,

		/// The [window] which this [request] clears an area of.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// A [`Match` error] is generated if this is an [`InputOnly`] [window].
		///
		/// [window]: Window
		/// [request]: crate::message::Request
		///
		/// [`InputOnly`]: crate::WindowClass::InputOnly
		///
		/// [`Window` error]: error::Window
		/// [`Match` error]: error::Match
		pub target: Window,

		/// The area of the `target` [window] which is cleared.
		///
		/// The `x` and `y` coordinates are relative to the top-left corner of
		/// the `target` [window].
		///
		/// [window]: Window
		pub area: Rectangle,
	}
}

request_error! {
	pub enum CopyAreaError for CopyArea {
		Drawable,
		GraphicsContext,
		Match,
	}
}

derive_xrb! {
	/// A [request] that copies an area of the given `source` [drawable] into
	/// the given `destination` [drawable].
	///
	/// [Regions][regions] of the `source` that are obscured and have not been
	/// [maintained], as well as [regions] specified by `source_coords` and
	/// `dimensions` fall outside of the `source` itself, are not copied. If the
	/// `destination` has a background, however, and those [regions] of the
	/// `source` which are not copied correspond to [regions] of the
	/// `destination` which are visible or [maintained], those [regions] will be
	/// filled with the `destination`'s background. If the `graphics_context`'s
	/// [`graphics_exposure`] is `true`, [`GraphicsExposure` events] will be
	/// generated for those [regions] (or a [`NoExposure` event] if none are
	/// generated).
	///
	/// # Graphics options used
	/// This [request] uses the following [options] of the `graphics_context`:
	/// - [`function`]
	/// - [`plane_mask`]
	/// - [`child_mode`]
	/// - [`graphics_exposure`]
	/// - [`clip_x`]
	/// - [`clip_y`]
	/// - [`clip_mask`]
	///
	/// # Errors
	/// A [`Drawable` error] is generated if either `source` or `destination` do
	/// not refer to defined [windows] nor [pixmaps].
	///
	/// A [`GraphicsContext` error] is generated if `graphics_context` does not
	/// refer to a defined [`GraphicsContext`].
	///
	/// [drawable]: Drawable
	/// [windows]: Window
	/// [pixmaps]: Pixmap
	/// [regions]: crate::Region
	/// [options]: GraphicsOptions
	/// [request]: crate::message::Request
	///
	/// [maintained]: crate::MaintainContents
	///
	/// [`function`]: GraphicsOptions::function
	/// [`plane_mask`]: GraphicsOptions::plane_mask
	/// [`child_mode`]: GraphicsOptions::child_mode
	/// [`graphics_exposure`]: GraphicsOptions::graphics_exposure
	/// [`clip_x`]: GraphicsOptions::clip_x
	/// [`clip_y`]: GraphicsOptions::clip_y
	/// [`clip_mask`]: GraphicsOptions::clip_y
	///
	/// [`GraphicsExposure` events]: crate::x11::event::GraphicsExposure
	/// [`NoExposure` event]: crate::x11::event::NoExposure
	///
	/// [`Drawable` error]: error::Drawable
	/// [`GraphicsContext` error]: error::GraphicsContext
	/// [`Match` error]: error::Match
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct CopyArea: Request(62, CopyAreaError) {
		/// The [drawable] from which the area is copied.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// A [`Match` error] is generated if this does not have the same root
		/// [window] and depth as the `destination`.
		///
		/// [drawable]: Drawable
		/// [window]: Window
		/// [pixmap]: Pixmap
		///
		/// [`Drawable` error]: error::Drawable
		/// [`Match` error]: error::Match
		pub source: Drawable,
		/// The [drawable] into which the area is copied.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// A [`Match` error] is generated if this does not have the same root
		/// [window] and depth as the `destination`.
		///
		/// [drawable]: Drawable
		/// [window]: Window
		/// [pixmap]: Pixmap
		///
		/// [`Drawable` error]: error::Drawable
		/// [`Match` error]: error::Match
		pub destination: Drawable,

		/// The [`GraphicsContext`] used in this graphics operation.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		pub graphics_context: GraphicsContext,

		/// The coordinates of the area which is copied from the `source`
		/// [drawable].
		///
		/// These coordinates are relative to the top-left corner of the
		/// `source`, and specify the top-left corner of the area which is
		/// copied.
		///
		/// [drawable]: Drawable
		pub source_coords: Coords,
		/// The coordinates at which the copied area will be placed within the
		/// `destination` [drawable].
		///
		/// These coordinates are relative to the top-left corner of the
		/// `destination`, and specify what the top-left corner of the copied
		/// area will be when it has been copied.
		///
		/// [drawable]: Drawable
		pub destination_coords: Coords,

		/// The dimensions of the area that is copied.
		pub dimensions: Dimensions,
	}
}

request_error! {
	#[doc(alias("CopyPlaneError"))]
	pub enum CopyBitPlaneError for CopyBitPlane {
		Drawable,
		GraphicsContext,
		Match,
		Value,
	}
}

derive_xrb! {
	/// A [request] that copies a [region] of the `source` [drawable], masked by the given
	/// `bit_plane`, and filled according to the [`foreground_color`] and [`background_color`] in
	/// the `graphics_context`.
	///
	/// Effectively, a [pixmap] with the given `dimensions` and the same depth
	/// as the `destination` [drawable] is created. It is filled with the
	/// [`foreground_color`] where the `bit_plane` in the `source` [drawable]
	/// contains a bit set to 1, and [`background_color`] where the `bit_plane`
	/// in the `source` [drawable] contains a bit set to 0.
	///
	/// # Graphics options used
	/// This [request] uses the following [options] of the `graphics_context`:
	/// - [`function`]
	/// - [`plane_mask`]
	/// - [`foreground_color`]
	/// - [`background_color`]
	/// - [`child_mode`]
	/// - [`graphics_exposure`]
	/// - [`clip_x`]
	/// - [`clip_y`]
	/// - [`clip_mask`]
	///
	/// # Errors
	/// A [`Drawable` error] is generated if either the `source` or the
	/// `destination` do not refer to defined [windows][window] nor
	/// [pixmaps][pixmap].
	///
	/// A [`GraphicsContext` error] is generated if the `graphics_context` does
	/// not refer to a defined [`GraphicsContext`].
	///
	/// A [`Match` error] is generated if the `source` [drawable] does not have
	/// the same root [window] as the `destination` [drawable].
	///
	/// A [`Value` error] is generated if the `bit_plane` does not have exactly
	/// one bit set to 1, or if the value of the `bit_plane` is not less than
	/// 2<sup>`depth`</sup>, where `depth` is the `source` [drawable]'s depth.
	///
	/// [drawable]: Drawable
	/// [pixmap]: Pixmap
	/// [window]: Window
	/// [region]: crate::Region
	/// [options]: GraphicsOptions
	/// [request]: crate::message::Request
	///
	/// [`function`]: GraphicsOptions::function
	/// [`plane_mask`]: GraphicsOptions::plane_mask
	/// [`foreground_color`]: GraphicsOptions::foreground_color
	/// [`background_color`]: GraphicsOptions::background_color
	/// [`child_mode`]: GraphicsOptions::child_mode
	/// [`graphics_exposure`]: GraphicsOptions::graphics_exposure
	/// [`clip_x`]: GraphicsOptions::clip_x
	/// [`clip_y`]: GraphicsOptions::clip_y
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	///
	/// [`Drawable` error]: error::Drawable
	/// [`GraphicsContext` error]: error::GraphicsContext
	/// [`Match` error]: error::Match
	/// [`Value` error]: error::Value
	#[doc(alias("CopyPlane"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct CopyBitPlane: Request(63, CopyBitPlaneError) {
		/// The [drawable] used as the source in this graphics operation.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// A [`Match` error] is generated if this does not have the same root
		/// [window] as the `destination`.
		///
		/// [drawable]: Drawable
		/// [pixmap]: Pixmap
		/// [window]: Window
		///
		/// [`Drawable` error]: error::Drawable
		/// [`Match` error]: error::Match
		#[doc(alias("src", "src_drawable", "source_drawable"))]
		pub source: Drawable,
		/// The [drawable] which the filled [region] is copied into.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// A [`Match` error] is generated if this does not have the same root
		/// [window] as the `source`.
		///
		/// [region]: crate::Region
		/// [drawable]: Drawable
		/// [pixmap]: Pixmap
		/// [window]: Window
		///
		/// [`Drawable` error]: error::Drawable
		/// [`Match` error]: error::Match
		#[doc(alias("dst", "dst_drawable", "destination_drawable"))]
		pub destination: Drawable,

		/// The [`GraphicsContext`] used in this graphics operation.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		#[doc(alias("gc", "context", "gcontext"))]
		pub graphics_context: GraphicsContext,

		/// The coordinates of the [region] within the `source` [drawable] which
		/// is used.
		///
		/// These coordinates are relative to the top-left corner of the
		/// `source`, and specify the top-left corner of the [region] which is
		/// copied.
		///
		/// [region]: crate::Region
		/// [drawable]: Drawable
		#[doc(alias("src_x", "src_y", "source_x", "source_y", "src_coords"))]
		pub source_coords: Coords,
		/// The coordinates at which the copied [region] will be placed within
		/// the `destination` [drawable].
		///
		/// These coordinates are relative to the top-left corner of the
		/// `destination`, and specify what the top-left corner of the copied
		/// [region] will be when it has been copied.
		///
		/// [region]: crate::Region
		/// [drawable]: Drawable
		#[doc(alias("dst_x", "dst_y", "destination_x", "destination_y", "dst_coords"))]
		pub destination_coords: Coords,

		/// The dimensions of the [region] that is copied.
		///
		/// [region]: crate::Region
		#[doc(alias("width", "height"))]
		pub dimensions: Dimensions,

		/// The bit plane that is copied.
		///
		/// Exactly one bit must be set and the value must be less than
		/// 2<sup>`depth`</sup>, where `depth` is the depth of the `source`
		/// [drawable].
		///
		/// # Errors
		/// A [`Value` error] is generated if this does not have exactly one bit
		/// set to 1, or if this value is not less than 2<sup>`depth`</sup>,
		/// where `depth` is the depth of the `source` [drawable].
		///
		/// [drawable]: Drawable
		///
		/// [`Value` error]: error::Value
		pub bit_plane: u32,
	}
}

request_error! {
	#[doc(alias("PolyPointError", "DrawPointError"))]
	pub enum DrawPointsError for DrawPoints {
		Drawable,
		GraphicsContext,
		Match,
		Value,
	}
}

/// Whether [coordinates] of elements to be drawn in graphics operations are
/// relative to the [drawable] or the previous element.
///
/// The first element is always relative to the [drawable].
///
/// [coordinates]: Coords
/// [drawable]: Drawable
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum CoordinateMode {
	/// [Coordinates] are relative to the top-left corner of the [drawable].
	///
	/// [Coordinates]: Coords
	/// [drawable]: Drawable
	Drawable,

	/// [Coordinates][coords] are relative to the [coordinates][coords] of the
	/// previous element.
	///
	/// [coords]: Coords
	Previous,
}

derive_xrb! {
	/// A [request] that draws the given `points` on the `target` [drawable].
	///
	/// The points are drawn by combining the `graphics_context`'s
	/// [`foreground_color`] with the existing color at those coordinates in the
	/// [drawable]. They are drawn in the order that they are specified in the
	/// list.
	///
	/// # Graphics options used
	/// This [request] uses the following [options] of the `graphics_context`:
	/// - [`function`]
	/// - [`plane_mask`]
	/// - [`foreground_color`]
	/// - [`child_mode`]
	/// - [`clip_x`]
	/// - [`clip_y`]
	/// - [`clip_mask`]
	///
	/// # Errors
	/// A [`Drawable` error] is generated if `target` does not refer to a
	/// defined [window] nor [pixmap].
	///
	/// A [`GraphicsContext` error] is generated if `graphics_context` does not
	/// refer to a defined [`GraphicsContext`].
	///
	/// [drawable]: Drawable
	/// [window]: Window
	/// [pixmap]: Pixmap
	/// [options]: GraphicsOptions
	/// [request]: crate::message::Request
	///
	/// [`function`]: GraphicsOptions::function
	/// [`plane_mask`]: GraphicsOptions::plane_mask
	/// [`foreground_color`]: GraphicsOptions::foreground_color
	/// [`child_mode`]: GraphicsOptions::child_mode
	/// [`clip_x`]: GraphicsOptions::clip_x
	/// [`clip_y`]: GraphicsOptions::clip_y
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	///
	/// [`Drawable` error]: error::Drawable
	/// [`GraphicsContext` error]: error::GraphicsContext
	/// [`Match` error]: error::Match
	#[doc(alias("PolyPoint", "DrawPoint"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct DrawPoints: Request(64, DrawPointsError) {
		/// Whether the `points` are drawn relative to the `target` or the
		/// previously drawn point.
		///
		/// The first point is always drawn relative to the `target`.
		///
		/// See [`CoordinateMode`] for more information.
		#[metabyte]
		pub coordinate_mode: CoordinateMode,

		/// The [drawable] on which the given `points` are drawn.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// [drawable]: Drawable
		/// [window]: Window
		/// [pixmap]: Pixmap
		///
		/// [`Drawable` error]: error::Drawable
		#[doc(alias("drawable"))]
		pub target: Drawable,

		/// The [`GraphicsContext`] used in this graphics operation.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		#[doc(alias("gc", "context", "gcontext"))]
		pub graphics_context: GraphicsContext,

		/// The points which are to be drawn.
		///
		/// Each point is represented by its [coordinates].
		///
		/// The points are drawn in the order that they appear in the list.
		///
		/// Each point is drawn by combining the [`foreground_color`] with the
		/// existing color of the pixel at the point's [coordinates].
		///
		/// [coordinates]: Coords
		///
		/// [`foreground_color`]: GraphicsOptions::foreground_color
		#[context(self::remaining => remaining / Coords::X11_SIZE)]
		pub points: Vec<Coords>,
	}
}

request_error! {
	#[doc(alias("PolyLineError", "DrawLinesError", "DrawLineError"))]
	pub enum DrawPathError for DrawPath {
		Drawable,
		GraphicsContext,
		Match,
		Value,
	}
}

derive_xrb! {
	/// A [request] that draws a path comprised of lines that join the given
	/// `points`.
	///
	/// The lines are drawn in the order that the points appear in `points`.
	/// They join at each intermediate point. If the first and last points are
	/// in the same location, they are also joined to create a closed path (with
	/// no endpoints).
	///
	/// Intersecting [thin] lines have their intersecting pixels drawn multiple
	/// times. Intersecting [thick] lines, however, only have their intersecting
	/// pixels drawn once.
	///
	/// # Graphics options used
	/// This [request] uses the following [options] of the `graphics_context`:
	/// - [`function`]
	/// - [`plane_mask`]
	/// - [`line_width`]
	/// - [`line_style`]
	/// - [`cap_style`]
	/// - [`join_style`]
	/// - [`fill_style`]
	/// - [`child_mode`]
	/// - [`clip_x`]
	/// - [`clip_y`]
	/// - [`clip_mask`]
	///
	/// This [request] may also use these [options], depending on the
	/// configuration of the `graphics_context`:
	/// - [`foreground_color`]
	/// - [`background_color`]
	/// - [`tile`]
	/// - [`stipple`]
	/// - [`tile_stipple_x`]
	/// - [`tile_stipple_y`]
	/// - [`dash_offset`]
	/// - [`dashes`]
	///
	/// # Errors
	/// A [`Drawable` error] is generated if `target` does not refer to a
	/// defined [window] nor [pixmap].
	///
	/// A [`GraphicsContext` error] is generated if `graphics_context` does not
	/// refer to a defined [`GraphicsContext`].
	///
	/// [thin]: crate::set::LineWidth::Thin
	/// [thick]: crate::set::LineWidth::Thick
	///
	/// [drawable]: Drawable
	/// [window]: Window
	/// [pixmap]: Pixmap
	/// [options]: GraphicsOptions
	/// [request]: crate::message::Request
	///
	/// [`function`]: GraphicsOptions::function
	/// [`plane_mask`]: GraphicsOptions::plane_mask
	/// [`line_width`]: GraphicsOptions::line_width
	/// [`line_style`]: GraphicsOptions::line_style
	/// [`cap_style`]: GraphicsOptions::cap_style
	/// [`join_style`]: GraphicsOptions::join_style
	/// [`fill_style`]: GraphicsOptions::fill_style
	/// [`child_mode`]: GraphicsOptions::child_mode
	/// [`clip_x`]: GraphicsOptions::clip_x
	/// [`clip_y`]: GraphicsOptions::clip_y
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	///
	/// [`foreground_color`]: GraphicsOptions::foreground_color
	/// [`background_color`]: GraphicsOptions::background_color
	/// [`tile`]: GraphicsOptions::tile
	/// [`stipple`]: GraphicsOptions::stipple
	/// [`tile_stipple_x`]: GraphicsOptions::tile_stipple_x
	/// [`tile_stipple_y`]: GraphicsOptions::tile_stipple_y
	/// [`dash_offset`]: GraphicsOptions::dash_offset
	/// [`dashes`]: GraphicsOptions::dashes
	///
	/// [`Drawable` error]: error::Drawable
	/// [`GraphicsContext` error]: error::GraphicsContext
	#[doc(alias("PolyLine", "DrawLines", "DrawLine"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct DrawPath: Request(65, DrawPathError) {
		/// Whether the [coordinates] of each point in `points` are relative to
		/// the `target` or to the previous point.
		///
		/// The first point is always relative to the `target` [drawable].
		///
		/// [coordinates]: Coords
		/// [drawable]: Drawable
		#[metabyte]
		pub coordinate_mode: CoordinateMode,

		/// The [drawable] on which the path is drawn.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// [drawable]: Drawable
		/// [window]: Window
		/// [pixmap]: Pixmap
		///
		/// [`Drawable` error]: error::Drawable
		#[doc(alias("drawable"))]
		pub target: Drawable,

		/// The [`GraphicsContext`] used in this graphics operation.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		#[doc(alias("gc", "context", "gcontext"))]
		pub graphics_context: GraphicsContext,

		/// The points which are to be connected by lines.
		///
		/// Each point is represented by its [coordinates].
		///
		/// The points are connected by lines in the order that they appear in
		/// this list.
		///
		/// [coordinates]: Coords
		#[context(self::remaining => remaining / Coords::X11_SIZE)]
		pub points: Vec<Coords>,
	}
}

request_error! {
	#[doc(alias("PolySegmentError", "DrawSegmentError"))]
	pub enum DrawLinesError for DrawLines {
		Drawable,
		GraphicsContext,
		Match,
	}
}

/// A line from the given `start` point to the given `end` point.
#[doc(alias("Segment"))]
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
pub struct Line {
	/// The start of the line.
	pub start: Coords,
	/// The end of the line.
	pub end: Coords,
}

derive_xrb! {
	/// A [request] that draws the given `lines`.
	///
	/// No join points are created. Intersecting [lines] have their intersecting
	/// pixels drawn multiple times.
	///
	/// # Graphics options used
	/// This [request] uses the following [options] of the `graphics_context`:
	/// - [`function`]
	/// - [`plane_mask`]
	/// - [`line_width`]
	/// - [`line_style`]
	/// - [`cap_style`]
	/// - [`fill_style`]
	/// - [`child_mode`]
	/// - [`clip_x`]
	/// - [`clip_y`]
	/// - [`clip_mask`]
	///
	/// This [request] may also use these [options], depending on the
	/// configuration of the `graphics_context`:
	/// - [`foreground_color`]
	/// - [`background_color`]
	/// - [`tile`]
	/// - [`stipple`]
	/// - [`tile_stipple_x`]
	/// - [`tile_stipple_y`]
	/// - [`dash_offset`]
	/// - [`dashes`]
	///
	/// # Errors
	/// A [`Drawable` error] is generated if `target` does not refer to a
	/// defined [window] nor [pixmap].
	///
	/// A [`GraphicsContext` error] is generated if `graphics_context` does not
	/// refer to a defined [`GraphicsContext`].
	///
	/// [drawable]: Drawable
	/// [window]: Window
	/// [pixmap]: Pixmap
	/// [lines]: Line
	/// [options]: GraphicsOptions
	/// [request]: crate::message::Request
	///
	/// [`function`]: GraphicsOptions::function
	/// [`plane_mask`]: GraphicsOptions::plane_mask
	/// [`line_width`]: GraphicsOptions::line_width
	/// [`line_style`]: GraphicsOptions::line_style
	/// [`cap_style`]: GraphicsOptions::cap_style
	/// [`fill_style`]: GraphicsOptions::fill_style
	/// [`child_mode`]: GraphicsOptions::child_mode
	/// [`clip_x`]: GraphicsOptions::clip_x
	/// [`clip_y`]: GraphicsOptions::clip_y
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	///
	/// [`foreground_color`]: GraphicsOptions::foreground_color
	/// [`background_color`]: GraphicsOptions::background_color
	/// [`tile`]: GraphicsOptions::tile
	/// [`stipple`]: GraphicsOptions::stipple
	/// [`tile_stipple_x`]: GraphicsOptions::tile_stipple_x
	/// [`tile_stipple_y`]: GraphicsOptions::tile_stipple_y
	/// [`dash_offset`]: GraphicsOptions::dash_offset
	/// [`dashes`]: GraphicsOptions::dashes
	///
	/// [`Drawable` error]: error::Drawable
	/// [`GraphicsContext` error]: error::GraphicsContext
	#[doc(alias("PolySegment", "DrawSegment"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct DrawLines: Request(66, DrawLinesError) {
		/// The [drawable] on which the given `lines` are drawn.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// [drawable]: Drawable
		/// [window]: Window
		/// [pixmap]: Pixmap
		///
		/// [`Drawable` error]: error::Drawable
		#[doc(alias("drawable"))]
		pub target: Drawable,

		/// The [`GraphicsContext`] used in this graphics operation.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		#[doc(alias("gc", "context", "gcontext"))]
		pub graphics_context: GraphicsContext,

		/// The lines which are to be drawn.
		///
		/// The lines are drawn in the order that they appear in this list.
		#[context(self::remaining => remaining / Line::X11_SIZE)]
		pub lines: Vec<Line>,
	}
}

request_error! {
	#[doc(alias("PolyRectangleError"))]
	pub enum DrawRectanglesError for DrawRectangles {
		Drawable,
		GraphicsContext,
		Match,
	}
}

derive_xrb! {
	/// A [request] that draws the outlines of the given `rectangles`.
	///
	/// The outlines are drawn as [lines] connecting each corner of the
	/// [rectangles], starting at the top-left corner and going clockwise, with
	/// a join point at each corner - thus forming a closed path.
	///
	/// If any of the given `rectangles` intersect, the intersecting pixels are
	/// drawn multiple times.
	///
	/// # Graphics options used
	/// This [request] uses the following [options] of the `graphics_context`:
	/// - [`function`]
	/// - [`plane_mask`]
	/// - [`line_width`]
	/// - [`line_style`]
	/// - [`cap_style`]
	/// - [`join_style`]
	/// - [`fill_style`]
	/// - [`child_mode`]
	/// - [`clip_x`]
	/// - [`clip_y`]
	/// - [`clip_mask`]
	///
	/// This [request] may also use these [options], depending on the
	/// configuration of the `graphics_context`:
	/// - [`foreground_color`]
	/// - [`background_color`]
	/// - [`tile`]
	/// - [`stipple`]
	/// - [`tile_stipple_x`]
	/// - [`tile_stipple_y`]
	/// - [`dash_offset`]
	/// - [`dashes`]
	///
	/// # Errors
	/// A [`Drawable` error] is generated if `target` does not refer to a
	/// defined [window] nor [pixmap].
	///
	/// A [`GraphicsContext` error] is generated if `graphics_context` does not
	/// refer to a defined [`GraphicsContext`].
	///
	/// [window]: Window
	/// [pixmap]: Pixmap
	/// [lines]: Line
	/// [rectangles]: Rectangle
	/// [options]: GraphicsOptions
	/// [request]: crate::message::Request
	///
	/// [`function`]: GraphicsOptions::function
	/// [`plane_mask`]: GraphicsOptions::plane_mask
	/// [`line_width`]: GraphicsOptions::line_width
	/// [`line_style`]: GraphicsOptions::line_style
	/// [`cap_style`]: GraphicsOptions::cap_style
	/// [`join_style`]: GraphicsOptions::join_style
	/// [`fill_style`]: GraphicsOptions::fill_style
	/// [`child_mode`]: GraphicsOptions::child_mode
	/// [`clip_x`]: GraphicsOptions::clip_x
	/// [`clip_y`]: GraphicsOptions::clip_y
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	///
	/// [`foreground_color`]: GraphicsOptions::foreground_color
	/// [`background_color`]: GraphicsOptions::background_color
	/// [`tile`]: GraphicsOptions::tile
	/// [`stipple`]: GraphicsOptions::stipple
	/// [`tile_stipple_x`]: GraphicsOptions::tile_stipple_x
	/// [`tile_stipple_y`]: GraphicsOptions::tile_stipple_y
	/// [`dash_offset`]: GraphicsOptions::dash_offset
	/// [`dashes`]: GraphicsOptions::dashes
	///
	/// [`Drawable` error]: error::Drawable
	/// [`GraphicsContext` error]: error::GraphicsContext
	#[doc(alias("PolyRectangle"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct DrawRectangles: Request(67, DrawRectanglesError) {
		/// The [drawable] on which the `rectangles`' outlines are drawn.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// [drawable]: Drawable
		/// [window]: Window
		/// [pixmap]: Pixmap
		///
		/// [`Drawable` error]: error::Drawable
		#[doc(alias("drawable"))]
		pub target: Drawable,

		/// The [`GraphicsContext`] used in this graphics operation.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		#[doc(alias("gc", "context", "gcontext"))]
		pub graphics_context: GraphicsContext,

		/// The [rectangles] which are to have their outlines drawn.
		///
		/// Their `x` and `y` coordinates are relative to the top-left corner of
		/// the `target` [drawable].
		///
		/// The [rectangles] are drawn in the order that they appear in this
		/// list.
		///
		/// [rectangles]: Rectangle
		/// [drawable]: Drawable
		#[context(self::remaining => remaining / Rectangle::X11_SIZE)]
		pub rectangles: Vec<Rectangle>,
	}
}

request_error! {
	#[doc(alias("PolyArcError"))]
	pub enum DrawArcsError for DrawArcs {
		Drawable,
		GraphicsContext,
		Match,
	}
}

derive_xrb! {
	/// A [request] that draws circular or elliptical [arcs][arc].
	///
	/// # Graphics options used
	/// This [request] uses the following [options] of the `graphics_context`:
	/// - [`function`]
	/// - [`plane_mask`]
	/// - [`line_width`]
	/// - [`line_style`]
	/// - [`cap_style`]
	/// - [`join_style`]
	/// - [`fill_style`]
	/// - [`child_mode`]
	/// - [`clip_x`]
	/// - [`clip_y`]
	/// - [`clip_mask`]
	///
	/// This [request] may also use these [options], depending on the
	/// configuration of the `graphics_context`:
	/// - [`foreground_color`]
	/// - [`background_color`]
	/// - [`tile`]
	/// - [`stipple`]
	/// - [`tile_stipple_x`]
	/// - [`tile_stipple_y`]
	/// - [`dash_offset`]
	/// - [`dashes`]
	///
	/// # Errors
	/// A [`Drawable` error] is generated if `target` does not refer to a
	/// defined [window] nor [pixmap].
	///
	/// A [`GraphicsContext` error] is generated if `graphics_context` does not
	/// refer to a defined [`GraphicsContext`].
	///
	/// [arc]: Arc
	/// [window]: Window
	/// [pixmap]: Pixmap
	/// [options]: GraphicsOptions
	/// [request]: crate::message::Request
	///
	/// [`function`]: GraphicsOptions::function
	/// [`plane_mask`]: GraphicsOptions::plane_mask
	/// [`line_width`]: GraphicsOptions::line_width
	/// [`line_style`]: GraphicsOptions::line_style
	/// [`cap_style`]: GraphicsOptions::cap_style
	/// [`join_style`]: GraphicsOptions::join_style
	/// [`fill_style`]: GraphicsOptions::fill_style
	/// [`child_mode`]: GraphicsOptions::child_mode
	/// [`clip_x`]: GraphicsOptions::clip_x
	/// [`clip_y`]: GraphicsOptions::clip_y
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	///
	/// [`foreground_color`]: GraphicsOptions::foreground_color
	/// [`background_color`]: GraphicsOptions::background_color
	/// [`tile`]: GraphicsOptions::tile
	/// [`stipple`]: GraphicsOptions::stipple
	/// [`tile_stipple_x`]: GraphicsOptions::tile_stipple_x
	/// [`tile_stipple_y`]: GraphicsOptions::tile_stipple_y
	/// [`dash_offset`]: GraphicsOptions::dash_offset
	/// [`dashes`]: GraphicsOptions::dashes
	///
	/// [`Drawable` error]: error::Drawable
	/// [`GraphicsContext` error]: error::GraphicsContext
	#[doc(alias("PolyArc"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct DrawArcs: Request(68, DrawArcsError) {
		/// The [drawable] on which the [arcs] are drawn.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// [arcs]: Arc
		/// [drawable]: Drawable
		/// [window]: Window
		/// [pixmap]: Pixmap
		///
		/// [`Drawable` error]: error::Drawable
		#[doc(alias("drawable"))]
		pub target: Drawable,

		/// The [`GraphicsContext`] used in this graphics operation.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		#[doc(alias("gc", "context", "gcontext"))]
		pub graphics_context: GraphicsContext,

		/// The [arcs][arc] which are to be drawn.
		///
		/// If the last point in an [arc] is in the same location as the first
		/// point in the next [arc], the two [arcs][arc] are joined with a
		/// joinpoint. If the last point of the last [arc] and the first point
		/// of the first [arc] are in the same location, they will also be
		/// joined.
		///
		/// [arc]: Arc
		#[context(self::remaining => remaining / Arc::X11_SIZE)]
		pub arcs: Vec<Arc>,
	}
}

request_error! {
	#[doc(alias("FillPolyError"))]
	pub enum FillPolygonError for FillPolygon {
		Drawable,
		GraphicsContext,
		Match,
		Value,
	}
}

/// Specifies properties of a polygon that may allow for optimizations when
/// drawing it.
///
/// This is used in the [`FillPolygon` request].
///
/// [`FillPolygon` request]: FillPolygon
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum ShapeMode {
	/// The shape may intersect itself.
	Complex,

	/// The shape may not intersect itself, but it is not (fully) convex.
	///
	/// If these properties are known to be true by the client, specifying this
	/// mode may improve performance over [`Complex`](ShapeMode::Complex).
	Nonconvex,

	/// The shape may not intersect itself and it is convex.
	///
	/// Convex means that no [line] could be drawn between two points in the
	/// shape which intersects the shape's path.
	///
	/// If these properties are known to be true by the client, specifying this
	/// mode may improve performance over [`Nonconvex`](ShapeMode::Nonconvex).
	///
	/// [line]: Line
	Convex,
}

derive_xrb! {
	/// A [request] that fills the area enclosed the given path.
	///
	/// # Graphics options used
	/// This [request] uses the following [options] of the `graphics_context`:
	/// - [`function`]
	/// - [`plane_mask`]
	/// - [`fill_style`]
	/// - [`fill_rule`]
	/// - [`child_mode`]
	/// - [`clip_x`]
	/// - [`clip_y`]
	/// - [`clip_mask`]
	///
	/// This [request] may also use these [options], depending on the
	/// configuration of the `graphics_context`:
	/// - [`foreground_color`]
	/// - [`background_color`]
	/// - [`tile`]
	/// - [`stipple`]
	/// - [`tile_stipple_x`]
	/// - [`tile_stipple_y`]
	///
	/// # Errors
	/// A [`Drawable` error] is generated if `target` does not refer to a
	/// defined [window] nor [pixmap].
	///
	/// A [`GraphicsContext` error] is generated if `graphics_context` does not
	/// refer to a defined [`GraphicsContext`].
	///
	/// [window]: Window
	/// [pixmap]: Pixmap
	/// [options]: GraphicsOptions
	/// [request]: crate::message::Request
	///
	/// [`function`]: GraphicsOptions::function
	/// [`plane_mask`]: GraphicsOptions::plane_mask
	/// [`fill_style`]: GraphicsOptions::fill_style
	/// [`fill_rule`]: GraphicsOptions::fill_rule
	/// [`child_mode`]: GraphicsOptions::child_mode
	/// [`clip_x`]: GraphicsOptions::clip_x
	/// [`clip_y`]: GraphicsOptions::clip_y
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	///
	/// [`foreground_color`]: GraphicsOptions::foreground_color
	/// [`background_color`]: GraphicsOptions::background_color
	/// [`tile`]: GraphicsOptions::tile
	/// [`stipple`]: GraphicsOptions::stipple
	/// [`tile_stipple_x`]: GraphicsOptions::tile_stipple_x
	/// [`tile_stipple_y`]: GraphicsOptions::tile_stipple_y
	///
	/// [`Drawable` error]: error::Drawable
	/// [`GraphicsContext` error]: error::GraphicsContext
	#[doc(alias("FillPoly"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct FillPolygon: Request(69, FillPolygonError) {
		/// The [drawable] on which the filled polygon is drawn.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// [drawable]: Drawable
		/// [window]: Window
		/// [pixmap]: Pixmap
		///
		/// [`Drawable` error]: error::Drawable
		#[doc(alias("drawable"))]
		pub target: Drawable,

		/// The [`GraphicsContext`] used in this graphics operation.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		#[doc(alias("gc", "context", "gcontext"))]
		pub graphics_context: GraphicsContext,

		/// Specifies whether the polygon self-intersects and whether it is
		/// convex.
		///
		/// If the relevant properties of the specified polygon are known by the
		/// client, specifying a more restrictive [shape mode] may improve
		/// performance.
		///
		/// See [`ShapeMode`] for more information.
		///
		/// [shape mode]: ShapeMode
		pub shape: ShapeMode,
		/// Whether the [coordinates] of each point in `points` are relative to
		/// the `target` or to the previous point.
		///
		/// The first point is always relative to the `target` [drawable].
		///
		/// [coordinates]: Coords
		/// [drawable]: Drawable
		pub coordinate_mode: CoordinateMode,
		[_; 2],

		/// The points which, when connected, specify the path of the polygon.
		///
		/// Each point is represented by its [coordinates].
		///
		/// The points are connected in the order that they appear in this list.
		/// If the last point is not in the same location as the first point, it
		/// is automatically connected to the first point to close the path.
		///
		/// [coordinates]: Coords
		#[context(self::remaining => remaining / Coords::X11_SIZE)]
		pub points: Vec<Coords>,
	}
}

request_error! {
	#[doc(alias("PolyFillRectangleError"))]
	pub enum FillRectanglesError for FillRectangles {
		Drawable,
		GraphicsContext,
		Match,
	}
}

derive_xrb! {
	/// A [request] that fills the given `rectangles`.
	///
	/// This is effectively the same as if a [`FillPolygon` request] were sent
	/// for each [rectangle] with the [rectangle]'s four points, starting with
	/// the top-left corner and going clockwise.
	///
	/// # Graphics options used
	/// This [request] uses the following [options] of the `graphics_context`:
	/// - [`function`]
	/// - [`plane_mask`]
	/// - [`fill_style`]
	/// - [`child_mode`]
	/// - [`clip_x`]
	/// - [`clip_y`]
	/// - [`clip_mask`]
	///
	/// This [request] may also use these [options], depending on the
	/// configuration of the `graphics_context`:
	/// - [`foreground_color`]
	/// - [`background_color`]
	/// - [`tile`]
	/// - [`stipple`]
	/// - [`tile_stipple_x`]
	/// - [`tile_stipple_y`]
	///
	/// # Errors
	/// A [`Drawable` error] is generated if `target` does not refer to a
	/// defined [window] nor [pixmap].
	///
	/// A [`GraphicsContext` error] is generated if `graphics_context` does not
	/// refer to a defined [`GraphicsContext`].
	///
	/// [window]: Window
	/// [pixmap]: Pixmap
	/// [options]: GraphicsOptions
	/// [rectangle]: Rectangle
	/// [request]: crate::message::Request
	///
	/// [`function`]: GraphicsOptions::function
	/// [`plane_mask`]: GraphicsOptions::plane_mask
	/// [`fill_style`]: GraphicsOptions::fill_style
	/// [`child_mode`]: GraphicsOptions::child_mode
	/// [`clip_x`]: GraphicsOptions::clip_x
	/// [`clip_y`]: GraphicsOptions::clip_y
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	///
	/// [`foreground_color`]: GraphicsOptions::foreground_color
	/// [`background_color`]: GraphicsOptions::background_color
	/// [`tile`]: GraphicsOptions::tile
	/// [`stipple`]: GraphicsOptions::stipple
	/// [`tile_stipple_x`]: GraphicsOptions::tile_stipple_x
	/// [`tile_stipple_y`]: GraphicsOptions::tile_stipple_y
	///
	/// [`FillPolygon` request]: FillPolygon
	///
	/// [`Drawable` error]: error::Drawable
	/// [`GraphicsContext` error]: error::GraphicsContext
	#[doc(alias("PolyFillRectangle"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct FillRectangles: Request(70, FillRectanglesError) {
		/// The [drawable] on which the [rectangles] are filled.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// [drawable]: Drawable
		/// [window]: Window
		/// [pixmap]: Pixmap
		/// [rectangles]: Rectangle
		///
		/// [`Drawable` error]: error::Drawable
		#[doc(alias("drawable"))]
		pub target: Drawable,

		/// The [`GraphicsContext`] used in this graphics operation.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		#[doc(alias("gc", "context", "gcontext"))]
		pub graphics_context: GraphicsContext,

		/// The [rectangles] which are to be filled.
		///
		/// The [rectangles] are filled in the order that they appear in this
		/// list. For intersecting [rectangles], the intersecting pixels are
		/// drawn multiple times.
		///
		/// [rectangles]: Rectangle
		#[context(self::remaining => remaining / Rectangle::X11_SIZE)]
		pub rectangles: Vec<Rectangle>,
	}
}

request_error! {
	#[doc(alias("PolyFillArcError"))]
	pub enum FillArcsError for FillArcs {
		Drawable,
		GraphicsContext,
		Match,
	}
}

derive_xrb! {
	/// A [request] that fills the given `arcs`.
	///
	/// If the `graphics_context`'s [`arc_mode`] is [`ArcMode::Chord`], the
	/// [arcs][arc] are filled by joining each [arc]'s endpoints with a single
	/// [line]. If the [`arc_mode`] is [`ArcMode::PieSlice`], however, the
	/// [arcs][arc] are filled by joining each [arc]'s endpoints with the center
	/// of that [arc], thus using two [lines][line].
	///
	/// # Graphics options used
	/// This [request] uses the following [options] of the `graphics_context`:
	/// - [`function`]
	/// - [`plane_mask`]
	/// - [`fill_style`]
	/// - [`arc_mode`]
	/// - [`child_mode`]
	/// - [`clip_x`]
	/// - [`clip_y`]
	/// - [`clip_mask`]
	///
	/// This [request] may also use these [options], depending on the
	/// configuration of the `graphics_context`:
	/// - [`foreground_color`]
	/// - [`background_color`]
	/// - [`tile`]
	/// - [`stipple`]
	/// - [`tile_stipple_x`]
	/// - [`tile_stipple_y`]
	///
	/// # Errors
	/// A [`Drawable` error] is generated if `target` does not refer to a
	/// defined [window] nor [pixmap].
	///
	/// A [`GraphicsContext` error] is generated if `graphics_context` does not
	/// refer to a defined [`GraphicsContext`].
	///
	/// [window]: Window
	/// [pixmap]: Pixmap
	/// [options]: GraphicsOptions
	/// [arc]: Arc
	/// [line]: Line
	/// [request]: crate::message::Request
	///
	/// [`ArcMode::Chord`]: crate::set::ArcMode::Chord
	/// [`ArcMode::PieSlice`]: crate::set::ArcMode::PieSlice
	///
	/// [`function`]: GraphicsOptions::function
	/// [`plane_mask`]: GraphicsOptions::plane_mask
	/// [`fill_style`]: GraphicsOptions::fill_style
	/// [`arc_mode`]: GraphicsOptions::arc_mode
	/// [`child_mode`]: GraphicsOptions::child_mode
	/// [`clip_x`]: GraphicsOptions::clip_x
	/// [`clip_y`]: GraphicsOptions::clip_y
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	///
	/// [`foreground_color`]: GraphicsOptions::foreground_color
	/// [`background_color`]: GraphicsOptions::background_color
	/// [`tile`]: GraphicsOptions::tile
	/// [`stipple`]: GraphicsOptions::stipple
	/// [`tile_stipple_x`]: GraphicsOptions::tile_stipple_x
	/// [`tile_stipple_y`]: GraphicsOptions::tile_stipple_y
	///
	/// [`Drawable` error]: error::Drawable
	/// [`GraphicsContext` error]: error::GraphicsContext
	#[doc(alias("PolyFillArc"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct FillArcs: Request(71, FillArcsError) {
		/// The [drawable] on which the [arcs] are filled.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// [drawable]: Drawable
		/// [window]: Window
		/// [pixmap]: Pixmap
		/// [arcs]: Arc
		///
		/// [`Drawable` error]: error::Drawable
		#[doc(alias("drawable"))]
		pub target: Drawable,

		/// The [`GraphicsContext`] used in this graphics operation.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		#[doc(alias("gc", "context", "gcontext"))]
		pub graphics_context: GraphicsContext,

		/// The [arcs] which are to be filled.
		///
		/// The [arcs] are filled in the order that they appear in this list. If
		/// there are intersecting [arcs], the intersecting pixels will be drawn
		/// multiple times.
		///
		/// [arcs]: Arc
		#[context(self::remaining => remaining / Arc::X11_SIZE)]
		pub arcs: Vec<Arc>,
	}
}

request_error! {
	#[doc(alias("PutImageError"))]
	pub enum PlaceImageError for PlaceImage {
		Drawable,
		GraphicsContext,
		Match,
		Value,
	}
}

/// The format of an image sent in a [`PlaceImage` request].
///
/// [`PlaceImage` request]: PlaceImage
#[doc(alias("PutImageFormat"))]
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum PlaceImageFormat {
	/// The image must be in XY format.
	///
	/// The `graphics_context`'s [`foreground_color`] is used where the image
	/// has bits set to `1`, while the [`background_color`] is used where the
	/// image has bits set to `0`.
	///
	/// [`foreground_color`]: GraphicsOptions::foreground_color
	/// [`background_color`]: GraphicsOptions::background_color
	Bitmap,

	/// The image must be in XY format.
	XyPixmap,

	/// The image must be in Z format.
	Zpixmap,
}

derive_xrb! {
	/// A [request] that places the given image on the given [drawable].
	///
	/// # Graphics options used
	/// This [request] uses the following [options] of the `graphics_context`:
	/// - [`function`]
	/// - [`plane_mask`]
	/// - [`child_mode`]
	/// - [`clip_x`]
	/// - [`clip_y`]
	/// - [`clip_mask`]
	///
	/// This [request] may also use these [options], depending on the
	/// provided `format`:
	/// - [`foreground_color`]
	/// - [`background_color`]
	///
	/// # Errors
	/// A [`Drawable` error] is generated if `target` does not refer to a
	/// defined [window] nor [pixmap].
	///
	/// A [`GraphicsContext` error] is generated if `graphics_context` does not
	/// refer to a defined [`GraphicsContext`].
	///
	/// A [`Match` error] is generated if [`PlaceImageFormat::XyPixmap`] or
	/// [`PlaceImageFormat::Zpixmap`] is used and `depth` does not match the
	/// depth of the `target` [drawable].
	///
	/// A [`Match` error] is generated if [`PlaceImageFormat::Bitmap`] is used
	/// and `depth` is not `1`.
	///
	/// A [`Match` error] is generated if [`PlaceImageFormat::Bitmap`] or
	/// [`PlaceImageFormat::XyPixmap`] is used and `left_padding` is not less
	/// than `bitmap_scanline_padding` (given in
	/// [`connection::ConnectionSuccess`]).
	///
	/// A [`Match` error] is generated if [`PlaceImageFormat::Zpixmap`] is used
	/// and `left_padding` is not `0`.
	///
	/// [drawable]: Drawable
	/// [window]: Window
	/// [pixmap]: Pixmap
	/// [options]: GraphicsOptions
	/// [request]: crate::message::Request
	///
	/// [`function`]: GraphicsOptions::function
	/// [`plane_mask`]: GraphicsOptions::plane_mask
	/// [`child_mode`]: GraphicsOptions::child_mode
	/// [`clip_x`]: GraphicsOptions::clip_x
	/// [`clip_y`]: GraphicsOptions::clip_y
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	///
	/// [`foreground_color`]: GraphicsOptions::foreground_color
	/// [`background_color`]: GraphicsOptions::background_color
	///
	/// [`connection::ConnectionSuccess`]: crate::connection::ConnectionSuccess
	///
	/// [`Drawable` error]: error::Drawable
	/// [`GraphicsContext` error]: error::GraphicsContext
	/// [`Match` error]: error::Match
	#[doc(alias("PutImage"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct PlaceImage: Request(72, PlaceImageError) {
		/// The [image format] used.
		///
		/// [image format]: PlaceImageFormat
		#[metabyte]
		pub format: PlaceImageFormat,

		/// The [drawable] on which the image is placed.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// [drawable]: Drawable
		/// [window]: Window
		/// [pixmap]: Pixmap
		///
		/// [`Drawable` error]: error::Drawable
		#[doc(alias("drawable"))]
		pub target: Drawable,

		/// The [`GraphicsContext`] used in this graphics operation.
		///
		/// # Errors
		/// A [`GraphicsContext` error] is generated if this does not refer to a
		/// defined [`GraphicsContext`].
		///
		/// [`GraphicsContext` error]: error::GraphicsContext
		#[doc(alias("gc", "context", "gcontext"))]
		pub graphics_context: GraphicsContext,

		/// The image's width and height.
		pub dimensions: Dimensions,
		/// The [coordinates] at which the image will be placed on the `target`
		/// [drawable].
		///
		/// These coordinates are relative to the top-left corner of the
		/// [drawable].
		///
		/// [drawable]: Drawable
		/// [coordinates]: Coords
		pub coordinates: Coords,

		/// The number of bits in each scanline of the image which are to be
		/// ignored by the X server.
		///
		/// The actual image begins this many bits into the scanline.
		///
		/// This is only used for [`PlaceImageFormat::Bitmap`] and
		/// [`PlaceImageFormat::XyPixmap`]. It must be `0` for
		/// [`PlaceImageFormat::Zpixmap`].
		///
		/// # Errors
		/// A [`Match` error] is generated if `format` is
		/// [`PlaceImageFormat::Bitmap`] or [`PlaceImageFormat::XyPixmap`] and
		/// this is not less than `bitmap_scanline_padding` (given in
		/// [`connection::ConnectionSuccess`]).
		///
		/// A [`Match` error] is generated if `format` is
		/// [`PlaceImageFormat::Zpixmap`] and this is not `0`.
		///
		/// [`connection::ConnectionSuccess`]: crate::connection::ConnectionSuccess
		///
		/// [`Match` error]: error::Match
		pub left_padding: u8,

		/// The depth of the image.
		///
		/// If `format` is [`PlaceImageFormat::Bitmap`], this must be `1`.
		///
		/// # Errors
		/// A [`Match` error] is generated if `format` is
		/// [`PlaceImageFormat::XyPixmap`] or [`PlaceImageFormat::Zpixmap`]
		/// and this does not match the depth of the `target` [drawable].
		///
		/// A [`Match` error] is generated if `format` is
		/// [`PlaceImageFormat::Bitmap`] and this is not `1`.
		///
		/// [drawable]: Drawable
		///
		/// [`Match` error]: error::Match
		pub depth: u8,
		[_; 2],

		// FIXME: how do we know what is padding and what is data??????
		/// The image's data.
		#[context(self::remaining => remaining)]
		pub data: Vec<u8>,
		[_; data => pad(data)],
	}
}
