// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::Wrap;

macro_rules! impl_wrap {
	($($($type:ty),+$(,)?)?) => {
		$($(
			impl Wrap for $type {
				type Integer = Self;
			}
		)+)?
	}
}

impl_wrap! {
	u8,
	u16,
	u32,
	u64,
}
