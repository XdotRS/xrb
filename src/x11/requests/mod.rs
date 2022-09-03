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

requests! {
	pub struct CreateWindow(1) {
		pub $depth: u8,
		pub window_id: Window,
		pub parent: Window,
		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,
		pub border_width: u16,
		pub class: Inherit<WindowClass>,
		pub visual: Inherit<VisualId>,
		pub value_mask: WinAttrMask,
		pub values: [WinAttr],
	}

	pub struct ChangeWindowAttributes(2) {
		pub target: Window,
		pub value_mask: WinAttrMask,
		pub values: [WinAttr],
	}

	pub struct GetWindowAttributes(3) -> GetWindowAttributesReply {
		pub target: Window,
	}

	pub struct GetWindowAttributesReply for GetWindowAttributes {
		pub $backing_store: BackingStore,
		pub visual: VisualId,
		pub class: WindowClass,
		pub bit_gravity: BitGravity,
		pub win_gravity: WinGravity,
		pub backing_planes: u32,
		pub backing_pixel: u32,
		pub save_under: bool,
		pub map_is_installed: bool,
		pub map_state: enum MapState {
			Unmapped = 0,
			Unviewable = 1,
			Viewable = 2,
		},
		pub override_redirect: bool,
		pub colormap: Option<Colormap>,
		pub all_event_masks: EventMask,
		pub your_event_mask: EventMask,
		pub do_not_propagate_mask: DeviceEventMask,
		()[2],
	}

	pub struct DestroyWindow(4): pub target: Window;
	pub struct DestroySubwindows(5): pub target: Window;

	pub struct ChangeSaveSet(6) {
		pub $mode: enum ChangeSaveSetMode {
			Insert = 0,
			Delete = 1,
		},
		pub target: Window,
	}

	pub struct ReparentWindow(7) {
		pub target: Window,
		pub new_parent: Window,
		pub new_x: i16,
		pub new_y: i16,
	}

	pub struct MapWindow(8): pub target: Window;
	pub struct MapSubwindows(9): pub target: Window;

	pub struct UnmapWindow(10): pub target: Window;
	pub struct UnmapSubwindows(11): pub target: Window;

	pub struct ConfigureWindow(12) {
		pub target: Window,
		pub value_mask: ConfigureWindowMask,
		pub values: &[ConfigureWindowValue],
	}

	pub struct CirculateWindow(13) {
		pub $direction: enum CirculateDirection {
			RaiseLowest = 0,
			RaiseHighest = 1,
		},
		pub target: Window,
	}

	pub struct GetGeometry(14) -> GetGeometryReply: pub target: Drawable;

	pub struct GetGeometryReply for GetGeometry {
		pub $depth: u8,
		pub root: Window,
		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,
		pub border_width: u16,
		()[10],
	}

	pub struct QueryTree(15) -> QueryTreeReply: pub target: Window;

	pub struct QueryTreeReply for QueryTree {
		pub root: Window,
		pub parent: Option<Window>,
		#children,
		()[14],
		pub children: [Window],
	}

	pub struct InternAtom(16) -> InternAtomReply {
		pub $only_if_exists: bool,
		#name,
		()[2],
		pub name: String,
		()[padding(name)],
	}

	pub struct InternAtomReply for InternAtom {
		pub atom: Option<Atom>,
		()[20],
	}

	pub struct GetAtomName(17) -> GetAtomNameReply: pub atom: Atom;

	pub struct GetAtomNameReply for GetAtomName {
		#name,
		()[22],
		pub name: String,
		()[padding(name)],
	}

	// The property requests (`ChangeProperty(18)`, `DeleteProperty(19)`,
	// `GetProperty(20)`, and `ListProperties(21)`) are special cases and need
	// to be defined manually. You can find them in `mod properties;`.

	pub struct SetSelectionOwner(22) {
		pub $owner: Option<Window>,
		pub selection: Atom,
		pub time: Time,
	}

	pub struct GetSelectionOwner(23) -> GetSelectionOwnerReply: pub selection: Atom;

	pub struct GetSelectionOwnerReply for GetSelectionOwner {
		pub owner: Option<Window>,
		()[20],
	}

	pub struct ConvertSelection(24) {
		pub requestor: Window,
		pub selection: Atom,
		pub target: Atom,
		pub property: Option<Atom>,
		pub time: Time,
	}

	pub struct SendEvent(25) {
		pub $propagate: bool,
		pub destination: Destination,
		pub event_mask: EventMask,
		pub event: Box<dyn Event>,
	}

	pub struct GrabPointer(26) -> GrabPointerReply {
		pub $owner_events: bool,
		pub target_window: Window,
		pub event_mask: PointerEventMask,
		pub pointer_mode: enum GrabMode {
			Synchronous = 0,
			Asynchronous = 1,
		},
		pub keyboard_mode: GrabMode,
		pub confine_to: Option<window>,
		pub cursor_override: Option<Cursor>,
		pub time: Time,
	}

	pub struct GrabPointerReply for GrabPointer {
		pub $status: enum GrabStatus {
			Success = 0,
			AlreadyGrabbed = 1,
			InvalidTime = 2,
			NotViewable = 3,
			Frozen = 4,
		},
		()[24],
	}

	pub struct UngrabPointer(27): pub time: Time;

	pub struct GrabButton(28) {
		pub $owner_events: bool,
		pub target_window: Window,
		pub event_mask: PointerEventMask,
		pub pointer_mode: GrabMode,
		pub keyboard_mode: GrabMode,
		pub confine_to: Option<Window>,
		pub cursor_override: Option<Cursor>,
		pub button: Specificity<Button>,
		()[1],
		pub modifiers: ModifierKeyMask,
	}

	pub struct UngrabButton(29) {
		pub $button: Specificity<Button>,
		pub target_window: Window,
		pub modifiers: ModifierKeyMask,
		()[2],
	}

	pub struct ChangeActivePointerGrab(30) {
		pub cursor_override: Option<Cursor>,
		pub time: Time,
		pub event_mask: PointerEventMask,
		()[2],
	}

	pub struct GrabKeyboard(31) -> GrabPointerReply {
		pub $owner_events: bool,
		pub target_window: Window,
		pub time: Time,
		pub pointer_mode: GrabMode,
		pub keyboard_mode: GrabMode,
		()[2],
	}

	pub struct GrabKeyboardReply for GrabKeyboard {
		pub $status: GrabStatus,
		()[24],
	}

	pub struct UngrabKeyboard(32): pub time: Time;

	pub struct GrabKey(33) {
		pub $owner_events: bool,
		pub target_window: Window,
		pub modifiers: ModifierKeyMask,
		pub key: Specificity<Keycode>,
		pub pointer_mode: GrabMode,
		pub keyboard_mode: GrabMode,
		()[3],
	}

	pub struct UngrabKey(34) {
		pub $key: Specificity<Keycode>,
		pub target_window: Window,
		pub modifiers: ModifierKeyMask,
		()[2],
	}

	pub struct AllowEvents(35) {
		pub $mode: enum AllowEventsMode {
			AsyncPointer = 0,
			SyncPointer = 1,
			ReplayPointer = 2,
			AsyncKeyboard = 3,
			SyncKeyboard = 4,
			ReplayKeyboard = 5,
			AsyncBoth = 6,
			SyncBoth = 7,
		},
		pub time: Time,
	}

	pub struct GrabServer(36);
	pub struct UngrabSever(37);

	pub struct QueryPointer(38) -> QueryPointerReply: pub target: Window;

	pub struct QueryPointerReply for QueryPointer {
		pub $same_screen: bool,
		pub root: Window,
		pub child: Option<Window>,
		pub root_x: i16,
		pub root_y: i16,
		pub win_x: i16,
		pub win_y: i16,
		pub mask: KeyButtonMask,
		()[6],
	}

	pub struct GetMotionEvents(39) -> GetMotionEventsReply {
		pub target: Window,
		pub start: Time,
		pub stop: Time,
	}

	pub struct GetMotionEventsReply for GetMotionEvents {
		#events[u32],
		()[20],
		pub events: [(Timestamp, (i16, i16))],
	}

	pub struct TranslateCoordinates(40) -> TranslateCoordinatesReply {
		pub source: Window,
		pub destination: Window,
		pub src_x: u16,
		pub src_y: u16,
	}

	pub struct TranslateCoordinatesReply for TranslateCoordinates {
		pub $same_screen: bool,
		pub child: Option<Window>,
		pub dest_x: i16,
		pub dest_y: i16,
		()[16],
	}

	pub struct WarpPointer(41) {
		pub source: Option<Window>,
		pub destination: Option<Window>,
		pub src_x: i16,
		pub src_y: i16,
		pub src_width: u16,
		pub src_height: u16,
		pub dest_x: u16,
		pub dest_y: u16,
	}

	pub struct SetInputFocus(42) {
		pub $revert_to: Option<RevertTo>,
		pub focus: Option<Focus<Window>>,
		pub time: Time,
	}

	pub struct GetInputFocus(43) -> GetInputFocusReply;

	pub struct GetInputFocusReply for GetInputFocus {
		pub $revert_to: Option<RevertTo>,
		pub focus: Option<Focus<Window>>,
		()[20],
	}

	pub struct QueryKeymap(44) -> QueryKeymapReply;

	pub struct QueryKeymapReply for QueryKeymap {
		pub keys: [u8; 32],
	}

	pub struct OpenFont(45) {
		pub font_id: Font,
		#name,
		()[2],
		pub name: String,
		()[padding(name)],
	}

	pub struct CloseFont(46): pub font: Font;

	pub struct QueryFont(47) -> QueryFontReply: pub font: Fontable;

	pub struct QueryFontReply for QueryFont {
		pub min_bounds: CharInfo,
		()[4],
		pub max_bounds: CharInfo,
		()[4],
		pub min_char_or_byte2: u16,
		pub max_char_or_byte2: u16,
		#properties,
		pub draw_direction: enum DrawDirection {
			LeftToRight = 0,
			RightToLeft = 1,
		},
		pub min_byte1: u8,
		pub max_byte1: u8,
		pub all_chars_exist: bool,
		pub font_ascent: i16,
		pub font_descent: i16,
		#charinfos[u32],
		pub properties: [FontProp],
		pub charinfos: [CharInfo],
	}

	pub struct QueryTextExtends(48) -> QueryTextExtentsReply {
		pub $odd_length: bool,
		pub font: Fontable,
		pub string: String16,
		()[padding(string)],
	}

	pub struct QueryTextExtentsReply for QueryTextExtents {
		pub $draw_direction: DrawDirection,
		pub font_ascent: i16,
		pub font_descent: i16,
		pub overall_ascent: i16,
		pub overall_Descent: i16,
		pub overall_width: i32,
		pub overall_left: i32,
		pub overall_right: i32,
		()[4],
	}

	pub struct ListFonts(49) -> ListFontsReply {
		pub max_names: u16,
		#pattern,
		pub pattern: String,
		()[padding(pattern)],
	}

	pub struct ListFontsReply for ListFonts {
		#names, // number of STRs in names??
		()[22],
		pub names: [Str], // STRs??
		()[padding(names)],
	}

	// ListFontsWithInfo has a special format for its reply that needs to be
	// done manually, so both the request and the reply are contained within the
	// `mod list_fonts_with_info;` module.

	pub struct SetFontPath(51) {
		#names, // number of STRs in path??
		()[2],
		pub path: [Str], // STRs??
		()[padding(path)],
	}

	// GetFontPath has a special format for its request. Both the request and
	// the reply are done manually and can be found in the `mod get_font_path;`
	// module.

	pub struct CreatePixmap(53) {
		pub $depth: u8,
		pub pixmap_id: Pixmap,
		pub drawable: Drawable,
		pub width: u16,
		pub height: u16,
	}

	pub struct FreePixmap(54): pub pixmap: Pixmap;

	pub struct CreateGcontext(55) {
		pub context_id: GraphicsContext,
		pub drawable: Drawable,
		pub value_mask: GraphicsContextMask,
		pub values: [GraphicsContextValue],
	}

	pub struct ChangeGraphicsContext(56) {
		pub context: GraphicsContext,
		pub value_mask: GraphicsContextMask,
		pub values: [GraphicsContextValue],
	}

	pub struct CopyGraphicsContext(57) {
		pub source: GraphicsContext,
		pub destination: GraphicsContext,
		pub value_mask: GraphicsContextMask,
	}

	pub struct SetDashes(58) {
		pub context: GraphicsContext,
		pub dash_offset: u16,
		#dashes,
		pub dashes: [u8],
		()[padding(dashes)],
	}

	pub struct SetClipRectangles(59) {
		pub $ordering: enum Ordering {
			Unsorted = 0,
			Ysorted = 1,
			YxSorted = 2,
			YxBanded = 3,
		},
		pub context: GraphicsContext,
		pub clip_x_origin: i16,
		pub clip_y_origin: i16,
		pub rectangles: [Rectangle],
	}

	pub struct FreeGraphicsContext(60): pub context: GraphicsContext;

	pub struct ClearArea(61) {
		pub $exposures: bool,
		pub target_window: Window,
		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,
	}

	pub struct CopyArea(62) {
		pub source: Drawable,
		pub destination: Drawable,
		pub context: GraphicsContext,
		pub src_x: i16,
		pub src_y: i16,
		pub dest_x: i16,
		pub dest_y: i16,
		pub width: u16,
		pub height: u16,
	}

	pub struct CopyPlane(63) {
		pub source: Drawable,
		pub destination: Drawable,
		pub context: GraphicsContext,
		pub src_x: i16,
		pub src_y: i16,
		pub dest_x: i16,
		pub dest_y: i16,
		pub width: u16,
		pub height: u16,
		pub bit_plane: u32,
	}

	pub struct PolyPoint(64) {
		pub $coordinate_mode: enum CoordinateMode {
			Origin = 0,
			Previous = 1,
		},
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub points: [(i16, i16)],
	}

	pub struct PolyLine(65) {
		pub $coordinate_mode: CoordinateMode,
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub points: [(i16, i16)],
	}

	pub struct PolySegment(66) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub segments: [((i16, i16), (i16, i16))],
	}

	pub struct PolyRectangle(67) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub rectangles: [Rectangle],
	}

	pub struct PolyArc(68) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub arcs: [GeomArc],
	}

	pub struct FillPoly(69) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub shape: enum Shape {
			Complex = 0,
			Nonconvex = 1,
			Convex = 2,
		},
		pub coordinate_mode: CoordinateMode,
		()[2],
		pub points: [(i16, i16)],
	}

	pub struct PolyFillRectangle(70) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub rectangles: [Rectangle],
	}

	pub struct PolyFillArc(71) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub arcs: [GeomArc],
	}

	pub struct PutImage(72) {
		pub $format: Bitmap<ImageFormat>,
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub width: u16,
		pub height: u16,
		pub dest_x: i16,
		pub dest_y: i16,
		pub left-padding: u8,
		pub depth: u8,
		()[2],
		pub data: [u8],
		()[padding(data)],
	}

	pub struct GetImage(73) -> GetImageReply {
		pub $format: ImageFormat,
		pub drawable: Drawable,
		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,
		pub plane_mask: u32,
	}

	pub struct GetImageReply for GetImage {
		pub $depth: u8,
		pub visual: Option<VisualId>,
		()[20],
		pub data: [u8],
		()[padding(data)],
	}

	pub struct PolyText(74) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub x: i16,
		pub y: i16,
		pub items: [TextItem],
		()[padding(items)],
	}

	pub struct PolyText16(75) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub x: i16,
		pub y: i16,
		pub items: [TextItem16],
		()[padding(items)],
	}

	pub struct ImageText(76) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub x: i16,
		pub y: i16,
		pub string: String,
		()[padding(string)],
	}

	pub struct ImageText16(77) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub x: i16,
		pub y: i16,
		pub string: String16,
		()[padding(string)],
	}

	pub struct CreateColormap(78) {
		pub $alloc: enum ColormapAlloc {
			None = 0,
			All = 1,
		},
		pub colormap_id: Colormap,
		pub window: Window,
		pub visual: VisualId,
	}

	pub struct FreeColormap(79): pub colormap: Colormap;

	pub struct CopyColormapAndFree(80) {
		pub colormap_id: Colormap,
		pub source: Colormap,
	}

	pub struct InstallColormap(81): pub colormap: Colormap;
	pub struct UninstallColormap(82): pub colormap: Colormap;

	pub struct ListInstalledColormaps(73) -> ListInstalledColormapsReply {
		pub target_window: Window,
	}

	pub struct ListInstalledColormapsReply for ListInstalledColormaps {
		#colormaps,
		()[22],
		pub colormaps: [Colormap],
	}

	pub struct AllocColor(84) -> AllocColorReply {
		pub colormap: Colormap,
		pub color: (u16, u16, u16),
		()[2],
	}

	pub struct AllocColorReply for AllocColor {
		pub color: (u16, u16, u16),
		()[2],
		pub pixel: u32,
		()[12],
	}

	pub struct AllocNamedColor(85) -> AllocNamedColorReply {
		pub colormap: Colormap,
		#name,
		()[2],
		pub name: String,
		()[padding(name)],
	}

	pub struct AllocNamedColorReply for AllocNamedColor {
		pub pixel: u32,
		pub exact_color: (u16, u16, u16),
		pub visual_color: (u16, u16, u16),
		()[8],
	}

	pub struct AllocColorCells(86) -> AllocColorCellsReply {
		pub $contiguous: bool,
		pub colormap: Colormap,
		pub num_colors: u16, // TODO: its just called `colors`... is it the number?
		pub planes: u16,
	}

	pub struct AllocColorCellsReply for AllocColorCells {
		#pixels,
		#masks,
		()[20],
		pub pixels: [u32],
		pub masks: [u32],
	}

	pub struct AllocColorPlanes(87) -> AllocColorPlanesReply {
		pub $contiguous: bool,
		pub colormap: Colormap,
		pub num_colors: u16, // TODO: its just called `colors`... is it the number?
		pub colors: (u16, u16, u16),
	}

	pub struct AllocColorPlanesReply for AllocColorPlanes {
		#pixels,
		()[2],
		pub color_mask: (u16, u16, u16),
		()[8],
		pub pixels: [u32],
	}

	pub struct FreeColors(88) {
		pub colormap: Colormap,
		pub plane_mask: u32,
		pub pixels: [u32],
	}

	pub struct StoreColors(89) {
		pub colormap: Colormap,
		pub items: [ColorItem],
	}

	pub struct StoreNamedColor(90) {
		pub $channel_mask: ColorChannelMask,
		pub colormap: Colormap,
		pub pixel: u32,
		#name,
		()[2],
		pub name: String,
		()[padding(name)],
	}

	// The QueryColorsReply for the QueryColors request uses a special format
	// for its list of colors, and so the reply must be done manually. The
	// reply and request have been put in `mod query_colors;`.

	pub struct LookupColor(92) -> LookupColorReply {
		pub colormap: Colormap,
		#name,
		()[2],
		pub name: String,
		()[padding(name)],
	}

	pub struct LookupColorReply for LookupColor {
		pub exact_color: (u16, u16, u16),
		pub visual_color: (u16, u16, u16),
		()[12],
	}

	pub struct CreateCursor(93) {
		pub cursor_id: Cursor,
		pub source: Pixmap,
		pub mask: Option<Pixmap>,
		pub foreground_color: (u16, u16, u16),
		pub background_color: (u16, u16, u16),
		pub x: u16,
		pub y: u16,
	}

	pub struct CreateGlyphCursor(94) {
		pub cursor_id: Cursor,
		pub source_font: Font,
		pub mask_font: Option<Font>,
		pub source_char: u16,
		pub mask_char: u16,
		pub foreground_color: (u16, u16, u16),
		pub background_color: (u16, u16, u16),
	}

	pub struct FreeCursor(95): pub cursor: Cursor;

	pub struct RecolorCursor(96) {
		pub cursor: Cursor,
		pub foreground_color: (u16, u16, u16),
		pub background_color: (u16, u16, u16),
	}

	pub struct QueryBestSize(97) -> QueryBestSizeReply {
		pub $class: enum SizeClass {
			Cursor = 0,
			Tile = 1,
			Stipple = 2,
		},
		pub drawable: Drawable,
		pub width: u16,
		pub height: u16,
	}

	pub struct QueryBestSizeReply for QueryBestSize {
		pub width: u16,
		pub height: u16,
		()[20],
	}

	pub struct QueryExtension(98) -> QueryExtensionReply {
		#name,
		()[2],
		pub name: String,
		()[padding(name)],
	}

	pub struct QueryExtensionReply for QueryExtension {
		pub $present: bool,
		pub major_opcode: u8,
		pub first_event: u8,
		pub first_error: u8,
		()[20],
	}
}
