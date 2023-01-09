// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Readable`] implementations for primitive types

use crate::{ReadResult, Readable, ReadableWithContext, X11Size};
use bytes::Buf;
use std::ops::{Range, RangeInclusive};

macro_rules! implement {
	($($reader:ident, $ty:ty => $expr:expr),*$(,)?) => {
		$(
			impl $crate::Readable for $ty {
				fn read_from($reader: &mut impl bytes::Buf) -> Result<Self, $crate::ReadError> {
					Ok($expr)
				}
			}
		)*
	};
}

implement! {
	reader, i8 => reader.get_i8(),
	reader, i16 => reader.get_i16(),
	reader, i32 => reader.get_i32(),
	reader, i64 => reader.get_i64(),
	reader, i128 => reader.get_i128(),

	reader, u8 => reader.get_u8(),
	reader, u16 => reader.get_u16(),
	reader, u32 => reader.get_u32(),
	reader, u64 => reader.get_u64(),
	reader, u128 => reader.get_u128(),

	reader, f32 => reader.get_f32(),
	reader, f64 => reader.get_f64(),

	reader, bool => reader.get_u8() != 0,
}

impl<T: Readable, const N: usize> Readable for [T; N] {
	fn read_from(reader: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		let mut vec = Vec::new();

		for _ in 0..N {
			vec.push(T::read_from(reader)?);
		}

		Ok(vec
			.try_into()
			.unwrap_or_else(|_| unreachable!("we know the length of this vec is `N`")))
	}
}

impl<T: Readable> Readable for Box<T> {
	fn read_from(reader: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self::new(T::read_from(reader)?))
	}
}

impl<T: Readable> ReadableWithContext for Vec<T> {
	type Context = usize;

	fn read_with(reader: &mut impl Buf, context: &Self::Context) -> ReadResult<Self>
	where
		Self: Sized,
	{
		let mut vec = Self::new();

		for _ in 0..*context {
			vec.push(T::read_from(reader)?);
		}

		Ok(vec)
	}
}

impl<T: X11Size + Clone> ReadableWithContext for Range<T> {
	type Context = (T, T);

	fn read_with(_buf: &mut impl Buf, (start, end): &(T, T)) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self {
			start: start.clone(),
			end: end.clone(),
		})
	}
}

impl<T: X11Size + Clone> ReadableWithContext for RangeInclusive<T> {
	type Context = (T, T);

	fn read_with(_buf: &mut impl Buf, (start, end): &(T, T)) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self::new(start.clone(), end.clone()))
	}
}
