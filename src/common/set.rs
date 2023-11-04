// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Sets used to specify optional options.
//!
//! In particular, this module includes the following sets and their masks and
//! builders:
//! - [`Attributes`]
//!   - [`AttributesBuilder`]
//!   - [`AttributesMask`]
//! - [`GraphicsOptions`]
//!   - [`GraphicsOptionsBuilder`]
//!   - [`GraphicsOptionsMask`]
//! - [`KeyboardOptions`]
//!   - [`KeyboardOptionsBuilder`]
//!   - [`KeyboardOptionsMask`]
//! - [`WindowConfig`]
//!   - [`WindowConfigBuilder`]
//!   - [`WindowConfigMask`]

use crate::unit::Px;
use xrbk::{
	Buf,
	BufMut,
	ConstantX11Size,
	ReadError,
	ReadResult,
	Readable,
	Writable,
	WriteResult,
	X11Size,
};

mod attribute;
mod graphics_options;
mod keyboard_options;
mod window_config;

pub use attribute::*;
pub use graphics_options::*;
pub use keyboard_options::*;
pub use window_config::*;

/// Reads an optional value for a set if the given `condition` is true.
///
/// This is not part of the public API.
 fn read_set_value<T: Readable>(
	buf: &mut impl Buf, x11_size: &mut usize, condition: bool,
) -> ReadResult<Option<T>> {
	Ok(if condition {
		let ret = T::read_from(buf)?;
		*x11_size += ret.x11_size();

		Some(ret)
	} else {
		None
	})
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
 struct __Px<Num>(Px<Num>);

impl<Num> ConstantX11Size for __Px<Num> {
	const X11_SIZE: usize = 4;
}

impl<Num> X11Size for __Px<Num> {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

/// Implements the internal `__Px` representation for the given numerical
/// primitives.
macro_rules! impl_px {
	(
		$get:ident, $put:ty => {
			$(
				$type:tt
			),*$(,)?
		}
	) => {
		$(
			impl Readable for __Px<$type> {
				fn read_from(buf: &mut impl Buf) -> ReadResult<Self> {
					Ok(Self(Px(match <$type>::try_from(buf.$get()) {
						Ok($type) => $type,

						Err(err) => return Err(ReadError::FailedConversion(Box::new(err))),
					})))
				}
			}

			impl Writable for __Px<$type> {
				fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
					let Self(Px($type)) = self;

					<$put>::from(*$type).write_to(buf)?;

					Ok(())
				}
			}
		)*
	};
}

impl_px!(get_u32, u32 => {
	u8,
	u16,
});

impl_px!(get_i32, i32 => {
	i8,
	i16,
});

/// Wraps a `u8` value, but writes it as four bytes in the X11 format.
///
/// This is not part of the public API.
#[allow(
	non_camel_case_types,
	reason = "The type name is chosen to indicate an internal representation of a u8 value."
)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
 struct __u8(u8);

impl ConstantX11Size for __u8 {
	const X11_SIZE: usize = 4;
}

impl X11Size for __u8 {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __u8 {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match u8::try_from(buf.get_u32()) {
			Ok(u8) => u8,
			Err(error) => return Err(ReadError::FailedConversion(Box::new(error))),
		}))
	}
}

impl Writable for __u8 {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(u8) = self;

		u32::from(*u8).write_to(buf)?;

		Ok(())
	}
}

/// Wraps a `u16` value, but writes it as four bytes in the X11 format.
///
/// This is not part of the public API.
#[allow(
	non_camel_case_types,
	reason = "The type name is chosen to indicate an internal representation of a u16 value."
)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
 struct __u16(u16);

impl ConstantX11Size for __u16 {
	const X11_SIZE: usize = 4;
}

impl X11Size for __u16 {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __u16 {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match u16::try_from(buf.get_u32()) {
			Ok(u16) => u16,
			Err(error) => return Err(ReadError::FailedConversion(Box::new(error))),
		}))
	}
}

impl Writable for __u16 {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(u16) = self;

		u32::from(*u16).write_to(buf)?;

		Ok(())
	}
}

/// Wraps an `i16` value, but writes it as four bytes in the X11 format.
///
/// This is not part of the public API.
#[allow(
	non_camel_case_types,
	reason = "The type name is chosen to indicate an internal representation of an i16 value."
)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
 struct __i16(i16);

impl ConstantX11Size for __i16 {
	const X11_SIZE: usize = 4;
}

impl X11Size for __i16 {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __i16 {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match i16::try_from(buf.get_i32()) {
			Ok(i16) => i16,
			Err(error) => return Err(ReadError::FailedConversion(Box::new(error))),
		}))
	}
}

impl Writable for __i16 {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(i16) = self;

		i32::from(*i16).write_to(buf)?;

		Ok(())
	}
}

/// Wraps a `bool`, but writes it as four bytes in the X11 format.
///
/// This is not part of the public API.
#[allow(
	non_camel_case_types,
	reason = "This is an internal representation of a `bool`. Its naming scheme is `__` to \
	          indicate that it is internal, and `bool` to indicate its wrapped type."
)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
 struct __bool(bool);

impl ConstantX11Size for __bool {
	const X11_SIZE: usize = 4;
}

impl X11Size for __bool {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __bool {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(buf.get_u32() != 0))
	}
}

impl Writable for __bool {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(bool) = self;

		if *bool {
			buf.put_u32(1);
		} else {
			buf.put_u32(0);
		}

		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_read_set_value_x11_size() {
		let mut x11_size = 0;
		let mut buf = &[0u8; 7][..];

		let _ = read_set_value::<u32>(&mut buf, &mut x11_size, true).unwrap();
		let _ = read_set_value::<u16>(&mut buf, &mut x11_size, true).unwrap();
		let _ = read_set_value::<u8>(&mut buf, &mut x11_size, true).unwrap();

		assert_eq!(x11_size, 7);
	}
}
