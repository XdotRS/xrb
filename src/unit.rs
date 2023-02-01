// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Types representing units like millimeters or hertz.

use std::{
	cmp::Ordering,
	fmt::{Display, Formatter},
};

use derive_more::{
	Add,
	AddAssign,
	Div,
	DivAssign,
	Mul,
	MulAssign,
	Rem,
	RemAssign,
	Sub,
	SubAssign,
	Sum,
};
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
#[derive(
	Debug,
	Hash,
	Copy,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Add,
	AddAssign,
	Sub,
	SubAssign,
	Mul,
	MulAssign,
	Div,
	DivAssign,
	Rem,
	RemAssign,
	Sum,
)]
pub struct Px<Num>(pub Num);

impl<Num> Px<Num> {
	/// Maps a `Px<Num>` to `Px<Output>` by applying the provided closure to the
	/// contained value.
	pub fn map<Output>(self, map: impl FnOnce(Num) -> Output) -> Px<Output> {
		Px(map(self.0))
	}

	/// Calls the provided closure with a reference to the contained value.
	pub fn inspect(&self, inspect: impl FnOnce(&Num)) {
		inspect(&self.0);
	}
}

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
#[derive(
	Debug,
	Hash,
	Copy,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Add,
	AddAssign,
	Sub,
	SubAssign,
	Mul,
	MulAssign,
	Div,
	DivAssign,
	Rem,
	RemAssign,
	Sum,
)]
pub struct Mm<Num>(pub Num);

impl<Num> Mm<Num> {
	/// Maps a `Mm<Num>` to `Mm<Output>` by applying the provided closure to the
	/// contained value.
	pub fn map<Output>(self, map: impl FnOnce(Num) -> Output) -> Mm<Output> {
		Mm(map(self.0))
	}

	/// Calls the provided closure with a reference to the contained value.
	pub fn inspect(&self, inspect: impl FnOnce(&Num)) {
		inspect(&self.0);
	}
}

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
#[derive(
	Debug,
	Hash,
	Copy,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Add,
	AddAssign,
	Sub,
	SubAssign,
	Mul,
	MulAssign,
	Div,
	DivAssign,
	Rem,
	RemAssign,
	Sum,
)]
pub struct Ms<Num>(pub Num);

impl<Num> Ms<Num> {
	/// Maps a `Ms<Num>` to `Ms<Output>` by applying the provided closure to the
	/// contained value.
	pub fn map<Output>(self, map: impl FnOnce(Num) -> Output) -> Ms<Output> {
		Ms(map(self.0))
	}

	/// Calls the provided closure with a reference to the contained value.
	pub fn inspect(&self, inspect: impl FnOnce(&Num)) {
		inspect(&self.0);
	}
}

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
#[derive(
	Debug,
	Hash,
	Copy,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Add,
	AddAssign,
	Sub,
	SubAssign,
	Mul,
	MulAssign,
	Div,
	DivAssign,
	Rem,
	RemAssign,
	Sum,
)]
pub struct Hz<Num>(pub Num);

impl<Num> Hz<Num> {
	/// Maps a `Hz<Num>` to `Hz<Output>` by applying the provided closure to the
	/// contained value.
	pub fn map<Output>(self, map: impl FnOnce(Num) -> Output) -> Hz<Output> {
		Hz(map(self.0))
	}

	/// Calls the provided closure with a reference to the contained value.
	pub fn inspect(&self, inspect: impl FnOnce(&Num)) {
		inspect(&self.0);
	}
}

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

impl Percentage {
	/// Calls the provided closure with a reference to the contained value.
	pub fn inspect(&self, inspect: impl FnOnce(u8)) {
		inspect(self.0);
	}
}

impl Display for Percentage {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}%", self.0)
	}
}

impl Percentage {
	/// Creates a new percentage.
	///
	/// # Errors
	/// Returns a [`ValueOutOfBounds`] error if the `percentage > 100`.
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
	pub const fn unwrap(&self) -> u8 {
		self.0
	}
}

impl PartialEq<u8> for Percentage {
	fn eq(&self, other: &u8) -> bool {
		self.0 == *other
	}
}

impl PartialEq<Percentage> for u8 {
	fn eq(&self, other: &Percentage) -> bool {
		*self == other.0
	}
}

impl PartialOrd<u8> for Percentage {
	fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
		self.0.partial_cmp(other)
	}
}

impl PartialOrd<Percentage> for u8 {
	fn partial_cmp(&self, other: &Percentage) -> Option<Ordering> {
		self.partial_cmp(&other.0)
	}
}

impl_xrbk_traits!(Percentage(u8));

/// A value measured as a percentage from -100% to 100%.
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignedPercentage(i8);

impl SignedPercentage {
	/// Calls the provided closure with a reference to the contained value.
	pub fn inspect(&self, inspect: impl FnOnce(i8)) {
		inspect(self.0);
	}
}

impl Display for SignedPercentage {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}%", self.0)
	}
}

impl SignedPercentage {
	/// Creates a new signed percentage.
	///
	/// # Errors
	/// Returns a [`ValueOutOfBounds`] error if `percentage < -100` or
	/// `percentage > 100`.
	pub const fn new(percentage: i8) -> Result<Self, ValueOutOfBounds<i8>> {
		match percentage {
			percentage if percentage >= -100 && percentage <= 100 => Ok(Self(percentage)),

			other => Err(ValueOutOfBounds {
				min: -100,
				max: 100,
				found: other,
			}),
		}
	}

	/// Creates a new signed percentage without ensuring it has the right
	/// bounds.
	///
	/// # Safety
	/// Callers of this function must ensure that the given percentage satisfies
	/// the bounds `-100 <= percentage <= 100`. Creating a [`SignedPercentage`]
	/// with a value less than -100 or greater than 100 is Undefined Behavior.
	#[must_use]
	pub const unsafe fn new_unchecked(percentage: i8) -> Self {
		Self(percentage)
	}

	/// Returns the wrapped percentage value.
	#[must_use]
	pub const fn unwrap(&self) -> i8 {
		self.0
	}
}

impl PartialEq<i8> for SignedPercentage {
	fn eq(&self, other: &i8) -> bool {
		self.0 == *other
	}
}

impl PartialEq<SignedPercentage> for i8 {
	fn eq(&self, other: &SignedPercentage) -> bool {
		*self == other.0
	}
}

impl PartialOrd<i8> for SignedPercentage {
	fn partial_cmp(&self, other: &i8) -> Option<Ordering> {
		self.0.partial_cmp(other)
	}
}

impl PartialOrd<SignedPercentage> for i8 {
	fn partial_cmp(&self, other: &SignedPercentage) -> Option<Ordering> {
		self.partial_cmp(&other.0)
	}
}

impl_xrbk_traits!(SignedPercentage(i8));
