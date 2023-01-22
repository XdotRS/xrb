// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Types representing units like millimeters or hertz.

use std::fmt::{Display, Formatter};
use thiserror::Error;
use xrbk::{Buf, BufMut, ConstantX11Size, ReadResult, Readable, Writable, WriteResult, X11Size};

/// An error generated when a value is outside of the required bounds.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Error)]
#[error("expected a value satisfying {} <= value <= {}, found {}", self.min, self.max, self.found)]
pub struct ValueOutOfBounds<Num: Display> {
	/// The minimum allowed value.
	pub min: Num,
	/// The maximum allowed value.
	pub max: Num,

	/// The value which did not satisfy the bounds.
	pub found: Num,
}

macro_rules! impl_xrbk_traits {
	($type:ident $(<$generic:ident>)?($inner:ident)) => {
		impl$(<$generic>)? ConstantX11Size for $type$(<$generic>)?
		$(where
			$generic: ConstantX11Size,)?
		{
			const X11_SIZE: usize = <$inner>::X11_SIZE;
		}

		impl$(<$generic>)? X11Size for $type$(<$generic>)?
		$(where
			$generic: X11Size,)?
		{
			fn x11_size(&self) -> usize {
				<$inner>::x11_size(&self.0)
			}
		}

		impl$(<$generic>)? Readable for $type$(<$generic>)?
		$(where
			$generic: Readable,)?
		{
			fn read_from(buf: &mut impl Buf) -> ReadResult<Self> {
				Ok(Self(<$inner>::read_from(buf)?))
			}
		}

		impl$(<$generic>)? Writable for $type$(<$generic>)?
		$(where
			$generic: Writable,)?
		{
			fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
				self.0.write_to(buf)?;

				Ok(())
			}
		}
	};
}

/// A value measured in pixels.
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Px<Num>(pub Num);

impl<Num> Display for Px<Num>
where
	Num: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} px", self.0)
	}
}

impl_xrbk_traits!(Px<Num>(Num));

/// A value measured in millimeters.
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mm<Num>(pub Num);

impl<Num> Display for Mm<Num>
where
	Num: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} mm", self.0)
	}
}

impl_xrbk_traits!(Mm<Num>(Num));

/// A value measured in milliseconds.
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ms<Num>(pub Num);

impl<Num> Display for Ms<Num>
where
	Num: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} ms", self.0)
	}
}

impl_xrbk_traits!(Ms<Num>(Num));

/// A value measured in hertz.
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hz<Num>(pub Num);

impl<Num> Display for Hz<Num>
where
	Num: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} Hz", self.0)
	}
}

impl_xrbk_traits!(Hz<Num>(Num));

/// A value measured as a percentage from 0% to 100%.
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Percentage(u8);

impl Display for Percentage {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}%", self.0)
	}
}

impl Percentage {
	/// Creates a new percentage.
	///
	/// # Errors
	/// Return a [`ValueOutOfBounds`] error if the `percentage > 100`.
	pub const fn new(percentage: u8) -> Result<Self, ValueOutOfBounds<u8>> {
		match percentage {
			percentage if percentage <= 100 => Ok(Self(percentage)),

			other => Err(ValueOutOfBounds {
				min: 0,
				max: 100,
				found: other,
			}),
		}
	}

	/// Creates a new percentage without ensuring it is has the right bounds.
	///
	/// # Safety
	/// Callers of this function must ensure that the given percentage satisfies
	/// the bounds `0 <= percentage <= 100`. Creating a [`Percentage`] with a
	/// value greater than 100 is Undefined Behavior.
	#[must_use]
	pub const unsafe fn new_unchecked(percentage: u8) -> Self {
		Self(percentage)
	}

	/// Returns the wrapped percentage value.
	#[must_use]
	pub const fn unwrap(self) -> u8 {
		self.0
	}
}

impl_xrbk_traits!(Percentage(u8));
