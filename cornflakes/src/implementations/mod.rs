// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::traits::{ByteReader, ByteWriter};
use bytes::{Buf, BufMut};

impl<T> ByteReader for T where T: Buf {}
impl<T> ByteWriter for T where T: BufMut {}

mod byte_sizes;
mod rw_bytes;
