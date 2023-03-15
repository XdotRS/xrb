// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Requests] defined in the [core X11 protocol] that relate to an X client or
//! the X server.
//!
//! [Requests] are messages sent from an X client to the X server.
//!
//! [Requests]: Request
//! [core X11 protocol]: crate::x11

extern crate self as xrb;

use std::convert::Infallible;
use xrbk::{
	pad,
	Buf,
	BufMut,
	ConstantX11Size,
	ReadError::FailedConversion,
	ReadResult,
	Readable,
	Writable,
	WriteResult,
	X11Size,
};
use xrbk_macro::{derive_xrb, Readable, Writable, X11Size};

use crate::{
	message::Request,
	unit::Sec,
	x11::{error, reply},
	Host,
	KillClientTarget,
	String8,
	Toggle,
	ToggleOrDefault,
	Window,
};

macro_rules! request_error {
	(
		$(#[$meta:meta])*
		$vis:vis enum $Name:ident for $Request:ty {
			$($($Error:ident),+$(,)?)?
		}
	) => {
		#[doc = concat!(
			"An [error](crate::message::Error) generated because of a failed [`",
			stringify!($Request),
			"` request](",
			stringify!($Request),
			")."
		)]
		#[doc = ""]
		$(#[$meta])*
		$vis enum $Name {
			$($(
				#[doc = concat!(
					"A [`",
					stringify!($Error),
					"` error](error::",
					stringify!($Error),
					")."
				)]
				$Error(error::$Error)
			),+)?
		}
	};
}

request_error! {
	pub enum ChangeSavedWindowsError for ChangeSavedWindows {
		Match,
		Value,
		Window,
	}
}

/// Whether something is added or removed.
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum AddOrRemove {
	/// The thing is added.
	Add,
	/// The thing is removed.
	Remove,
}

derive_xrb! {
	/// A [request] that [adds] or [removes] the specified [window] from the
	/// set of [windows][window] which you have chosen to save.
	///
	/// When a client's resources are destroyed, each of the client's saved
	/// [windows] which are descendents of [windows] created by the client is
	/// [reparented] to the closest ancestor which is not created by the client.
	///
	/// # Errors
	/// The given `window` must not be a [window] created by you, else a
	/// [`Match` error] is generated.
	///
	/// A [`Window` error] is generated if the `window` does not refer to a
	/// defined [window].
	///
	/// A [`Value` error] is generated if the `change_mode` is encoded
	/// incorrectly. It is a bug in X Rust Bindings if that happens.
	///
	/// [window]: Window
	/// [windows]: Window
	/// [request]: Request
	///
	/// [adds]: AddOrRemove::Add
	/// [removes]: AddOrRemove::Remove
	///
	/// [reparented]: super::ReparentWindow
	#[doc(alias = "ChangeSaveSet")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct ChangeSavedWindows: Request(6, ChangeSavedWindowsError) {
		#[metabyte]
		/// Whether the `window` is added to or removed from your saved
		/// [windows].
		///
		/// [windows]: Window
		#[doc(alias = "mode")]
		pub change_mode: AddOrRemove,

		/// The [window] which is added to or removed from your saved
		/// [windows][window].
		///
		/// # Errors
		/// A [`Match` error] is generated if you created this [window].
		///
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [`Match` error]: error::Match
		/// [`Window` error]: error::Window
		pub window: Window,
	}

	/// A [request] that returns whether the specified extension is present and
	/// the message codes associated with it if it is.
	///
	/// # Replies
	/// This [request] generates a [`QueryExtension` reply].
	///
	/// [request]: Request
	///
	/// [`QueryExtension` reply]: reply::QueryExtension
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct QueryExtension: Request(98) -> reply::QueryExtension {
		// Length of `name`.
		#[allow(clippy::cast_possible_truncation)]
		let name_len: u16 = name => name.len() as u16,
		[_; 2],

		/// The name of the extension which is to be queried.
		///
		/// This name should use ISO Latin-1 encoding. Uppercase and lowercase
		/// matter.
		#[context(name_len => usize::from(*name_len))]
		pub name: String8,
		[_; name => pad(name)],
	}

	/// A [request] that returns the names of all extensions supported by the X server.
	///
	/// # Replies
	/// This [request] generates a [`ListExtensions` reply].
	///
	/// [request]: Request
	///
	/// [`ListExtensions` reply]: reply::ListExtensions
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct ListExtensions: Request(99) -> reply::ListExtensions;
}

/// The delay used for `timeout` and `interval` in the
/// [`SetScreenSaver` request].
///
/// [`SetScreenSaver` request]: SetScreenSaver
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Delay {
	/// The default option is used.
	Default,
	/// The option is disabled.
	Disabled,

	/// The option is enabled after the given delay.
	Enabled(Sec<u8>),
}

impl ConstantX11Size for Delay {
	const X11_SIZE: usize = i16::X11_SIZE;
}

impl X11Size for Delay {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for Delay {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		match buf.get_i16() {
			-1 => Ok(Self::Default),
			0 => Ok(Self::Disabled),

			other => match u8::try_from(other) {
				Ok(sec) => Ok(Self::Enabled(Sec(sec))),
				Err(error) => Err(FailedConversion(Box::new(error))),
			},
		}
	}
}

impl Writable for Delay {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		match self {
			Self::Default => buf.put_i16(-1),
			Self::Disabled => buf.put_i16(0),

			Self::Enabled(Sec(sec)) => i16::from(*sec).write_to(buf)?,
		}

		Ok(())
	}
}

derive_xrb! {
	/// A [request] that configures options for the screensaver.
	///
	/// The screensaver is enabled if [`timeout`] is
	/// [`Enabled`](Delay::Enabled). When it is enabled, after [`timeout`]
	/// seconds without any cursor or keyboard input, the screensaver is
	/// activated.
	///
	/// If [`prefer_blanking`] is [`Enabled`], displays that support blanking
	/// will go blank when the screensaver is activated.
	///
	/// Otherwise, if [`prefer_blanking`] is [`Disabled`] or the display does
	/// not support blanking and either [`allow_expose_events`] is [`Enabled`]
	/// or the [screen] can be changed without generating [`Expose` events], the
	/// [screen] is changed with a server-specific screensaver.
	///
	/// Otherwise, if [`prefer_blanking`] is [`Disabled`], the display does
	/// not support blanking, or [`allow_expose_events`] is [`Disabled`] and the
	/// [screen] cannot be changed without generating [`Expose` events], no
	/// screensaver is activated.
	///
	/// [screen]: crate::visual::Screen
	/// [request]: Request
	///
	/// [`Enabled`]: ToggleOrDefault::Enabled
	/// [`Disabled`]: ToggleOrDefault::Disabled
	///
	/// [`timeout`]: SetScreenSaver::timeout
	/// [`prefer_blanking`]: SetScreenSaver::prefer_blanking
	/// [`allow_expose_events`]: SetScreenSaver::allow_expose_events
	///
	/// [`Expose` events]: crate::x11::event::Expose
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct SetScreenSaver: Request(107, error::Value) {
		/// Whether the screensaver is [`Enabled`] and, if so, how long without
		/// input before it is activated.
		///
		/// [`Enabled`]: Delay::Enabled
		pub timeout: Delay,
		/// A hint for screensavers with periodic changes as to the interval
		/// between those changes.
		///
		/// If [`Delay::Disabled`] is specified, this hints that no periodic
		/// change should be made.
		pub interval: Delay,

		/// Whether it is preferred that displays that support blanking go blank
		/// when the screensaver is activated.
		pub prefer_blanking: ToggleOrDefault,
		/// Whether screensavers which generate [`Expose` events] are allowed.
		///
		/// [`Expose` events]: crate::x11::event::Expose
		pub allow_expose_events: ToggleOrDefault,
		[_; 2],
	}

	/// A [request] that returns the current [screensaver options].
	///
	/// See also: [`SetScreenSaver`].
	///
	/// # Replies
	/// This [request] generates a [`GetScreenSaver` reply].
	///
	/// [screensaver options]: SetScreenSaver
	/// [request]: Request
	///
	/// [`GetScreenSaver` reply]: reply::GetScreenSaver
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct GetScreenSaver: Request(108) -> reply::GetScreenSaver;
}

request_error! {
	pub enum ChangeHostsError for ChangeHosts {
		Access,
		Value,
	}
}

derive_xrb! {
	/// A [request] that [adds] or [removes] the specified host from the access
	/// control list.
	///
	/// ***Note: the use of the access control list is deprecated: more secure
	/// forms of authentication, such as those based on shared secrets or public
	/// key encryption are recommended.***
	///
	/// When access control is enabled and a client attempts to establish a
	/// connection to the X server, the client's host must be in the access
	/// control list - if it is not, and the client has not been granted
	/// permission by some other server-specific functionality, the connection
	/// is refused.
	///
	/// To send this [request], your client must be on the same host as the X
	/// server, or have been granted permission by some other server-specific
	/// functionality.
	///
	/// # Errors
	/// An [`Access` error] is generated if your client is not on the same host
	/// as the X server and has not been granted permission to send this
	/// [request] in some other server-specific way.
	///
	/// [request]: Request
	///
	/// [adds]: AddOrRemove::Add
	/// [removes]: AddOrRemove::Remove
	///
	/// [`Access` error]: error::Access
	#[deprecated(note = "more secure forms of authentication are preferred.")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ChangeHosts: Request(109, ChangeHostsError) {
		/// Whether the `host` is to be [added] to or [removed] from the access
		/// control list.
		///
		/// [added]: AddOrRemove::Add
		/// [removed]: AddOrRemove::Remove
		#[metabyte]
		pub mode: AddOrRemove,

		/// The [host] which is to be [added] to or [removed] from the access
		/// control list.
		///
		/// [host]: Host
		///
		/// [added]: AddOrRemove::Add
		/// [removed]: AddOrRemove::Remove
		pub host: Host,
	}

	/// A [request] that returns whether access control is [enabled] and the
	/// [hosts] on the access control list.
	///
	/// ***Note: the use of the access control list is deprecated: more secure
	/// forms of authentication, such as those based on shared secrets or public
	/// key encryption are recommended.***
	///
	/// # Replies
	/// This [request] generates a [`QueryAccessControl` reply].
	///
	/// [hosts]: Host
	/// [enabled]: Toggle::Enabled
	/// [request]: Request
	///
	/// [`QueryAccessControl` reply]: reply::QueryAccessControl
	#[doc(alias("ListHosts"))]
	#[deprecated(note = "more secure forms of authentication are preferred.")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct QueryAccessControl: Request(110) -> reply::QueryAccessControl;
}

request_error! {
	pub enum SetAccessControlError for SetAccessControl {
		Access,
		Value,
	}
}

derive_xrb! {
	/// A [request] that sets access control to either [enabled] or [disabled].
	///
	/// ***Note: the use of the access control list is deprecated: more secure
	/// forms of authentication, such as those based on shared secrets or public
	/// key encryption are recommended.***
	///
	/// [request]: Request
	///
	/// [enabled]: Toggle::Enabled
	/// [disabled]: Toggle::Disabled
	#[deprecated(note = "more secure forms of authentication are preferred.")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct SetAccessControl: Request(111, SetAccessControlError) {
		/// Whether access control is [enabled] or [disabled].
		///
		/// [enabled]: Toggle::Enabled
		/// [disabled]: Toggle::Disabled
		#[metabyte]
		pub mode: Toggle,
	}
}

/// Defines what happens to a client's resources when its connection ends.
///
/// The default mode (i.e. the mode set when a connection is first set up)
/// is [`Destroy`].
///
/// [`Destroy`]: RetainResourcesMode::Destroy
#[doc(alias("CloseDownMode"))]
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum RetainResourcesMode {
	/// All of the client's resources are destroyed immediately.
	///
	/// Descendents of [windows] created by the client that are
	/// [chosen to be saved] are reparented as described in the
	/// [`ChangeSavedWindows` request].
	///
	/// Ending a connection with [`RetainResourcesMode::Destroy`] will, if it is
	/// the only remaining connection to the X server, cause a server reset: the
	/// X server's state is reset as if it had just been started. That includes
	/// destroying remaining resources retained due to the end of
	/// [`RetainResourcesMode::RetainPermanently`] or
	/// [`RetainResourcesMode::RetainTemporarily`] connections.
	///
	/// [windows]: Window
	/// [chosen to be saved]: ChangeSavedWindows
	///
	/// [`ChangeSavedWindows` request]: ChangeSavedWindows
	Destroy,

	/// All of the client's resources are marked as permanently retained.
	///
	/// Ending a connection with [`RetainResourcesMode::RetainPermanently`] will
	/// not cause the X server to reset.
	#[doc(alias("RetainPermanent"))]
	RetainPermanently,
	/// All of the client's resources are marked as temporarily retained.
	///
	/// Ending a connection with [`RetainResourcesMode::RetainTemporarily`] will
	/// not cause the X server to reset.
	#[doc(alias("RetainTemporary"))]
	RetainTemporarily,
}

derive_xrb! {
	/// A [request] that changes your client's [`RetainResourcesMode`].
	///
	/// The default mode (i.e. the mode set when a connection is first set up)
	/// is [`RetainResourcesMode::Destroy`].
	///
	/// See [`RetainResourcesMode`] for more information.
	///
	/// [request]: Request
	#[doc(alias("SetCloseDownMode"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct SetRetainResourcesMode: Request(112, error::Value) {
		/// The [`RetainResourcesMode`] set for your client.
		///
		/// See [`RetainResourcesMode`] for more information.
		#[metabyte]
		pub mode: RetainResourcesMode,
	}

	/// A [request] that either kills the `target` client or deletes retained
	/// resources.
	///
	/// If [`KillClientTarget::KillClient`] is specified, the client which
	/// created the given resource is killed if it still has an active
	/// connection. If its connection has already ended, all resources retained
	/// by that client (whether with [`RetainResourcesMode::RetainTemporarily`]
	/// or with [`RetainResourcesMode::RetainPermanently`]) are destroyed.
	///
	/// If [`KillClientTarget::DestroyTemporarilyRetainedResources`] is
	/// specified, all resources of all clients whose connections have ended
	/// with [`RetainResourcesMode::RetainTemporarily`] are destroyed.
	///
	/// [request]: Request
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct KillClient: Request(113, error::Value) {
		/// The target of this `KillClient` [request].
		///
		/// See [`KillClient`] and [`KillClientTarget`] for more information.
		///
		/// [request]: Request
		pub target: KillClientTarget,
	}
}

/// Whether a [`ForceScreenSaver` request] [resets the activation timer] or
/// [activates the screensaver]
///
/// [resets the activation timer]: ForceScreenSaverMode::Reset
/// [activates the screensaver]: ForceScreenSaverMode::Activate
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum ForceScreenSaverMode {
	/// If the screensaver is currently [enabled], the activation timer (i.e.
	/// the time left before its activation) is reset and, if the screensaver is
	/// active, the screensaver is deactivated.
	///
	/// [enabled]: ToggleOrDefault::Enabled
	Reset,

	/// If the screensaver is not currently active, it is forcibly activated.
	///
	/// The screensaver is activated even if [`timeout`] is [`Disabled`].
	///
	/// [`timeout`]: SetScreenSaver::timeout
	/// [`Disabled`]: ToggleOrDefault::Disabled
	Activate,
}

derive_xrb! {
	/// A [request] that either
	/// [resets the timer until the screensaver is activated][reset], or
	/// [forcibly activates the screensaver][activate].
	///
	/// See [`ForceScreenSaverMode`] for more information.
	///
	/// [request]: Request
	///
	/// [reset]: ForceScreenSaverMode::Reset
	/// [activate]: ForceScreenSaverMode::Activate
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ForceScreenSaver: Request(115, error::Value) {
		/// Whether the screensaver's [activation timer is reset] or the
		/// screensaver is [forcibly activated].
		///
		/// See [`ForceScreenSaverMode`] for more information.
		///
		/// [activation timer is reset]: ForceScreenSaverMode::Reset
		/// [forcibly activated]: ForceScreenSaverMode::Activate
		#[metabyte]
		pub mode: ForceScreenSaverMode,
	}
}

/// A [request] that has no effect.
///
/// The use of this [request] comes with padding: `4 + (4 * unused_units)` bytes
/// are sent.
///
/// This can be used by X libraries which find it convenient to force
/// [requests][request] to be aligned to 8 bytes.
///
/// [request]: Request
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct NoOp {
	/// The number of unused 4-byte units to add to the [request] after the
	/// initial 4-byte header.
	pub unused_units: u16,
}

impl Request for NoOp {
	type OtherErrors = Infallible;
	type Reply = ();

	fn major_opcode() -> u8 {
	    127
	}
	const MINOR_OPCODE: Option<u8> = None;
}

impl X11Size for NoOp {
	fn x11_size(&self) -> usize {
		const HEADER: usize = 4;
		const ALIGNMENT: usize = 4;

		HEADER + (usize::from(self.unused_units) * ALIGNMENT)
	}
}

impl Readable for NoOp {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self> {
		const ALIGNMENT: usize = 4;

		// Unused metabyte.
		buf.advance(1);

		// One unit is subtracted for the header.
		let unused_units = buf.get_u16() - 1;

		let buf = &mut buf.take(usize::from(unused_units) * ALIGNMENT);
		// Unused bytes.
		buf.advance(buf.remaining());

		Ok(Self { unused_units })
	}
}

impl Writable for NoOp {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		const ALIGNMENT: usize = 4;

		let buf = &mut buf.limit(self.x11_size());

		Self::major_opcode().write_to(buf)?;
		// Unused metabyte.
		buf.put_u8(0);
		// Message length.
		self.length().write_to(buf)?;

		// Unused bytes.
		buf.put_bytes(0, usize::from(self.unused_units) * ALIGNMENT);

		Ok(())
	}
}
