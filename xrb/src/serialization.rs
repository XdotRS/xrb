// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bytes::{Buf, BufMut};

pub trait Serialize {
    /// Write to a [BufMut](bytes::BufMut).
    fn write(&self, buf: &mut impl BufMut);
}
pub trait Deserialize {
    /// Construct [`Self`] from a [Buf](bytes::Buf). Must be the inverse of [Serialize::write].
    fn read(buf: &mut impl Buf) -> Self;
}

impl Serialize for u8 {
    fn write(&self, buf: &mut impl BufMut) {
        buf.put_u8(*self);
    }
}

impl Deserialize for u8 {
    fn read(buf: &mut impl Buf) -> Self {
        buf.get_u8()
    }
}

impl Serialize for u16 {
    fn write(&self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_u16(*self);
        } else {
            buf.put_u16_le(*self);
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
    fn write(&self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_u32(*self);
        } else {
            buf.put_u32_le(*self);
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
    fn write(&self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_u64(*self);
        } else {
            buf.put_u64_le(*self);
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
    fn write(&self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_u128(*self);
        } else {
            buf.put_u128_le(*self);
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
    fn write(&self, buf: &mut impl BufMut) {
        buf.put_i8(*self);
    }
}

impl Deserialize for i8 {
    fn read(buf: &mut impl Buf) -> Self {
        buf.get_i8()
    }
}

impl Serialize for i16 {
    fn write(&self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_i16(*self);
        } else {
            buf.put_i16_le(*self);
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
    fn write(&self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_i32(*self);
        } else {
            buf.put_i32_le(*self);
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
    fn write(&self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_i64(*self);
        } else {
            buf.put_i64_le(*self);
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
    fn write(&self, buf: &mut impl BufMut) {
        if cfg!(target_endian = "big") {
            buf.put_i128(*self);
        } else {
            buf.put_i128_le(*self);
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
