// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xrbk::{Buf, BufMut, ConstantX11Size, ReadResult, Readable, Writable, WriteResult, X11Size};

mod attribute;
mod graphics_options;
mod window_configs;

pub use attribute::*;
pub use graphics_options::*;
pub use window_configs::*;

pub fn read_set_value<T: Readable>(
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

/// Wraps a `bool` but writes it as four bytes in the X11 format.
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
