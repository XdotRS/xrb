// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::traits::{ByteSize, StaticByteSize};

/// Easily generate [`StaticByteSize`] implementations.
macro_rules! static_byte_sizes {
    ($($ty:ty: $size:expr),+$(,)?) => {
        $(
            impl StaticByteSize for $ty {
                fn static_byte_size() -> usize {
                    $size
                }
            }

            impl ByteSize for $ty {
                fn byte_size(&self) -> usize {
                    $size
                }
            }
        )+
    };
}

impl<T> StaticByteSize for &T
where
	T: StaticByteSize,
{
	fn static_byte_size() -> usize {
		T::static_byte_size()
	}
}

impl<T> ByteSize for &T
where
	T: ByteSize,
{
	fn byte_size(&self) -> usize {
		(*self).byte_size()
	}
}

static_byte_sizes! {
	bool: 1,
	char: 4,

	// Unsigned
	u8: 1,
	u16: 2,
	u32: 4,
	u64: 8,
	u128: 16,

	// Signed
	i8: 1,
	i16: 2,
	i32: 4,
	i64: 8,
	i128: 16,

	// Floating-point
	f32: 4,
	f64: 16,

	// While the unit while holds no data, it is useful to represent unused
	// bytes, so for such purpose it is assigned a byte size of `1`.
	(): 1,
}

impl<T> StaticByteSize for Option<T>
where
	T: StaticByteSize,
{
	fn static_byte_size() -> usize {
		T::static_byte_size()
	}
}

impl<T> ByteSize for Option<T>
where
	T: StaticByteSize,
{
	fn byte_size(&self) -> usize {
		T::static_byte_size()
	}
}

impl<T, const LEN: usize> StaticByteSize for [T; LEN]
where
	T: StaticByteSize,
{
	fn static_byte_size() -> usize {
		LEN * T::static_byte_size()
	}
}

impl<T, const LEN: usize> ByteSize for [T; LEN]
where
	T: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.iter().map(|elem| elem.byte_size()).sum()
	}
}

impl<T> ByteSize for [T]
where
	T: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.iter().map(|elem| elem.byte_size()).sum()
	}
}

impl<T> ByteSize for &[T]
where
	T: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.iter().map(|elem| elem.byte_size()).sum()
	}
}

impl<T> ByteSize for Vec<T>
where
	T: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.iter().map(|elem| elem.byte_size()).sum()
	}
}

impl ByteSize for String {
	fn byte_size(&self) -> usize {
		self.len()
	}
}

impl ByteSize for str {
	fn byte_size(&self) -> usize {
		self.len()
	}
}

impl ByteSize for &str {
	fn byte_size(&self) -> usize {
		self.len()
	}
}

impl<A, B> StaticByteSize for (A, B)
where
	A: StaticByteSize,
	B: StaticByteSize,
{
	fn static_byte_size() -> usize
	where
		Self: Sized,
	{
		A::static_byte_size() + B::static_byte_size()
	}
}

impl<A, B, C> StaticByteSize for (A, B, C)
where
	A: StaticByteSize,
	B: StaticByteSize,
	C: StaticByteSize,
{
	fn static_byte_size() -> usize
	where
		Self: Sized,
	{
		A::static_byte_size() + B::static_byte_size() + C::static_byte_size()
	}
}

impl<A, B, C, D> StaticByteSize for (A, B, C, D)
where
	A: StaticByteSize,
	B: StaticByteSize,
	C: StaticByteSize,
	D: StaticByteSize,
{
	fn static_byte_size() -> usize
	where
		Self: Sized,
	{
		A::static_byte_size()
			+ B::static_byte_size()
			+ C::static_byte_size()
			+ D::static_byte_size()
	}
}

impl<A, B, C, D, E> StaticByteSize for (A, B, C, D, E)
where
	A: StaticByteSize,
	B: StaticByteSize,
	C: StaticByteSize,
	D: StaticByteSize,
	E: StaticByteSize,
{
	fn static_byte_size() -> usize
	where
		Self: Sized,
	{
		A::static_byte_size()
			+ B::static_byte_size()
			+ C::static_byte_size()
			+ D::static_byte_size()
			+ E::static_byte_size()
	}
}

impl<A, B, C, D, E, F> StaticByteSize for (A, B, C, D, E, F)
where
	A: StaticByteSize,
	B: StaticByteSize,
	C: StaticByteSize,
	D: StaticByteSize,
	E: StaticByteSize,
	F: StaticByteSize,
{
	fn static_byte_size() -> usize
	where
		Self: Sized,
	{
		A::static_byte_size()
			+ B::static_byte_size()
			+ C::static_byte_size()
			+ D::static_byte_size()
			+ E::static_byte_size()
			+ F::static_byte_size()
	}
}

impl<A, B, C, D, E, F, G> StaticByteSize for (A, B, C, D, E, F, G)
where
	A: StaticByteSize,
	B: StaticByteSize,
	C: StaticByteSize,
	D: StaticByteSize,
	E: StaticByteSize,
	F: StaticByteSize,
	G: StaticByteSize,
{
	fn static_byte_size() -> usize
	where
		Self: Sized,
	{
		A::static_byte_size()
			+ B::static_byte_size()
			+ C::static_byte_size()
			+ D::static_byte_size()
			+ E::static_byte_size()
			+ F::static_byte_size()
			+ G::static_byte_size()
	}
}

impl<A, B, C, D, E, F, G, H> StaticByteSize for (A, B, C, D, E, F, G, H)
where
	A: StaticByteSize,
	B: StaticByteSize,
	C: StaticByteSize,
	D: StaticByteSize,
	E: StaticByteSize,
	F: StaticByteSize,
	G: StaticByteSize,
	H: StaticByteSize,
{
	fn static_byte_size() -> usize
	where
		Self: Sized,
	{
		A::static_byte_size()
			+ B::static_byte_size()
			+ C::static_byte_size()
			+ D::static_byte_size()
			+ E::static_byte_size()
			+ F::static_byte_size()
			+ G::static_byte_size()
			+ H::static_byte_size()
	}
}

impl<A, B, C, D, E, F, G, H, I> StaticByteSize for (A, B, C, D, E, F, G, H, I)
where
	A: StaticByteSize,
	B: StaticByteSize,
	C: StaticByteSize,
	D: StaticByteSize,
	E: StaticByteSize,
	F: StaticByteSize,
	G: StaticByteSize,
	H: StaticByteSize,
	I: StaticByteSize,
{
	fn static_byte_size() -> usize
	where
		Self: Sized,
	{
		A::static_byte_size()
			+ B::static_byte_size()
			+ C::static_byte_size()
			+ D::static_byte_size()
			+ E::static_byte_size()
			+ F::static_byte_size()
			+ G::static_byte_size()
			+ H::static_byte_size()
			+ I::static_byte_size()
	}
}

impl<A, B, C, D, E, F, G, H, I, J> StaticByteSize for (A, B, C, D, E, F, G, H, I, J)
where
	A: StaticByteSize,
	B: StaticByteSize,
	C: StaticByteSize,
	D: StaticByteSize,
	E: StaticByteSize,
	F: StaticByteSize,
	G: StaticByteSize,
	H: StaticByteSize,
	I: StaticByteSize,
	J: StaticByteSize,
{
	fn static_byte_size() -> usize
	where
		Self: Sized,
	{
		A::static_byte_size()
			+ B::static_byte_size()
			+ C::static_byte_size()
			+ D::static_byte_size()
			+ E::static_byte_size()
			+ F::static_byte_size()
			+ G::static_byte_size()
			+ H::static_byte_size()
			+ I::static_byte_size()
			+ J::static_byte_size()
	}
}

impl<A, B, C, D, E, F, G, H, I, J, K> StaticByteSize for (A, B, C, D, E, F, G, H, I, J, K)
where
	A: StaticByteSize,
	B: StaticByteSize,
	C: StaticByteSize,
	D: StaticByteSize,
	E: StaticByteSize,
	F: StaticByteSize,
	G: StaticByteSize,
	H: StaticByteSize,
	I: StaticByteSize,
	J: StaticByteSize,
	K: StaticByteSize,
{
	fn static_byte_size() -> usize
	where
		Self: Sized,
	{
		A::static_byte_size()
			+ B::static_byte_size()
			+ C::static_byte_size()
			+ D::static_byte_size()
			+ E::static_byte_size()
			+ F::static_byte_size()
			+ G::static_byte_size()
			+ H::static_byte_size()
			+ I::static_byte_size()
			+ J::static_byte_size()
			+ K::static_byte_size()
	}
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> StaticByteSize for (A, B, C, D, E, F, G, H, I, J, K, L)
where
	A: StaticByteSize,
	B: StaticByteSize,
	C: StaticByteSize,
	D: StaticByteSize,
	E: StaticByteSize,
	F: StaticByteSize,
	G: StaticByteSize,
	H: StaticByteSize,
	I: StaticByteSize,
	J: StaticByteSize,
	K: StaticByteSize,
	L: StaticByteSize,
{
	fn static_byte_size() -> usize
	where
		Self: Sized,
	{
		A::static_byte_size()
			+ B::static_byte_size()
			+ C::static_byte_size()
			+ D::static_byte_size()
			+ E::static_byte_size()
			+ F::static_byte_size()
			+ G::static_byte_size()
			+ H::static_byte_size()
			+ I::static_byte_size()
			+ J::static_byte_size()
			+ K::static_byte_size()
			+ L::static_byte_size()
	}
}

impl<A, B> ByteSize for (A, B)
where
	A: ByteSize,
	B: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.0.byte_size() + self.1.byte_size()
	}
}

impl<A, B, C> ByteSize for (A, B, C)
where
	A: ByteSize,
	B: ByteSize,
	C: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.0.byte_size() + self.1.byte_size() + self.2.byte_size()
	}
}

impl<A, B, C, D> ByteSize for (A, B, C, D)
where
	A: ByteSize,
	B: ByteSize,
	C: ByteSize,
	D: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.0.byte_size() + self.1.byte_size() + self.2.byte_size() + self.3.byte_size()
	}
}

impl<A, B, C, D, E> ByteSize for (A, B, C, D, E)
where
	A: ByteSize,
	B: ByteSize,
	C: ByteSize,
	D: ByteSize,
	E: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.0.byte_size()
			+ self.1.byte_size()
			+ self.2.byte_size()
			+ self.3.byte_size()
			+ self.4.byte_size()
	}
}

impl<A, B, C, D, E, F> ByteSize for (A, B, C, D, E, F)
where
	A: ByteSize,
	B: ByteSize,
	C: ByteSize,
	D: ByteSize,
	E: ByteSize,
	F: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.0.byte_size()
			+ self.1.byte_size()
			+ self.2.byte_size()
			+ self.3.byte_size()
			+ self.4.byte_size()
			+ self.5.byte_size()
	}
}

impl<A, B, C, D, E, F, G> ByteSize for (A, B, C, D, E, F, G)
where
	A: ByteSize,
	B: ByteSize,
	C: ByteSize,
	D: ByteSize,
	E: ByteSize,
	F: ByteSize,
	G: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.0.byte_size()
			+ self.1.byte_size()
			+ self.2.byte_size()
			+ self.3.byte_size()
			+ self.4.byte_size()
			+ self.5.byte_size()
			+ self.6.byte_size()
	}
}

impl<A, B, C, D, E, F, G, H> ByteSize for (A, B, C, D, E, F, G, H)
where
	A: ByteSize,
	B: ByteSize,
	C: ByteSize,
	D: ByteSize,
	E: ByteSize,
	F: ByteSize,
	G: ByteSize,
	H: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.0.byte_size()
			+ self.1.byte_size()
			+ self.2.byte_size()
			+ self.3.byte_size()
			+ self.4.byte_size()
			+ self.5.byte_size()
			+ self.6.byte_size()
			+ self.7.byte_size()
	}
}

impl<A, B, C, D, E, F, G, H, I> ByteSize for (A, B, C, D, E, F, G, H, I)
where
	A: ByteSize,
	B: ByteSize,
	C: ByteSize,
	D: ByteSize,
	E: ByteSize,
	F: ByteSize,
	G: ByteSize,
	H: ByteSize,
	I: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.0.byte_size()
			+ self.1.byte_size()
			+ self.2.byte_size()
			+ self.3.byte_size()
			+ self.4.byte_size()
			+ self.5.byte_size()
			+ self.6.byte_size()
			+ self.7.byte_size()
			+ self.8.byte_size()
	}
}

impl<A, B, C, D, E, F, G, H, I, J> ByteSize for (A, B, C, D, E, F, G, H, I, J)
where
	A: ByteSize,
	B: ByteSize,
	C: ByteSize,
	D: ByteSize,
	E: ByteSize,
	F: ByteSize,
	G: ByteSize,
	H: ByteSize,
	I: ByteSize,
	J: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.0.byte_size()
			+ self.1.byte_size()
			+ self.2.byte_size()
			+ self.3.byte_size()
			+ self.4.byte_size()
			+ self.5.byte_size()
			+ self.6.byte_size()
			+ self.7.byte_size()
			+ self.8.byte_size()
			+ self.9.byte_size()
	}
}

impl<A, B, C, D, E, F, G, H, I, J, K> ByteSize for (A, B, C, D, E, F, G, H, I, J, K)
where
	A: ByteSize,
	B: ByteSize,
	C: ByteSize,
	D: ByteSize,
	E: ByteSize,
	F: ByteSize,
	G: ByteSize,
	H: ByteSize,
	I: ByteSize,
	J: ByteSize,
	K: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.0.byte_size()
			+ self.1.byte_size()
			+ self.2.byte_size()
			+ self.3.byte_size()
			+ self.4.byte_size()
			+ self.5.byte_size()
			+ self.6.byte_size()
			+ self.7.byte_size()
			+ self.8.byte_size()
			+ self.9.byte_size()
			+ self.10.byte_size()
	}
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> ByteSize for (A, B, C, D, E, F, G, H, I, J, K, L)
where
	A: ByteSize,
	B: ByteSize,
	C: ByteSize,
	D: ByteSize,
	E: ByteSize,
	F: ByteSize,
	G: ByteSize,
	H: ByteSize,
	I: ByteSize,
	J: ByteSize,
	K: ByteSize,
	L: ByteSize,
{
	fn byte_size(&self) -> usize {
		self.0.byte_size()
			+ self.1.byte_size()
			+ self.2.byte_size()
			+ self.3.byte_size()
			+ self.4.byte_size()
			+ self.5.byte_size()
			+ self.6.byte_size()
			+ self.7.byte_size()
			+ self.8.byte_size()
			+ self.9.byte_size()
			+ self.10.byte_size()
			+ self.11.byte_size()
	}
}
