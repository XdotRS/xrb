// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xrbk::{Buf, BufMut, ConstantX11Size, X11Size, Readable, Writable, ReadResult, WriteResult, ReadError};

use crate::{Atom, Button, Colormap, Keycode, Pixmap, Timestamp, Window, WindowClass};

macro_rules! impl_constant_x11_size {
	($type:ty {
		$($token:tt)*
	}) => {
		impl ConstantX11Size for $type {
			const X11_SIZE: usize = { $($token)* };
		}

		impl X11Size for $type {
			fn x11_size(&self) -> usize {
				Self::X11_SIZE
			}
		}
	}
}

macro_rules! impl_readable {
	($type:ty: $buf:ident {
		$($token:tt)*
	}) => {
		impl Readable for $type {
			fn read_from($buf: &mut impl Buf) -> ReadResult<Self> {
				$($token)*
			}
		}
	};
}

macro_rules! impl_writable {
	($type:ty: &$self:ident, $buf:ident {
		$($token:tt)*
	}) => {
		impl Writable for $type {
			fn write_to(&$self, $buf: &mut impl BufMut) -> WriteResult {
				$($token)*
			}
		}
	}
}

pub enum CopyableFromParent<T> { // {{{
	CopyFromParent,
	Other(T),
}

impl_constant_x11_size!(CopyableFromParent<WindowClass> {
	WindowClass::X11_SIZE
});

impl_readable!(CopyableFromParent<WindowClass>: buf {
	match buf.get_u32() {
		discrim if discrim == 0 => Ok(Self::CopyFromParent),

		discrim if discrim == 1 => Ok(Self::Other(WindowClass::InputOutput)),
		discrim if discrim == 2 => Ok(Self::Other(WindowClass::InputOnly)),

		other_discrim => Err(ReadError::UnrecognizedDiscriminant(other_discrim as usize)),
	}
});

impl_writable!(CopyableFromParent<WindowClass>: &self, buf {
	match self {
		Self::CopyFromParent => buf.put_u32(0),
		Self::Other(class) => class.write_to(buf)?,
	}

	Ok(())
});

impl_constant_x11_size!(CopyableFromParent<Pixmap> {
	Pixmap::X11_SIZE
});

impl_readable!(CopyableFromParent<Pixmap>: buf {
	Ok(match buf.get_u32() {
		discrim if discrim == 0 => Self::CopyFromParent,
		val => Self::Other(Pixmap::new(val)),
	})
});

impl_writable!(CopyableFromParent<Pixmap>: &self, buf {
	match self {
		Self::CopyFromParent => buf.put_u32(0),
		Self::Other(val) => val.write_to(buf)?,
	}

	Ok(())
});

impl_constant_x11_size!(CopyableFromParent<Colormap> {
	Colormap::X11_SIZE
});

impl_readable!(CopyableFromParent<Colormap>: buf {
	Ok(match buf.get_u32() {
		discrim if discrim == 0 => Self::CopyFromParent,
		val => Self::Other(Colormap::new(val)),
	})
});

impl_writable!(CopyableFromParent<Colormap>: &self, buf {
	match self {
		Self::CopyFromParent => buf.put_u32(0),
		Self::Other(val) => val.write_to(buf)?,
	}

	Ok(())
}); // }}}

pub enum ParentRelatable<T> { // {{{
	ParentRelative,
	Other(T),
}

impl_constant_x11_size!(ParentRelatable<Option<Pixmap>> {
	Pixmap::X11_SIZE
});

impl_readable!(ParentRelatable<Option<Pixmap>>: buf {
	Ok(match buf.get_u32() {
		discrim if discrim == 0 => Self::Other(None),

		discrim if discrim == 1 => Self::ParentRelative,

		val => Self::Other(Some(Pixmap::new(val))),
	})
});

impl_writable!(ParentRelatable<Option<Pixmap>>: &self, buf {
	match self {
		Self::ParentRelative => buf.put_u32(1),

		Self::Other(None) => buf.put_u32(0),
		Self::Other(Some(pixmap)) => pixmap.write_to(buf)?,
	}

	Ok(())
}); // }}}

pub enum Any<T> { // {{{
	Any,
	Other(T),
}

impl_constant_x11_size!(Any<Atom> {
	Atom::X11_SIZE
});

impl_readable!(Any<Atom>: buf {
	Ok(match buf.get_u32() {
		discrim if discrim == 0 => Self::Any,
		val => Self::Other(Atom::new(val)),
	})
});

impl_writable!(Any<Atom>: &self, buf {
	match self {
		Self::Any => buf.put_u32(0),
		Self::Other(atom) => atom.write_to(buf)?,
	}

	Ok(())
});

impl_constant_x11_size!(Any<Button> {
	Button::X11_SIZE
});

impl_readable!(Any<Button>: buf {
	Ok(match buf.get_u8() {
		discrim if discrim == 0 => Self::Any,
		val => Self::Other(Button::new(val)),
	})
});

impl_writable!(Any<Button>: &self, buf {
	match self {
		Self::Any => buf.put_u8(0),
		Self::Other(button) => button.write_to(buf)?,
	}

	Ok(())
});

impl_constant_x11_size!(Any<Keycode> {
	Keycode::X11_SIZE
});

impl_readable!(Any<Keycode>: buf {
	Ok(match buf.get_u8() {
		discrim if discrim == 0 => Self::Any,
		val => Self::Other(Keycode::new(val)),
	})
});

impl_writable!(Any<Keycode>: &self, buf {
	match self {
		Self::Any => buf.put_u8(0),
		Self::Other(keycode) => keycode.write_to(buf)?,
	}

	Ok(())
}); // }}}

pub enum CurrentableTime { // {{{
	CurrentTime,
	Other(Timestamp),
}

impl_constant_x11_size!(CurrentableTime {
	Timestamp::X11_SIZE
});

impl_readable!(CurrentableTime: buf {
	Ok(match buf.get_u32() {
		discrim if discrim == 0 => Self::CurrentTime,
		val => Self::Other(Timestamp::new(val)),
	})
});

impl_writable!(CurrentableTime: &self, buf {
	match self {
		Self::CurrentTime => buf.put_u32(0),
		Self::Other(timestamp) => timestamp.write_to(buf)?,
	}

	Ok(())
}); // }}}

pub enum DestinationWindow { // {{{
	Cursor,
	Focus,
	Other(Window),
}

impl_constant_x11_size!(DestinationWindow {
	Window::X11_SIZE
});

impl_readable!(DestinationWindow: buf {
	Ok(match buf.get_u32() {
		discrim if discrim == 0 => Self::Cursor,
		discrim if discrim == 1 => Self::Focus,

		val => Self::Other(Window::new(val)),
	})
});

impl_writable!(DestinationWindow: &self, buf {
	match self {
		Self::Cursor => buf.put_u32(0),
		Self::Focus => buf.put_u32(1),

		Self::Other(window) => window.write_to(buf)?,
	}

	Ok(())
}); // }}}

pub enum FocusWindow { // {{{
	None,
	CursorRoot,
	Other(Window),
}

impl_constant_x11_size!(FocusWindow {
	Window::X11_SIZE
});

impl_readable!(FocusWindow: buf {
	Ok(match buf.get_u32() {
		discrim if discrim == 0 => Self::None,
		discrim if discrim == 1 => Self::CursorRoot,

		val => Self::Other(Window::new(val)),
	})
});

impl_writable!(FocusWindow: &self, buf {
	match self {
		Self::None => buf.put_u32(0),
		Self::CursorRoot => buf.put_u32(1),
	}

	Ok(())
}); // }}}

pub enum KillClientTarget { // {{{
	AllTemporarilyRetainedClients,
	Other(u32),
}

impl_constant_x11_size!(KillClientTarget {
	u32::X11_SIZE
});

impl_readable!(KillClientTarget: buf {
	Ok(match buf.get_u32() {
		discrim if discrim == 0 => Self::AllTemporarilyRetainedClients,

		val => Self::Other(val),
	})
});

impl_writable!(KillClientTarget: &self, buf {
	match self {
		Self::AllTemporarilyRetainedClients => buf.put_u32(0),
		Self::Other(val) => buf.put_u32(*val),
	}

	Ok(())
}); // }}}
