// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Requests] defined in the [core X11 protocol] that relate to the
//! configuration and creation of graphics-related types.
//!
//! [Requests] are messages sent from an X client to the X server.
//!
//! [Requests]: Request
//! [core X11 protocol]: crate::x11

extern crate self as xrb;

use xrbk::{pad, ConstantX11Size};
use xrbk_macro::{derive_xrb, Readable, Writable, X11Size};

use crate::{
	message::Request,
	set::{GraphicsOptions, GraphicsOptionsMask},
	unit::Px,
	visual::RgbColor,
	x11::{error, reply},
	CursorAppearance,
	Dimensions,
	Drawable,
	Font,
	GraphicsContext,
	Pixmap,
	Rectangle,
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
	/// [request]: Request
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
		/// [request]: Request
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
	/// [request]: Request
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
	pub enum CreateGraphicsContextError for CreateGraphicsContext {
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
	/// [request]: Request
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
	/// [request]: Request
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
		/// [request]: Request
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
	/// [request]: Request
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
	/// [request]: Request
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
		/// [request]: Request
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
	/// [request]: Request
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
		/// [request]: Request
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
	/// [request]: Request
	///
	/// [`GraphicsContext` ID]: GraphicsContext
	///
	/// [`GraphicsContext` error]: error::GraphicsContext
	#[doc(alias("FreeGc", "FreeGcontext", "FreeGraphicsContext"))]
	#[doc(alias("DestroyGc", "DestroyGcontext"))]
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
	#[doc(alias("CreateCursorError"))]
	pub enum CreateCursorAppearanceError for CreateCursorAppearance {
		ResourceIdChoice,
		Match,
		Pixmap,
	}
}

derive_xrb! {
	/// A [request] that creates a new [`CursorAppearance`].
	///
	/// The given `cursor_appearance_id` is assigned to the
	/// [`CursorAppearance`] that is created.
	///
	/// The options provided in this [request] may be arbitrarily transformed by
	/// the X server to meet display limitations.
	///
	/// The effect of changes to the `source` or `mask` [pixmaps][pixmap] made
	/// after the creation of this [`CursorAppearance`] have an undefined
	/// effect: the X server may or may not create a copy of the provided
	/// [pixmaps][pixmap].
	///
	/// # Errors
	/// An [`Alloc` error] is generated if the X server fails to allocate the
	/// [`CursorAppearance`].
	///
	/// A [`ResourceIdChoice` error] is generated if `cursor_appearance_id`
	/// specifies an ID already used for another resource, or an ID not
	/// allocated to your client.
	///
	/// A [`Match` error] is generated if `source`'s depth is not `1`.
	///
	/// A [`Match` error] is generated if `mask` is [`Some`] and `mask`'s depth
	/// is not `1`.
	///
	/// A [`Match` error] is generated if `mask` is [`Some`] and `mask` is not
	/// the same size as `source`.
	///
	/// A [`Match` error] is generated if `hotspot_x` is not contained within
	/// the `source` [pixmap].
	///
	/// A [`Match` error] is generated if `hotspot_y` is not contained within
	/// the `source` [pixmap].
	///
	/// A [`Pixmap` error] is generated if `source` does not refer to a defined
	/// [pixmap].
	///
	/// A [`Pixmap` error] is generated if `mask` is [`Some`] and does not refer
	/// to a defined [pixmap].
	///
	/// [pixmap]: Pixmap
	/// [request]: Request
	///
	/// [`Alloc` error]: error::Alloc
	/// [`ResourceIdChoice` error]: error::ResourceIdChoice
	/// [`Match` error]: error::Match
	/// [`Pixmap` error]: error::Pixmap
	#[doc(alias("CreateCursor"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct CreateCursorAppearance: Request(93, CreateCursorAppearanceError) {
		/// The [`CursorAppearance` ID] which is to be assigned to the
		/// [`CursorAppearance`].
		///
		/// # Errors
		/// A [`ResourceIdChoice` error] is generated if this resource ID is
		/// already used or if it isn't allocated to your client.
		///
		/// [`CursorAppearance` ID]: CursorAppearance
		///
		/// [`ResourceIdChoice` error]: error::ResourceIdChoice
		#[doc(alias("cid", "cursor_id", "caid"))]
		pub cursor_appearance_id: CursorAppearance,

		/// The [pixmap] that specifies the appearance of the cursor.
		///
		/// For each bit set to `1` in this [pixmap], the [`foreground_color`]
		/// is used. For each bit set to `0`, the [`background_color`] is used.
		///
		/// # Errors
		/// A [`Pixmap` error] is generated if this does not refer to a defined
		/// [pixmap].
		///
		/// A [`Match` error] is generated if this [pixmap] does not have a
		/// depth of `1`.
		///
		/// [pixmap]: Pixmap
		///
		/// [`foreground_color`]: CreateCursorAppearance::foreground_color
		/// [`background_color`]: CreateCursorAppearance::background_color
		///
		/// [`Pixmap` error]: error::Pixmap
		pub source: Pixmap,
		/// An optional [pixmap] that applies a mask to the cursor's appearance.
		///
		/// If this is [`Some`], it masks the `source` [pixmap]. For each bit
		/// set to `1`, the corresponding pixel in the `source` is shown. For
		/// each bit set to `0`, the corresponding pixel in the `source` is
		/// hidden.
		///
		/// # Errors
		/// A [`Pixmap` error] is generated if this is [`Some`] but does not
		/// refer to a defined [pixmap].
		///
		/// A [`Match` error] is generated if this is [`Some`] but does not have
		/// the same size as the `source` [pixmap].
		///
		/// A [`Match` error] is generated if this is [`Some`] but does not have
		/// a depth of `1`.
		///
		/// [pixmap]: Pixmap
		///
		/// [`Pixmap` error]: error::Pixmap
		/// [`Match` error]: error::Match
		pub mask: Option<Pixmap>,

		/// The foreground color used for the cursor's visual appearance.
		///
		/// This foreground color is used for each bit set to `1` in the
		/// `source` [pixmap].
		///
		/// [pixmap]: Pixmap
		#[doc(alias("fore_red", "fore_green", "fore_blue"))]
		#[doc(alias("foreground_red", "foreground_green", "foreground_blue"))]
		#[doc(alias("fore_color"))]
		pub foreground_color: RgbColor,
		/// The background color used for the cursor's visual appearance.
		///
		/// This background color is used for each bit set to `0` in the
		/// `source` [pixmap].
		///
		/// [pixmap]: Pixmap
		#[doc(alias("back_red", "back_green", "back_blue"))]
		#[doc(alias("background_red", "background_green", "background_blue"))]
		#[doc(alias("back_color"))]
		pub background_color: RgbColor,

		/// The x coordinate of the cursor's 'hotspot'.
		///
		/// This coordinate is relative to the top-left corner of the `source`
		/// [pixmap].
		///
		/// The hotspot refers to the point within the cursor's appearance which
		/// is placed directly over the coordinates of the cursor. For example,
		/// for a normal arrow cursor, that will be the tip of the arrow. For a
		/// pen cursor, that will be the tip of the pen.
		///
		/// # Errors
		/// A [`Match` error] is generated if this coordinate is not within the
		/// `source` [pixmap] (i.e. it is greater than or equal to its width).
		///
		/// [pixmap]: Pixmap
		///
		/// [`Match` error]: error::Match
		#[doc(alias("x"))]
		pub hotspot_x: Px<u16>,
		/// The y coordinate of the cursor's 'hotspot'.
		///
		/// This coordinate is relative to the top-left corner of the `source`
		/// [pixmap].
		///
		/// The hotspot refers to the point within the cursor's appearance which
		/// is placed directly over the coordinates of the cursor. For example,
		/// for a normal arrow cursor, that will be the tip of the arrow. For a
		/// pen cursor, that will be the tip of the pen.
		///
		/// # Errors
		/// A [`Match` error] is generated if this coordinate is not within the
		/// `source` [pixmap] (i.e. it is greater than or equal to its height).
		///
		/// [pixmap]: Pixmap
		///
		/// [`Match` error]: error::Match
		#[doc(alias("y"))]
		pub hotspot_y: Px<u16>,
	}
}

request_error! {
	#[doc(alias("CreateGlyphCursorError"))]
	pub enum CreateGlyphCursorAppearanceError for CreateGlyphCursorAppearance {
		Font,
		ResourceIdChoice,
		Value,
	}
}

derive_xrb! {
	/// A [request] that creates a new [`CursorAppearance`] using the specified
	/// glyphs.
	///
	/// The given `cursor_appearance_id` is assigned to the
	/// [`CursorAppearance`] that is created.
	///
	/// The hotspot (that is, the point that is aligned to the exact coordinates
	/// of the cursor: for a typical arrow cursor, that's the tip of the arrow)
	/// is the top-left corner of the `source_char`.
	///
	/// The options provided in this [request] may be arbitrarily transformed by
	/// the X server to meet display limitations.
	///
	/// # Errors
	/// An [`Alloc` error] is generated if the X server fails to allocate the
	/// [`CursorAppearance`]; see [`RequestError::Alloc`].
	///
	/// A [`ResourceIdChoice` error] is generated if `cursor_appearance_id`
	/// specifies an ID already used for another resource, or an ID not
	/// allocated to your client.
	///
	/// A [`Font` error] is generated if `source_font` does not refer to a
	/// defined [font].
	///
	/// A [`Font` error] is generated if `mask_font` is [`Some`] and `mask_font`
	/// does not refer to a defined [font].
	///
	/// A [`Value` error] is generated if `source_char` does not refer to a
	/// glyph defined in the [font] specified by `source_font`.
	///
	/// A [`Value` error] is generated if `mask_font` is [`Some`] but
	/// `mask_char` is [`None`].
	///
	/// A [`Value` error] is generated if both `mask_font` and `mask_char` are
	/// [`Some`] but `mask_char` does not refer to a glyph defined in the font
	/// specified by `mask_font`.
	///
	/// [font]: Font
	/// [request]: Request
	///
	/// [`RequestError::Alloc`]: crate::message::RequestError::Alloc
	/// [`Alloc` error]: error::Alloc
	/// [`ResourceIdChoice` error]: error::ResourceIdChoice
	/// [`Font` error]: error::Font
	/// [`Value` error]: error::Value
	#[doc(alias("CreateGlyphCursor"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct CreateGlyphCursorAppearance: Request(94, CreateGlyphCursorAppearanceError) {
		/// The [`CursorAppearance` ID] which is to be assigned to the
		/// [`CursorAppearance`].
		///
		/// # Errors
		/// A [`ResourceIdChoice` error] is generated if this resource ID is
		/// already used or if it isn't allocated to your client.
		///
		/// [`CursorAppearance` ID]: CursorAppearance
		///
		/// [`ResourceIdChoice` error]: error::ResourceIdChoice
		#[doc(alias("cid", "cursor_id", "caid"))]
		pub cursor_appearance_id: CursorAppearance,

		/// The [font] that is used for the [`source_char`].
		///
		/// # Errors
		/// A [`Font` error] is generated if this does not refer to a defined
		/// [font].
		///
		/// [`source_char`]: CreateGlyphCursorAppearance::source_char
		///
		/// [font]: Font
		///
		/// [`Font` error]: error::Font
		pub source_font: Font,
		/// The [font] that is used for the [`mask_char`].
		///
		/// If this is specified (is [`Some`]), the [`mask_char`] must be
		/// specified too.
		///
		/// # Errors
		/// A [`Font` error] is generated if this is [`Some`] but does not refer
		/// to a defined [font].
		///
		/// [`mask_char`]: CreateGlyphCursorAppearance::mask_char
		///
		/// [font]: Font
		///
		/// [`Font` error]: error::Font
		pub mask_font: Option<Font>,

		/// The character used as the appearance of the cursor.
		///
		/// This character is displayed in the [font] specified by
		/// [`source_font`].
		///
		/// For [fonts][font] that use two-byte matrix indexing, this value
		/// should be specified like so:
		/// ```
		/// use xrb::Char16;
		///
		/// # let (byte1, byte2) = (0, 0);
		/// #
		/// let char = Char16::new(byte1, byte2);
		///
		/// let source_char = u16::from(char);
		/// ```
		///
		/// # Errors
		/// A [`Value` error] is generated if this does not refer to a glyph
		/// defined in the [font] specified by [`source_font`].
		///
		/// [`source_font`]: CreateGlyphCursorAppearance::source_font
		///
		/// [font]: Font
		///
		/// [`Value` error]: error::Value
		pub source_char: u16,
		/// An optional character which masks the appearance of the cursor.
		///
		/// If this is [`Some`], it masks the character specified by
		/// [`source_char`].
		///
		/// For [fonts][font] that use two-byte matrix indexing, this value
		/// should be specified like so:
		/// ```
		/// use xrb::Char16;
		///
		/// # let (byte1, byte2) = (0, 0);
		/// #
		/// let char = Char16::new(byte1, byte2);
		///
		/// let mask_char = Some(u16::from(char));
		/// ```
		///
		/// # Errors
		/// A [`Value` error] is generated if [`mask_font`] is [`Some`] but this
		/// is [`None`].
		///
		/// A [`Value` error] is generated if this does not refer to a glyph
		/// defined in the [font] specified by [`mask_font`].
		///
		/// [`mask_font`]: CreateGlyphCursorAppearance::mask_font
		///
		/// [font]: Font
		///
		/// [`Value` error]: error::Value
		pub mask_char: Option<u16>,

		/// The foreground color used for the cursor's visual appearance.
		///
		/// This foreground color is used for the [`source_char`].
		///
		/// [`source_char`]: CreateGlyphCursorAppearance::source_char
		pub foreground_color: RgbColor,
		/// The background color used for the cursor's visual appearance.
		///
		/// This background color is used for areas of the cursor's visual
		/// appearance which are not the [`source_char`].
		///
		/// [`source_char`]: CreateGlyphCursorAppearance::source_char
		pub background_color: RgbColor,
	}

	/// A [request] that deletes the association between the given
	/// [`CursorAppearance` ID] and the [`CursorAppearance`] it refers to.
	///
	/// The [`CursorAppearance`] will be deleted once no resources reference it
	/// any longer.
	///
	/// # Errors
	/// A [`CursorAppearance` error] is generated if `target` does not refer to
	/// a defined [`CursorAppearance`].
	///
	/// [request]: Request
	///
	/// [`CursorAppearance` ID]: CursorAppearance
	///
	/// [`CursorAppearance` error]: error::CursorAppearance
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct DestroyCursorAppearance: Request(95, error::CursorAppearance) {
		/// The [`CursorAppearance`] that is to be deleted.
		///
		/// # Errors
		/// A [`CursorAppearance` error] is generated if this does not refer to
		/// a defined [`CursorAppearance`].
		///
		/// [`CursorAppearance` error]: error::CursorAppearance
		#[doc(alias("cursor", "cursor_appearance"))]
		pub target: CursorAppearance,
	}

	/// A [request] that changes the foreground and background colors of a
	/// [`CursorAppearance`].
	///
	/// If the [`CursorAppearance`] is currently being displayed, this change
	/// will be immediately visible.
	///
	/// # Errors
	/// A [`CursorAppearance` error] is generated if `target` does not refer to
	/// a defined [`CursorAppearance`].
	///
	/// [request]: Request
	///
	/// [`CursorAppearance` error]: error::CursorAppearance
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct RecolorCursorAppearance: Request(96, error::CursorAppearance) {
		/// The [`CursorAppearance`] which is to be recolored.
		///
		/// # Errors
		/// A [`CursorAppearance` error] is generated if this does not refer to
		/// a defined [`CursorAppearance`].
		///
		/// [`CursorAppearance` error]: error::CursorAppearance
		#[doc(alias("cursor", "cursor_appearance"))]
		pub target: CursorAppearance,

		/// The new foreground color for the [`CursorAppearance`].
		///
		/// This foreground color is used for each bit set to `1` in the
		/// `target` [`CursorAppearance`]'s `source` [pixmap].
		///
		/// If the `target`'s `source` is a character, then this foreground
		/// color is used for that character.
		///
		/// [pixmap]: Pixmap
		pub foreground_color: RgbColor,
		/// The new background color for the [`CursorAppearance`].
		///
		/// This background color is used for each bit set to `0` in the
		/// `target` [`CursorAppearance`]'s `source` [pixmap].
		///
		/// If the `target`'s `source` is a character, then this background
		/// color is used for the parts of the `target`'s `source` which is not
		/// the character.
		///
		/// [pixmap]: Pixmap
		pub background_color: RgbColor,
	}
}

request_error! {
	#[doc(alias("QueryBestSizeError"))]
	pub enum QueryIdealDimensionsError for QueryIdealDimensions {
		Drawable,
		Match,
		Value,
	}
}

/// Specifies how the ideal [dimensions] should be chosen in a
/// [`QueryIdealDimensions` request].
///
/// [dimensions]: Dimensions
///
/// [`QueryIdealDimension` request]: QueryIdealDimensions
#[doc(alias("QueryBestSizeClass", "QueryIdealDimensionsClass"))]
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum DimensionClass {
	/// The largest [`CursorAppearance`] [dimensions] that can be fully
	/// displayed are returned.
	///
	/// [dimensions]: Dimensions
	CursorAppearance,

	/// The [dimensions] which can be tiled fastest are returned.
	///
	/// See [`GraphicsOptions::tile`] for more information on tiling.
	///
	/// [dimensions]: Dimensions
	Tile,
	/// The [dimensions] which can be stippled fastest are returned.
	///
	/// See [`GraphicsOptions::stipple`] for more information on stippling.
	///
	/// [dimensions]: Dimensions
	Stipple,
}

derive_xrb! {
	/// A [request] that returns the ideal [dimensions] for the given
	/// [`DimensionClass`] and baseline [dimensions].
	///
	/// For [`DimensionClass::CursorAppearance`], the largest
	/// [`CursorAppearance`] [dimensions] that can be fully displayed are
	/// returned.
	///
	/// For [`DimensionClass::Tile`], the [dimensions] that can be tiled
	/// fastest are returned.
	///
	/// For [`DimensionClass::Stipple`], the [dimensions] that can be stippled
	/// fastest are returned.
	///
	/// # Errors
	/// A [`Drawable` error] is generated if `drawable` does not refer to a
	/// defined [window] nor [pixmap].
	///
	/// A [`Match` error] is generated if `class` is [`Tile`] or [`Stipple`] and
	/// `drawable` is an [`InputOnly`] [window].
	///
	/// [window]: crate::Window
	/// [pixmap]: Pixmap
	/// [dimensions]: Dimensions
	/// [request]: Request
	///
	/// [`Tile`]: DimensionClass::Tile
	/// [`Stipple`]: DimensionClass::Stipple
	///
	/// [`InputOnly`]: crate::WindowClass::InputOnly
	///
	/// [`Drawable` error]: error::Drawable
	/// [`Match` error]: error::Match
	#[doc(alias("QueryBestSize"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct QueryIdealDimensions: Request(
		97,
		QueryIdealDimensionsError,
	) -> reply::QueryIdealDimensions {
		/// The [class] of ideal [dimensions] requested.
		///
		/// [class]: DimensionClass
		/// [dimensions]: Dimensions
		#[metabyte]
		pub class: DimensionClass,

		/// Indicates the desired [screen] and possibly the depth as well.
		///
		/// For [`DimensionClass::CursorAppearance`], this only indicates the
		/// desired [screen].
		///
		/// For [`DimensionClass::Tile`] or [`DimensionClass::Stipple`], this
		/// possibly indicates the depth as well.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// A [`Match` error] is generated if an [`InputOnly`] [window] is used
		/// for [`DimensionClass::Tile`] or [`DimensionClass::Stipple`].
		///
		/// [window]: crate::Window
		/// [pixmap]: Pixmap
		/// [screen]: crate::visual::Screen
		///
		/// [window class]: crate::WindowClass
		/// [`InputOnly`]: crate::WindowClass::InputOnly
		///
		/// [`Drawable` error]: error::Drawable
		/// [`Match` error]: error::Match
		pub drawable: Drawable,

		/// The [dimensions] which the returned ideal [dimensions] are closest
		/// to.
		///
		/// [dimensions]: Dimensions
		#[doc(alias("width", "height"))]
		pub dimensions: Dimensions,
	}
}
