// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{Writable, WriteResult};
use bytes::BufMut;

macro_rules! implement {
	($($ident:ident: &$ty:ty => BufMut::$fun:ident($expr:expr)),*$(,)?) => {
		$(
			impl $crate::Writable for $ty {
				fn write_to(
					&self,
					writer: &mut impl bytes::BufMut,
				) -> Result<(), $crate::WriteError> {
					let $ident = self;
					writer.$fun($expr);

					Ok(())
				}
			}
		)*
	};
}

implement! {
	n: &i8 => BufMut::put_i8(*n),
	n: &i16 => BufMut::put_i16(*n),
	n: &i32 => BufMut::put_i32(*n),
	n: &i64 => BufMut::put_i64(*n),
	n: &i128 => BufMut::put_i128(*n),

	n: &u8 => BufMut::put_u8(*n),
	n: &u16 => BufMut::put_u16(*n),
	n: &u32 => BufMut::put_u32(*n),
	n: &u64 => BufMut::put_u64(*n),
	n: &u128 => BufMut::put_u128(*n),

	n: &f32 => BufMut::put_f32(*n),
	n: &f64 => BufMut::put_f64(*n),

	b: &bool => BufMut::put_u8(*b as u8),
}

impl<T: Writable> Writable for &[T] {
	fn write_to(&self, writer: &mut impl BufMut) -> WriteResult {
		for x in *self {
			x.write_to(writer)?;
		}

		Ok(())
	}
}

impl<T: Writable, const N: usize> Writable for [T; N] {
	fn write_to(&self, writer: &mut impl BufMut) -> WriteResult {
		for x in self {
			x.write_to(writer)?;
		}

		Ok(())
	}
}

impl<T: Writable> Writable for Vec<T> {
	fn write_to(&self, writer: &mut impl BufMut) -> WriteResult {
		for x in self {
			x.write_to(writer)?;
		}

		Ok(())
	}
}

impl<T: Writable> Writable for &T {
	fn write_to(&self, writer: &mut impl BufMut) -> WriteResult {
		T::write_to(self, writer)?;

		Ok(())
	}
}

impl<T: Writable> Writable for &mut T {
	fn write_to(&self, writer: &mut impl BufMut) -> WriteResult {
		T::write_to(self, writer)?;

		Ok(())
	}
}

impl<T: Writable> Writable for Box<T> {
	fn write_to(&self, writer: &mut impl BufMut) -> WriteResult {
		T::write_to(self, writer)?;

		Ok(())
	}
}
