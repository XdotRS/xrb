// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod create_window;

pub use create_window::*;

use bitflags::bitflags;

use crate::x11::common::masks::*;
use crate::x11::common::values::*;
use crate::x11::wrappers::*;

use crate::rw::Serialize;

use crate::values;

use xrb_proc_macros::requests;

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

requests! {
	// #1: CreateWindow - waiting on algebraic length expressions
	// #2: ChangeWindowAttributes - waiting on algebraic length expressions

	/// Gets the window attributes associated with the `target` [`Window`].
	3: pub struct GetWindowAttributes<2> -> GetWindowAttributesReply {
		target: Window[4],
	}

	/// Destroys the `target` [`Window`].
	4: pub struct DestroyWindow<2> target: Window[4];
	5: pub struct DestroySubwindows<2> target: Window[4];

	// Just need the `Mode` enum for this one:
	// 6: pub struct ChangeSaveSet<2>(mode: Mode) window: Window[4];

	/// Switches a child window's parent window to another one. Often used for
	/// window decorations.
	7: pub struct ReparentWindow<4> {
		/// The target window.
		target: Window[4],
		/// The new parent window for the target window.
		parent: Window[4],
		/// The new x-coordinate of the child window relative to its new parent.
		x: i16[2],
		/// The new y-coordinate of the child window relative to its new parent.
		y: i16[2],
	}

	/// Maps the `target` [`Window`].
	///
	/// You can think of this as showing the target window, but it does not
	/// necessarily guarantee that it will be visible, if the window manager
	/// chooses to honor this request at all.
	8: pub struct MapWindow<2> target: Window[4];
	9: pub struct MapSubwindows<2> target: Window[4];
	/// Unmaps the `target` [`Window`].
	///
	/// You can think of this as hiding the target window.
	10: pub struct UnmapWindow<2> target: Window[4];
	11: pub struct UnmapSubwindows<2> window: Window[4];

	// 12: ConfigureWindow - waiting on algebraic length expressions

	// Just need the `Direction` enum for this one:
	// 13: pub struct CirculateWindow<2>(direction: Direction) window: Window[4];

	/// Gets the geometry of the `target` [`Drawable`], such as its dimensions and
	/// coordinates.
	14: pub struct GetGeometry<2> target: Drawable[4] -> GetGeometryReply;
	/// Queries the 'window tree' of the `target` [`Window`], meaning its parent
	/// and children.
	15: pub struct QueryTree<2> target: Window[4] -> QueryTreeReply;

	// 16: InternAtom - waiting on algebraic length expressions

	/// Gets the name of the given [`Atom`] ID.
	///
	/// For example, the name might be `WM_PROTOCOLS` or `_NET_WM_NAME`.
	17: pub struct GetAtomName<2> atom: Atom[4] -> GetAtomNameReply;

	// 18: ChangeProperty - waiting on algebraic length expressions

	/// Deletes the property specified by the given [`Atom`] on the `target_window`.
	19: pub struct DeleteProperty<3> {
		target_window: Window[4],
		property: Atom[4],
	}

	/// Gets the property specified by the given [`Atom`] on the `target_window`.
	20: pub struct GetProperty<6>(delete: bool) -> GetPropertyReply {
		target_window: Window[4],
		property: Atom[4],
		property_type: Specificity<Atom>[4],
		long_offset: u32[4],
		long_length: u32[4],
	}

	/// Lists the properties associated with the `target` [`Window`].
	21: pub struct ListProperties<2> target: Window[4] -> ListPropertiesReply;

	/// Sets the `owner` [`Window`] of the specified `selection`.
	22: pub struct SetSelectionOwner<4> {
		owner: Option<Window>[4],
		selection: Atom[4],
		/// The time this request was sent.
		time: Time[4],
	}

	/// Gets the owner of the specified `selection`.
	23: pub struct GetSelectionOwner<2> -> GetSelectionOwnerReply {
		selection: Atom[4],
	}

	24: pub struct ConvertSelection<6> {
		/// The [`Window`] requesting this conversion.
		requestor: Window[4],
		selection: Atom[4],
		target: Atom[4],
		property: Option<Atom>[4],
		/// The time this request was sent.
		time: Time[4],
	}

	// I'm assuming it's best for all requests to be supported by this macro,
	// considering how many are. In that case, we'll want some kinda syntax to
	// show fields referring to _structures_, like so:
	// ```
	// 25: pub struct SendEvent(propagate: bool)<11> {
	//     destination: Destination<Window>[4],
	//     event_mask: EventMask[4],
	//     event: Box<dyn Event>{32},
	// }
	// ```

	// This syntax is fully supported, just need some other types for these
	// fields.
	//
	// 26: pub struct GrabPointer<6>(owner_events: bool) -> GrabPointerReply {
	// 	grab_window: Window[4],
	// 	event_mask: PointerEventMask[2],
	// 	pointer_mode: GrabMode[1],
	// 	keyboard_mode: GrabMode[1],
	// 	confine_to: Option<Window>[4],
	// 	cursor: Option<Cursor>[4],
	// 	time: Time[4],
	// }

	/// Ceases a pointer grab.
	27: pub struct UngrabPointer<2> {
		/// The time this request was sent.
		time: Time[4],
	}

	// This syntax is fully supported, just need some other types for these
	// fields.
	//
	// 28: pub struct GrabButton<6>(owner_events: bool) {
	// 	grab_window: Windo[4],
	// 	event_mask: PointerEventMask[2],
	// 	pointer_mode: GrabMode[1],
	// 	keyboard_mode: GrabMode[1],
	// 	confine_to: Option<Window>[4],
	// 	cursor: Option<Cursor>[4],
	// 	button: Specificity<Button>[1],
	// 	?[1],
	// 	key_mask: KeyMask[2],
	// }

	29: pub struct UngrabButton<3>(button: Specificity<Button>) {
		grab_window: Window[4],
		modifiers: Window[2],
		?[2], // 2 unused bytes
	}

	30: pub struct ChangeActivePointerGrab<4> {
		cursor: Option<Cursor>[4],
		time: Time[4],
		event_mask: PointerEventMask[2],
		?[2], // 2 unused bytes
	}

	// 31: pub struct GrabKeyboard<4>(owner_events: bool) -> GrabKeyboardReply {
	// 	grab_window: Window[4],
	// 	pointer_mode: GrabMode[1],
	// 	keyboard_mode: GrabMode[1],
	// 	?[2],
	// }

	/// Ceases a keyboard grab.
	32: pub struct UngrabKeyboard<2> {
		/// The time this request was sent.
		time: Time[4],
	}

	// 33: pub struct GrabKey<4>(owner_events: bool) {
	//     grab_window: Window[4],
	//     modifiers: KeyMask[2],
	//     key: Specificity<Keycode>[1],
	//     pointer_mode: GrabMode[1],
	//     keyboard_mode: GrabMode[1],
	//     ?[3],
	// }

	34: pub struct UngrabKey<3>(key: Specificity<Keycode>) {
		grab_window: Window[4],
		modifiers: KeyMask[2],
		?[2], // 2 unused bytes
	}

	// 35: pub struct AllowEvents<2>(mode: AllowEventsMode) time: Time[4];

	/// Grabs the server, preventing the server from processing until the grab
	/// ceases.
	///
	/// See also: [UngrabServer]
	36: pub struct GrabServer;
	/// Ceases a server grab.
	///
	/// See also: [GrabServer]
	37: pub struct UngrabServer;

	38: pub struct QueryPointer<2> target_window: Window[4] -> QueryPointerReply;

	/// Gets pointer motion events generated on the `target_window` between the
	/// `start` and `end` time.
	39: pub struct GetMotionEvents<4> -> GetMotionEventsReply {
		target_window: Window[4],
		start: Time[4],
		stop: Time[4],
	}

	/// Translate coordinates from one window's coordinate space to another.
	///
	/// Coordinates within a window are relative to itself, including the
	/// coordinates of child windows. That means that a window's coordinates
	/// are relative to its parent. This request translates coordinates in the
	/// relative coordinate space of a source window to the relative coordinate
	/// space of the destination window. This will often be used to translate a
	/// window's coordinates to coordinates relative to the root window.
	40: pub struct TranslateCoordinates<4> -> TranslateCoordinatesReply {
		/// The source window, particularly its coordinate space.
		source_window: Window[4],
		/// The destination window, particularly its coordinate space.
		destination_window: Window[4],
		/// The x-coordinate in the source window's coordinate space that is to
		/// be translated.
		source_x: i16[2],
		/// The y-coordinate in the source window's coordinate space that is to
		/// be translated.
		source_y: i16[2],
	}

	41: pub struct WarpPointer<6> {
		source_window: Option<Window>[4],
		destination_window: Option<Window>[4],
		source_x: i16[2],
		source_y: i16[2],
		source_width: u16[2],
		source_height: u16[2],
		destination_x: i16[2],
		destination_y: i16[2],
	}

	// 42: pub struct SetInputFocus<3>(revert_to: RevertTo) {
	//     focus: Option<Focus<Window>>[4],
	//     time: Time[4],
	// }

	/// Query the current input focus.
	43: pub struct GetInputFocus -> GetInputFocusReply;
	/// Query the current keymap (the mapping of [`Keycode`]s to [`Keysym`]s).
	44: pub struct QueryKeymap -> QueryKeymapReply;

	// 45: OpenFont - waiting on algebraic length expressions

	46: pub struct CloseFont<2> font: Font[4];
	47: pub struct QueryFont<2> font: Fontable[4] -> QueryFontReply;

	// 48: QueryTextExtents - waiting on algebraic length expressions
	// 49: ListFonts - waiting on algebraic length expressions
	// 50: ListFontsWithInfo - waiting on algebraic length expressions
	// 51: SetFontPath - waiting on algebraic length expressions

	52: pub struct GetFontPath -> GetFontPathReply;

	/// Creates a new [`Pixmap`] with the given `pixmap_id`, `width`, and `height` from the given
	/// `drawable`.
	53: pub struct CreatePixmap<4>(depth: u8) {
		pixmap_id: Pixmap[4],
		drawable: Drawable[4],
		width: u16[2],
		height: u16[2],
	}

	54: pub struct FreePixmap<2> pixmap: Pixmap[4];

	// 55: CreateGcontext - waiting on algebraic length expressions
	// 56: ChangeGcontext - waiting on algebraic length expressions

	// 57: pub struct CopyGcontext<4> {
	// 	source_gcontext: Gcontext[4],
	// 	destination_gcontext: Gcontext[4],
	// 	value_mask: GcontextValueMask[4],
	// }

	// 58: SetDashes - waiting on algebraic length expressions
	// 59: SetClipRectangles - waiting on algebraic length expressions

	60: pub struct FreeGcontext<2> gcontext: Gcontext[4];

	61: pub struct ClearArea<4>(exposures: bool) {
		window: Window[4],
		x: i16[2],
		y: i16[2],
		width: u16[2],
		height: u16[2],
	}

	62: pub struct CopyArea<7> {
		source: Drawable[4],
		destination: Drawable[4],
		gcontext: Gcontext[4],
		source_x: i16[2],
		source_y: i16[2],
		destination_x: i16[2],
		destination_y: i16[2],
		width: u16[2],
		height: u16[2],
	}

	63: pub struct CopyPlane<8> {
		source: Drawable[4],
		destination: Drawable[4],
		gcontext: Gcontext[4],
		source_x: i16[2],
		source_y: i16[2],
		destination_x: i16[2],
		destination_y: i16[2],
		width: u16[2],
		height: u16[2],
		bit_plane: u32[4],
	}

	// 64: PolyPoint - waiting on algebraic length expressions
	// 65: PolyLine - waiting on algebraic length expressions
	// 66: PolySegment - waiting on algebraic length expressions
	// 67: PolyRectangle - waiting on algebraic length expressions
	// 68: PolyArc - waiting on algebraic length expressions
	// 69: FillPoly - waiting on algebraic length expressions
	// 70: PolyFillRectangle - waiting on algebraic length expressions
	// 71: PolyFillArc - waiting on algebraic length expressions
	// 72: PutImage - waiting on algebraic length expressions

	// 73: pub struct GetImage<5>(format: ImageFormat) -> GetImageReply {
	//     drawable: Drawable[4],
	//     x: i16[2],
	//     y: i16[2],
	//     width: u16[2],
	//     height: u16[2],
	//     plane_mask: u32[4],
	// }

	// 74: PolyText8 - waiting on algebraic length expressions
	// 75: PolyText16 - waiting on algebraic length expressions
	// 76: ImageText8 - waiting on algebraic length expressions
	// 77: ImageText16 - waiting on algebraic length expressions

	// 78: pub struct CreateColormap<4>(alloc: Allocation) {
	// 	colormap_id: Colormap[4],
	// 	owindow: Window[4],
	// 	visual: VisualId[4],
	// }

	79: pub struct FreeColormap<2> target: Colormap[4];

	80: pub struct CopyColormapAndFree<3> {
		colormap_id: Colormap[4],
		source_colormap: Colormap[4],
	}

	81: pub struct InstallColormap<2> colormap: Colormap[4];
	82: pub struct UninstallColormap<2> colormap: Colormap[4];
	83: pub struct ListInstalledColormaps<2> target_window: Window[4] -> ListInstalledColormapsReply;

	// 84: AllocColor - waiting on structure field syntax
	// 85: AllocNamedColor - waiting on algebraic length expressions

	86: pub struct AllocColorCells<3>(contiguous: bool) -> AllocColorCellsReply {
		target_colormap: Colormap[4],
		colors: u16[2],
		planes: u16[2],
	}

	// 87: AllocColorPlanes - waiting on structure field syntax
	// 88: FreeColors - waiting on algebraic length expressions
	// 89: StoreColors - waiting on algebraic length expressions
	// 90: StoroeNamedColor - waiting on algebraic length expressions
	// 91: QueryColors - waiting on algebraic length expressions
	// 92: LookupColor - waiting on algebraic length expressions
	// 93: CreateCursor - waiting on algebraic length expressions
	// 94: CreateGlyphCursor - waiting on structure field syntax

	95: pub struct FreeCursor<2> target: Cursor[4];

	// 96: RecolorCursor - waiting on structure field syntax

	// 97: pub struct QueryBestSize<3>(class: BestSizeClass) -> QueryBestSizeReply {
	//     drawable: Drawable[4],
	//     width: u16[2],
	//     height: u16[2],
	// }

	// 98: QueryExtension - waiting on algebraic length expressions

	/// Lists the current extensions.
	99: pub struct ListExtensions -> ListExtensionsReply;

	// 100: ChangeKeyboardMapping - waiting on algebraic length expressions

	101: pub struct GetKeyboardMapping<2> -> GetKeyboardMappingReply {
		first_keycode: Keycode[1],
		count: u8[1],
		?[2], // 2 unused bytes
	}

	// 102: ChangeKeyboardControl - waiting on algebraic length expressions

	103: pub struct GetKeyboardControl -> GetKeyboardControlReply;
	104: pub struct Bell(percent: u8);

	105: pub struct ChangePointerControl<3> {
		acceleration_numerator: i16[2],
		acceleration_denominator: i16[2],
		threshold: i16[2],
		accelerate: bool,
		enforce_threshold: bool,
	}

	106: pub struct GetPointerControl -> GetPointerControlReply;

	// 107: pub struct SetScreenSaver<3> {
	//     timeout: i16[2],
	//     interval: i16[2],
	//     prefer_blanking: Default<bool>,
	//     allow_exposure: Default<bool>,
	//     ?[2], // unused bytes
	// }

	108: pub struct GetScreenSaver -> GetScreenSaverReply;

	// 109: ChangeHosts - waiting on algebraic length expressions

	110: pub struct ListHosts -> ListHostsReply;
	111: pub struct SetAccessControl(enabled: bool);

	// 112: pub struct SetCloseDownMode(mode: CloseDownMode);

	// 113: pub struct KillClient<2> resource: AllTemporary<u32>;

	// 114: RotateProperties - waiting on algebraic length expressions

	// 115: pub struct ForceScreenSaver(mode: enum ForceScreenSaverMode {
	//     Reset = 0,
	//     Activate = 1,
	// });

	// 116: SetPointerMapping - waiting on algebraic length expressions

	117: pub struct GetPointerMapping -> GetPointerMappingReply;

	// 118: SetModifierMapping - waiting on algebraic length expressions

	119: pub struct GetModifierMapping -> GetModifierMappingReply;

	// 127: NoOperation - waiting on algebraic length expressions
}

// These are temporary until the reply macro syntax is complete and these can
// actually be defined. Most of these also need the algebraic length expressions
// and structure field syntax too.
struct GetWindowAttributesReply;
struct GetGeometryReply;
struct QueryTreeReply;
struct GetAtomNameReply;
struct GetPropertyReply;
struct ListPropertiesReply;
struct GetSelectionOwnerReply;
struct QueryPointerReply;
struct GetMotionEventsReply;
struct TranslateCoordinatesReply;
struct GetInputFocusReply;
struct QueryKeymapReply;
struct QueryFontReply;
struct GetFontPathReply;
struct ListInstalledColormapsReply;
struct AllocColorCellsReply;
struct ListExtensionsReply;
struct GetKeyboardMappingReply;
struct GetKeyboardControlReply;
struct GetPointerControlReply;
struct GetScreenSaverReply;
struct ListHostsReply;
struct GetPointerMappingReply;
struct GetModifierMappingReply;

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
