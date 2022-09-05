// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::traits::{
	ByteReader, ByteWriter, FromBytes, FromBytesWithSize, StaticByteSize, ToBytes, ToBytesWithSize,
};

use crate::IoResult;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

// Don't even ask about the generated code from this macro. It implements
// `FromBytesWithSize` and `ToBytesWithSize`, that's what matters ;)
macro_rules! with_size {
	(
		$(
			$ty:ty$([$static_size:expr])? $({
				$($size:expr => $sty:ty),+$(,)?
			})?;
		)+
	) => {
		$(
			impl FromBytesWithSize for $ty {
				fn read_from_with_size(reader: &mut impl ByteReader, size: usize) -> IoResult<Self> {
					match size {
						$($static_size => Ok(reader.read()?),)?
						$($($size => reader.read_with_size::<$sty>(size)?
								.try_into()
								.ok()
								.ok_or(IoError::from(IoErrorKind::InvalidData))
							),+,)?
						_ => Err(IoError::from(IoErrorKind::InvalidInput)),
					}
				}
			}

			impl ToBytesWithSize for $ty {
				fn write_to_with_size(&self, writer: &mut impl ByteWriter, size: usize) -> IoResult {
					match size {
						$($static_size => writer.write(*self)?,)?
						$($($size => writer.write_with_size::<$sty>(
								<$sty as TryFrom<$ty>>::try_from(*self)
										.ok()
										.ok_or(IoError::from(IoErrorKind::InvalidData))?,
								size
							)?),+,)?
						_ => return Err(IoError::from(IoErrorKind::InvalidInput)),
					};

					Ok(())
				}
			}
		)+
	};
}

with_size! {
	// Unsigned
	u8[1] {
		2 => u16,
		4 => u32,
		8 => u64,
		16 => u128,
	};

	u16[2] {
		1 => u8,
		4 => u32,
		8 => u64,
		16 => u128,
	};

	u32[4] {
		1 => u8,
		2 => u16,
		8 => u64,
		16 => u128,
	};

	u64[8] {
		1 => u8,
		2 => u16,
		4 => u32,
		16 => u128,
	};

	u128[16] {
		1 => u8,
		2 => u16,
		4 => u32,
		8 => u32,
	};

	//Signed
	i8[1] {
		2 => i16,
		4 => i32,
		8 => i64,
		16 => i128,
	};

	i16[2] {
		1 => i8,
		4 => i32,
		8 => i64,
		16 => i128,
	};

	i32[4] {
		1 => i8,
		2 => i16,
		8 => i64,
		16 => i128,
	};

	i64[8] {
		1 => i8,
		2 => i16,
		4 => i32,
		16 => i128,
	};

	i128[16] {
		1 => i8,
		2 => i16,
		4 => i32,
		8 => i64,
	};

	// Char
	char[4] {
		8 => u32,
		16 => u32,
	};
}

impl FromBytes for bool {
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		Ok(reader.read_u8() != 0)
	}
}

impl ToBytes for bool {
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
		writer.write_u8(*self as u8);

		Ok(())
	}
}

impl FromBytes for char {
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		char::from_u32(reader.read_u32_ne()).ok_or(IoError::new(
			IoErrorKind::InvalidData,
			"attempted to read invalid UTF-8 `char`",
		))
	}
}

impl ToBytes for char {
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
		writer.write_u32_ne(*self as u32);

		Ok(())
	}
}

macro_rules! numeric {
    ($($ty:ty: $read:ident(), $write:ident()),+$(,)?) => {
        $(
            impl FromBytes for $ty {
                fn read_from(reader: &mut impl ByteReader) -> IoResult<Self> {
                    Ok(reader.$read())
                }
            }

            impl ToBytes for $ty {
                fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
                    writer.$write(*self);

                    Ok(())
                }
            }
        )+
    }
}

numeric! {
	// Unsigned
	u8: read_u8(), write_u8(),
	u16: read_u16_ne(), write_u16_ne(),
	u32: read_u32_ne(), write_u32_ne(),
	u64: read_u64_ne(), write_u64_ne(),
	u128: read_u128_ne(), write_u128_ne(),

	// Signed
	i8: read_i8(), write_i8(),
	i16: read_i16_ne(), write_i16_ne(),
	i32: read_i32_ne(), write_i32_ne(),
	i64: read_i64_ne(), write_i64_ne(),
	i128: read_i128_ne(), write_i128_ne(),

	// Floating-point
	f32: read_f32_ne(), write_f32_ne(),
	f64: read_f64_ne(), write_f64_ne(),
}

impl FromBytes for () {
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		reader.advance(1);

		Ok(())
	}

	fn read_vectored_from(reader: &mut impl ByteReader) -> IoResult<Vec<Self>>
	where
		Self: Sized,
	{
		let count = reader.remaining();
		reader.advance(count);

		Ok(vec![(); count])
	}
}

impl ToBytes for () {
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
		writer.write_u8(0);

		Ok(())
	}

	fn write_vectored_to(selves: &[Self], writer: &mut impl ByteWriter) -> IoResult
	where
		Self: Sized,
	{
		writer.write_many(0, selves.len());

		Ok(())
	}
}

impl<T> FromBytes for Option<T>
where
	T: FromBytes + StaticByteSize,
{
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		let mut bytes = reader.copy_to_bytes(T::static_byte_size());

		if bytes.iter().sum::<u8>() == 0 {
			Ok(None)
		} else {
			Ok(Some(T::read_from(&mut bytes)?))
		}
	}
}

impl<T> ToBytes for Option<T>
where
	T: ToBytes + StaticByteSize,
{
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
		match self {
			None => writer.write_many(0, T::static_byte_size()),
			Some(thing) => thing.write_to(writer)?,
		};

		Ok(())
	}
}

impl<T, const N: usize> FromBytes for [T; N]
where
	T: FromBytes + Default + Copy,
{
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		let mut selves = [T::default(); N];

		for i in 0..N {
			selves[i] = reader.read()?;
		}

		Ok(selves)
	}
}

impl<T> FromBytesWithSize for [T]
where
	T: FromBytes + StaticByteSize,
	[T]: Clone,
{
	fn read_from_with_size(reader: &mut impl ByteReader, size: usize) -> Result<Self, IoError>
	where
		Self: Sized,
	{
		if size % T::static_byte_size() != 0 {
			return Err(IoError::from(IoErrorKind::InvalidInput));
		}

		let length = size / T::static_byte_size();
		let mut selves: Vec<T> = vec![];

		for _ in 0..length {
			selves.push(reader.read()?);
		}

		Ok(AsRef::<[T]>::as_ref(&selves).clone())
	}
}

impl<A, B> FromBytes for (A, B)
where
	A: FromBytes,
	B: FromBytes,
{
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		Ok((reader.read()?, reader.read()?))
	}
}

impl<A, B> ToBytes for (A, B)
where
	A: ToBytes,
	B: ToBytes,
{
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
		self.0.write_to(writer)?;
		self.1.write_to(writer)?;

		Ok(())
	}
}

impl<A, B, C> FromBytes for (A, B, C)
where
	A: FromBytes,
	B: FromBytes,
	C: FromBytes,
{
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		Ok((reader.read()?, reader.read()?, reader.read()?))
	}
}

impl<A, B, C> ToBytes for (A, B, C)
where
	A: ToBytes,
	B: ToBytes,
	C: ToBytes,
{
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
		self.0.write_to(writer)?;
		self.1.write_to(writer)?;
		self.2.write_to(writer)?;

		Ok(())
	}
}

impl<A, B, C, D> FromBytes for (A, B, C, D)
where
	A: FromBytes,
	B: FromBytes,
	C: FromBytes,
	D: FromBytes,
{
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		Ok((
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
		))
	}
}

impl<A, B, C, D> ToBytes for (A, B, C, D)
where
	A: ToBytes,
	B: ToBytes,
	C: ToBytes,
	D: ToBytes,
{
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
		self.0.write_to(writer)?;
		self.1.write_to(writer)?;
		self.2.write_to(writer)?;
		self.3.write_to(writer)?;

		Ok(())
	}
}

impl<A, B, C, D, E> FromBytes for (A, B, C, D, E)
where
	A: FromBytes,
	B: FromBytes,
	C: FromBytes,
	D: FromBytes,
	E: FromBytes,
{
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		Ok((
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
		))
	}
}

impl<A, B, C, D, E> ToBytes for (A, B, C, D, E)
where
	A: ToBytes,
	B: ToBytes,
	C: ToBytes,
	D: ToBytes,
	E: ToBytes,
{
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
		self.0.write_to(writer)?;
		self.1.write_to(writer)?;
		self.2.write_to(writer)?;
		self.3.write_to(writer)?;
		self.4.write_to(writer)?;

		Ok(())
	}
}

impl<A, B, C, D, E, F> FromBytes for (A, B, C, D, E, F)
where
	A: FromBytes,
	B: FromBytes,
	C: FromBytes,
	D: FromBytes,
	E: FromBytes,
	F: FromBytes,
{
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		Ok((
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
		))
	}
}

impl<A, B, C, D, E, F> ToBytes for (A, B, C, D, E, F)
where
	A: ToBytes,
	B: ToBytes,
	C: ToBytes,
	D: ToBytes,
	E: ToBytes,
	F: ToBytes,
{
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
		self.0.write_to(writer)?;
		self.1.write_to(writer)?;
		self.2.write_to(writer)?;
		self.3.write_to(writer)?;
		self.4.write_to(writer)?;
		self.5.write_to(writer)?;

		Ok(())
	}
}

impl<A, B, C, D, E, F, G> FromBytes for (A, B, C, D, E, F, G)
where
	A: FromBytes,
	B: FromBytes,
	C: FromBytes,
	D: FromBytes,
	E: FromBytes,
	F: FromBytes,
	G: FromBytes,
{
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		Ok((
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
		))
	}
}

impl<A, B, C, D, E, F, G> ToBytes for (A, B, C, D, E, F, G)
where
	A: ToBytes,
	B: ToBytes,
	C: ToBytes,
	D: ToBytes,
	E: ToBytes,
	F: ToBytes,
	G: ToBytes,
{
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
		self.0.write_to(writer)?;
		self.1.write_to(writer)?;
		self.2.write_to(writer)?;
		self.3.write_to(writer)?;
		self.4.write_to(writer)?;
		self.5.write_to(writer)?;
		self.6.write_to(writer)?;

		Ok(())
	}
}

impl<A, B, C, D, E, F, G, H> FromBytes for (A, B, C, D, E, F, G, H)
where
	A: FromBytes,
	B: FromBytes,
	C: FromBytes,
	D: FromBytes,
	E: FromBytes,
	F: FromBytes,
	G: FromBytes,
	H: FromBytes,
{
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		Ok((
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
		))
	}
}

impl<A, B, C, D, E, F, G, H> ToBytes for (A, B, C, D, E, F, G, H)
where
	A: ToBytes,
	B: ToBytes,
	C: ToBytes,
	D: ToBytes,
	E: ToBytes,
	F: ToBytes,
	G: ToBytes,
	H: ToBytes,
{
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
		self.0.write_to(writer)?;
		self.1.write_to(writer)?;
		self.2.write_to(writer)?;
		self.3.write_to(writer)?;
		self.4.write_to(writer)?;
		self.5.write_to(writer)?;
		self.6.write_to(writer)?;
		self.7.write_to(writer)?;

		Ok(())
	}
}

impl<A, B, C, D, E, F, G, H, I> FromBytes for (A, B, C, D, E, F, G, H, I)
where
	A: FromBytes,
	B: FromBytes,
	C: FromBytes,
	D: FromBytes,
	E: FromBytes,
	F: FromBytes,
	G: FromBytes,
	H: FromBytes,
	I: FromBytes,
{
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		Ok((
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
		))
	}
}

impl<A, B, C, D, E, F, G, H, I> ToBytes for (A, B, C, D, E, F, G, H, I)
where
	A: ToBytes,
	B: ToBytes,
	C: ToBytes,
	D: ToBytes,
	E: ToBytes,
	F: ToBytes,
	G: ToBytes,
	H: ToBytes,
	I: ToBytes,
{
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
		self.0.write_to(writer)?;
		self.1.write_to(writer)?;
		self.2.write_to(writer)?;
		self.3.write_to(writer)?;
		self.4.write_to(writer)?;
		self.5.write_to(writer)?;
		self.6.write_to(writer)?;
		self.7.write_to(writer)?;
		self.8.write_to(writer)?;

		Ok(())
	}
}

impl<A, B, C, D, E, F, G, H, I, J> FromBytes for (A, B, C, D, E, F, G, H, I, J)
where
	A: FromBytes,
	B: FromBytes,
	C: FromBytes,
	D: FromBytes,
	E: FromBytes,
	F: FromBytes,
	G: FromBytes,
	H: FromBytes,
	I: FromBytes,
	J: FromBytes,
{
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		Ok((
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
		))
	}
}

impl<A, B, C, D, E, F, G, H, I, J> ToBytes for (A, B, C, D, E, F, G, H, I, J)
where
	A: ToBytes,
	B: ToBytes,
	C: ToBytes,
	D: ToBytes,
	E: ToBytes,
	F: ToBytes,
	G: ToBytes,
	H: ToBytes,
	I: ToBytes,
	J: ToBytes,
{
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
		self.0.write_to(writer)?;
		self.1.write_to(writer)?;
		self.2.write_to(writer)?;
		self.3.write_to(writer)?;
		self.4.write_to(writer)?;
		self.5.write_to(writer)?;
		self.6.write_to(writer)?;
		self.7.write_to(writer)?;
		self.8.write_to(writer)?;
		self.9.write_to(writer)?;

		Ok(())
	}
}

impl<A, B, C, D, E, F, G, H, I, J, K> FromBytes for (A, B, C, D, E, F, G, H, I, J, K)
where
	A: FromBytes,
	B: FromBytes,
	C: FromBytes,
	D: FromBytes,
	E: FromBytes,
	F: FromBytes,
	G: FromBytes,
	H: FromBytes,
	I: FromBytes,
	J: FromBytes,
	K: FromBytes,
{
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		Ok((
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
		))
	}
}

impl<A, B, C, D, E, F, G, H, I, J, K> ToBytes for (A, B, C, D, E, F, G, H, I, J, K)
where
	A: ToBytes,
	B: ToBytes,
	C: ToBytes,
	D: ToBytes,
	E: ToBytes,
	F: ToBytes,
	G: ToBytes,
	H: ToBytes,
	I: ToBytes,
	J: ToBytes,
	K: ToBytes,
{
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
		self.0.write_to(writer)?;
		self.1.write_to(writer)?;
		self.2.write_to(writer)?;
		self.3.write_to(writer)?;
		self.4.write_to(writer)?;
		self.5.write_to(writer)?;
		self.6.write_to(writer)?;
		self.7.write_to(writer)?;
		self.8.write_to(writer)?;
		self.9.write_to(writer)?;
		self.10.write_to(writer)?;

		Ok(())
	}
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> FromBytes for (A, B, C, D, E, F, G, H, I, J, K, L)
where
	A: FromBytes,
	B: FromBytes,
	C: FromBytes,
	D: FromBytes,
	E: FromBytes,
	F: FromBytes,
	G: FromBytes,
	H: FromBytes,
	I: FromBytes,
	J: FromBytes,
	K: FromBytes,
	L: FromBytes,
{
	fn read_from(reader: &mut impl ByteReader) -> IoResult<Self>
	where
		Self: Sized,
	{
		Ok((
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
			reader.read()?,
		))
	}
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> ToBytes for (A, B, C, D, E, F, G, H, I, J, K, L)
where
	A: ToBytes,
	B: ToBytes,
	C: ToBytes,
	D: ToBytes,
	E: ToBytes,
	F: ToBytes,
	G: ToBytes,
	H: ToBytes,
	I: ToBytes,
	J: ToBytes,
	K: ToBytes,
	L: ToBytes,
{
	fn write_to(&self, writer: &mut impl ByteWriter) -> IoResult {
		self.0.write_to(writer)?;
		self.1.write_to(writer)?;
		self.2.write_to(writer)?;
		self.3.write_to(writer)?;
		self.4.write_to(writer)?;
		self.5.write_to(writer)?;
		self.6.write_to(writer)?;
		self.7.write_to(writer)?;
		self.8.write_to(writer)?;
		self.9.write_to(writer)?;
		self.10.write_to(writer)?;
		self.11.write_to(writer)?;

		Ok(())
	}
}
