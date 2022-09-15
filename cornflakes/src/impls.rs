// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::*;

macro_rules! impl_basic {
	($($ty:ty[$size:expr]: Reader::$read:ident(), Writer::$write:ident()),*$(,)?) => {
		$(
			impl Readable for $ty {
				fn read_from(reader: &mut impl Reader) -> Result<Self, ReadError> {
					reader.$read()
				}
			}

			impl Writable for $ty {
				fn write_to(&self, writer: &mut impl Writer) -> Result<(), WriteError> {
					writer.$write(*self)
				}
			}

			impl ByteSize for $ty {
				fn byte_size(&self) -> usize {
					$size
				}
			}
		)*
	};
}

impl_basic! {
	bool[1]: Reader::read_bool(), Writer::write_bool(),

	// Unsigned
	u8[1]: Reader::read_u8(), Writer::write_u8(),
	u16[2]: Reader::read_u16_ne(), Writer::write_u16_ne(),
	u32[4]: Reader::read_u32_ne(), Writer::write_u32_ne(),
	u64[8]: Reader::read_u64_ne(), Writer::write_u64_ne(),
	u128[16]: Reader::read_u128_ne(), Writer::write_u128_ne(),

	// Signed
	i8[1]: Reader::read_i8(), Writer::write_i8(),
	i16[2]: Reader::read_i16_ne(), Writer::write_i16_ne(),
	i32[4]: Reader::read_i32_ne(), Writer::write_i32_ne(),
	i64[8]: Reader::read_i64_ne(), Writer::write_i64_ne(),
	i128[16]: Reader::read_i128_ne(), Writer::write_i128_ne(),

	// Floating-point
	f32[4]: Reader::read_f32_ne(), Writer::write_f32_ne(),
	f64[8]: Reader::read_f64_ne(), Writer::write_f64_ne(),
}

impl<T> ByteSize for &T
where
	T: ByteSize,
{
	fn byte_size(&self) -> usize {
		(*self).byte_size()
	}
}

impl<T> Writable for &T
where
	T: Writable,
{
	fn write_to(&self, writer: &mut impl Writer) -> Result<(), WriteError>
	where
		Self: Sized,
	{
		(*self).write_to(writer)
	}
}

impl<T> Writable for Vec<T>
where
	T: Writable + ByteSize,
{
	fn write_to(&self, writer: &mut impl Writer) -> Result<(), WriteError>
	where
		Self: Sized,
	{
		for thing in self {
			writer.write(thing)?;
		}

		Ok(())
	}
}

impl<T> ReadableWithLength for Vec<T>
where
	T: Readable,
{
	fn read_from_with_length(reader: &mut impl Reader, length: usize) -> Result<Self, ReadError>
	where
		Self: Sized,
	{
		let mut things = vec![];

		for _ in 0..length {
			things.push(reader.read()?);
		}

		Ok(things)
	}
}

impl<T> Writable for &[T]
where
	T: Writable + ByteSize,
{
	fn write_to(&self, writer: &mut impl Writer) -> Result<(), WriteError>
	where
		Self: Sized,
	{
		for thing in *self {
			writer.write(thing)?;
		}

		Ok(())
	}
}

impl<T, const LEN: usize> Readable for [T; LEN]
where
	T: Readable,
{
	fn read_from(reader: &mut impl Reader) -> Result<Self, ReadError>
	where
		Self: Sized,
	{
		let arr: [T; LEN] = array_init::array_init(|_| reader.read().unwrap());

		Ok(arr)
	}
}

impl<T, const LEN: usize> Writable for [T; LEN]
where
	T: Writable + ByteSize,
{
	fn write_to(&self, writer: &mut impl Writer) -> Result<(), WriteError>
	where
		Self: Sized,
	{
		for thing in self {
			writer.write(thing)?;
		}

		Ok(())
	}
}

impl<T> ByteSize for Vec<T>
where
	T: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.iter().map(|thing| thing.byte_size()).sum()
	}
}

impl<T> ByteSize for &[T]
where
	T: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.iter().map(|thing| thing.byte_size()).sum()
	}
}

impl<T, const LEN: usize> ByteSize for [T; LEN]
where
	T: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.iter().map(|thing| thing.byte_size()).sum()
	}
}
