// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Replies] defined in the [core X11 protocol] for
//! [requests that relate to colors].
//!
//! [Replies] are messages sent from the X server to an X client in response to
//! a [request].
//!
//! [Replies]: Reply
//! [request]: crate::message::Request
//! [core X11 protocol]: crate::x11
//!
//! [requests that relate to colors]: request::color

extern crate self as xrb;

use derivative::Derivative;
use xrbk::{Buf, BufMut, ConstantX11Size, ReadResult, Readable, Writable, WriteResult, X11Size};

use xrbk_macro::derive_xrb;

use crate::{
	message::Reply,
	visual::{ColorId, RgbColor},
	x11::request,
	Colormap,
};

derive_xrb! {
	/// The [reply] to a [`ListInstalledColormaps` request].
	///
	/// [reply]: Reply
	///
	/// [`ListInstalledColormaps` request]: request::ListInstalledColormaps
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct ListInstalledColormaps: Reply for request::ListInstalledColormaps {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		// The length of `colormaps`.
		#[allow(clippy::cast_possible_truncation)]
		let colormaps_len: u16 = colormaps => colormaps.len() as u16,
		[_; 22],

		/// The [colormaps] which are currently installed on the given
		/// `target`'s [screen].
		///
		/// This list is in no particular order.
		///
		/// This list has no indication as to which [colormaps] are contained in
		/// the [screen]'s list of required [colormaps].
		///
		/// [colormaps]: Colormaps
		/// [screen]: crate::visual::Screen
		#[context(colormaps_len => usize::from(*colormaps_len))]
		pub colormaps: Vec<Colormap>,
	}

	/// The [reply] to an [`AllocateColor` request].
	///
	/// [reply]: Reply
	///
	/// [`AllocateColor` request]: request::AllocateColor
	#[doc(alias("AllocColor"))]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct AllocateColor: Reply for request::AllocateColor {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// The actual RGB values that were allocated.
		///
		/// These are the closest RGB values to those requested that the display
		/// could provide.
		pub actual_color: RgbColor,
		[_; 2],

		/// The [`ColorId`] referring to the `actual_color`.
		pub color_id: ColorId,
		[_; ..],
	}

	/// The [reply] to an [`AllocateNamedColor` request].
	///
	/// [reply]: Reply
	///
	/// [`AllocateNamedColor` request]: request::AllocateNamedColor
	#[doc(alias("AllocNamedColor"))]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct AllocateNamedColor: Reply for request::AllocateNamedColor {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// The ideal or 'true' color which the name represents.
		pub ideal_color: RgbColor,
		/// The closest color that the display was able to provide.
		pub actual_color: RgbColor,
		[_; ..],
	}

	/// The [reply] to an [`AllocateColorCells` request].
	///
	/// [reply]: Reply
	///
	/// [`AllocateColorCells` request]: request::AllocateColorCells
	#[doc(alias("AllocColorCells"))]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct AllocateColorCells: Reply for request::AllocateColorCells {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		// The length of `colors`.
		#[allow(clippy::cast_possible_truncation)]
		let colors_len: u16 = colors => colors.len() as u16,
		// The length of `plane_masks`.
		#[allow(clippy::cast_possible_truncation)]
		let plane_masks_len: u16 = plane_masks => plane_masks.len() as u16,
		[_; 20],

		/// The colors that were used for the allocated [colormap] entries.
		///
		/// [colormap]: Colormap
		#[context(colors_len => usize::from(*colors_len))]
		pub colors: Vec<ColorId>,
		/// The bit plane masks that were used for the allocated [colormap]
		/// entries.
		///
		/// For [`VisualClass::GrayScale`] or [`VisualClass::PseudoColor`], each
		/// plane mask will have one bit set to `1` (because there is only one
		/// color channel).
		///
		/// For [`VisualClass::DirectColor`], each plane mask will have 3 bits
		/// sets to `1` (because there are three color channels: red, green, and
		/// blue).
		///
		/// No plane mask will have bits in common with any other plane mask,
		/// nor with any of the `colors`.
		///
		/// [colormap]: Colormap
		///
		/// [`VisualClass::GrayScale`]: crate::visual::VisualClass::GrayScale
		/// [`VisualClass::PseudoColor`]: crate::visual::VisualClass::PseudoColor
		/// [`VisualClass::DirectColor`]: crate::visual::VisualClass::DirectColor
		#[context(plane_masks_len => usize::from(*plane_masks_len))]
		pub plane_masks: Vec<u32>,
	}

	/// The [reply] to an [`AllocateColorPlanes` request].
	///
	/// [reply]: Reply
	///
	/// [`AllocateColorPlanes` request]: request::AllocateColorPlanes
	#[doc(alias("AllocColorPlanes"))]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct AllocateColorPlanes: Reply for request::AllocateColorPlanes {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		// The length of `colors`.
		#[allow(clippy::cast_possible_truncation)]
		let colors_len: u16 = colors => colors.len() as u16,
		[_; 2],

		/// The union of all the red bit plane masks which were applied to the
		/// `colors` to produce the [colormap] entries which were allocated.
		///
		/// [colormap]: Colormap
		pub red_plane_mask: u32,
		/// The union of all the green bit plane masks which were applied to the
		/// `colors` to produce the [colormap] entries which were allocated.
		///
		/// [colormap]: Colormap
		pub green_plane_mask: u32,
		/// The union of all the blue bit plane masks which were applied to the
		/// `colors` to produce the [colormap] entries which were allocated.
		///
		/// [colormap]: Colormap
		pub blue_plane_mask: u32,
		[_; 8],

		/// The colors that were combined with the plane masks to produce the
		/// [colormap] entries which were allocated.
		///
		/// [colormap]: Colormap
		#[context(colors_len => usize::from(*colors_len))]
		pub colors: Vec<ColorId>,
	}
}

/// The [reply] to a [`QueryColors` request].
///
/// [reply]: Reply
///
/// [`QueryColors` request]: request::QueryColors
#[derive(Derivative, Debug)]
#[derivative(Hash, PartialEq, Eq)]
pub struct QueryColors {
	/// The sequence number identifying the [request] that generated this
	/// [reply].
	///
	/// See [`Reply::sequence`] for more information.
	///
	/// [request]: crate::message::Request
	/// [reply]: Reply
	///
	/// [`Reply::sequence`]: Reply::sequence
	#[derivative(Hash = "ignore", PartialEq = "ignore")]
	pub sequence: u16,

	/// The [RGB values] of the requested [colormap] entries.
	///
	/// The [RGB values] returned for unallocated [colormap] entries is
	/// undefined.
	///
	/// [RGB values]: RgbColor
	/// [colormap]: Colormap
	pub colors: Vec<RgbColor>,
}

impl Reply for QueryColors {
	type Request = request::QueryColors;

	fn sequence(&self) -> u16 {
		self.sequence
	}
}

impl X11Size for QueryColors {
	fn x11_size(&self) -> usize {
		const HEADER: usize = 8;

		HEADER + u16::X11_SIZE + 22 + self.colors.x11_size()
	}
}

impl Readable for QueryColors {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self> {
		buf.advance(1);
		let sequence = buf.get_u16();

		let length = (buf.get_u32() as usize) * 4;
		let buf = &mut buf.take(length - 8);

		let colors_len = buf.get_u16();
		buf.advance(22);

		let colors = {
			let mut colors = vec![];

			for _ in 0..colors_len {
				colors.push(RgbColor::read_from(buf)?);
				buf.advance(2);
			}

			colors
		};

		Ok(Self { sequence, colors })
	}
}

impl Writable for QueryColors {
	#[allow(clippy::cast_possible_truncation)]
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let buf = &mut buf.limit((self.length() as usize) * 4);

		buf.put_u8(1);
		buf.put_u8(0);
		self.sequence.write_to(buf)?;
		buf.put_u32(self.length());

		buf.put_u16(self.colors.len() as u16);
		buf.put_bytes(0, 22);

		for color in &self.colors {
			color.write_to(buf)?;
			buf.put_bytes(0, 2);
		}

		Ok(())
	}
}

derive_xrb! {
	/// The [reply] to a [`GetNamedColor` request].
	///
	/// [reply]: Reply
	///
	/// [`GetNamedColor` request]: request::GetNamedColor
	#[doc(alias("LookupColor"))]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetNamedColor: Reply for request::GetNamedColor {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// The ideal [RGB values] of the color.
		///
		/// [RGB values]: RgbColor
		pub ideal_color: RgbColor,
		/// The closest [RGB values] to the `ideal_color` that the display could
		/// provide.
		///
		/// [RGB values]: RgbColor
		pub actual_color: RgbColor,
		[_; ..],
	}
}
