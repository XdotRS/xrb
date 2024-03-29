// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(missing_docs)]

use bitflags::bitflags;
use xrbk_macro::{ConstantX11Size, Readable, Writable, X11Size};

bitflags! {
	#[derive(Default, X11Size, Readable, ConstantX11Size, Writable)]
	pub struct ColorChannelMask: u8 {
		/// Whether the red color channel is enabled.
		const RED = 0x01;
		/// Whether the green color channel is enabled.
		const GREEN = 0x02;
		/// Whether the blue color channel is enabled.
		const BLUE = 0x04;
	}

	/// A mask of events.
	#[derive(Default, X11Size, Readable, ConstantX11Size, Writable)]
	pub struct EventMask: u32 {
		/// Key press events.
		const KEY_PRESS = 0x0000_0001;
		/// Key release events.
		const KEY_RELEASE = 0x0000_0002;

		/// Mouse button press events.
		const BUTTON_PRESS = 0x0000_0004;
		/// Mouse button release events.
		const BUTTON_RELEASE = 0x0000_0008;

		/// Cursor events generated when the cursor enters a window.
		///
		/// `ENTER_WINDOW` events are generated not only when the cursor moves
		/// to enter another window, but when the window under the cursor's
		/// current position changes.
		const ENTER_WINDOW = 0x0000_0010;
		/// Cursor events generated when the cursor leaves a window.
		///
		/// `LEAVE_WINDOW` events are generated not only when the cursor moves
		/// away from a window, but when the window under the cursor's current
		/// position moves or changes to a different window.
		const LEAVE_WINDOW = 0x0000_0020;

		/// Cursor motion events generated when the cursor's position changes.
		const ANY_MOTION = 0x0000_0040;
		// TODO: MOTION_HINT docs!
		const MOTION_HINT = 0x0000_0080;
		/// Cursor 'drag' events when the primary mouse button is held.
		///
		/// The primary mouse button is usually the one on the left, but many
		/// tools offer options to switch the positions of the primary and
		/// secondary mouse buttons.
		const BUTTON_1_MOTION = 0x0000_0100;
		/// Cursor 'drag' events when the middle mouse button is held.
		const BUTTON_2_MOTION = 0x0000_0200;
		/// Cursor 'drag' events when the secondary mouse button is held.
		///
		/// The secondary mouse button is usually the one on the right, but many
		/// tools offer options to switch the positions of the primary and
		/// secondary mouse buttons.
		const BUTTON_3_MOTION = 0x0000_0400;
		/// Cursor 'drag' events when 'mouse button 4' is held.
		const BUTTON_4_MOTION = 0x0000_0800;
		/// Cursor 'drag' events when 'mouse button 5' is held.
		const BUTTON_5_MOTION = 0x0000_1000;
		/// Cursor 'drag' events when any mouse button is held.
		const ANY_BUTTON_MOTION = 0x0000_2000;

		/// Events generated after every [`EnterWindow`] and [`Focus`] event
		/// reporting the currently held keys.
		///
		/// [`EnterWindow`]: crate::x11::event::EnterWindow
		/// [`Focus`]: crate::x11::event::Focus
		const KEYBOARD_STATE = 0x0000_4000;

		/// Events generated for arbitrary rectangular areas of windows that
		/// need to be rendered.
		///
		/// [Exposure] events[^ex] are generated when there are no valid contents
		/// available for region(s) of a window. For example, this might be true
		/// when a window is resized to become larger and new parts of the
		/// window are exposed for rendering, or the content of the window that
		/// is to be rendered has changed.
		///
		/// [^ex]: crate::x11::events::Exposure
		/// [Exposure]: crate::x11::events::Exposure
		const EXPOSURE = 0x0000_8000;
		/// Events generated when the [visibility] of a window changes.
		///
		/// [visibility]: crate::x11::MapState
		const VISIBILITY_CHANGE = 0x0001_0000;

		/// Events generated when the structure of a window changes.
		///
		/// In contrast to [`SUBSTRUCTURE_NOTIFY`], `STRUCTURE_NOTIFY` events
		/// are generated when the structure of a window itself changes, rather
		/// than when the structure of its children changes.
		///
		/// [`SUBSTRUCTURE_NOTIFY`]: EventMask::SUBSTRUCTURE_NOTIFY
		const STRUCTURE_NOTIFY = 0x0002_0000;
		/// Events generated when another client sends a
		/// [`ConfigureWindow` request] for a window which attempts to change
		/// its size.
		///
		/// [`ConfigureWindow` request]: crate::x11::request::ConfigureWindow
		const RESIZE_REDIRECT = 0x0004_0000;
		/// Events generated when the substructure of a window changes.
		///
		/// In contrast to [`STRUCTURE_NOTIFY`], `SUBSTRUCTURE_NOTIFY` events
		/// are generated when the structure of a window's _children_ changes,
		/// rather than when the structure of that window itself changes.
		///
		/// A window manager will commonly select for `SUBSTRUCTURE_NOTIFY` and
		/// [`SUBSTRUCTURE_REDIRECT`] on the root window. The
		/// `SUBSTRUCTURE_NOTIFY` mask allows it to gather information about
		/// changes occurring to windows (i.e., direct children of the root
		/// window).
		///
		/// [`STRUCTURE_NOTIFY`]: EventMask::STRUCTURE_NOTIFY
		/// [`SUBSTRUCTURE_REDIRECT`]: EventMask::SUBSTRUCTURE_REDIRECT
		const SUBSTRUCTURE_NOTIFY = 0x0008_0000;
		/// Redirects certain structural requests to the selecting client.
		///
		/// `SUBSTRUCTURE_REDIRECT` allows a client to have certain requests
		/// relating to the structure of the direct children of the selected
		/// window redirected to itself. It is commonly selected by window
		/// managers so that they can have their own 'verdict' on whether to
		/// honor, modify, or reject certain requests sent by a window.
		const SUBSTRUCTURE_REDIRECT = 0x0010_0000;

		/// Events generated when there are changes to the current input focus.
		// TODO: improve FOCUS_CHANGE docs
		const FOCUS_CHANGE = 0x0020_0000;

		/// Events generated when the properties of a window change.
		const PROPERTY_CHANGE = 0x0040_0000;

		// TODO: docs for COLORMAP_CHANGE
		const COLORMAP_CHANGE = 0x0080_0000;

		// TODO: docs for OWNER_GRAB_BUTTON
		const OWNER_GRAB_BUTTON = 0x0100_0000;
	}

	/// A mask of events relevant to the cursor and buttons.
	///
	/// The difference between a `CursorEventMask` and an [`EventMask`] is that
	/// the following events are unavailable in the mask:
	/// - `KEY_PRESS`
	/// - `KEY_RELEASE`
	/// - `EXPOSURE`
	/// - `VISIBILITY_CHANGE`
	/// - `STRUCTURE_NOTIFY`
	/// - `SUBSTRUCTURE_NOTIFY`
	/// - `SUBSTRUCTURE_REDIRECT`
	/// - `FOCUS_CHANGE`
	/// - `PROPERTY_CHANGE`
	/// - `COLORMAP_CHANGE`
	/// - `OWNER_GRAB_BUTTON`
	#[derive(Default, X11Size, Readable, ConstantX11Size, Writable)]
	pub struct CursorEventMask: u32 {
		// removes KEY_PRESS and KEY_RELEASE
		/// Mouse button press events.
		const BUTTON_PRESS = 0x0000_0004;
		/// Mouse button release events.
		const BUTTON_RELEASE = 0x0000_0008;

		/// Cursor events generated when the cursor enters a window.
		///
		/// `ENTER_WINDOW` events are generated not only when the cursor moves
		/// to enter another window, but when the window under the cursor's
		/// current position changes.
		const ENTER_WINDOW = 0x0000_0010;
		/// Cursor events generated when the cursor leaves a window.
		///
		/// `LEAVE_WINDOW` events are generated not only when the cursor moves
		/// away from a window, but when the window under the cursor's current
		/// position moves or changes to a different window.
		const LEAVE_WINDOW = 0x0000_0020;

		/// Cursor motion events generated when the cursor's position changes.
		const ANY_MOTION = 0x0000_0040;
		const MOTION_HINT = 0x0000_0080;
		/// Cursor 'drag' events when the primary mouse button is held.
		///
		/// The primary mouse button is usually the one on the left, but many
		/// tools offer options to switch the positions of the primary and
		/// secondary mouse buttons.
		const BUTTON_1_MOTION = 0x0000_0100;
		/// Cursor 'drag' events when the middle mouse button is held.
		const BUTTON_2_MOTION = 0x0000_0200;
		/// Cursor 'drag' events when the secondary mouse button is held.
		///
		/// The secondary mouse button is usually the one on the right, but many
		/// tools offer options to switch the positions of the primary and
		/// secondary mouse buttons.
		const BUTTON_3_MOTION = 0x0000_0400;
		/// Cursor 'drag' events when 'mouse button 4' is held.
		const BUTTON_4_MOTION = 0x0000_0800;
		/// Cursor 'drag' events when 'mouse button 5' is held.
		const BUTTON_5_MOTION = 0x0000_1000;
		/// Cursor 'drag' events when any mouse button is held.
		const ANY_BUTTON_MOTION = 0x0000_2000;

		/// Events generated after every [`EnterWindow`] and [`Focus`] event
		/// reporting the currently held keys.
		///
		/// [`EnterWindow`]: crate::x11::event::EnterWindow
		/// [`Focus`]: crate::x11::event::Focus
		const KEY_STATE = 0x0000_4000;

		// removes other events irrelevant to the cursor and buttons
	}

	/// A mask of only input events.
	///
	/// These are events that do not carry contextual information specific to X;
	/// for example, `ENTER_WINDOW` is not available because it specifically
	/// relates to the state of the cursor in relation to the windows on the
	/// screen.
	///
	/// The difference between a `DeviceEventMask` and an [`EventMask`] is
	/// therefore that the following events are unavailable:
	/// - `ENTER_WINDOW`
	/// - `LEAVE_WINDOW`
	/// - `MOTION_HINT`
	/// - `EXPOSURE`
	/// - `VISIBILITY_CHANGE`
	/// - `STRUCTURE_NOTIFY`
	/// - `SUBSTRUCTURE_NOTIFY`
	/// - `SUBSTRUCTURE_REDIRECT`
	/// - `FOCUS_CHANGE`
	/// - `PROPERTY_CHANGE`
	/// - `COLORMAP_CHANGE`
	/// - `OWNER_GRAB_BUTTON`
	#[derive(Default, X11Size, Readable, ConstantX11Size, Writable)]
	pub struct DeviceEventMask: u32 {
		/// Key press events.
		const KEY_PRESS = 0x0000_0001;
		/// Key release events.
		const KEY_RELEASE = 0x0000_0002;

		/// Mouse button press events.
		const BUTTON_PRESS = 0x0000_0004;
		/// Mouse button release events.
		const BUTTON_RELEASE = 0x0000_0008;

		// removes ENTER_WINDOW and LEAVE_WINDOW

		/// Cursor motion events generated when the cursor's position changes.
		const ANY_MOTION = 0x0000_0040;

		// removes MOTION_HINT

		/// Cursor 'drag' events when the primary mouse button is held.
		///
		/// The primary mouse button is usually the one on the left, but many
		/// tools offer options to switch the positions of the primary and
		/// secondary mouse buttons.
		const BUTTON_1_MOTION = 0x0000_0100;
		/// Cursor 'drag' events when the middle mouse button is held.
		const BUTTON_2_MOTION = 0x0000_0200;
		/// Cursor 'drag' events when the secondary mouse button is held.
		///
		/// The secondary mouse button is usually the one on the right, but many
		/// tools offer options to switch the positions of the primary and
		/// secondary mouse buttons.
		const BUTTON_3_MOTION = 0x0000_0400;
		/// Cursor 'drag' events when 'mouse button 4' is held.
		const BUTTON_4_MOTION = 0x0000_0800;
		/// Cursor 'drag' events when 'mouse button 5' is held.
		const BUTTON_5_MOTION = 0x0000_1000;
		/// Cursor 'drag' events when any mouse button is held.
		const ANY_BUTTON_MOTION = 0x0000_2000;

		// removes all other events from this point on
	}

	/// A mask of the currently held modifier keys and mouse buttons.
	///
	/// This is the same as [`ModifierKeyMask`], but with masks for currently
	/// held mouse buttons.
	#[derive(Default, X11Size, Readable, ConstantX11Size, Writable)]
	pub struct ModifierMask: u16 {
		/// Whether `Shift` is held.
		const SHIFT = 0x0001;
		/// Whether `Caps Lock` is active.
		const LOCK = 0x0002;
		/// Whether `Ctrl` is held.
		const CONTROL = 0x0004;

		/// Whether 'modifier key 1' is held.
		const MOD_1 = 0x0008;
		/// Whether 'modifier key 2' is held.
		const MOD_2 = 0x0010;
		/// Whether 'modifier key 3' is held.
		const MOD_3 = 0x0020;
		/// Whether the `Super`/`Meta` key is held.
		///
		/// This key is commonly known as the 'windows key' on Windows devices,
		/// and as 'command' or 'cmd' on MacOS devices.
		const MOD_4 = 0x0040;
		/// Whether 'modifier key 5' is held.
		const MOD_5 = 0x0080;

		/// Whether the primary mouse button is held.
		///
		/// The primary mouse button is usually the one on the left, but many
		/// tools offer options to switch the positions of the primary and
		/// secondary mouse buttons.
		const BUTTON_1 = 0x0100;
		/// Whether the middle mouse button is held.
		const BUTTON_2 = 0x0200;
		/// Whether the secondary mouse button is held.
		///
		/// The secondary mouse button is usually the one on the right, but many
		/// tools offer options to switch the positions of the primary and
		/// secondary mouse buttons.
		const BUTTON_3 = 0x0400;
		/// Whether 'mouse button 4' is held.
		const BUTTON_4 = 0x0800;
		/// Whether 'mouse button 5' is held.
		const BUTTON_5 = 0x1000;
	}

	/// A mask of currently held modifier keys.
	///
	/// This is the same as [`ModifierKeyMask`], but without mouse
	/// button masks. Unlike [`AnyModifierKeyMask`], this does not include a
	/// mask for [`ANY_MODIFIER`].
	///
	/// [`ANY_MODIFIER`]: AnyModifierKeyMask::ANY_MODIFIER
	#[derive(Default, X11Size, Readable, ConstantX11Size, Writable)]
	pub struct ModifierKeyMask: u16 {
		/// Whether `Shift` is held.
		const SHIFT = 0x0001;
		/// Whether `Caps Lock` is active.
		const LOCK = 0x0002;
		/// Whether `Ctrl` is held.
		const CONTROL = 0x0004;

		/// Whether 'modifier key 1' is held.
		const MOD_1 = 0x0008;
		/// Whether 'modifier key 2' is held.
		const MOD_2 = 0x0010;
		/// Whether 'modifier key 3' is held.
		const MOD_3 = 0x0020;
		/// Whether the `Super`/`Meta` key is held.
		///
		/// This key is commonly known as the 'windows key' on Windows devices,
		/// and as 'command' or 'cmd' on MacOS devices.
		const MOD_4 = 0x0040;
		/// Whether 'modifier key 5' is held.
		const MOD_5 = 0x0080;

		// removes BUTTON_#
	}

	/// A mask of currently held modifier keys and a mask for [`ANY_MODIFIER`].
	///
	/// This is the same as [`ModifierKeyMask`], but with the addition of
	/// [`ANY_MODIFIER`].
	///
	/// [`ANY_MODIFIER`]: AnyModifierKeyMask::ANY_MODIFIER
	#[derive(Default, X11Size, Readable, ConstantX11Size, Writable)]
	pub struct AnyModifierKeyMask: u16 {
		/// Whether `Shift` is held.
		const SHIFT = 0x0001;
		/// Whether `Caps Lock` is active.
		const LOCK = 0x0002;
		/// Whether `Ctrl` is held.
		const CONTROL = 0x0004;

		/// Whether 'modifier key 1' is held.
		const MOD_1 = 0x0009;
		/// Whether 'modifier key 2' is held.
		const MOD_2 = 0x0010;
		/// Whether 'modifier key 3' is held.
		const MOD_3 = 0x0020;
		/// Whether the `Super`/`Meta` key is held.
		///
		/// This key is commonly known as the 'windows key' on Windows devices,
		/// and as 'command' or 'cmd' on MacOS devices.
		const MOD_4 = 0x0040;
		/// Whether 'modifier key 5' is held.
		const MOD_5 = 0x0080;

		// removes BUTTON_#
		// adds ANY_MODIFIER
		/// Whether _any_ modifier key is held.
		const ANY_MODIFIER = 0x8000;
	}
}
