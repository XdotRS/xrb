// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Requests] defined in the [core X11 protocol] that relate to graphics
//! operations.
//!
//! [Requests] are messages sent from an X client to the X server.
//!
//! [Requests]: Request
//! [core X11 protocol]: crate::x11

mod config;
pub use config::*;

extern crate self as xrb;

use thiserror::Error;

use xrbk::{
	pad,
	Buf,
	BufMut,
	ConstantX11Size,
	ReadResult,
	Readable,
	ReadableWithContext,
	Writable,
	WriteResult,
	X11Size,
};
use xrbk_macro::{derive_xrb, ConstantX11Size, Readable, Writable, X11Size};

use crate::{
	message::Request,
	unit::Px,
	x11::{error, reply},
	Arc,
	Coords,
	Dimensions,
	Drawable,
	Font,
	GraphicsContext,
	Rectangle,
	String16,
	String8,
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
	/// [request]: Request
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
		/// [request]: Request
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
	/// [request]: Request
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
	/// [request]: Request
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
	/// [request]: Request
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
	/// [request]: Request
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
	/// [request]: Request
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
	/// [request]: Request
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
	/// [request]: Request
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
	/// [request]: Request
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
	/// [request]: Request
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
	/// [request]: Request
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
	/// [request]: Request
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

request_error! {
	#[doc(alias("GetImageError"))]
	pub enum CaptureImageError for CaptureImage {
		Drawable,
		Match,
		Value,
	}
}

/// The format of the image returned in a [`CaptureImage` reply].
///
/// This is used in the [`CaptureImage` request].
///
/// [`CaptureImage` request]: CaptureImage
/// [`CaptureImage` reply]: reply::CaptureImage
#[doc(alias("GetImageFormat"))]
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum CaptureImageFormat {
	/// The image is returned in XY format.
	XyPixmap,

	/// The image is returned in Z format.
	Zpixmap,
}

derive_xrb! {
	/// A [request] that returns the contents of the given `area` of the given
	/// [drawable] as an image.
	///
	/// If the `target` is a [window], the [window]'s border may be included in
	/// the image.
	///
	/// If the `target` is a [window], its contents are [maintained], and it is
	/// obscured by a [window] which is not one of its descendents, the
	/// [maintained] contents will be returned for those obscured regions.
	/// Otherwise, if the contents are not [maintained], or the [window] is
	/// obscured by one of its descendents, the contents of those obscured
	/// regions in the returned image are undefined. The contents of visible
	/// regions of descendents with a different depth than the `target` are also
	/// undefined.
	///
	/// The cursor is never included in the returned image.
	///
	/// # Replies
	/// This [request] generates a [`CaptureImage` reply].
	///
	/// # Errors
	/// A [`Drawable` error] is generated if `target` does not refer to a
	/// defined [window] nor [pixmap].
	///
	/// A [`Match` error] is generated if the given area is not fully contained
	/// within the `target` [drawable].
	///
	/// A [`Match` error] is generated if the `target` is a [window] and the
	/// [window] is not viewable.
	///
	/// A [`Match` error] is generated if the `target` is a [window] and the
	/// [window] is not fully contained within the [screen].
	///
	/// [drawable]: Drawable
	/// [window]: Window
	/// [pixmap]: Pixmap
	/// [screen]: crate::visual::Screen
	/// [request]: Request
	///
	/// [maintained]: crate::MaintainContents
	///
	/// [`CaptureImage` reply]: reply::CaptureImage
	///
	/// [`Drawable` error]: error::Drawable
	/// [`Match` error]: error::Match
	#[doc(alias("GetImage"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct CaptureImage: Request(73, CaptureImageError) -> reply::CaptureImage {
		/// The [image format] of the image that is returned in the
		/// [`CaptureImage` reply].
		///
		/// [image format]: CaptureImageFormat
		/// [`CaptureImage` reply]: reply::CaptureImage
		#[metabyte]
		pub format: CaptureImageFormat,

		/// The [drawable] for which this [request] captures an image from the
		/// given `area`.
		///
		/// # Errors
		/// A [`Drawable` error] is generated if this does not refer to a
		/// defined [window] nor [pixmap].
		///
		/// [drawable]: Drawable
		/// [window]: Window
		/// [pixmap]: Pixmap
		/// [request]: Request
		///
		/// [`Drawable` error]: error::Drawable
		#[doc(alias("drawable"))]
		pub target: Drawable,

		/// The area of the `target` [drawable] which this [request] captures an
		/// image of.
		///
		/// [drawable]: Drawable
		/// [request]: Request
		#[doc(alias("x", "y", "width", "height"))]
		pub area: Rectangle,

		/// A mask applied to the returned image's bit planes.
		///
		/// If the given `format` is [`CaptureImageFormat::XyPixmap`], only the
		/// bit planes specified in this mask are transmitted, and they are
		/// transmitted from most significant to least significant in bit order.
		///
		/// If the given `format` is [`CaptureImageFormat::Zpixmap`], all bits
		/// in planes not specified by this mask are zero.
		pub plane_mask: u32,
	}
}

request_error! {
	#[doc(alias("PolyText8Error"))]
	pub enum DrawText8Error for DrawText8 {
		Drawable,
		Font,
		GraphicsContext,
		Match,
	}
}

/// A 'text item' specified in a [`DrawText8` request].
///
/// [`DrawText8` request]: DrawText8
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum TextItem8 {
	/// Specifies text that is to be drawn with the `graphics_context`'s current
	/// [font].
	///
	/// [font]: Font
	Text(Box<Text8>),

	/// Changes the `graphics_context`'s current [font].
	///
	/// This new [font] will be used for subsequent text items.
	///
	/// [font]: Font
	// Font must always be big-endian
	Font(Font),
}

impl X11Size for TextItem8 {
	fn x11_size(&self) -> usize {
		match self {
			Self::Text(text) => text.x11_size(),
			Self::Font(font) => font.x11_size() + u8::X11_SIZE,
		}
	}
}

impl Readable for TextItem8 {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(match buf.get_u8() {
			font_shift if font_shift == 255 => Self::Font(Font::new(buf.get_u32())),
			string_len => Self::Text(Box::new(Text8::read_with(buf, &string_len)?)),
		})
	}
}

impl Writable for TextItem8 {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		match self {
			Self::Text(text) => text.write_to(buf)?,

			Self::Font(font) => {
				// Font-shift indicator
				buf.put_u8(255);

				font.write_to(buf)?;
			},
		}

		Ok(())
	}
}

/// A [text item] that specifies [`String8`] text to be drawn using the
/// `graphics_context`'s current [`font`].
///
/// This is used in the [`DrawText8` request].
///
/// [text item]: TextItem8
/// [`font`]: GraphicsOptions::font
///
/// [`DrawText8` request]: DrawText8
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Text8 {
	horizontal_offset: Px<i8>,
	string: String8,
}

/// An error returned when the given string is too long.
#[derive(Debug, Hash, PartialEq, Eq, Error)]
#[error("the maximum length allowed here is {max}, found {found}")]
pub struct TextTooLong {
	/// The maximum length of the string.
	pub max: u8,
	/// The length of the string that was given.
	pub found: usize,
}

impl Text8 {
	/// Creates a new `Text8` with the given `horizontal_offset` and `string`.
	///
	/// `horizontal_offset` specifies the offset that is applied to the start of
	/// the `string`.
	///
	/// # Errors
	/// A [`TextTooLong`] error is returned if `string.len() > 255`.
	pub fn new(horizontal_offset: Px<i8>, string: String8) -> Result<Self, TextTooLong> {
		if string.len() > 255 {
			Err(TextTooLong {
				max: 255,
				found: string.len(),
			})
		} else {
			Ok(Self {
				horizontal_offset,
				string,
			})
		}
	}

	/// The horizontal offset applied to the start of the [`string`].
	///
	/// [`string`]: Text8::string
	#[must_use]
	pub const fn horizontal_offset(&self) -> Px<i8> {
		self.horizontal_offset
	}

	/// The text which is to be drawn.
	#[must_use]
	pub const fn string(&self) -> &String8 {
		&self.string
	}

	/// Unwraps this `Text8`, returning the `horizontal_offset` and `string`.
	#[must_use]
	#[allow(clippy::missing_const_for_fn, reason = "false positive")]
	pub fn unwrap(self) -> (Px<i8>, String8) {
		(self.horizontal_offset, self.string)
	}
}

impl X11Size for Text8 {
	fn x11_size(&self) -> usize {
		u8::X11_SIZE + i8::X11_SIZE + self.string.x11_size()
	}
}

impl ReadableWithContext for Text8 {
	type Context = u8;

	fn read_with(buf: &mut impl Buf, string_len: &u8) -> ReadResult<Self> {
		Ok(Self {
			horizontal_offset: Px(i8::read_from(buf)?),
			string: String8::read_with(buf, &usize::from(*string_len))?,
		})
	}
}

impl Writable for Text8 {
	#[allow(clippy::cast_possible_truncation)]
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		buf.put_u8(self.string.len() as u8);
		self.horizontal_offset.write_to(buf)?;
		self.string.write_to(buf)?;

		Ok(())
	}
}

/// A [request] that draws the given [`String8`] text on the given [drawable].
///
/// # Graphics options used
/// This [request] uses the following [options] of the `graphics_context`:
/// - [`function`]
/// - [`plane_mask`]
/// - [`fill_style`]
/// - [`font`]
/// - [`child_mode`]
/// - [`clip_x`]
/// - [`clip_y`]
/// - [`clip_mask`]
///
/// This [request] may also use these [options], depending on the configuration
/// of the `graphics_context`:
/// - [`foreground_color`]
/// - [`background_color`]
/// - [`tile`]
/// - [`stipple`]
/// - [`tile_stipple_x`]
/// - [`tile_stipple_y`]
///
/// # Errors
/// A [`Drawable` error] is generated if `target` does not refer to a defined
/// [window] nor [pixmap].
///
/// A [`GraphicsContext` error] is generated if `graphics_context` does not
/// refer to a defined [`GraphicsContext`].
///
/// A [`Font` error] is generated if a [font item] given in `text_items` does
/// not refer to a defined [font]. Previous [text items] may have already been
/// drawn.
///
/// [drawable]: Drawable
/// [window]: Window
/// [pixmap]: Pixmap
/// [font]: Font
/// [font item]: TextItem8::Font
/// [text items]: TextItem8
/// [options]: GraphicsOptions
/// [request]: Request
///
/// [`function`]: GraphicsOptions::function
/// [`plane_mask`]: GraphicsOptions::plane_mask
/// [`fill_style`]: GraphicsOptions::fill_style
/// [`font`]: GraphicsOptions::font
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
/// [`Font` error]: error::Font
#[doc(alias("PolyText8"))]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DrawText8 {
	/// The [drawable] on which the text is drawn.
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

	/// The starting position of the first character of text.
	///
	/// These coordinates are relative to the top-left corner of the `target`
	/// [drawable].
	///
	/// [drawable]: Drawable
	#[doc(alias("x", "y"))]
	pub coordinates: Coords,

	/// The [text items] which control the drawing of the text.
	///
	/// Each item can either [specify text], in which case that text is drawn
	/// (with the provided [`horizontal_offset`] applied), or a [font item], in
	/// which case that [font] is stored in the `graphics_context` and used for
	/// subsequent text.
	///
	/// # Errors
	/// A [`Font` error] is generated if a [font item] does not refer to a
	/// defined [font]. Previous [text items] may have already been drawn.
	///
	/// [specify text]: TextItem8::Text
	/// [font item]: TextItem8::Font
	/// [`horizontal_offset`]: Text8::horizontal_offset
	/// [text items]: TextItem8
	///
	/// [font]: Font
	#[doc(alias("items"))]
	pub text_items: Vec<TextItem8>,
}

impl Request for DrawText8 {
	type OtherErrors = DrawText8Error;
	type Reply = ();

	const MAJOR_OPCODE: u8 = 74;
	const MINOR_OPCODE: Option<u16> = None;
}

impl X11Size for DrawText8 {
	fn x11_size(&self) -> usize {
		const HEADER: usize = 4;

		const CONSTANT_SIZES: usize = {
			HEADER
			+ Drawable::X11_SIZE // `target`
			+ GraphicsContext::X11_SIZE // `graphics_context`
			+ Coords::X11_SIZE // `coordinates`
		};

		CONSTANT_SIZES + self.text_items.x11_size() + pad(&self.text_items)
	}
}

impl Readable for DrawText8 {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		/// The maximum number of padding bytes that there could be at the end.
		///
		/// As long as there are more than this many bytes remaining, we know
		/// that there are still text items left to read.
		const MAX_PADDING: usize = 1;

		// major opcode is already read

		// Metabyte position is unused.
		buf.advance(1);

		// Read the length and bound buf to not read more than it.
		let length = (usize::from(buf.get_u16()) * 4) - 2;
		let buf = &mut buf.take(length);

		let target = Drawable::read_from(buf)?;
		let graphics_context = GraphicsContext::read_from(buf)?;
		let coordinates = Coords::read_from(buf)?;

		let text_items = {
			let mut text_items = Vec::new();

			while buf.remaining() > MAX_PADDING {
				text_items.push(TextItem8::read_from(buf)?);
			}

			text_items
		};

		// Advance the padding bytes at the end.
		buf.advance(pad(&text_items));

		Ok(Self {
			target,
			graphics_context,
			coordinates,
			text_items,
		})
	}
}

impl Writable for DrawText8 {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let buf = &mut buf.limit(self.x11_size());

		buf.put_u8(Self::MAJOR_OPCODE);
		// Unused metabyte position.
		buf.put_u8(0);
		#[cfg(not(feature = "big-requests"))]
		buf.put_u16(self.length());
		#[cfg(feature = "big-requests")]
		{
			let length = self.length();
			if let Ok(length) = length.try_into() {
				buf.put_u16(length);
			} else {
				buf.put_u16(0);
				buf.put_u32(length);
			}
		}

		self.target.write_to(buf)?;
		self.graphics_context.write_to(buf)?;
		self.coordinates.write_to(buf)?;
		self.text_items.write_to(buf)?;

		// Unused padding bytes at the end.
		buf.put_bytes(0, pad(&self.text_items));

		Ok(())
	}
}

request_error! {
	#[doc(alias("PolyText16Error"))]
	pub enum DrawText16Error for DrawText16 {
		Drawable,
		Font,
		GraphicsContext,
		Match,
	}
}

/// A 'text item' specified in a [`DrawText16` request].
///
/// [`DrawText16` request]: DrawText16
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum TextItem16 {
	/// Specifies text that is to be drawn with the `graphics_context`'s current
	/// [font].
	///
	/// [font]: Font
	Text(Box<Text16>),

	/// Changes the `graphics_context`'s current [font].
	///
	/// This new [font] will be used for subsequent text items.
	///
	/// [font]: Font
	// Font must always be big-endian
	Font(Font),
}

impl X11Size for TextItem16 {
	fn x11_size(&self) -> usize {
		match self {
			Self::Text(text) => text.x11_size(),
			Self::Font(font) => font.x11_size() + u8::X11_SIZE,
		}
	}
}

impl Readable for TextItem16 {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(match buf.get_u8() {
			font_shift if font_shift == 255 => Self::Font(Font::new(buf.get_u32())),
			string_len => Self::Text(Box::new(Text16::read_with(buf, &string_len)?)),
		})
	}
}

impl Writable for TextItem16 {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		match self {
			Self::Text(text) => text.write_to(buf)?,

			Self::Font(font) => {
				// Font-shift indicator
				buf.put_u8(255);

				font.write_to(buf)?;
			},
		}

		Ok(())
	}
}

/// A [text item] that specifies [`String16`] text to be drawn using the
/// `graphics_context`'s current [`font`].
///
/// This is used in the [`DrawText16` request].
///
/// [text item]: TextItem16
/// [`font`]: GraphicsOptions::font
///
/// [`DrawText16` request]: DrawText16
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Text16 {
	horizontal_offset: Px<i8>,
	string: String16,
}

impl Text16 {
	/// Creates a new `Text16` with the given `horizontal_offset` and `string`.
	///
	/// `horizontal_offset` specifies the offset that is applied to the start of
	/// the `string`.
	///
	/// # Errors
	/// A [`TextTooLong`] error is returned if `string.len() > 255`.
	pub fn new(horizontal_offset: Px<i8>, string: String16) -> Result<Self, TextTooLong> {
		if string.len() > 255 {
			Err(TextTooLong {
				max: 255,
				found: string.len(),
			})
		} else {
			Ok(Self {
				horizontal_offset,
				string,
			})
		}
	}

	/// The horizontal offset applied to the start of the [`string`].
	///
	/// [`string`]: Text16::string
	#[must_use]
	pub const fn horizontal_offset(&self) -> Px<i8> {
		self.horizontal_offset
	}

	/// The text which is to be drawn.
	#[must_use]
	pub const fn string(&self) -> &String16 {
		&self.string
	}

	/// Unwraps this `Text16`, returning the `horizontal_offset` and `string`.
	#[must_use]
	#[allow(clippy::missing_const_for_fn, reason = "false positive")]
	pub fn unwrap(self) -> (Px<i8>, String16) {
		(self.horizontal_offset, self.string)
	}
}

impl X11Size for Text16 {
	fn x11_size(&self) -> usize {
		u8::X11_SIZE + i8::X11_SIZE + self.string.x11_size()
	}
}

impl ReadableWithContext for Text16 {
	type Context = u8;

	fn read_with(buf: &mut impl Buf, string_len: &u8) -> ReadResult<Self> {
		Ok(Self {
			horizontal_offset: Px(i8::read_from(buf)?),
			string: String16::read_with(buf, &usize::from(*string_len))?,
		})
	}
}

impl Writable for Text16 {
	#[allow(clippy::cast_possible_truncation)]
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		buf.put_u8(self.string.len() as u8);
		self.horizontal_offset.write_to(buf)?;
		self.string.write_to(buf)?;

		Ok(())
	}
}

/// A [request] that draws the given [`String16`] text on the given [drawable].
///
/// For [fonts][font] using linear indexing rather than two-byte matrix
/// indexing, the X server will interpret each [`Char16`] as a `u16` index.
///
/// # Graphics options used
/// This [request] uses the following [options] of the `graphics_context`:
/// - [`function`]
/// - [`plane_mask`]
/// - [`fill_style`]
/// - [`font`]
/// - [`child_mode`]
/// - [`clip_x`]
/// - [`clip_y`]
/// - [`clip_mask`]
///
/// This [request] may also use these [options], depending on the configuration
/// of the `graphics_context`:
/// - [`foreground_color`]
/// - [`background_color`]
/// - [`tile`]
/// - [`stipple`]
/// - [`tile_stipple_x`]
/// - [`tile_stipple_y`]
///
/// # Errors
/// A [`Drawable` error] is generated if `target` does not refer to a defined
/// [window] nor [pixmap].
///
/// A [`GraphicsContext` error] is generated if `graphics_context` does not
/// refer to a defined [`GraphicsContext`].
///
/// A [`Font` error] is generated if a [font item] given in `text_items` does
/// not refer to a defined [font]. Previous [text items] may have already been
/// drawn.
///
/// [drawable]: Drawable
/// [window]: Window
/// [pixmap]: Pixmap
/// [font]: Font
/// [font item]: TextItem16::Font
/// [text items]: TextItem16
/// [options]: GraphicsOptions
/// [request]: Request
///
/// [`Char16`]: crate::Char16
///
/// [`function`]: GraphicsOptions::function
/// [`plane_mask`]: GraphicsOptions::plane_mask
/// [`fill_style`]: GraphicsOptions::fill_style
/// [`font`]: GraphicsOptions::font
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
/// [`Font` error]: error::Font
#[doc(alias("PolyText16"))]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DrawText16 {
	/// The [drawable] on which the text is drawn.
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

	/// The starting position of the first character of text.
	///
	/// These coordinates are relative to the top-left corner of the `target`
	/// [drawable].
	///
	/// [drawable]: Drawable
	#[doc(alias("x", "y"))]
	pub coordinates: Coords,

	/// The [text items] which control the drawing of the text.
	///
	/// Each item can either [specify text], in which case that text is drawn
	/// (with the provided [`horizontal_offset`] applied), or a [font item], in
	/// which case that [font] is stored in the `graphics_context` and used for
	/// subsequent text.
	///
	/// # Errors
	/// A [`Font` error] is generated if a [font item] does not refer to a
	/// defined [font]. Previous [text items] may have already been drawn.
	///
	/// [specify text]: TextItem16::Text
	/// [font item]: TextItem16::Font
	/// [`horizontal_offset`]: Text16::horizontal_offset
	/// [text items]: TextItem16
	///
	/// [font]: Font
	#[doc(alias("items"))]
	pub text_items: Vec<TextItem16>,
}

impl Request for DrawText16 {
	type OtherErrors = DrawText8Error;
	type Reply = ();

	const MAJOR_OPCODE: u8 = 75;
	const MINOR_OPCODE: Option<u16> = None;
}

impl X11Size for DrawText16 {
	fn x11_size(&self) -> usize {
		const HEADER: usize = 4;

		const CONSTANT_SIZES: usize = {
			HEADER
				+ Drawable::X11_SIZE // `target`
				+ GraphicsContext::X11_SIZE // `graphics_context`
				+ Coords::X11_SIZE // `coordinates`
		};

		CONSTANT_SIZES + self.text_items.x11_size() + pad(&self.text_items)
	}
}

impl Readable for DrawText16 {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		/// The maximum number of padding bytes that there could be at the end.
		///
		/// As long as there are more than this many bytes remaining, we know
		/// that there are still text items left to read.
		const MAX_PADDING: usize = 1;

		// major opcode is already read

		// Metabyte position is unused.
		buf.advance(1);

		// Read the length and bound buf to not read more than it.
		let length = (usize::from(buf.get_u16()) * 4) - 2;
		let buf = &mut buf.take(length);

		let target = Drawable::read_from(buf)?;
		let graphics_context = GraphicsContext::read_from(buf)?;
		let coordinates = Coords::read_from(buf)?;

		let text_items = {
			let mut text_items = Vec::new();

			while buf.remaining() > MAX_PADDING {
				text_items.push(TextItem16::read_from(buf)?);
			}

			text_items
		};

		// Advance the padding bytes at the end.
		buf.advance(pad(&text_items));

		Ok(Self {
			target,
			graphics_context,
			coordinates,
			text_items,
		})
	}
}

impl Writable for DrawText16 {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let buf = &mut buf.limit(self.x11_size());

		buf.put_u8(Self::MAJOR_OPCODE);
		// Unused metabyte position.
		buf.put_u8(0);
		#[cfg(not(feature = "big-requests"))]
		buf.put_u16(self.length());
		#[cfg(feature = "big-requests")]
		{
			let length = self.length();
			if let Ok(length) = length.try_into() {
				buf.put_u16(length);
			} else {
				buf.put_u16(0);
				buf.put_u32(length);
			}
		}

		self.target.write_to(buf)?;
		self.graphics_context.write_to(buf)?;
		self.coordinates.write_to(buf)?;
		self.text_items.write_to(buf)?;

		// Unused padding bytes at the end.
		buf.put_bytes(0, pad(&self.text_items));

		Ok(())
	}
}

request_error! {
	pub enum ImageText8Error for ImageText8 {
		Drawable,
		GraphicsContext,
		Match,
	}
}

derive_xrb! {
	/// A [request] that draws [`String8`] text on a rectangular background on
	/// the given [drawable].
	///
	/// The text is filled with the `graphics_context`'s [`foreground_color`],
	/// while the background is filled with the `graphics_context`'s
	/// [`background_color`]
	///
	/// In relation to the text extents returned in the
	/// [`QueryTextExtents` reply], the background [rectangle] is defined as:
	/// ```
	/// # use xrb::{Rectangle, Coords, unit::Px};
	/// #
	/// # fn main() -> Result<(), <u16 as TryFrom<i16>>::Error> {
	/// #     let coordinates = Coords::new(Px(0), Px(0));
	/// #
	/// #     let (font_ascent, font_descent) = (Px(0), Px(0));
	/// #     let overall_width = Px(1);
	/// #
	/// Rectangle {
	///     x: coordinates.x,
	///     y: coordinates.y - font_ascent,
	///     width: overall_width,
	///     height: (font_ascent + font_descent).map(|height| height as u16),
	/// }
	/// #     ;
	/// #
	/// #     Ok(())
	/// # }
	/// ```
	///
	/// `graphics_context`'s [`function`] and [`fill_style`] are ignored in this
	/// [request]. Effectively, [`Function::Copy`] and [`FillStyle::Solid`] are
	/// used.
	///
	/// For [fonts] using two-byte indexing, each [`Char8`] `char` is
	/// interpreted as <code>[Char16]::[new](0, char.[unwrap()])</code>.
	///
	/// [`Char8`]: crate::Char8
	/// [unwrap()]: crate::Char8::unwrap
	///
	/// [Char16]: crate::Char16
	/// [new]: crate::Char16::new
	///
	/// # Graphics options used
	/// This [request] uses the following [options] of the `graphics_context`:
	/// - [`plane_mask`]
	/// - [`foreground_color`]
	/// - [`background_color`]
	/// - [`font`]
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
	/// [fonts]: Font
	/// [rectangle]: Rectangle
	/// [options]: GraphicsOptions
	/// [request]: Request
	///
	/// [`Function::Copy`]: crate::set::Function
	/// [`FillStyle::Solid`]: crate::set::FillStyle::Solid
	///
	/// [`plane_mask`]: GraphicsOptions::plane_mask
	/// [`foreground_color`]: GraphicsOptions::foreground_color
	/// [`background_color`]: GraphicsOptions::background_color
	/// [`font`]: GraphicsOptions::font
	/// [`child_mode`]: GraphicsOptions::child_mode
	/// [`clip_x`]: GraphicsOptions::clip_x
	/// [`clip_y`]: GraphicsOptions::clip_y
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	///
	/// [`QueryTextExtents` reply]: reply::QueryTextExtents
	///
	/// [`Drawable` error]: error::Drawable
	/// [`GraphicsContext` error]: error::GraphicsContext
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ImageText8: Request(76, ImageText8Error) {
		// The length of `string`.
		#[metabyte]
		#[allow(clippy::cast_possible_truncation)]
		let string_len: u8 = string => string.len() as u8,

		/// The [drawable] on which the text is drawn.
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

		/// The coordinates of the background [rectangle] before the
		/// `font_ascent` is subtracted[^subtracted].
		///
		/// [^subtracted]: See the [request docs].
		///
		/// [rectangle]: Rectangle
		/// [request docs]: ImageText8
		pub coordinates: Coords,

		/// The text which is to be drawn.
		#[context(string_len => usize::from(*string_len))]
		pub string: String8,
		[_; string => pad(string)],
	}
}

request_error! {
	pub enum ImageText16Error for ImageText16 {
		Drawable,
		GraphicsContext,
		Match,
	}
}

derive_xrb! {
	/// A [request] that draws [`String16`] text on a rectangular background on
	/// the given [drawable].
	///
	/// The text is filled with the `graphics_context`'s [`foreground_color`],
	/// while the background is filled with the `graphics_context`'s
	/// [`background_color`]
	///
	/// In relation to the text extents returned in the
	/// [`QueryTextExtents` reply], the background [rectangle] is defined as:
	/// ```
	/// # use xrb::{Rectangle, Coords, unit::Px};
	/// #
	/// # fn main() -> Result<(), <u16 as TryFrom<i16>>::Error> {
	/// #     let coordinates = Coords::new(Px(0), Px(0));
	/// #
	/// #     let (font_ascent, font_descent) = (Px(0), Px(0));
	/// #     let overall_width = Px(1);
	/// #
	/// Rectangle {
	///     x: coordinates.x,
	///     y: coordinates.y - font_ascent,
	///     width: overall_width,
	///     height: (font_ascent + font_descent).map(|height| height as u16),
	/// }
	/// #     ;
	/// #
	/// #     Ok(())
	/// # }
	/// ```
	///
	/// `graphics_context`'s [`function`] and [`fill_style`] are ignored in this
	/// [request]. Effectively, [`Function::Copy`] and [`FillStyle::Solid`] are
	/// used.
	///
	/// For [fonts] using linear indexing, each [`Char16`] `char` is interpreted
	/// as a big-endian `u16` value.
	///
	/// # Graphics options used
	/// This [request] uses the following [options] of the `graphics_context`:
	/// - [`plane_mask`]
	/// - [`foreground_color`]
	/// - [`background_color`]
	/// - [`font`]
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
	/// [fonts]: Font
	/// [rectangle]: Rectangle
	/// [options]: GraphicsOptions
	/// [request]: Request
	///
	/// [`Char16`]: crate::Char16
	///
	/// [`Function::Copy`]: crate::set::Function
	/// [`FillStyle::Solid`]: crate::set::FillStyle::Solid
	///
	/// [`plane_mask`]: GraphicsOptions::plane_mask
	/// [`foreground_color`]: GraphicsOptions::foreground_color
	/// [`background_color`]: GraphicsOptions::background_color
	/// [`font`]: GraphicsOptions::font
	/// [`child_mode`]: GraphicsOptions::child_mode
	/// [`clip_x`]: GraphicsOptions::clip_x
	/// [`clip_y`]: GraphicsOptions::clip_y
	/// [`clip_mask`]: GraphicsOptions::clip_mask
	///
	/// [`QueryTextExtents` reply]: reply::QueryTextExtents
	///
	/// [`Drawable` error]: error::Drawable
	/// [`GraphicsContext` error]: error::GraphicsContext
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ImageText16: Request(77, ImageText16Error) {
		// The length of `string`.
		#[metabyte]
		#[allow(clippy::cast_possible_truncation)]
		let string_len: u8 = string => string.len() as u8,

		/// The [drawable] on which the text is drawn.
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

		/// The coordinates of the background [rectangle] before the
		/// `font_ascent` is subtracted[^subtracted].
		///
		/// [^subtracted]: See the [request docs].
		///
		/// [rectangle]: Rectangle
		/// [request docs]: ImageText16
		pub coordinates: Coords,

		/// The text which is to be drawn.
		#[context(string_len => usize::from(*string_len))]
		pub string: String16,
		[_; string => pad(string)],
	}
}
