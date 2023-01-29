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

use xrbk_macro::{derive_xrb, Readable, Writable, X11Size};

use crate::{message::Request, visual::VisualId, x11::error, Colormap, Window};

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
}
