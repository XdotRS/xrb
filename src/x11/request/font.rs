// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Requests] defined in the [core X11 protocol] that relate to fonts.
//!
//! [Requests] are messages sent from an X client to the X server.
//!
//! [Requests]: crate::message::Request
//! [core X11 protocol]: crate::x11

extern crate self as xrb;

use xrbk::{pad, ConstantX11Size};
use xrbk_macro::derive_xrb;

use crate::{
	x11::{error, reply},
	Char16,
	Font,
	Fontable,
	LengthString8,
	String16,
	String8,
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
	pub enum AssignFontError for AssignFont {
		ResourceIdChoice,
		Name,
	}
}

derive_xrb! {
	/// A [request] that associates the font by the given `name` with the given
	/// `font_id`.
	///
	/// [request]: crate::message::Request
	#[doc(alias("OpenFont", "CreateFont", "LoadFont", "AddFont"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct AssignFont: Request(45, AssignFontError) {
		/// The [`Font` ID] to associate with the font specified by `name`.
		///
		/// [`Font` ID]: Font
		pub font_id: Font,

		// The length of `name`.
		#[allow(clippy::cast_possible_truncation)]
		let name_len: u16 = name => name.len() as u16,
		[_; 2],

		/// A pattern match against the name of the font.
		///
		/// The name uses ISO Latin-1 encoding.
		///
		/// The character `?` matches against any single character (equivalent
		/// to `.` in regular expressions) and `*` matches against any number of
		/// characters (like `.*` in regular expressions).
		#[context(name_len => usize::from(*name_len))]
		pub name: String8,
		[_; name => pad(name)],
	}

	/// A [request] that removes the association between a given [`Font` ID] and
	/// the font it is associated with.
	///
	/// [request]: crate::message::Request
	/// [`Font` ID]: Font
	#[doc(alias("CloseFont", "DeleteFont", "UnloadFont", "RemoveFont"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct UnassignFont: Request(46) {
		/// The [`Font` ID] which is having its association with a font removed.
		///
		/// [`Font` ID]: Font
		pub target: Font,
	}

	/// A [request] that returns information about the given `target`
	/// font.
	///
	/// # Replies
	/// This [request] generates a [`QueryFont` reply].
	///
	/// # Errors
	/// A [`Font` error] is generated if the `target` does not refer to a
	/// defined [`Font`] nor [`GraphicsContext`].
	///
	/// [request]: crate::message::Request
	///
	/// [`GraphicsContext`]: crate::GraphicsContext
	///
	/// [`QueryFont` reply]: reply::QueryFont
	///
	/// [`Font` error]: error::Font
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct QueryFont: Request(47, error::Font) -> reply::QueryFont {
		/// The font which this [request] returns information about.
		///
		/// # Errors
		/// A [`Font` error] is generated if this does not refer to a defined
		/// [`Font`] nor [`GraphicsContext`].
		///
		/// [request]: crate::message::Request
		///
		/// [`GraphicsContext`]: crate::GraphicsContext
		///
		/// [`Font` error]: error::Font
		pub target: Fontable,
	}
}

/// A private function used in [`QueryTextExtents`] to
/// determine padding.
#[inline]
const fn query_text_extents_padding(odd_length: bool) -> usize {
	if odd_length {
		2 // Char16::X11_SIZE
	} else {
		0
	}
}

derive_xrb! {
	/// A [request] that returns the extents of the given `text` when displayed
	/// with the given `font`.
	///
	/// If the font has no specified `fallback_character`, undefined characters
	/// in the `text` are ignored.
	///
	/// # Replies
	/// This [request] generates a [`QueryTextExtents` reply].
	///
	/// # Errors
	/// A [`Font` error] is generated if `font` does not refer to a defined
	/// [`Font`] nor [`GraphicsContext`].
	///
	/// [request]: crate::message::Request
	///
	/// [`GraphicsContext`]: crate::GraphicsContext
	///
	/// [`QueryTextExtents` reply]: reply::QueryTextExtents
	///
	/// [`Font` error]: error::Font
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct QueryTextExtents: Request(48, error::Font) -> reply::QueryTextExtents {
		// Whether `text` is of odd length. Is it is, it has 2 bytes of padding
		// following it.
		#[metabyte]
		let odd_length: bool = text => text.len() % 2 != 0,

		/// The font used in the `text`.
		///
		/// # Errors
		/// A [`Font` error] is generated if this does not refer to a defined
		/// [`Font`] nor [`GraphicsContext`].
		///
		/// [`GraphicsContext`]: crate::GraphicsContext
		///
		/// [`Font` error]: error::Font
		pub font: Fontable,

		/// The text for which this [request] gets the extents when displayed
		/// with `font`.
		///
		/// [request]: crate::message::Request
		#[context(self::remaining, odd_length => {
			// We remove the padding at the end, which can be determined from `odd_length`.
			let remaining = remaining - query_text_extents_padding(*odd_length);

			// We then divide the length, which is the number of bytes, by the number of bytes
			// per character.
			remaining / Char16::X11_SIZE
		})]
		pub text: String16,
		[_; odd_length => query_text_extents_padding(*odd_length)]
	}

	/// A [request] that lists the names of available fonts (as controlled by
	/// the [font search path]).
	///
	/// # Replies
	/// This [request] generates a [`ListFonts` reply].
	///
	/// [request]: crate::message::Request
	///
	/// [font search path]: SetFontSearchDirectories
	///
	/// [`ListFonts` reply]: reply::ListFonts
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ListFonts: Request(49) -> reply::ListFonts {
		/// The maximum number of names that will appear in the returned font
		/// `names`.
		#[doc(alias("max_names", "max_names_len"))]
		pub max_names_count: u16,

		#[allow(clippy::cast_possible_truncation)]
		let pattern_len: u16 = pattern => pattern.len() as u16,
		/// A pattern match against the font names.
		///
		/// The case (uppercase or lowercase) of the pattern does not matter:
		/// font names are converted to lowercase, as is the pattern.
		///
		/// Font names use ISO Latin-1 encoding.
		///
		/// The character `?` matches against any single character (equivalent
		/// to `.` in regular expressions) and `*` matches against any number of
		/// characters (like `.*` in regular expressions).
		#[context(pattern_len => usize::from(*pattern_len))]
		pub pattern: String8,
		[_; pattern => pad(pattern)],
	}

	/// A [request] that lists available fonts (as controlled by the
	/// [font search path]) with information about them.
	///
	/// The information returned for each font almost entirely matches that
	/// returned in a [`QueryFont` reply].
	///
	/// # Replies
	/// This [request] generates [`ListFontsWithInfo` replies].
	///
	/// [request]: crate::message::Request
	///
	/// [font search path]: SetFontSearchDirectories
	///
	/// [`ListFontsWithInfo` replies]: reply::ListFontsWithInfo
	/// [`QueryFont` reply]: reply::QueryFont
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ListFontsWithInfo: Request(50) -> reply::ListFontsWithInfo {
		/// The maximum number of [`FontWithInfo` replies] that will be returned.
		///
		/// [`FontWithInfo` replies]: reply::FontWithInfo
		#[doc(alias("max_names", "max_names_len"))]
		pub max_fonts_count: u16,

		#[allow(clippy::cast_possible_truncation)]
		let pattern_len: u16 = pattern => pattern.len() as u16,
		/// A pattern match against the font names.
		///
		/// The case (uppercase or lowercase) of the pattern does not matter:
		/// font names are converted to lowercase, as is the pattern.
		///
		/// Font names use ISO Latin-1 encoding.
		///
		/// The character `?` matches against any single character (equivalent
		/// to `.` in regular expressions) and `*` matches against any number of
		/// characters (like `.*` in regular expressions).
		#[context(pattern_len => usize::from(*pattern_len))]
		pub pattern: String8,
		[_; pattern => pad(pattern)],
	}

	/// A [request] that defines the directories which are searched for
	/// available fonts.
	///
	/// # Errors
	/// A [`Value` error] is generated if the operating system rejects the given
	/// paths for whatever reason.
	///
	/// [request]: crate::message::Request
	///
	/// [`Value` error]: error::Value
	#[doc(alias = "SetFontPath")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct SetFontSearchDirectories: Request(51, error::Value) {
		// The length of `directories`.
		#[allow(clippy::cast_possible_truncation)]
		let directories_len: u16 = directories => directories.len() as u16,
		[_; 2],

		/// The directories to be searched in the order listed.
		///
		/// Specifying an empty list here restores the default font search
		/// directories defined for the X server.
		#[doc(alias = "path")]
		#[context(directories_len => usize::from(*directories_len))]
		pub directories: Vec<LengthString8>,
		[_; directories => pad(directories)],
	}

	/// A [request] that returns the current directories which are searched to
	/// find available fonts.
	///
	/// See also: [`SetFontSearchDirectories`].
	///
	/// [request]: crate::message::Request
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct GetFontSearchDirectories: Request(52) -> reply::GetFontSearchDirectories;
}
