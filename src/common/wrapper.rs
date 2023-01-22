// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xrbk::{
	Buf,
	BufMut,
	ConstantX11Size,
	ReadError,
	ReadResult,
	Readable,
	Writable,
	WriteResult,
	X11Size,
};

use crate::{
	atom::Atom,
	visual::VisualId,
	Button,
	Colormap,
	Keycode,
	Pixmap,
	Timestamp,
	Window,
	WindowClass,
};

macro_rules! impl_constant_x11_size { // {{{
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
} // }}}

/// Values which may be copied from the 'parent'.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum CopyableFromParent<T> {
	/// A value is initialized by copying the matching value of the parent.
	///
	/// For example, when creating a [window] with a [`CreateWindow` request],
	/// the class is <code>CopyableFromParent<[WindowClass]></code> -
	/// `CopyFromParent` in that case means to copy the [`WindowClass`] of the
	/// [window]'s parent.
	///
	/// [`CreateWindow` request]: crate::x11::request::CreateWindow
	/// [window]: Window
	/// [`WindowClass`]: WindowClass
	/// [WindowClass]: WindowClass
	CopyFromParent,

	/// The value is initialized as this value.
	Other(T),
}

impl_constant_x11_size!(CopyableFromParent<WindowClass> { // {{{
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

impl_constant_x11_size!(CopyableFromParent<VisualId> {
	VisualId::X11_SIZE
});

impl_readable!(CopyableFromParent<VisualId>: buf {
	Ok(match buf.get_u32() {
		discrim if discrim == 0 => Self::CopyFromParent,
		val => Self::Other(VisualId::new(val)),
	})
});

impl_writable!(CopyableFromParent<VisualId>: &self, buf {
	match self {
		Self::CopyFromParent => buf.put_u32(0),
		Self::Other(id) => id.write_to(buf)?,
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
});

impl_constant_x11_size!(CopyableFromParent<u8> {
	u8::X11_SIZE
});

impl_readable!(CopyableFromParent<u8>: buf {
	Ok(match buf.get_u8() {
		discrim if discrim == 0 => Self::CopyFromParent,
		val => Self::Other(val),
	})
});

impl_writable!(CopyableFromParent<u8>: &self, buf {
	match self {
		Self::CopyFromParent => buf.put_u32(0),
		Self::Other(val) => val.write_to(buf)?,
	}

	Ok(())
}); // }}}

/// Values which may be the same as the 'parent' as long as the parent has the
/// same `depth`.
///
/// This is only used for [pixmaps]. The purpose of specifying `T` is to clearly
/// show that it 'wraps' a [pixmap].
///
/// [pixmaps]: Pixmap
/// [pixmap]: Pixmap
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ParentRelatable<T> {
	/// The value of the 'parent' is used, as long as the parent has the same
	/// `depth`.
	ParentRelative,

	/// This value is used.
	Other(T),
}

impl_constant_x11_size!(ParentRelatable<Option<Pixmap>> { // {{{
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

/// Either [`Any`] value or a specific value.
///
/// [`Any`]: Any::Any
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Any<T> {
	/// Any value.
	Any,

	/// This specific value.
	Other(T),
}

impl_constant_x11_size!(Any<Atom> { // {{{
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

/// A time which may simply fill in for the current server time.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum CurrentableTime {
	/// The X server should treat this time as its current time.
	CurrentTime,

	/// The X server should treat this time as this `Timestamp`.
	Other(Timestamp),
}

impl_constant_x11_size!(CurrentableTime { // {{{
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

/// The `destination` of a [`SendEvent` request].
///
/// [`SendEvent` request]: crate::x11::request::SendEvent
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum DestinationWindow {
	/// The [window] that the cursor is currently located within.
	///
	/// [window]: Window
	Cursor,
	/// The [window] which is currently focused.
	///
	/// [window]: Window
	Focus,

	/// This [window] in particular.
	///
	/// [window]: Window
	Other(Window),
}

impl_constant_x11_size!(DestinationWindow { // {{{
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

/// The [window] which is focused.
///
/// [window]: Window
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum FocusWindow {
	/// No [window] is focused.
	///
	/// [window]: Window
	None,
	/// The root [window] of whichever [window] the cursor is located within is
	/// focused.
	///
	/// This dynamically changes root [window] based on the location of the
	/// cursor.
	///
	/// [window]: Window
	CursorRoot,

	/// This specific [window].
	Other(Window),
}

impl_constant_x11_size!(FocusWindow { // {{{
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

		Self::Other(window) => window.write_to(buf)?,
	}

	Ok(())
}); // }}}

/// The target client(s) of a [`KillClient` request].
///
/// [`KillClient` request]: crate::request::KillClient
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum KillClientTarget {
	/// Kill all clients with [`CloseDownMode::RetainTemporary`].
	///
	/// [`CloseDownMode::RetainTemporary`]: crate::CloseDownMode::RetainTemporary
	// FIXME: correct CloseDownMode link
	AllTemporarilyRetainedClients,

	/// Kill the client which created the resource specified by this resource
	/// ID.
	Other(u32),
}

impl_constant_x11_size!(KillClientTarget { // {{{
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
