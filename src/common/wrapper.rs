// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{Timestamp, Window};
use xrbk::{ConstantX11Size, Wrap};
use xrbk_macro::{Readable, Wrapper, Writable};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Wrapper, Readable, Writable)]
/// Values which may be copied from the 'parent'.
#[wrapper(T::Integer)]
pub enum ParentCopyable<T> where T: Wrap {
	/// A value is initialized by copying the matching value of the parent.
	///
	/// For example, when creating a [window] with a [`CreateWindow` request], the
	/// class is <code>ParentCopyable<[WindowClass]></code> - `CopyFromParent`
	/// in that case means to copy the [`WindowClass`] of the [window]'s parent.
	///
	/// [`CreateWindow` request]: crate::x11::request::CreateWindow
	/// [window]: Window
	/// [`WindowClass`]: crate::WindowClass
	/// [WindowClass]: crate::WindowClass
	CopyFromParent,

	/// The value is initialized as this value.
	#[wrapper(fallback)]
	Other(T),
}

impl<T: Wrap> ConstantX11Size for ParentCopyable<T> {
	const X11_SIZE: usize = T::X11_SIZE;
}

#[derive(Wrapper)]
/// Values which may be the same as the 'parent' as long as the parent has the
/// same `depth`.
///
/// This is only used for [pixmaps]. The purpose of specifying `T` is to clearly
/// show that it 'wraps' a [pixmap].
///
/// [pixmaps]: crate::Pixmap
/// [pixmap]: crate::Pixmap
#[wrapper(T::Integer)]
pub enum ParentRelatable<T> where T: Wrap {
	/// The value of the 'parent' is used, as long as the parent has the same
	/// `depth`.
	ParentRelative = 1,

	/// This value is used.
	#[wrapper(fallback)]
	Other(T),
}

impl<T: Wrap> ConstantX11Size for ParentRelatable<T> {
	const X11_SIZE: usize = T::X11_SIZE;
}

#[derive(Wrapper)]
/// Either [`Any`] value or a specific value.
///
/// [`Any`]: MaybeAny::Any
#[wrapper(T::Integer)]
pub enum MaybeAny<T> where T: Wrap {
	/// Any value.
	Any,

	#[wrapper(fallback)]
	/// This specific value.
	Other(T),
}

impl<T: Wrap> ConstantX11Size for MaybeAny<T> {
	const X11_SIZE: usize = T::X11_SIZE;
}

#[derive(Wrapper)]
/// A time which may simply fill in for the current server time.
#[wrapper(u32)]
pub enum CurrentableTime {
	/// The X server should treat this time as its current time.
	CurrentTime,

	#[wrapper(fallback)]
	/// The X server should treat this time as this `Timestamp`.
	Other(Timestamp),
}

impl ConstantX11Size for CurrentableTime {
	const X11_SIZE: usize = Timestamp::X11_SIZE;
}

#[derive(Wrapper)]
/// The `destination` of a [`SendEvent` request].
///
/// [`SendEvent` request]: crate::x11::request::SendEvent
#[wrapper(u32)]
pub enum DestinationWindow {
	/// The [window] that the cursor is currently located within.
	///
	/// [window]: Window
	CursorWindow,
	/// The [window] which is currently focused.
	///
	/// [window]: Window
	Focus,

	/// This [window] in particular.
	///
	/// [window]: Window
	#[wrapper(fallback)]
	Other(Window),
}

impl ConstantX11Size for DestinationWindow {
	const X11_SIZE: usize = Window::X11_SIZE;
}

#[derive(Wrapper)]
/// The [window] which is focused.
///
/// [window]: Window
#[wrapper(u32)]
pub enum WindowFocus {
	/// No focused [window].
	///
	/// [window]: Window
	None,
	/// The root [window] of whichever [window] the cursor is located within.
	///
	/// This dynamically changes root [window] based on the location of the
	/// cursor.
	///
	/// [window]: Window
	CursorRoot,

	#[wrapper(fallback)]
	/// This specific [window].
	Other(Window),
}

impl ConstantX11Size for WindowFocus {
	const X11_SIZE: usize = Window::X11_SIZE;
}

#[derive(Wrapper)]
/// The target client(s) of a [`KillClient` request].
///
/// [`KillClient` request]: crate::request::KillClient
#[wrapper(u32)]
pub enum KillClientTarget {
	/// Kill all clients with [`CloseDownMode::RetainTemporary`].
	///
	/// [`CloseDownMode::RetainTemporary`]: crate::CloseDownMode::RetainTemporary
	AllTemporarilyRetainedClients,

	#[wrapper(fallback)]
	/// Kill the client which created the resource specified by this resource ID.
	Other(u32),
}

impl ConstantX11Size for KillClientTarget {
	const X11_SIZE: usize = u32::X11_SIZE;
}
