// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod create_window;

pub use create_window::*;

use bitflags::bitflags;

use crate::x11::common::masks::{DeviceEventMask, EventMask};
use crate::x11::common::values::*;

use crate::x11::wrappers::*;

use crate::rw::Serialize;

use crate::requests;
use crate::values;

use xrb_proc_macros::request;

/// A request is a message sent from an X client to the X server.
///
/// Since an X client will never receive an actual request message,
/// deserialization is not implemented for requests for the sake of simplicity.
pub trait Request<REPLY = ()>: Serialize {
	/// The major opcode that uniquely identifies this request or extension.
	///
	/// X core protocol requests have unique major opcodes, but each extension
	/// is only assigned one major opcode. Extensions are assigned major opcodes
	/// from 127 through to 255.
	fn opcode() -> u8;

	/// The minor opcode that uniquely identifies this request within its
	/// extension.
	///
	/// As each extension is only assigned one major opcode, the minor opcode
	/// can be used to distinguish different requests contained within an
	/// extension.
	///
	/// [`None`] means that either this request is not from an extension, or the
	/// extension does not make use of the minor opcode, likely because it only
	/// has one request.
	///
	/// [`Some`] means that there is indeed a minor opcode associated with this
	/// request. This request is therefore from an extension.
	fn minor_opcode() -> Option<u8>;

	/// The length of this request, including the header, in 4-byte units.
	///
	/// Every request contains a header whcih is 4 bytes long. This header is
	/// included in the length, so the minimum length is 1 unit (4 bytes). The
	/// length represents the _exact_ length of the request: padding bytes may
	/// need to be added to the end of the request to ensure its length is
	/// brought up to a multiple of 4, if it is not already.
	fn length(&self) -> u16;
}

request! {
	4! pub struct DesroyWindow {
		window: Window[4],
	}
}

requests! {
	// CreateWindow(1): mod create_window;
	// ChangeWindowAttributes(2): mod change_window_attributes;
	// GetWindowAttributes(3): mod get_window_attributes;

	pub struct DestroyWindow(4);
	pub struct DestroySubwindows(5);

	// pub struct ChangeSaveSet(6)[2] {
	//	pub mode: Mode,
	//	pub window(4): Window,
	// }

	pub struct ReparentWindow(7)[4] {
		pub window(4): Window,
		pub parent(4): Window,
		pub x(2): i16,
		pub y(2): i16,
	}

	pub struct MapWindow(8);
	pub struct MapSubwindows(9);
	pub struct UnmapWindow(10);
	pub struct UnmapSubwindows(11);

	// ConfigureWindow(12): mod configure_window;

	// pub struct CirculateWindow(13)[2] {
	//	pub direction: Direction,
	//	pub window(4): Window,
	// }

	// GetGeometry(14): mod get_geometry;
	// QueryTree(15): mod query_tree;
	// InternAtom(16): mod intern_atom;
	// GetAtomName(17): mod get_atom_name;
	// ChangeProperty(18): mod change_property;

	pub struct DeleteProperty(19)[3] {
		pub window(4): Window,
		pub property(4): Atom,
	}

	// GetProperty(20): mod get_property;
	// ListProperties(21): mod list_properties;

	// GetSelectionOwner(22): mod get_selection_owner;
	// GetSelectionOwner(23): mod get_selection_owner;

	pub struct ConvertSelection(24)[6] {
		pub requestor(4): Window,
		pub selection(4): Atom,
		pub target(4): Atom,
		pub property(4): Option<Atom>,
		pub time(4): Time,
	}

	// SendEvent(25): mod send_event;
	// GrabPointer(26): mod grab_pointer;

	pub struct UngrabPointer(27)[2] {
		pub time(4): Time,
	}

	// GrabButton(28): mod grab_button;
	// UngrabButton(29): mod ungrab_button;
	// ChangeActivePointerGrab(30): mod change_active_pointer_grab;
	// GrabKeyboard(31): mod grab_keyboard;

	pub struct UngrabKeyboard(32)[2] {
		pub time(4): Time,
	}

	// GrabKey(33): mod grab_key;
	// UngrabKey(34): mod ungrab_key;
	// AllowEvents(35): mod allow_events;

	pub struct GrabSever(36)[1] {}
	pub struct UngrabServer(37)[1] {}

	// QueryPointer(38): mod query_pointer;
	// GetMotionEvents(39): mod get_motion_events;
	// TranslateCoordinates(40): mod translate_coordinates;

	pub struct WarpPointer(41)[6] {
		pub source(4): Option<Window>,
		pub destination(4): Option<Window>,
		pub source_x(2): i16,
		pub source_y(2): i16,
		pub source_width(2): u16,
		pub source_height(2): u16,
		pub destination_x(2): i16,
		pub destination_y(2): i16,
	}

	// SetInputFocus(42): mod set_input_focus;
	// GetInputFocus(43): mod get_input_focus;
	// QueryKeymap(44): mod query_keymap;
	// OpenFont(45): mod open_font;

	pub struct CloseFont(46)[2] {
		pub font(4): Font,
	}

	// QueryFont(47): mod query_font;
	// QueryTextExtents(48): mod query_text_extents;
	// ListFonts(49): mod list_fonts;
	// ListFontsWithInfo(50): mod list_fonts_with_info;
	// SetFontPath(51): mod set_font_path;
	// GetFontPath(52): mod get_font_path;

	pub struct CreatePixmap(53)[4] {
		pub pixmap_id(4): Pixmap,
		pub drawable(4): Drawable,
		pub width(2): u16,
		pub height(2): u16,
	}

	pub struct FreePixmap(54)[2] {
		pub pixmap(4): Pixmap,
	}

	// CreateGcontext(55): mod create_gcontext;
	// ChangeGcontext(56): mod change_gcontext;
	// CopyGcontext(57): mod  copy_gcontext;
	// SetDashes(58): mod set_dashes;
	// SetClipRectangles(59): mod set_clip_rectangles;

	pub struct FreeGcontext(60)[2] {
		pub gcontext(4): Gcontext,
	}

	pub struct ClearArea(61)[4] {
		pub exposures: bool,
		pub window(4): Window,
		pub x(2): i16,
		pub y(2): i16,
		pub width(2): u16,
		pub height(2): u16,
	}

	pub struct CopyArea(62)[7] {
		pub source(4): Drawable,
		pub destination(4): Drawable,
		pub gcontext(4): Gcontext,
		pub source_x(2): i16,
		pub source_y(2): i16,
		pub destination_x(2): i16,
		pub desination_y(2): i16,
		pub width(2): u16,
		pub height(2): u16,
	}

	pub struct CopyPlane(63)[8] {
		pub source(4): Drawable,
		pub destination(4): Drawable,
		pub gcontext(4): Gcontext,
		pub source_x(2): i16,
		pub source_y(2): i16,
		pub destination_x(2): i16,
		pub destination_y(2): i16,
		pub width(2): u16,
		pub height(2): u16,
		pub bit_plane(4): u32,
	}

	// PolyPoint(64): mod poly_point;
	// PolyLine(65): mod poly_line;
	// PolySegment(66): mod poly_segment;
	// PolyRectangle(67): mod poly_rectangle;
	// PolyArc(68): mod poly_arc;
	// FillPoly(69): mod fill_poly;
	// PolyFillRectangle(70): mod poly_fill_rectangle;
	// PolyFillArc(71): mod poly_fill_arc;
	// PutImage(72): mod put_image;
	// GetImage(73): mod get_image;
	// PolyText8(74): mod poly_text_8;
	// PolyText16(75): mod poly_text_16;
	// ImageText8(76): mod image_text_8;
	// ImageText16(77): mod image_text_16;

	// pub struct CreateColormap(78)[4] {
	//	pub alloc: Alloc,
	//	pub colormap_id(4): Colormap,
	//	pub window(4): Window,
	//	pub visual(4): VisualId,
	// }

	pub struct FreeColormap(79)[2] {
		pub colormap(4): Colormap,
	}

	pub struct CopyColormapAndFree(80)[3] {
		pub colormap_id(4): Colormap,
		pub source(4): Colormap,
	}

	pub struct InstallColormap(81)[2] {
		pub colormap(4): Colormap,
	}

	pub struct UninstallColormap(82)[2] {
		pub colormap(4): Colormap,
	}

	// ListInstalledColormaps(83): mod list_installed_colormaps;
	// AllocColor(84): mod alloc_color;
	// AllocNamedColor(85): mod alloc_named_color;
	// AllocColorCells(86): mod alloc_color_cells;
	// AllocColorPlanes(87): mod alloc_color_planes;
	// FreeColors(88): mod free_colors;
	// StoreColors(89): mod store_colors;
	// StoreNamedColor(90): mod store_named_color;
	// QueryColors(91): mod query_colors;
	// LookupColor(92): mod lookup_color;

	pub struct CreateCursor(93)[8] {
		pub cursor_id(4): Cursor,
		pub source(4): Pixmap,
		pub mask(4): Option<Pixmap>,
		pub foreground_red(2): u16,
		pub foreground_green(2): u16,
		pub foreground_blue(2): u16,
		pub background_red(2): u16,
		pub background_green(2): u16,
		pub background_blue(2): u16,
		pub x(2): u16,
		pub y(2): u16,
	}

	pub struct CreateGlyphCursor(94)[8] {
		pub cursor_id(4): Cursor,
		pub source_font(4): Font,
		pub mask_font(4): Option<Font>,
		pub source_char(2): Char2b,
		pub mask_char(2): Char2b,
		pub foreground_red(2): u16,
		pub foreground_green(2): u16,
		pub foreground_blue(2): u16,
		pub background_red(2): u16,
		pub background_green(2): u16,
		pub background_blue(2): u16,
	}

	pub struct FreeCursor(95)[2] {
		pub cursor(4): Cursor,
	}

	pub struct RecolorCursor(96)[5] {
		pub cursor(4): Cursor,
		pub foreground_red(2): u16,
		pub foreground_green(2): u16,
		pub foreground_blue(2): u16,
		pub background_red(2): u16,
		pub background_green(2): u16,
		pub background_blue(2): u16,
	}

	// QueryBestSize(97): mod query_best_size;
	// QueryExtension(98): mod query_extension;
	// ListExtensions(99): mod list_extensions;
	// ChangeKeyboardMapping(100): mod change_keyboard_mapping;
	// GetKeyboardMapping(101): mod get_keyboard_mapping;
	// ChangeKeyboardControl(102): mod change_keyboard_control;
	// GetKeyboardControl(103): mod get_keyboard_control;

	pub struct Bell(104)[1] {
		pub percent: i8,
	}

	pub struct ChangePointerControl(105)[3] {
		pub acceleration_numerator(2): i16,
		pub acceleration_denominator(2): i16,
		pub threshold(2): i16,
		pub accelerate(1): bool,
		pub enable_threshold(1): bool,
	}

	// GetPointerControl(106): mod get_pointer_control;
	// SetScreenSaver(107): mod set_screen_saver;
	// GetScreenSaver(108): mod get_screen_saver;
	// ChangeHosts(109): mod change_hosts;
	// ListHosts(110): mod list_hosts;

	pub struct SetAccessControl(111)[1] {
		pub enable_access_control: bool,
	}

	// SetCloseDownMode(112): mod set_close_down_mode;
	// KillClient(113): mod kill_client;
	// RotateProperties(114): mod rotate_properties;
	// ForceScreenSaver(115): mod force_screen_saver;
	// SetPointerMapping(116): mod set_pointer_mapping;
	// GetPointerMapping(117): mod get_pointer_mapping;
	// SetModifierMapping(118): mod set_modifier_mapping;
	// GetModifierMapping(119): mod  get_modifier_mapping;

	// NoOperation(127): mod no_operation;
}

values! {
	/// Window attributes that can be configured in various requests.
	///
	/// Attributes given in `values` vectors MUST be in the order given in this
	/// enum, so that they match the order of the [`WinAttrMask`].
	pub enum WinAttr<WinAttrMask> {
		BackgroundPixmap(Option<Relative<Pixmap>>): BACKGROUND_PIXMAP,
		BackgroundPixel(u32): BACKGROUND_PIXEL,
		BorderPixmap(Inherit<Pixmap>): BORDER_PIXMAP,
		BorderPixel(u32): BORDER_PIXEL,
		BitGravity(BitGravity): BIT_GRAVITY,
		WinGravity(WinGravity): WIN_GRAVITY,
		BackingStore(BackingStore): BACKING_STORE,
		BackingPlanes(u32): BACKING_PLANES,
		BackingPixel(u32): BACKING_PIXEL,
		OverrideRedirect(bool): OVERRIDE_REDIRECT,
		SaveUnder(bool): SAVE_UNDER,
		EventMask(EventMask): EVENT_MASK,
		DoNotPropagateMask(DeviceEventMask): DO_NOT_PROPAGATE_MASK,
		Colormap(Inherit<Colormap>): COLORMAP,
		Cursor(Option<Cursor>): CURSOR,
	}
}

bitflags! {
	/// A mask of [window attributes] that can be used in various requests.
	///
	/// [window attributes]:WinAttr
	pub struct WinAttrMask: u32 {
		/// The [`BackgroundPixmap`] [attribute](WinAttr).
		///
		/// [`BackgroundPixmap`]:WinAttr::BackgroundPixmap
		const BACKGROUND_PIXMAP = 0x_0000_0001;
		/// The [`BackgroundPixel] [CreateWindow] request [value](WinAttr).
		///
		/// [`BackgroundPixel`]:WinAttr::BackgroundPixel
		const BACKGROUND_PIXEL = 0x_0000_0002;
		/// The [`BorderPixmap`] [attribute](WinAttr).
		///
		/// [`BorderPixmap`]:WinAttr:BorderPixmap
		const BORDER_PIXMAP = 0x_0000_0004;
		/// The [`BorderPixel`] [attribute](WinAttr).
		///
		/// [`BorderPixel`]:WinAttr::BorderPixel
		const BORDER_PIXEL = 0x_0000_0008;
		/// The [`BitGravity`] [attribute](WinAttr).
		///
		/// [`BitGravity`]:WinAttr::BitGravity
		const BIT_GRAVITY = 0x_0000_0010;
		/// The [`WinGravity`] [attribute](WinAttr).
		///
		/// [`WinGravity`]:WinAttr::WinGravity
		const WIN_GRAVITY = 0x_0000_0020;
		/// The [`BackingStore`] [attribute](WinAttr).
		///
		/// [`BackingStore`]:WinAttr::BackingStore
		const BACKING_STORE = 0x_0000_0040;
		/// The [`BackingPlanes`] [attribute](WinAttr).
		///
		/// [`BackingPlanes`]:WinAttr::BackingPlanes
		const BACKING_PLANES = 0x_0000_0080;
		/// The [`BackingPixel`] [attribute](WinAttr).
		///
		/// [`BackingPixel`]:WinAttr::BackingPixel
		const BACKING_PIXEL = 0x_0000_0100;
		/// The [`OverrideRedirect`] [attribute](WinAttr).
		///
		/// [`OverrideRedirect`]:WinAttr::OverrideRedirect
		const OVERRIDE_REDIRECT = 0x_0000_0200;
		/// The [`SaveUnder`] [attribute](WinAttr).
		///
		/// [`SaveUnder`]:WinAttr::SaveUnder
		const SAVE_UNDER = 0x_0000_0400;
		/// The [`EventMask`] [attribute](WinAttr).
		///
		/// [`EventMask`]:WinAttr::EventMask
		const EVENT_MASK = 0x_0000_0800;
		/// The [`DoNotPropagateMask`] [attribute](WinAttr).
		///
		/// [`DoNotPropagateMask`]:WinAttr::DoNotPropagateMask
		const DO_NOT_PROPAGATE_MASK = 0x_0000_1000;
		/// The [`Colormap`] [attribute](WinAttr).
		///
		/// [`Colormap`]:WinAttr::Colormap
		const COLORMAP = 0x_0000_2000;
		/// The [`Cursor`] [attribute](WinAttr).
		///
		/// [`Cursor`]:WinAttr::Cursor
		const CURSOR = 0x_0000_4000;
	}
}
