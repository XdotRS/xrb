// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`X11Size`] and [`ConstantX11Size`] implementations for primitive types

use crate::{ConstantX11Size, X11Size};
use std::ops::{Range, RangeInclusive};

/// Simple macro for easely defining size for primitive types
macro_rules! constant_x11_size {
	($($type:ty),+$(,)?) => {
		$(
			impl ConstantX11Size for $type {
				const X11_SIZE: usize = std::mem::size_of::<Self>();
			}

			impl X11Size for $type {
				fn x11_size(&self) -> usize {
					Self::X11_SIZE
				}
			}
		)+
	};
}

constant_x11_size! {
	i8,
	i16,
	i32,
	i64,
	i128,

	u8,
	u16,
	u32,
	u64,
	u128,

	f32,
	f64,

	bool,
}

impl<T: X11Size> X11Size for Vec<T> {
	fn x11_size(&self) -> usize {
		self.iter().map(X11Size::x11_size).sum()
	}
}

impl<T: X11Size, const N: usize> X11Size for [T; N] {
	fn x11_size(&self) -> usize {
		let mut data_size = 0;

		for x in self {
			data_size += x.x11_size();
		}

		data_size
	}
}

impl<T: X11Size> X11Size for &[T] {
	fn x11_size(&self) -> usize {
		let mut x11_size: usize = 0;

		for x in *self {
			x11_size += x.x11_size();
		}

		x11_size
	}
}

impl<T: X11Size> X11Size for [T] {
	fn x11_size(&self) -> usize {
		let mut x11_size: usize = 0;

		for element in self {
			x11_size += element.x11_size();
		}

		x11_size
	}
}

impl X11Size for &str {
	fn x11_size(&self) -> usize {
		self.len()
	}
}

impl<T: ConstantX11Size> X11Size for Option<T> {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl<T: ConstantX11Size> ConstantX11Size for Option<T> {
	const X11_SIZE: usize = T::X11_SIZE;
}

// Size for references will be the same as the owned type.

impl<T: X11Size> X11Size for &T {
	default fn x11_size(&self) -> usize {
		T::x11_size(self)
	}
}

impl<T: ConstantX11Size + X11Size> ConstantX11Size for &T {
	const X11_SIZE: usize = T::X11_SIZE;
}

impl<T: X11Size> X11Size for &mut T {
	default fn x11_size(&self) -> usize {
		T::x11_size(self)
	}
}

impl<T: X11Size + ConstantX11Size> ConstantX11Size for &mut T {
	const X11_SIZE: usize = T::X11_SIZE;
}

impl<T: X11Size> X11Size for Box<T> {
	default fn x11_size(&self) -> usize {
		T::x11_size(self)
	}
}

impl<T: X11Size + ConstantX11Size> ConstantX11Size for Box<T> {
	const X11_SIZE: usize = T::X11_SIZE;
}

impl<T: X11Size> X11Size for Range<T> {
	fn x11_size(&self) -> usize {
		self.start.x11_size() + u8::X11_SIZE
	}
}

impl<T: X11Size + ConstantX11Size> ConstantX11Size for Range<T> {
	const X11_SIZE: usize = T::X11_SIZE + u8::X11_SIZE;
}

impl<T: X11Size> X11Size for RangeInclusive<T> {
	fn x11_size(&self) -> usize {
		self.start().x11_size() + u8::X11_SIZE
	}
}

impl<T: X11Size + ConstantX11Size> ConstantX11Size for RangeInclusive<T> {
	const X11_SIZE: usize = T::X11_SIZE + u8::X11_SIZE;
}

#[cfg(test)]
mod test {
	use super::X11Size;

	#[test]
	fn test_x11_size_vec() {
		let data = vec![i16::default(); 100];
		assert_eq!(data.x11_size(), 200);
	}

	#[test]
	fn test_x11_size_constant() {
		let data: Option<u64> = None;
		assert_eq!(data.x11_size(), 8);
	}

	// TODO: More tests ?
}
