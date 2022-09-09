// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::bitflags;
use xrb_proc_macros::{ByteSize, StaticByteSize};

bitflags! {
	/// A mask of events.
	#[derive(StaticByteSize, ByteSize)]
	pub struct EventMask: u32 {
		/// Key press events.
		const KEY_PRESS = 0x00000001;
		/// Key release events.
		const KEY_RELEASE = 0x00000002;
		/// Mouse button press events.
		const BUTTON_PRESS = 0x00000004;
		/// Mouse button release events.
		const BUTTON_RELEASE = 0x00000008;
		/// Pointer events generated when the pointer enters a window.
		///
		/// `ENTER_WINDOW` events are generated not only when the pointer moves
		/// to enter another window, but when the window under the pointer's
		/// current position changes.
		const ENTER_WINDOW = 0x00000010;
		/// Pointer events generated when the pointer leaves a window.
		///
		/// `LEAVE_WINDOW` events are generated not only when the pointer moves
		/// away from a window, but when the window under the pointer's current
		/// position moves or changes to a different window.
		const LEAVE_WINDOW = 0x00000020;
		/// Pointer motion events generated when the pointer's position changes.
		const POINTER_MOTION = 0x00000040;
		// TODO: POINTER_MOTION_HINT docs!
		const POINTER_MOTION_HINT = 0x00000080;
		/// Pointer 'drag' events when the primary mouse button is held.
		///
		/// The primary mouse button is usually the one on the left, but many
		/// tools offer options to switch the positions of the primary and
		/// secondary mouse buttons.
		const BUTTON_1_MOTION = 0x00000100;
		/// Pointer 'drag' events when the middle mouse button is held.
		const BUTTON_2_MOTION = 0x00000200;
		/// Pointer 'drag' events when the secondary mouse button is held.
		///
		/// The secondary mouse button is usually the one on the right, but many
		/// tools offer options to switch the positions of the primary and
		/// secondary mouse buttons.
		const BUTTON_3_MOTION = 0x00000400;
		/// Pointer 'drag' events when 'mouse button 4' is held.
		///
		/// > Help wanted! If you know what 'mouse button 4' represents in terms
		/// > of its common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const BUTTON_4_MOTION = 0x00000800;
		/// Pointer 'drag' events when 'mouse button 5' is held.
		///
		/// > Help wanted! If you know what 'mouse button 5' represents in terms
		/// > of its common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const BUTTON_5_MOTION = 0x00001000;
		/// Pointer 'drag' events when any mouse button is held.
		const BUTTON_MOTION = 0x00002000;
		/// Events generated when the state of the keymap changes.
		///
		/// keymap refers to the mapping between the [`Keycode`]s interpreted
		/// by the kernel and the [`Keysym`]s outputted by X.
		///
		/// [`Keycode`]: crate::x11::common::values::Keycode
		/// [`Keysym`]: crate::x11::common::values::Keysym
		const KEYMAP_STATE = 0x00004000;
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
		const EXPOSURE = 0x00008000;
		/// Events generated when the [visibility] of a window changes.
		///
		/// [visibility]: crate::x11::requests::MapState
		const VISIBILITY_CHANGE = 0x00010000;
		/// Events generated when the structure of a window changes.
		///
		/// In constrast to [`SUBSTRUCTURE_NOTIFY`], `STRUCTURE_NOTIFY` events
		/// are generated when the structure of a window itself changes, rather
		/// than when the structure of its children changes.
		const STRUCTURE_NOTIFY = 0x00020000;
		/// Events generated when the substructure of a window changes.
		///
		/// In constrast to [`STRUCTURE_NOTIFY`], `SUBSTRUCTURE_NOTIFY` events
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
		const SUBSTRUCTURE_NOTIFY = 0x00040000;
		/// Redirects certain structural requests to the selecting client.
		///
		/// `SUBSTRUCTURE_REDIRECT` allows a client to have certain requests
		/// relating to the structure of the direct children of the slected
		/// window redirected to itself. It is commonly selected by window
		/// managers so that they can have their own 'verdict' on whether to
		/// honor, modify, or reject certain requests sent by a window.
		const SUBSTRUCTURE_REDIRECT = 0x00080000;
		/// Events generated when there are changes to the current input focus.
		// TODO: improve FOCUS_CHANGE docs
		const FOCUS_CHANGE = 0x00100000;
		/// Events generated when the properties of a window change.
		const PROPERTY_CHANGE = 0x00400000;
		// TODO: docs for COLORMAP_CHANGE
		const COLORMAP_CHANGE = 0x00800000;
		// TODO: docs for OWNER_GRAB_BUTTON
		const OWNER_GRAB_BUTTON = 0x01000000;
	}

	/// A mask of events relevant to the pointer and buttons.
	///
	/// The difference between a `PointerEventMask` and an [`EventMask`] is that
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
	#[derive(StaticByteSize, ByteSize)]
	pub struct PointerEventMask: u32 {
		// removes KEY_PRESS and KEY_RELEASE
		/// Mouse button press events.
		const BUTTON_PRESS = 0x00000004;
		/// Mouse button release events.
		const BUTTON_RELEASE = 0x00000008;
		/// Pointer events generated when the pointer enters a window.
		///
		/// `ENTER_WINDOW` events are generated not only when the pointer moves
		/// to enter another window, but when the window under the pointer's
		/// current position changes.
		const ENTER_WINDOW = 0x00000010;
		/// Pointer events generated when the pointer leaves a window.
		///
		/// `LEAVE_WINDOW` events are generated not only when the pointer moves
		/// away from a window, but when the window under the pointer's current
		/// position moves or changes to a different window.
		const LEAVE_WINDOW = 0x00000020;
		/// Pointer motion events generated when the pointer's position changes.
		const POINTER_MOTION = 0x00000040;
		const POINTER_MOTION_HINT = 0x00000080;
		/// Pointer 'drag' events when the primary mouse button is held.
		///
		/// The primary mouse button is usually the one on the left, but many
		/// tools offer options to switch the positions of the primary and
		/// secondary mouse buttons.
		const BUTTON_1_MOTION = 0x00000100;
		/// Pointer 'drag' events when the middle mouse button is held.
		const BUTTON_2_MOTION = 0x00000200;
		/// Pointer 'drag' events when the secondary mouse button is held.
		///
		/// The secondary mouse button is usually the one on the right, but many
		/// tools offer options to switch the positions of the primary and
		/// secondary mouse buttons.
		const BUTTON_3_MOTION = 0x00000400;
		/// Pointer 'drag' events when 'mouse button 4' is held.
		///
		/// > Help wanted! If you know what 'mouse button 4' represents in terms
		/// > of its common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const BUTTON_4_MOTION = 0x00000800;
		/// Pointer 'drag' events when 'mouse button 5' is held.
		///
		/// > Help wanted! If you know what 'mouse button 5' represents in terms
		/// > of its common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const BUTTON_5_MOTION = 0x00001000;
		/// Pointer 'drag' events when any mouse button is held.
		const BUTTON_MOTION = 0x00002000;
		/// Events generated when the state of the keymap changes.
		///
		/// keymap refers to the mapping between the [`Keycode`]s interpreted
		/// by the kernel and the [`Keysym`]s outputted by X.
		///
		/// [`Keycode`]: crate::x11::common::values::Keycode
		/// [`Keysym`]: crate::x11::common::values::Keysym
		const KEYMAP_STATE = 0x00004000;
		// removes other events irrelevant to the pointer and buttons
	}

	/// A mask of only input events.
	///
	/// These are events that do not carry contextual information specific to X;
	/// for example, `ENTER_WINDOW` is not available because it specifically
	/// relates to the state of the pointer in relation to the windows on the
	/// screen.
	///
	/// The difference between a `DeviceEventMask` and an [`EventMask`] is
	/// therefore that the following events are unavailable:
	/// - `ENTER_WINDOW`
	/// - `LEAVE_WINDOW`
	/// - `POINTER_MOTION_HINT`
	/// - `EXPOSURE`
	/// - `VISIBILITY_CHANGE`
	/// - `STRUCTURE_NOTIFY`
	/// - `SUBSTRUCTURE_NOTIFY`
	/// - `SUBSTRUCTURE_REDIRECT`
	/// - `FOCUS_CHANGE`
	/// - `PROPERTY_CHANGE`
	/// - `COLORMAP_CHANGE`
	/// - `OWNER_GRAB_BUTTON`
	#[derive(StaticByteSize, ByteSize)]
	pub struct DeviceEventMask: u32 {
		/// Key press events.
		const KEY_PRESS = 0x00000001;
		/// Key release events.
		const KEY_RELEASE = 0x00000002;
		/// Mouse button press events.
		const BUTTON_PRESS = 0x00000004;
		/// Mouse button release events.
		const BUTTON_RELEASE = 0x00000008;
		// removes ENTER_WINDOW and LEAVE_WINDOW
		/// Pointer motion events generated when the pointer's position changes.
		const POINTER_MOTION = 0x00000040;
		// removes POINTER_MOTION_HINT
		/// Pointer 'drag' events when the primary mouse button is held.
		///
		/// The primary mouse button is usually the one on the left, but many
		/// tools offer options to switch the positions of the primary and
		/// secondary mouse buttons.
		const BUTTON_1_MOTION = 0x00000100;
		/// Pointer 'drag' events when the middle mouse button is held.
		const BUTTON_2_MOTION = 0x00000200;
		/// Pointer 'drag' events when the secondary mouse button is held.
		///
		/// The secondary mouse button is usually the one on the right, but many
		/// tools offer options to switch the positions of the primary and
		/// secondary mouse buttons.
		const BUTTON_3_MOTION = 0x00000400;
		/// Pointer 'drag' events when 'mouse button 4' is held.
		///
		/// > Help wanted! If you know what 'mouse button 4' represents in terms
		/// > of its common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const BUTTON_4_MOTION = 0x00000800;
		/// Pointer 'drag' events when 'mouse button 5' is held.
		///
		/// > Help wanted! If you know what 'mouse button 5' represents in terms
		/// > of its common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const BUTTON_5_MOTION = 0x00001000;
		/// Pointer 'drag' events when any mouse button is held.
		const BUTTON_MOTION = 0x00002000;
		// removes all other events from this point on
	}

	/// A mask of the currently held modifier keys and mouse buttons.
	///
	/// This is the same as [`ModifierKeyMask`], but with masks for currently
	/// held mouse buttons.
	#[derive(StaticByteSize, ByteSize)]
	pub struct ModifierMask: u16 {
		/// Whether `Shift` is held.
		const SHIFT = 0x0001;
		/// Whether `Caps Lock` is active.
		const LOCK = 0x0002;
		/// Whether `Ctrl` is held.
		const CONTROL = 0x0004;
		/// Whether 'modifier key 1' is held.
		///
		/// > Help wanted! If you know what `MOD_1` represents in terms of its
		/// > common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const MOD_1 = 0x0008;
		/// Whether 'modifier key 2' is held.
		///
		/// > Help wanted! If you know what `MOD_2` represents in terms of its
		/// > common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const MOD_2 = 0x0010;
		/// Whether 'modifier key 3' is held.
		///
		/// > Help wanted! If you know what `MOD_3` represents in terms of its
		/// > common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const MOD_3 = 0x0020;
		/// Whether the `Super`/`Meta` key is held.
		///
		/// This key is commonly known as the 'windows key' on Windows devices,
		/// and as 'command' or 'cmd' on MacOS devices.
		const MOD_4 = 0x0040;
		/// Whether 'modifier key 5' is held.
		///
		/// > Help wanted! If you know what `MOD_5` represents in terms of its
		/// > common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
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
		///
		/// > Help wanted! If you know what `BUTTON_4` represents in terms of
		/// > its common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const BUTTON_4 = 0x0800;
		/// Whether 'mouse button 5' is held.
		///
		/// > Help wanted! If you know what `BUTTON_5` represents in terms of
		/// > its common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const BUTTON_5 = 0x1000;
	}

	/// A mask of currently held modifier keys.
	///
	/// This is the same as [`ButtonsModifierKeyMask`], but without mouse
	/// button masks. Unlike [`AnyModifierKeyMask`], this does not include a
	/// mask for [`ANY_MODIFIER`].
	///
	/// [`ANY_MODIFIER`]: AnyModifierKeyMask::ANY_MODIFIER
	#[derive(StaticByteSize, ByteSize)]
	pub struct ModifierKeyMask: u16 {
		/// Whether `Shift` is held.
		const SHIFT = 0x0001;
		/// Whether `Caps Lock` is active.
		const LOCK = 0x0002;
		/// Whether `Ctrl` is held.
		const CONTROL = 0x0004;
		/// Whether 'modifier key 1' is held.
		///
		/// > Help wanted! If you know what `MOD_1` represents in terms of its
		/// > common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const MOD_1 = 0x0008;
		/// Whether 'modifier key 2' is held.
		///
		/// > Help wanted! If you know what `MOD_2` represents in terms of its
		/// > common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const MOD_2 = 0x0010;
		/// Whether 'modifier key 3' is held.
		///
		/// > Help wanted! If you know what `MOD_3` represents in terms of its
		/// > common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const MOD_3 = 0x0020;
		/// Whether the `Super`/`Meta` key is held.
		///
		/// This key is commonly known as the 'windows key' on Windows devices,
		/// and as 'command' or 'cmd' on MacOS devices.
		const MOD_4 = 0x0040;
		/// Whether 'modifier key 5' is held.
		///
		/// > Help wanted! If you know what `MOD_5` represents in terms of its
		/// > common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const MOD_5 = 0x0080;
		// removes BUTTON_#
	}

	/// A mask of currently held modifier keys and a mask for [`ANY_MODIFIER`].
	///
	/// This is the same as [`ModifierKeyMask`], but with the addition of
	/// [`ANY_MODIFIER`].
	///
	/// [`ANY_MODIFIER`]: AnyModifierKeyMask::ANY_MODIFIER
	#[derive(StaticByteSize, ByteSize)]
	pub struct AnyModifierKeyMask: u16 {
		/// Whether `Shift` is held.
		const SHIFT = 0x0001;
		/// Whether `Caps Lock` is active.
		const LOCK = 0x0002;
		/// Whether `Ctrl` is held.
		const CONTROL = 0x0004;
		/// Whether 'modifier key 1' is held.
		///
		/// > Help wanted! If you know what `MOD_1` represents in terms of its
		/// > common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const MOD_1 = 0x0009;
		/// Whether 'modifier key 2' is held.
		///
		/// > Help wanted! If you know what `MOD_2` represents in terms of its
		/// > common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const MOD_2 = 0x0010;
		/// Whether 'modifier key 3' is held.
		///
		/// > Help wanted! If you know what `MOD_3` represents in terms of its
		/// > common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const MOD_3 = 0x0020;
		/// Whether the `Super`/`Meta` key is held.
		///
		/// This key is commonly known as the 'windows key' on Windows devices,
		/// and as 'command' or 'cmd' on MacOS devices.
		const MOD_4 = 0x0040;
		/// Whether 'modifier key 5' is held.
		///
		/// > Help wanted! If you know what `MOD_5` represents in terms of its
		/// > common name, please [let us know].
		///
		/// [let us know]: https://github.com/XdotRS/xrb/issues
		const MOD_5 = 0x0080;
		// removes BUTTON_#
		// adds ANY_MODIFIER
		/// Whether _any_ modifier key is held.
		const ANY_MODIFIER = 0x8000;
	}

	#[derive(StaticByteSize, ByteSize)]
	pub struct GraphicsContextMask: u32 {
		const FUNCTION = 0x00000001;
		const PLANE_MASK = 0x00000002;
		const FOREGROUND = 0x00000004;
		const BACKGROUND = 0x00000008;
		const LINE_WIDTH = 0x00000010;
		const LINE_STYLE = 0x00000020;
		const CAP_STYLE = 0x00000040;
		const JOIN_STYLE = 0x00000080;
		const FILL_STYLE = 0x00000100;
		const FILL_RULE = 0x00000200;
		const TILE = 0x00000400;
		const STIPPLE = 0x00000800;
		const TILE_STIPPLE_X_ORIGIN = 0x00001000;
		const TILE_STIPPLE_Y_ORIGIN = 0x00002000;
		const FONT = 0x00004000;
		const SUBWINDOW_MODE = 0x00008000;
		const GRAPHICS_EXPOSURE = 0x00010000;
		const CLIP_X_ORIGIN = 0x00020000;
		const CLIP_Y_ORIGIN = 0x00040000;
		const CLIP_MASK = 0x00080000;
		const DASH_OFFSET = 0x00100000;
		const DASHES = 0x00200000;
		const ARC_MODE = 0x00400000;
	}

	/// A mask of attributes given for a window.
	///
	/// The following table shows each attribute, its default value if it is
	/// not explicitly initialized in the [`CreateWindow`] request, and the
	/// [window classes] that it can be set with.
	///
	/// |Attribute           |Default value              |Class                          |
	/// |--------------------|---------------------------|-------------------------------|
	/// |[BackgroundPixmap]  |[`None`]                   |[`InputOutput`]                |
	/// |[BorderPixmap]      |[`Inherit::CopyFromParent`]|[`InputOutput`]                |
	/// |[BitGravity]        |[`BitGravity::Forget`]     |[`InputOutput`]                |
	/// |[WinGravity]        |[`WinGravity::NorthWest`]  |[`InputOutput`] & [`InputOnly`]|
	/// |[BackingStore]      |[`BackingStore::NotUseful`]|[`InputOutput`]                |
	/// |[BackingPlanes]     |`0xFFFFFFFF`               |[`InputOutput`]                |
	/// |[BackingPixel]      |`0`                        |[`InputOutput`]                |
	/// |[SaveUnder]         |`false`                    |[`InputOutput`]                |
	/// |[EventMask]         |[`EventMask::none()`]      |[`InputOutput`] & [`InputOnly`]|
	/// |[DoNotPropagateMask]|[`DeviceEventMask::none()`]|[`InputOutput`] & [`InputOnly`]|
	/// |[OverrideRedirect]  |`false`                    |[`InputOutput`] & [`InputOnly`]|
	/// |[Colormap]          |[`Inherit::CopyFromParent`]|[`InputOutput`]                |
	/// |[Cursor]            |[`None`]                   |[`InputOutput`] & [`InputOnly`]|
	///
	/// [`CreateWindow`]: crate::requests::CreateWindow
	/// [window classes]: crate::x11::requests::WindowClass
	/// [`InputOutput`]: crate::x11::requests::WindowClass::InputOutput
	/// [`InputOnly`]: crate::x11::requests::WindowClass::InputOnly
	/// [BackgroundPixmap]: crate::x11::requests::Attribute::BackgroundPixmap
	/// [BorderPixmap]: crate::x11::requests::Attribute::BorderPixmap
	/// [BitGravity]: crate::x11::requests::Attribute::BitGravity
	/// [WinGravity]: crate::x11::requests::Attribute::WinGravity
	/// [BackingStore]: crate::x11::requests::Attribute::BackingStore
	/// [BackingPlanes]: crate::x11::requests::Attribute::BackingPlanes
	/// [BackingPixel]: crate::x11::requests::Attribute::BackingPixel
	/// [SaveUnder]: crate::x11::requests::Attribute::SaveUnder
	/// [EventMask]: crate::x11::requests::Attribute::EventMask
	/// [DoNotPropagateMask]: crate::x11::requests::Attribute::DoNotPropagateMask
	/// [OverrideRedirect]: crate::x11::requests::Attribute::OverrideRedirect
	/// [Colormap]: crate::x11::requests::Attribute::Colormap
	/// [Cursor]: crate::x11::requests::Attribute::Cursor
	/// [`EventMask::none()`]: EventMask::none
	/// [`DeviceEventMask::none()`]: DeviceEventMask::none
	#[derive(StaticByteSize, ByteSize)]
	pub struct AttributeMask: u32 {
		/// See also: [`BackgroundPixmap`]
		///
		/// [`BackgroundPixmap`]: crate::x11::requests::Attribute::BackgroundPixmap
		const BACKGROUND_PIXMAP = 0x00000001;
		/// See also: [`BackgroundPixel`]
		///
		/// [`BackgroundPixel`]: crate::x11::requests::Attribute::BackgroundPixel
		const BACKGROUND_PIXEL = 0x00000002;
		/// See also: [`BorderPixmap`]
		///
		/// [`BorderPixmap`]: crate::x11::requests::Attribute::BorderPixmap
		const BORDER_PIXMAP = 0x00000004;
		/// See also: [`BorderPixel`]
		///
		/// [`BorderPixel`]: crate::x11::requests::Attribute::BorderPixel
		const BORDER_PIXEL = 0x00000008;
		/// See also: [`BitGravity`]
		///
		/// [`BitGravity`]: crate::x11::requests::Attribute::BitGravity
		const BIT_GRAVITY = 0x00000010;
		/// See also: [`WinGravity`]
		///
		/// [`WinGravity`]: crate::x11::requests::Attribute::WinGravity
		const WIN_GRAVITY = 0x00000020;
		/// See also: [`BackingStore`]
		///
		/// [`BackingStore`]: crate::x11::requests::Attribute::BackingStore
		const BACKING_STORE = 0x00000040;
		/// See also: [`BackingPlanes`]
		///
		/// [`BackingPlanes`]: crate::x11::requests::Attribute::BackingPlanes
		const BACKING_PLANES = 0x00000080;
		/// See also: [`BackingPixel`]
		///
		/// [`BackingPixel`]: crate::x11::requests::Attribute::BackingPixel
		const BACKING_PIXEL = 0x00000100;
		/// See also: [`OverrideRedirect`]
		///
		/// [`OverrideRedirect`]: crate::x11::requests::Attribute::OverrideRedirect
		const OVERRIDE_REDIRECT = 0x00000200;
		/// See also: [`SaveUnder`]
		///
		/// [`SaveUnder`]: crate::x11::requests::Attribute::SaveUnder
		const SAVE_UNDER = 0x00000400;
		/// See also: [`EventMask`]
		///
		/// [`Eventmask`]: crate::x11::requests::Attribute::EventMask
		const EVENT_MASK = 0x00000800;
		/// See also: [`DoNotPropagateMask`]
		///
		/// [`DoNotPropagateMask`]: crate::x11::requests::Attribute::DoNotPropagateMask
		const DO_NOT_PROPAGATE_MASK = 0x00001000;
		/// See also: [`Colormap`]
		///
		/// [`Colormap`]: crate::x11::requests::Attribute::Colormap
		const COLORMAP = 0x00002000;
		/// See also: [`Cursor`]
		///
		/// [`Cursor`]: crate::x11::requests::Attribute::Cursor
		const CURSOR = 0x00004000;
	}
}
