// This source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at https://mozilla.org/mpl/2.0/.

mod connection;
mod events;
mod raw;
pub mod util {
    pub use crate::raw::Rect;
}

pub use connection::connect;
pub use connection::Connection;
pub use connection::DisplayServer;

pub use events::Event;

pub use events::notifications as notif;
pub use events::queries as query;
pub use events::replies as reply;
pub use events::requests as req;

pub use raw::Atom;
pub use raw::ConnError;
pub use raw::ConnResult;
pub use raw::Resource;
pub use raw::Xid;
