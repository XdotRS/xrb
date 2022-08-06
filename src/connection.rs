// This source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at https://mozilla.org/mpl/2.0/.

use crate::Event;

pub struct Connection {}

impl Connection {
    pub fn send(event: impl Event) {}
}

/// Initiates a [Connection] to the X server.
///
/// If provided, `preferred_screen` indicates the name of the screen to which this connection
/// should be made. If `preferred_screen` is [`None`], the screen name provided by the
/// `DISPLAY` environment variable will be used instead.
///
/// ```rust
/// // Connect to the X server on the default screen.
/// let conn = xrs::connect(None);
/// ```
pub fn connect(preferred_screen: Option<&str>) -> Connection {
    Connection {}
}
