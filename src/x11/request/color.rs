// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Requests] defined in the [core X11 protocol] that relate to colors.
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
	visual::{ColorId, RgbColor, VisualId},
	x11::{error, reply},
	ColorChannelMask,
	Colormap,
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
	pub enum CreateColormapError for CreateColormap {
		ResourceIdChoice,
		Match,
		Value,
		Window,
	}
}

// FIXME: docs for `InitialColormapAllocation` and `CreateColormap` need to be
//        rewritten when it comes to the visual classes and what they do.

/// Whether a [colormap] initially has [no entries allocated] or
/// [all entries allocated].
///
/// [no entries allocated]: InitialColormapAllocation::None
/// [all entries allocated]: InitialColormapAllocation::All
///
/// [colormap]: Colormap
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum InitialColormapAllocation {
	/// The [colormap] initially has no entries, or those initial entries are
	/// defined elsewhere.
	///
	/// For [`VisualClass::StaticGray`], [`VisualClass::StaticColor`], and
	/// [`VisualClass::TrueColor`], the initial entries are defined, but by the
	/// specific visual, not by the [core X11 protocol].
	///
	/// For [`VisualClass::GrayScale`], [`VisualClass::PseudoColor`], and
	/// [`VisualClass::DirectColor`], the initial entries are undefined.
	///
	/// Clients can allocate entries after the [colormap] is created.
	///
	/// [colormap]: Colormap
	/// [core X11 protocol]: crate::x11
	///
	/// [`VisualClass::StaticGray`]: crate::visual::VisualClass::StaticGray
	/// [`VisualClass::StaticColor`]: crate::visual::VisualClass::StaticColor
	/// [`VisualClass::TrueColor`]: crate::visual::VisualClass::TrueColor
	///
	/// [`VisualClass::GrayScale`]: crate::visual::VisualClass::GrayScale
	/// [`VisualClass::PseudoColor`]: crate::visual::VisualClass::PseudoColor
	/// [`VisualClass::DirectColor`]: crate::visual::VisualClass::DirectColor
	None,

	/// The entire [colormap] is allocated as writable.
	///
	/// None of these entries can be removed with [`DeleteColormapEntry`].
	///
	/// [colormap]: Colormap
	All,
}

derive_xrb! {
	/// A [request] that creates a [colormap] for the given [window]'s [screen].
	///
	/// # Errors
	/// A [`ResourceIdChoice` error] is generated if the given `colormap_id` is
	/// already in use or if it is not allocated to your client.
	///
	/// A [`Window` error] is generated if `window` does not refer to a defined
	/// [window].
	///
	/// A [`Match` error] is generated if the given `visual` does not match the
	/// [visual type] of the `target` [window]'s [screen].
	///
	/// A [`Match` error] is generated the `visual` is
	/// [`VisualClass::StaticGray`], [`VisualClass::StaticColor`], or
	/// [`VisualClass::TrueColor`] and `alloc` is not
	/// [`InitialColormapAllocation::None`].
	///
	/// [colormap]: Colormap
	/// [window]: Window
	/// [screen]: crate::visual::Screen
	/// [visual type]: crate::visual::VisualType
	/// [request]: Request
	///
	/// [`VisualClass::StaticGray`]: crate::visual::VisualClass::StaticGray
	/// [`VisualClass::StaticColor`]: crate::visual::VisualClass::StaticColor
	/// [`VisualClass::TrueColor`]: crate::visual::VisualClass::TrueColor
	///
	/// [`ResourceIdChoice` error]: error::ResourceIdChoice
	/// [`Window` error]: error::Window
	/// [`Match` error]: error::Match
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct CreateColormap: Request(78, CreateColormapError) {
		/// Whether this [colormap] begins with [no entries allocated] or
		/// [all entries allocated].
		///
		/// See [`InitialColormapAllocation`] for more information.
		///
		/// [no entries allocated]: InitialColormapAllocation::None
		/// [all entries allocated]: InitialColormapAllocation::All
		///
		/// [colormap]: Colormap
		#[metabyte]
		pub initial_allocation: InitialColormapAllocation,

		/// The [`Colormap` ID] that will be associated with the [colormap].
		///
		/// # Errors
		/// A [`ResourceIdChoice` error] is generated if this [`Colormap` ID] is
		/// already in use or if it is not allocated to your client.
		///
		/// [colormap]: Colormap
		/// [`Colormap` ID]: Colormap
		///
		/// [`ResourceIdChoice` error]: error::ResourceIdChoice
		pub colormap_id: Colormap,

		/// The [window] for which this [colormap] is created.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		/// [colormap]: Colormap
		///
		/// [`Window` error]: error::Window
		pub window: Window,

		/// The [colormap]'s [visual type].
		///
		/// # Errors
		/// A [`Match` error] is generated if the [visual type] is not one
		/// supported by [window]'s [screen].
		///
		/// A [`Match` error] is generated if [visual class] is [`StaticGray`],
		/// [`StaticColor`], or [`TrueColor`] but `alloc` is not
		/// [`InitialColormapAllocation::None`].
		///
		/// [colormap]: Colormap
		/// [window]: Window
		/// [screen]: crate::visual::Screen
		/// [visual type]: crate::visual::VisualType
		/// [visual class]: crate::visual::VisualClass
		///
		/// [`StaticGray`]: crate::visual::VisualClass::StaticGray
		/// [`StaticColor`]: crate::visual::VisualClass::StaticColor
		/// [`TrueColor`]: crate::visual::VisualClass::TrueColor
		///
		/// [`Match` error]: error::Match
		pub visual: VisualId,
	}

	/// A [request] that deletes the given [colormap].
	///
	/// The association between the [`Colormap` ID] and the [colormap] itself is
	/// removed in the process.
	///
	/// If the [colormap] is installed on a [screen], it is [uninstalled]. If
	/// the [colormap] is a [window]'s [`colormap` attribute], the
	/// [`colormap` attribute] is set to [`None`] and a [`Colormap` event] is
	/// generated.
	///
	/// # Errors
	/// A [`Colormap` error] is generated if `target` does not refer to a
	/// defined [colormap].
	///
	/// [colormap]: Colormap
	/// [window]: Window
	/// [screen]: crate::visual::Screen
	/// [request]: Request
	///
	/// [uninstalled]: UninstallColormap
	///
	/// [`Colormap` ID]: Colormap
	/// [`colormap` attribute]: crate::set::Attributes::colormap
	///
	/// [`Colormap` event]: crate::x11::event::Colormap
	///
	/// [`Colormap` error]: error::Colormap
	#[doc(alias("FreeColormap"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct DestroyColormap: Request(79, error::Colormap) {
		/// The [colormap] which is to be deleted.
		///
		/// # Errors
		/// A [`Colormap` error] is generated if this does not refer to a
		/// defined [colormap].
		///
		/// [colormap]: Colormap
		///
		/// [`Colormap` error]: error::Colormap
		#[doc(alias("colormap", "cmap", "map"))]
		pub target: Colormap,
	}
}

request_error! {
	#[doc(alias("CopyColormapAndFreeError"))]
	pub enum MoveColormapError for MoveColormap {
		Colormap,
		ResourceIdChoice,
	}
}

derive_xrb! {
	/// A [request] that moves all of the values of a `source` [colormap] into a
	/// new [colormap], then destroys the `source` [colormap].
	///
	/// # Errors
	/// A [`ResourceIdChoice` error] is generated if `colormap_id` is already in
	/// use or if it is not allocated to your client.
	///
	/// A [`Colormap` error] is generated if `source` does not refer to a
	/// defined [colormap].
	///
	/// [colormap]: Colormap
	/// [request]: Request
	///
	/// [`ResourceIdChoice` error]: error::ResourceIdChoice
	/// [`Colormap` error]: error::Colormap
	#[doc(alias("CopyColormapAndFree"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct MoveColormap: Request(80, MoveColormapError) {
		/// The [`Colormap` ID] that will be associated with the new [colormap].
		///
		/// # Errors
		/// A [`ResourceIdChoice` error] is generated if this [`Colormap` ID] is
		/// already in use or if it is not allocated to your client.
		///
		/// [colormap]: Colormap
		///
		/// [`Colormap` ID]: Colormap
		///
		/// [`ResourceIdChoice` error]: error::ResourceIdChoice
		pub colormap_id: Colormap,

		/// The [colormap] which is copied to create the new [colormap], then
		/// destroyed.
		///
		/// # Errors
		/// A [`Colormap` error] is generated if this does not refer to a
		/// defined [colormap].
		///
		/// [colormap]: Colormap
		///
		/// [`Colormap` error]: error::Colormap
		pub source: Colormap,
	}

	/// A [request] that installs the given [colormap] on its [screen].
	///
	/// All [windows][window] associated with the given [colormap] will
	/// immediately switch to their true colors.
	///
	/// If the given `target` [colormap] was not already installed,
	/// a [`Colormap` event] is generated for every [window] which specifies the
	/// [colormap] in its [`colormap` attribute].
	///
	/// As a side effect of this [request], additional [colormaps][colormap] may
	/// be implicitly installed or uninstalled by the X server. For each
	/// [colormap] that is implicitly installed or uninstalled as a result of
	/// this [request], a [`Colormap` event] is generated for every [window]
	/// which specifies that [colormap] in its [`colormap` attribute].
	///
	/// ## Required colormaps list
	/// When a [colormap] is explicitly installed (that is, it is installed with
	/// this [request]), it is added as the head of the [screen]'s required
	/// [colormaps][colormap] list.
	///
	/// The length of the required [colormaps][colormap] list is no more than
	/// the [screen]'s [`min_installed_colormaps`]: if installing this
	/// [colormap] causes the list to exceed that limit, the list is truncated
	/// at the tail to make room.
	///
	/// [Explicitly uninstalling](UninstallColormap) a [colormap] means to
	/// remove it from the list of required [colormaps][colormap]. It may or may
	/// not actually be uninstalled as a result.
	///
	/// The X server may implicitly uninstall any [colormap] _not_ in the list
	/// of required [colormaps][colormap] at any time.
	///
	/// # Errors
	/// A [`Colormap` error] is generated if `target` does not refer to a
	/// defined [colormap].
	///
	/// [colormap]: Colormap
	/// [window]: Window
	/// [screen]: crate::visual::Screen
	/// [request]: Request
	///
	/// [`colormap` attribute]: crate::set::Attributes::colormap
	/// [`min_installed_colormaps`]: crate::visual::Screen::min_installed_colormaps
	///
	/// [`Colormap` event]: crate::x11::event::Colormap
	///
	/// [`Colormap` error]: error::Colormap
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct InstallColormap: Request(81, error::Colormap) {
		/// The [colormap] that is to be installed.
		///
		/// # Errors
		/// A [`Colormap` error] is generated if this does not refer to a
		/// defined [colormap].
		///
		/// [colormap]: Colormap
		///
		/// [`Colormap` error]: error::Colormap
		pub target: Colormap,
	}

	/// A [request] that removes the given [colormap] from the
	/// list of required colormaps for its [screen].
	///
	/// If the given [colormap] is uninstalled as a result of this [request], a
	/// [`Colormap` event] is generated for every [window] which specifies the
	/// [colormap] in its [`colormap` attribute].
	///
	/// As a side effect of this [request], additional [colormaps][colormap] may
	/// be implicitly installed or uninstalled by the X server. For each
	/// [colormap] that is implicitly installed or uninstalled as a result of
	/// this [request], a [`Colormap` event] is generated for every [window]
	/// which specifies that [colormap] in its [`colormap` attribute].
	///
	/// ## Required colormaps list
	/// When a [colormap] is [explicitly installed](InstallColormap), it is
	/// added as the head of the [screen]'s required [colormaps][colormap] list.
	///
	/// The length of the required [colormaps][colormap] list is no more than
	/// the [screen]'s [`min_installed_colormaps`].
	///
	/// Explicitly uninstalling a [colormap] (that is, it is uninstalled with
	/// this [request]) actually removes that [colormap] from the [screen]'s
	/// list of required [colormaps][colormap]. As a result, it may or may not
	/// be uninstalled by the X server.
	///
	/// The X server may implicitly uninstall any [colormap] _not_ in the list
	/// of required [colormaps][colormap] at any time.
	///
	/// # Errors
	/// A [`Colormap` error] is generated if `target` does not refer to a
	/// defined [colormap].
	///
	/// [colormap]: Colormap
	/// [window]: Window
	/// [screen]: crate::visual::Screen
	/// [request]: Request
	///
	/// [`colormap` attribute]: crate::set::Attributes::colormap
	/// [`min_installed_colormaps`]: crate::visual::Screen::min_installed_colormaps
	///
	/// [`Colormap` event]: crate::x11::event::Colormap
	///
	/// [`Colormap` error]: error::Colormap
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct UninstallColormap: Request(82, error::Colormap) {
		/// The [colormap] that is to be uninstalled.
		///
		/// # Errors
		/// A [`Colormap` error] is generated if this does not refer to a
		/// defined [colormap].
		///
		/// [colormap]: Colormap
		///
		/// [`Colormap` error]: error::Colormap
		pub target: Colormap,
	}

	/// A [request] that returns a list of the given [window]'s [screen]'s
	/// currently installed [colormaps].
	///
	/// # Replies
	/// This [request] generates a [`ListInstalledColormaps` reply].
	///
	/// # Errors
	/// A [`Window` error] is generated if `target` does not refer to a defined
	/// [window].
	///
	/// [colormaps]: Colormap
	/// [window]: Window
	/// [screen]: crate::visual::Screen
	/// [request]: Request
	///
	/// [`ListInstalledColormaps` reply]: reply::ListInstalledColormaps
	///
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ListInstalledColormaps: Request(83, error::Window) -> reply::ListInstalledColormaps {
		/// The [window] for which this [request] returns its installed
		/// [colormaps].
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [colormaps]: Colormap
		/// [window]: Window
		/// [request]: Request
		///
		/// [`Window` error]: error::Window
		pub target: Window,
	}

	/// A [request] that allocates a read-only [colormap] entry for the given
	/// color on the given [colormap].
	///
	/// The closest [RGB values] provided by the display are chosen.
	///
	/// Multiple clients may be assigned the same read-only [colormap] entry if
	/// they request the same closest [RGB values].
	///
	/// The [`ColorId`] and [RGB values] that were actually used are returned.
	///
	/// # Replies
	/// This [request] generates an [`AllocateColor` reply].
	///
	/// # Errors
	/// A [`Colormap` error] is generated if `target` does not refer to a
	/// defined [colormap].
	///
	/// [RGB values]: RgbColor
	/// [colormap]: Colormap
	/// [request]: Request
	///
	/// [`ColorId`]: crate::visual::ColorId
	///
	/// [`AllocateColor` reply]: reply::AllocateColor
	///
	/// [`Colormap` error]: error::Colormap
	#[doc(alias("AllocColor"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct AllocateColor: Request(84, error::Colormap) -> reply::AllocateColor {
		/// The [colormap] for which the [colormap] entry is allocated.
		///
		/// # Errors
		/// A [`Colormap` error] is generated if this does not refer to a
		/// defined [colormap].
		///
		/// [colormap]: Colormap
		///
		/// [`Colormap` error]: error::Colormap
		pub target: Colormap,

		/// The color which is to be allocated.
		pub color: RgbColor,
		[_; 2],
	}
}

request_error! {
	#[doc(alias("AllocNamedColor"))]
	pub enum AllocateNamedColorError for AllocateNamedColor {
		Colormap,
		Name,
	}
}

derive_xrb! {
	/// A [request] that allocates a read-only [colormap] entry on the given
	/// [colormap] for the color by the given `name`.
	///
	/// The `name` is looked up on the [screen] associated with the [colormap].
	///
	/// # Errors
	/// A [`Colormap` error] is generated if `target` does not refer to a
	/// defined [colormap].
	///
	/// A [`Name` error] is generated if no color by the given `name` exists for
	/// the [screen] associated with the `target` [colormap].
	///
	/// [colormap]: Colormap
	/// [screen]: crate::visual::Screen
	/// [request]: Request
	///
	/// [`Colormap` error]: error::Colormap
	/// [`Name` error]: error::Name
	#[doc(alias("AllocNamedColor"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct AllocateNamedColor: Request(
		85,
		AllocateNamedColorError,
	) -> reply::AllocateNamedColor {
		/// The [colormap] for which a [colormap] entry for the color by the
		/// given `name` is allocated.
		///
		/// # Errors
		/// A [`Colormap` error] is generated if this does not refer to a
		/// defined [colormap].
		///
		/// [colormap]: Colormap
		///
		/// [`Colormap` error]: error::Colormap
		pub target: Colormap,

		// The length of `name`.
		#[allow(clippy::cast_possible_truncation)]
		let name_len: u16 = name => name.len() as u16,
		[_; 2],

		/// The name of the color that is looked up on the [screen] associated
		/// with the [colormap].
		///
		/// The name should be specified in ISO Latin-1 encoding. Which case (e.g.
		/// uppercase, lowercase) the name is specified in does not matter.
		///
		/// [colormap]: Colormap
		/// [screen]: crate::visual::Screen
		#[context(name_len => usize::from(*name_len))]
		pub name: String8,
		[_; name => pad(name)],
	}
}

request_error! {
	#[doc(alias("AllocColorCellsError"))]
	pub enum AllocateColorCellsError for AllocateColorCells {
		Colormap,
		Value,
	}
}

derive_xrb! {
	// TODO: improve docs
	/// A [request] that allocates a [colormap] entry for every color and plane
	/// mask combination up to the given `color_count` and `plane_mask_count`.
	///
	/// By applying each plane mask to each color,
	/// <code>color_count * 2<sup>plane_count</sup></code> distinct [colormap]
	/// entries are allocated.
	///
	/// Each [colormap] entry is allocated as writable.
	///
	/// The initial RGB values of the allocated [colormap] entries are
	/// undefined.
	///
	/// # Replies
	/// This [request] generates an [`AllocateColorCells` reply].
	///
	/// # Errors
	/// A [`Colormap` error] is generated if `target` does not refer to a
	/// defined [colormap].
	///
	/// A [`Value` error] is generated if `color_count` is not greater than
	/// zero.
	///
	/// [colormap]: Colormap
	/// [request]: Request
	///
	/// [`AllocateColorCells` reply]: reply::AllocateColorCells
	///
	/// [`Colormap` error]: error::Colormap
	/// [`Value` error]: error::Value
	#[doc(alias("AllocColorCells"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct AllocateColorCells: Request(
		86,
		AllocateColorCellsError,
	) -> reply::AllocateColorCells {
		/// Whether the plane masks should be combined (i.e. bitwise OR) to form
		/// a contiguous set of bits for each color channel.
		///
		/// For [`VisualClass::GrayScale`] or [`VisualClass::PseudoColor`], one
		/// contiguous set of bits are formed by bitwise OR-ing all of the plane
		/// masks.
		///
		/// For [`VisualClass::DirectColor`], three (one for each color channel:
		/// red, green, and blue) contiguous sets of bits are formed by bitwise
		/// OR-ing all of the plane masks.
		///
		/// [`VisualClass::GrayScale`]: crate::visual::VisualClass::GrayScale
		/// [`VisualClass::PseudoColor`]: crate::visual::VisualClass::PseudoColor
		/// [`VisualClass::DirectColor`]: crate::visual::VisualClass::DirectColor
		#[metabyte]
		pub contiguous: bool,

		/// The [colormap] for which the [colormap] entries are allocated.
		///
		/// # Errors
		/// A [`Colormap` error] is generated if this does not refer to a
		/// defined [colormap].
		///
		/// [colormap]: Colormap
		///
		/// [`Colormap` error]: error::Colormap
		#[doc(alias("colormap"))]
		pub target: Colormap,

		/// The number of colors that are to be allocated.
		///
		/// # Errors
		/// A [`Value` error] is generated if this is not greater than zero.
		///
		/// [`Value` error]: error::Value
		pub color_count: u16,
		/// The number of bit plane masks that are to be allocated.
		pub plane_count: u16,
	}
}

request_error! {
	#[doc(alias("AllocColorPlanesError"))]
	pub enum AllocateColorPlanesError for AllocateColorPlanes {
		Colormap,
		Value,
	}
}

derive_xrb! {
	// TODO: improve docs
	/// A [request] that allocates a [colormap] entry for every color and bit
	/// plane combination up to the given `color_count` and plane counts.
	///
	/// By combining subsets of plane masks with colors, a total of
	/// <code>color_count * 2<sup>plane_count</sup></code> (where `plane_count`
	/// represents `red_plane_count + green_plane_count + blue_plane_count`)
	/// [colormap] entries are allocated:
	/// <code>color_count * 2<sup>red_plane_count</sup></code> independent red
	/// entries, <code>color_count * 2<sup>green_plane_count</sup></code>
	/// independent green entries, and
	/// <code>color_count * 2<sup>blue_plane_count</sup></code> independent blue
	/// entries.
	///
	/// Each [colormap] entry is allocated as writable.
	///
	/// The initial RGB values of the allocated [colormap] entries are
	/// undefined.
	///
	/// # Replies
	/// This [request] generates an [`AllocateColorPlanes` reply].
	///
	/// # Errors
	/// A [`Colormap` error] is generated if `target` does not refer to a
	/// defined [colormap].
	///
	/// A [`Value` error] is generated if `color_count` is not greater than
	/// zero.
	///
	/// An [`Alloc` error] is generated if the X server failed to allocate the
	/// requested [colormap] entries; see [`RequestError::Alloc`].
	///
	/// [colormap]: Colormap
	/// [request]: Request
	///
	/// [`AllocateColorPlanes` reply]: reply::AllocateColorPlanes
	///
	/// [`Colormap` error]: error::Colormap
	/// [`Value` error]: error::Value
	/// [`Alloc` error]: error::Alloc
	///
	/// [`RequestError::Alloc`]: crate::message::RequestError::Alloc
	#[doc(alias("AllocColorPlanes"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct AllocateColorPlanes: Request(
		87,
		AllocateColorPlanesError,
	) -> reply::AllocateColorPlanes {
		/// Whether the returned plane masks will have a contiguous set of bits.
		#[metabyte]
		pub contiguous: bool,

		/// The [colormap] for which the [colormap] entries are allocated.
		///
		/// # Errors
		/// A [`Colormap` error] is generated if this does not refer to a
		/// defined [colormap].
		///
		/// [colormap]: Colormap
		///
		/// [`Colormap` error]: error::Colormap
		#[doc(alias("colormap"))]
		pub target: Colormap,

		/// The number of colors that will be returned.
		///
		/// # Errors
		/// A [`Value` error] is generated if this is not greater than zero.
		///
		/// [`Value` error]: error::Value
		pub color_count: u16,

		/// The number of bits set in the returned `red_plane_mask`.
		pub red_plane_count: u16,
		/// The number of bits set in the returned `green_plane_mask`.
		pub green_plane_count: u16,
		/// The number of bits set in the returned `blue_plane_mask`.
		pub blue_plane_count: u16,
	}
}

request_error! {
	#[doc(alias("FreeColorsError"))]
	pub enum DestroyColormapEntriesError for DestroyColormapEntries {
		Access,
		Colormap,
		Value,
	}
}

derive_xrb! {
	/// A [request] that deletes every requested [colormap] entry that was
	/// allocated by your client in the given [colormap].
	///
	/// The [colormap] entries to be deleted are found by applying every subset
	/// of the given `plane_mask` with each of the given `colors` - the entries
	/// which were allocated by your client are deleted.
	///
	/// The requested [colormap] entries are only actually deleted under the
	/// following conditions:
	/// - If you have allocated a particular [colormap] entry multiple times,
	///   you must send a `DestroyColormapEntries` [request] for each time you
	///   allocated it before it is actually deleted.
	/// - A read-only [colormap] entry is not actually deleted until all clients
	///   have deleted it.
	///
	/// A particular [`ColorId`] allocated by an [`AllocateColorPlanes` request]
	/// may not be reused until all of its related [`ColorId`s] have been
	/// deleted too.
	///
	/// Even if an [error] is generated because of one of the requested
	/// [colormap] entries, all other [colormap] entries which do not generate
	/// [errors][error] will still be deleted.
	///
	/// # Errors
	/// A [`Colormap` error] is generated if `target` does not refer to a
	/// defined [colormap].
	///
	/// An [`Access` error] is generated if a requested [colormap] entry is not
	/// actually allocated, or it is not allocated by your client.
	///
	/// A [`Value` error] is generated if a requested [`ColorId`] is not a valid
	/// index into the `target` [colormap].
	///
	/// [colormap]: Colormap
	/// [colors]: ColorId
	/// [request]: Request
	/// [error]: crate::message::Error
	///
	/// [`ColorId`s]: ColorId
	///
	/// [`AllocateColorPlanes` request]: AllocateColorPlanes
	///
	/// [`Access` error]: error::Access
	/// [`Colormap` error]: error::Colormap
	/// [`Value` error]: error::Value
	// TODO: rename all Destroy* requests to Delete*
	#[doc(alias("FreeColors"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct DestroyColormapEntries: Request(88, DestroyColormapEntriesError) {
		/// The [colormap] for which the [colormap] entries are deleted.
		///
		/// # Errors
		/// A [`Colormap` error] is generated if this does not refer to a
		/// defined [colormap].
		///
		/// [colormap]: Colormap
		///
		/// [`Colormap` error]: error::Colormap
		#[doc(alias("colormap"))]
		pub target: Colormap,

		/// The bit plane mask every subset of which is combined with each
		/// [`ColorId`] in `colors` to produce the requested [colormap] entries.
		///
		/// [colormap]: Colormap
		pub plane_mask: u32,
		/// The [`ColorId`s] which are combined with every subset of the
		/// `plane_mask` to produce the requested [colormap] entries.
		///
		/// [colormap]: Colormap
		///
		/// [`ColorId`s]: ColorId
		#[context(self::remaining => remaining / ColorId::X11_SIZE)]
		pub colors: Vec<ColorId>,
	}
}

request_error! {
	pub enum StoreColorsError for StoreColors {
		Access,
		Colormap,
		Value,
	}
}

derive_xrb! {
	/// A change to a [colormap] entry made in a [`StoreColors` request].
	///
	/// [colormap]: Colormap
	///
	/// [`StoreColors` request]: StoreColors
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ColormapEntryChange {
		/// The [`ColorId`] of the changed [colormap] entry.
		///
		/// [colormap]: Colormap
		pub id: ColorId,

		/// The new color.
		pub color: RgbColor,
		/// The mask for which of the [colormap] entry's color channels are
		/// changed.
		///
		/// [colormap]: Colormap
		pub mask: ColorChannelMask,
		_,
	}

	impl ConstantX11Size for ColormapEntryChange {
		const X11_SIZE: usize = {
			ColorId::X11_SIZE
			+ RgbColor::X11_SIZE
			+ ColorChannelMask::X11_SIZE
			+ 1
		};
	}

	/// A [request] that changes the [RGB values] of the given [colormap]
	/// entries.
	///
	/// See also: [`StoreNamedColor`].
	///
	/// # Errors
	/// A [`Colormap` error] is generated if `target` does not refer to a
	/// defined [colormap].
	///
	/// An [`Access` error] is generated if a requested [colormap] entry is
	/// read-only or it is not allocated.
	///
	/// A [`Value` error] is generated if a requested [`ColorId`] is not a valid
	/// index into the `target` [colormap].
	///
	/// [RGB values]: RgbColor
	/// [colormap]: Colormap
	/// [request]: Request
	///
	/// [`Access` error]: error::Access
	/// [`Colormap` error]: error::Colormap
	/// [`Value` error]: error::Value
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct StoreColors: Request(89, StoreColorsError) {
		/// The [colormap] for which the [colormap] entries are changed.
		///
		/// # Errors
		/// A [`Colormap` error] is generated if this does not refer to a
		/// defined [colormap].
		///
		/// [colormap]: Colormap
		///
		/// [`Colormap` error]: error::Colormap
		#[doc(alias("colormap"))]
		pub target: Colormap,

		/// The requested [colormap] entry changes.
		///
		/// # Errors
		/// An [`Access` error] is generated if a requested [colormap] entry is
		/// read-only or it is not allocated.
		///
		/// A [`Value` error] is generated if a requested [`ColorId`] is not a
		/// valid index into the `target` [colormap].
		///
		/// [colormap]: Colormap
		///
		/// [`Access` error]: error::Access
		/// [`Value` error]: error::Value
		#[doc(alias("items"))]
		#[context(self::remaining => remaining / ColormapEntryChange::X11_SIZE)]
		pub changes: Vec<ColormapEntryChange>,
	}
}

request_error! {
	pub enum StoreNamedColorError for StoreNamedColor {
		Access,
		Colormap,
		Name,
		Value,
	}
}

derive_xrb! {
	/// A [request] that changes the [RGB values] of the given [colormap] entry
	/// to the color of the given `name` in the given [colormap]'s [screen].
	///
	/// See also: [`StoreColors`], [`ColormapEntryChange`].
	///
	/// # Errors
	/// A [`Colormap` error] is generated if `target` does not refer to a
	/// defined [colormap].
	///
	/// A [`Name` error] is generated if `name` does not refer to a defined
	/// color in the `target` [colormap]'s [screen].
	///
	/// An [`Access` error] is generated if the requested [colormap] entry is
	/// read-only or it is not allocated.
	///
	/// A [`Value` error] is generated if the requested [`ColorId`] is not a
	/// valid into into the `target` [colormap].
	///
	/// [RGB values]: RgbColor
	/// [colormap]: Colormap
	/// [screen]: crate::visual::Screen
	/// [request]: Request
	///
	/// [`Access` error]: error::Access
	/// [`Colormap` error]: error::Colormap
	/// [`Value` error]: error::Value
	/// [`Name` error]: error::Name
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct StoreNamedColor: Request(90, StoreNamedColorError) {
		/// The mask for which of the [colormap] entry's color channels are
		/// changed.
		///
		/// [colormap]: Colormap
		pub mask: ColorChannelMask,

		/// The [colormap] for which the [colormap] entry is changed.
		///
		/// # Errors
		/// A [`Colormap` error] is generated if this does not refer to a
		/// defined [colormap].
		///
		/// [colormap]: Colormap
		///
		/// [`Colormap` error]: error::Colormap
		#[doc(alias("colormap"))]
		pub target: Colormap,
		/// The [`ColorId`] of the [colormap] entry which is to be changed.
		///
		/// [colormap]: Colormap
		pub id: ColorId,

		// The length of `name`.
		#[allow(clippy::cast_possible_truncation)]
		let name_len: u16 = name => name.len() as u16,
		[_; 2],

		/// The name of the color in the `target`'s [screen] which the requested
		/// [colormap] entry is changed to.
		///
		/// # Errors
		/// A [`Name` error] is generated if this does not refer to a defined
		/// color in the `target` [colormap]'s [screen].
		///
		/// [colormap]: Colormap
		/// [screen]: crate::visual::Screen
		///
		/// [`Name` error]: error::Name
		#[context(name_len => usize::from(*name_len))]
		pub name: String8,
		[_; name => pad(name)],
	}
}
