// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Requests] defined in the [core X11 protocol] that relate to an X client or
//! the X server.
//!
//! [Requests] are messages sent from an X client to the X server.
//!
//! [Requests]: Request
//! [core X11 protocol]: crate::x11

extern crate self as xrb;

use xrbk::pad;
use xrbk_macro::{derive_xrb, Readable, Writable, X11Size};

use crate::{
	message::Request,
	x11::{error, reply},
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
	pub enum ChangeSavedWindowsError for ChangeSavedWindows {
		Match,
		Value,
		Window,
	}
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
	/// [request]: Request
	///
	/// [adds]: AddOrRemove::Add
	/// [removes]: AddOrRemove::Remove
	///
	/// [reparented]: super::ReparentWindow
	#[doc(alias = "ChangeSaveSet")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
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

	/// A [request] that returns whether the specified extension is present and
	/// the message codes associated with it if it is.
	///
	/// # Replies
	/// This [request] generates a [`QueryExtension` reply].
	///
	/// [request]: Request
	///
	/// [`QueryExtension` reply]: reply::QueryExtension
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct QueryExtension: Request(98) -> reply::QueryExtension {
		// Length of `name`.
		#[allow(clippy::cast_possible_truncation)]
		let name_len: u16 = name => name.len() as u16,
		[_; 2],

		/// The name of the extension which is to be queried.
		///
		/// This name should use ISO Latin-1 encoding. Uppercase and lowercase
		/// matter.
		#[context(name_len => usize::from(*name_len))]
		pub name: String8,
		[_; name => pad(name)],
	}
}
