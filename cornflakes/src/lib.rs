// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod implementations;
mod traits;

pub use implementations::*;
pub use traits::*;

pub(crate) type IoResult<T = ()> = Result<T, std::io::Error>;
