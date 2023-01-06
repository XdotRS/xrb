// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`DataSize`] and [`StaticDataSize`] implementations for primitive types

use crate::{DataSize, StaticDataSize};

/// Simple macro for easely defining size for primitive types
macro_rules! static_data_size {
	($($type:ty),+$(,)?) => {
		$(
			impl StaticDataSize for $type {
				fn static_data_size() -> usize {
					std::mem::size_of::<$type>()
				}
			}
			impl DataSize for $type {
				fn data_size(&self) -> usize {
					Self::static_data_size()
				}
			}
		)+
	};
}

static_data_size! {
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

impl<T: DataSize> DataSize for Vec<T> {
	fn data_size(&self) -> usize {
		self.iter().map(DataSize::data_size).sum()
	}
}

impl<T: DataSize, const N: usize> DataSize for [T; N] {
	fn data_size(&self) -> usize {
		let mut data_size = 0;

		for element in self {
			data_size += element.data_size();
		}

		data_size
	}
}

impl<T: DataSize> DataSize for &[T] {
	fn data_size(&self) -> usize {
		let size: &mut usize = &mut 0;
		for e in *self {
			*size += e.data_size();
		}
		*size
	}
}

impl<T: DataSize> DataSize for [T] {
	fn data_size(&self) -> usize {
		let size: &mut usize = &mut 0;
		for e in self {
			*size += e.data_size();
		}
		*size
	}
}

impl DataSize for &str {
	fn data_size(&self) -> usize {
		self.len()
	}
}

impl<T: DataSize> DataSize for Option<T> {
	default fn data_size(&self) -> usize {
		match &self {
			None => 1,
			Some(v) => v.data_size(),
		}
	}
}

impl<T: StaticDataSize> StaticDataSize for Option<T> {
	fn static_data_size() -> usize {
		T::static_data_size()
	}
}

// Size for references will be the same as the owned type.

impl<T: DataSize> DataSize for &T {
	default fn data_size(&self) -> usize {
		<T as DataSize>::data_size(self)
	}
}

impl<T: DataSize + StaticDataSize> DataSize for &T {
	fn data_size(&self) -> usize {
		<Self as StaticDataSize>::static_data_size()
	}
}

impl<T: StaticDataSize> StaticDataSize for &T {
	fn static_data_size() -> usize {
		<T>::static_data_size()
	}
}

impl<T: DataSize> DataSize for &mut T {
	default fn data_size(&self) -> usize {
		<T as DataSize>::data_size(self)
	}
}

impl<T: DataSize + StaticDataSize> DataSize for &mut T {
	fn data_size(&self) -> usize {
		<Self as StaticDataSize>::static_data_size()
	}
}

impl<T: StaticDataSize> StaticDataSize for &mut T {
	fn static_data_size() -> usize {
		<T>::static_data_size()
	}
}

impl<T: DataSize> DataSize for Box<T> {
	default fn data_size(&self) -> usize {
		<T as DataSize>::data_size(self)
	}
}

impl<T: DataSize + StaticDataSize> DataSize for Box<T> {
	fn data_size(&self) -> usize {
		<Self as StaticDataSize>::static_data_size()
	}
}

impl<T: StaticDataSize> StaticDataSize for Box<T> {
	fn static_data_size() -> usize {
		<T>::static_data_size()
	}
}

#[cfg(test)]
mod test {
	use super::DataSize;

	#[test]
	fn test_datasize_vec() {
		let data = vec![i16::default(); 100];
		assert_eq!(data.data_size(), 200);
	}

	#[test]
	fn test_datasize_option_static() {
		let data: Option<u64> = None;
		assert_eq!(data.data_size(), 8);
	}

	#[test]
	fn test_datasize_option_dynamic() {
		let data: Option<Vec<i64>> = Some(vec![i64::default(); 10]);
		assert_eq!(data.data_size(), 80);
	}

	// TODO: More tests ?
}
