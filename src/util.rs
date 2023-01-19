// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xrbk::{Buf, ReadResult, Readable};

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
