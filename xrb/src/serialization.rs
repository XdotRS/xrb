// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bytes::{Buf, BufMut};

pub trait Serialize {
    /// Write to a [BufMut](bytes::BufMut).
    fn write(self, buf: &mut impl BufMut);
}

pub trait Deserialize {
    /// Construct [`Self`] from a [Buf](bytes::Buf). Must be the inverse of [Serialize::write].
    fn read(buf: &mut impl Buf) -> Self;
}

impl Serialize for u8 {
    fn write(self, buf: &mut impl BufMut) {
        buf.put_u8(self);
    }
}

impl Deserialize for u8 {
    fn read(buf: &mut impl Buf) -> Self {
        buf.get_u8()
    }
}

impl Serialize for u16 {
    fn write(self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_u16(self);
        } else {
            buf.put_u16_le(self);
        }
    }
}

impl Deserialize for u16 {
    fn read(buf: &mut impl Buf) -> Self {
        if cfg!(target_endian = "big") {
            buf.get_u16()
        } else {
            buf.get_u16_le()
        }
    }
}

impl Serialize for u32 {
    fn write(self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_u32(self);
        } else {
            buf.put_u32_le(self);
        }
    }
}

impl Deserialize for u32 {
    fn read(buf: &mut impl Buf) -> Self {
        if cfg!(target_endian = "big") {
            buf.get_u32()
        } else {
            buf.get_u32_le()
        }
    }
}

impl Serialize for u64 {
    fn write(self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_u64(self);
        } else {
            buf.put_u64_le(self);
        }
    }
}

impl Deserialize for u64 {
    fn read(buf: &mut impl Buf) -> Self {
        if cfg!(target_endian = "big") {
            buf.get_u64()
        } else {
            buf.get_u64_le()
        }
    }
}

impl Serialize for u128 {
    fn write(self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_u128(self);
        } else {
            buf.put_u128_le(self);
        }
    }
}

impl Deserialize for u128 {
    fn read(buf: &mut impl Buf) -> Self {
        if cfg!(target_endian = "big") {
            buf.get_u128()
        } else {
            buf.get_u128_le()
        }
    }
}

impl Serialize for i8 {
    fn write(self, buf: &mut impl BufMut) {
        buf.put_i8(self);
    }
}

impl Deserialize for i8 {
    fn read(buf: &mut impl Buf) -> Self {
        buf.get_i8()
    }
}

impl Serialize for i16 {
    fn write(self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_i16(self);
        } else {
            buf.put_i16_le(self);
        }
    }
}

impl Deserialize for i16 {
    fn read(buf: &mut impl Buf) -> Self {
        if cfg!(target_endian = "big") {
            buf.get_i16()
        } else {
            buf.get_i16_le()
        }
    }
}

impl Serialize for i32 {
    fn write(self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_i32(self);
        } else {
            buf.put_i32_le(self);
        }
    }
}

impl Deserialize for i32 {
    fn read(buf: &mut impl Buf) -> Self {
        if cfg!(target_endian = "big") {
            buf.get_i32()
        } else {
            buf.get_i32_le()
        }
    }
}

impl Serialize for i64 {
    fn write(self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_i64(self);
        } else {
            buf.put_i64_le(self);
        }
    }
}

impl Deserialize for i64 {
    fn read(buf: &mut impl Buf) -> Self {
        if cfg!(target_endian = "big") {
            buf.get_i64()
        } else {
            buf.get_i64_le()
        }
    }
}

impl Serialize for i128 {
    fn write(self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_i128(self);
        } else {
            buf.put_i128_le(self);
        }
    }
}

impl Deserialize for i128 {
    fn read(buf: &mut impl Buf) -> Self {
        if cfg!(target_endian = "big") {
            buf.get_i128()
        } else {
            buf.get_i128_le()
        }
    }
}

impl Serialize for bool {
    fn write(self, buf: &mut impl BufMut) {
        if self { 1u8 } else { 0u8 }.write(buf);
    }
}

impl Deserialize for bool {
    fn read(buf: &mut impl Buf) -> Self {
        match u8::read(buf) {
            0u8 => false,
            1u8 => true,
            _ => panic!("tried to read bool but value read was not a bool"),
        }
    }
}

impl Serialize for String {
    fn write(self, buf: &mut impl BufMut) {
        let length = self.len() as u8;
        let bytes = self.as_bytes();

        length.write(buf);
        bytes.write(buf);
    }
}

impl Deserialize for String {
    fn read(buf: &mut impl Buf) -> Self {
        let length = u8::read(buf);
        let bytes = buf.copy_to_bytes(length.into());

        String::from_utf8(bytes.to_vec()).unwrap()
    }
}

impl<A, B> Serialize for (A, B)
where
    A: Serialize,
    B: Serialize,
{
    fn write(self, buf: &mut impl BufMut) {
        self.0.write(buf);
        self.1.write(buf);
    }
}

impl<A, B> Deserialize for (A, B)
where
    A: Deserialize,
    B: Deserialize,
{
    fn read(buf: &mut impl Buf) -> Self {
        (A::read(buf), B::read(buf))
    }
}

impl<A, B, C> Serialize for (A, B, C)
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
{
    fn write(self, buf: &mut impl BufMut) {
        self.0.write(buf);
        self.1.write(buf);
        self.2.write(buf);
    }
}

impl<A, B, C> Deserialize for (A, B, C)
where
    A: Deserialize,
    B: Deserialize,
    C: Deserialize,
{
    fn read(buf: &mut impl Buf) -> Self {
        (A::read(buf), B::read(buf), C::read(buf))
    }
}

impl Serialize for () {
    fn write(self, _: &mut impl BufMut) {}
}

impl Deserialize for () {
    fn read(_: &mut impl Buf) -> Self {}
}

// As this does not serialize the length, we can't know how to deserialize any given &[T].
impl<T> Serialize for &[T]
where
    T: Serialize + Copy,
{
    fn write(self, buf: &mut impl BufMut) {
        self.iter().for_each(|t| {
            t.write(buf);
        });
    }
}

// In the very specific circumstance that an array type has an explicit `LENGTH` and `T` is
// [`Copy`], we can initialize `mut `[`Self`] by reading `T` once and copying it to all positions,
// then, if `LENGTH` is greater than `1`, we set any remaining elements with [T::read()] again.
//
// This only works because we can get the length from the type parameter, otherwise we'd need to
// use macros or functions.
impl<T, const LENGTH: usize> Deserialize for [T; LENGTH]
where
    T: Deserialize + Copy,
{
    fn read(buf: &mut impl Buf) -> Self {
        let mut array: [T; LENGTH] = [T::read(buf); LENGTH];

        for i in 1..LENGTH {
            array[i] = T::read(buf);
        }

        array
    }
}

// As this does not serialize the length, we can't know how to serialize any given Vec<T>.
impl<T> Serialize for Vec<T>
where
    T: Serialize + Copy,
{
    fn write(self, buf: &mut impl BufMut) {
        self.iter().for_each(|t| {
            t.write(buf);
        });
    }
}
