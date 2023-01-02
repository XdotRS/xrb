// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate self as xrb;

use xrb::common::*;
use xrbk_macro::derive_xrb;

derive_xrb! {
	/// Creates an unmapped window with the given `window_id`.
	///
	/// # Events
	/// - [CreateNotify]
	///
	/// # Errors
	/// - [Alloc]
	/// - [Colormap]
	/// - [Cursor]
	/// - [IdChoice]
	/// - [Match] -- Generated if the `class` is [`InputOutput`] and the `visual`
	///   type and `depth` are not a combination supported by the screen, or if
	///   the `class` is [`InputOnly`] and the `depth` is not [`CopyFromParent`]
	///   or `0`.
	/// - [Pixmap]
	/// - [Value]
	/// - [Window]
	///
	/// [Alloc]: crate::x11::error::Alloc
	/// [Colormap]: crate::x11::error::Colormap
	/// [Cursor]: crate::x11::error::Cursor
	/// [IdChoice]: crate::x11::error::IdChoice
	/// [Match]: crate::x11::error::Match
	/// [Pixmap]: crate::x11::error::Pixmap
	/// [Value]: crate::x11::error::Value
	/// [Window]: crate::x11::error::Window
	pub struct CreateWindow: Request(1) {
		/// The color depth of the window in bits per pixel.
		///
		/// If the class is not [`InputOnly`], [`CopyFromParent`] will copy the
		/// `depth` from the parent. __If the class is [`InputOnly`], this must
		/// be set to [`CopyFromParent`]__, else a [`Match`] error shall occur.
		///
		/// [`InputOnly`]: WindowClass::InputOnly
		/// [`CopyFromParent`]: Inherit::CopyFromParent
		/// [`Match`]: crate::x11::error::Match
		#[metabyte]
		pub depth: Inheritable<u8>,

		/// The resource ID given to the window.
		pub window_id: Window,
		/// The parent of which the window will be created as a child of.
		pub parent: Window,
		/// The initial x-coordinate of the window relative to its parent's
		/// top-left corner.
		pub x: i16,
		/// The initial y-coordinate of the window relative to its parent's
		/// top-right corner.
		pub y: i16,
		/// The width of the window.
		pub width: u16,
		/// The height of the window.
		pub height: u16,
		pub border_width: u16,
		/// The [window class] of the window.
		///
		/// For [`InputOutput`], the `visual` type and `depth` must be a
		/// combination supported by the screen, else a [`Match`] error occurs.
		///
		/// For [`InputOnly`], the `depth` must be [`CopyFromParent`] (or `0`).
		///
		/// [`InputOutput`]: WindowClass::InputOutput
		/// [`InputOnly`]: WindowClass::InputOnly
		/// [window class]: WindowClass
		/// [`Match`]: crate::x11::error::Match
		pub class: Inheritable<WindowClass>,
		pub visual: Inheritable<VisualId>,
		pub value_mask: AttributeMask,
		/// A list of [window attributes] that are to configured for the window.
		///
		/// [window attributes]: Attribute
		pub values: Vec<Attribute>, // Window is a placeholder until WinAttr is done
	}

	pub struct ChangeWindowAttributes: Request(2) {
		pub target: Window,
		pub value_mask: AttributeMask,
		pub values: Vec<Attribute>,
	}

	pub struct GetWindowAttributes: Request(3) -> GetWindowAttributesReply {
		pub target: Window,
	}

	pub struct GetWindowAttributesReply: Reply for GetWindowAttributes {
		#[metabyte]
		pub backing_store: BackingStore,
		#[sequence]
		pub sequence: u16,

		pub visual: VisualId,
		pub class: WindowClass,
		pub bit_gravity: BitGravity,
		pub win_gravity: WinGravity,
		pub backing_planes: u32,
		pub backing_pixel: u32,
		pub save_under: bool,
		pub map_is_installed: bool,
		pub map_state: MapState,
		pub override_redirect: bool,
		pub colormap: Option<Colormap>,
		pub all_event_masks: EventMask,
		pub your_event_mask: EventMask,
		pub do_not_propagate_mask: DeviceEventMask,
		[_; ..],
	}

	pub struct DestroyWindow: Request(4) {
		pub target: Window,
	}

	pub struct DestroySubwindows: Request(5) {
		pub target: Window,
	}

	pub struct ChangeSaveSet: Request(6) {
		#[metabyte]
		pub mode: EditMode,

		pub target: Window,
	}

	pub struct ReparentWindow: Request(7) {
		pub target: Window,
		pub new_parent: Window,
		pub new_x: i16,
		pub new_y: i16,
	}

	pub struct MapWindow: Request(8) {
		pub target: Window,
	}

	pub struct MapSubwindows: Request(9) {
		pub target: Window,
	}

	pub struct UnmapWindow: Request(10) {
		pub target: Window,
	}

	pub struct UnmapSubwindows: Request(11) {
		pub target: Window,
	}

	pub struct ConfigureWindow: Request(12) {
		pub target: Window,
		pub value_mask: ConfigureWindowMask,
		pub values: Vec<ConfigureWindowValue>,
	}

	pub struct CirculateWindow: Request(13) {
		#[metabyte]
		pub direction: CirculateDirection,

		pub target: Window,
	}

	pub struct GetGeometry: Request(14) -> GetGeometryReply {
		pub target: Drawable,
	}

	pub struct GetGeometryReply: Reply for GetGeometry {
		#[metabyte]
		pub depth: u8,
		#[sequence]
		pub sequence: u16,

		pub root: Window,
		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,
		pub border_width: u16,
		[_; ..],
	}

	pub struct QueryTree: Request(15) -> QueryTreeReply {
		pub target: Window,
	}

	pub struct QueryTreeReply: Reply for QueryTree {
		#[sequence]
		pub sequence: u16,

		pub root: Window,
		pub parent: Option<Window>,
		let children_len: u16 = children => children.len() as u16,
		[_; 14],

		// Wouldn't it be better to use &'a [Window] instead ?
		#[context(children_len => *children_len as usize)]
		pub children: Vec<Window>,
	}

	pub struct InternAtom: Request(16) -> InternAtomReply {
		#[metabyte]
		pub only_if_exists: bool,

		let name_len: u16 = name => name.len() as u16,
		[_; 2],

		#[context(name_len => *name_len as usize)]
		pub name: String8,
		[_; ..],
	}

	pub struct InternAtomReply: Reply for InternAtom {
		#[sequence]
		pub sequence: u16,

		pub atom: Option<Atom>,
		[_; ..],
	}

	pub struct GetAtomName: Request(17) -> GetAtomNameReply {
		pub atom: Atom,
	}

	pub struct GetAtomNameReply: Reply for GetAtomName {
		#[sequence]
		pub sequence: u16,

		let name_len: u16 = name => name.len() as u16,
		[_; 22],

		#[context(name_len => *name_len as usize)]
		pub name: String8,
		[_; ..],
	}

	// The property requests (`ChangeProperty(18)`, `DeleteProperty(19)`,
	// `GetProperty(20)`, and `ListProperties(21)`) are special cases and need
	// to be defined manually. You can find them in `mod properties;`.

	pub struct SetSelectionOwner: Request(22) {
		pub owner: Option<Window>,
		pub selection: Atom,
		pub time: Time,
	}

	pub struct GetSelectionOwner: Request(23) -> GetSelectionOwnerReply {
		pub selection: Atom,
	}

	pub struct GetSelectionOwnerReply: Reply for GetSelectionOwner {
		#[sequence]
		pub sequence: u16,

		pub owner: Option<Window>,
		[_; ..],
	}

	pub struct ConvertSelection: Request(24) {
		pub requestor: Window,
		pub selection: Atom,
		pub target: Atom,
		pub property: Option<Atom>,
		pub time: Time,
	}

	pub struct SendEvent: Request(25) {
		#[metabyte]
		pub propagate: bool,

		pub destination: Destination,
		pub event_mask: EventMask,
		//pub event: Box<dyn Event>,
	}

	pub struct GrabPointer: Request(26) -> GrabPointerReply {
		#[metabyte]
		pub owner_events: bool,

		pub target_window: Window,
		pub event_mask: PointerEventMask,
		pub pointer_mode: GrabMode,
		pub keyboard_mode: GrabMode,
		pub confine_to: Option<Window>,
		pub cursor_override: Option<Cursor>,
		pub time: Time,
	}

	pub struct GrabPointerReply: Reply for GrabPointer {
		#[metabyte]
		pub status: GrabStatus,
		#[sequence]
		pub sequence: u16,

		[_; ..],
	}

	pub struct UngrabPointer: Request(27) {
		pub time: Time,
	}

	pub struct GrabButton: Request(28) {
		#[metabyte]
		pub owner_events: bool,

		pub target_window: Window,
		pub event_mask: PointerEventMask,
		pub pointer_mode: GrabMode,
		pub keyboard_mode: GrabMode,
		pub confine_to: Option<Window>,
		pub cursor_override: Option<Cursor>,
		pub button: Any<Button>,
		_,

		pub modifiers: AnyModifierKeyMask,
	}

	pub struct UngrabButton: Request(29) {
		#[metabyte]
		pub button: Any<Button>,

		pub target_window: Window,
		[_; ..],
	}

	pub struct ChangeActivePointerGrab: Request(30) {
		pub cursor_override: Option<Cursor>,
		pub time: Time,
		pub event_mask: PointerEventMask,
		[_; ..],
	}

	pub struct GrabKeyboard: Request(31) -> GrabKeyboardReply {
		#[metabyte]
		pub owner_events: bool,

		pub target_window: Window,
		pub time: Time,
		pub pointer_mode: GrabMode,
		pub keyboard_mode: GrabMode,
		[_; ..],
	}

	pub struct GrabKeyboardReply: Reply for GrabKeyboard {
		#[metabyte]
		pub status: GrabStatus,
		#[sequence]
		pub sequence: u16,

		[_; ..],
	}

	pub struct UngrabKeyboard: Request(32) {
		pub time: Time,
	}

	pub struct GrabKey: Request(33) {
		#[metabyte]
		pub owner_events: bool,

		pub target_window: Window,
		pub modifiers: AnyModifierKeyMask,
		pub key: Any<Keycode>,
		pub pointer_mode: GrabMode,
		pub keyboard_mode: GrabMode,
		[_; ..],
	}

	pub struct UngrabKey: Request(34) {
		#[metabyte]
		pub key: Any<Keycode>,

		pub target_window: Window,
		pub modifiers: AnyModifierKeyMask,
		[_; ..],
	}

	pub struct AllowEvents: Request(35) {
		#[metabyte]
		pub mode: AllowEventsMode,

		pub time: Time,
	}

	pub struct GrabServer: Request(36);
	pub struct UngrabSever: Request(37);

	pub struct QueryPointer: Request(38) -> QueryPointerReply {
		pub target: Window,
	}

	pub struct QueryPointerReply: Reply for QueryPointer {
		#[metabyte]
		pub same_screen: bool,
		#[sequence]
		pub sequence: u16,

		pub root: Window,
		pub child: Option<Window>,
		pub root_x: i16,
		pub root_y: i16,
		pub win_x: i16,
		pub win_y: i16,
		pub mask: ModifierMask,
		[_; ..],
	}

	pub struct GetMotionEvents: Request(39) -> GetMotionEventsReply {
		pub target: Window,
		pub start: Time,
		pub stop: Time,
	}

	pub struct GetMotionEventsReply: Reply for GetMotionEvents {
		#[sequence]
		pub sequence: u16,

		let events_len: u32 = events => events.len() as u32,
		[_; 20],

		#[context(events_len => *events_len as usize)]
		pub events: Vec<(Timestamp, (i16, i16))>,
	}

	pub struct TranslateCoordinates: Request(40) -> TranslateCoordinatesReply {
		pub source: Window,
		pub destination: Window,
		pub src_x: u16,
		pub src_y: u16,
	}

	pub struct TranslateCoordinatesReply: Reply for TranslateCoordinates {
		#[metabyte]
		pub same_screen: bool,
		#[sequence]
		pub sequence: u16,

		pub child: Option<Window>,
		pub dest_x: i16,
		pub dest_y: i16,
		[_; ..],
	}

	pub struct WarpPointer: Request(41) {
		pub source: Option<Window>,
		pub destination: Option<Window>,
		pub src_x: i16,
		pub src_y: i16,
		pub src_width: u16,
		pub src_height: u16,
		pub dest_x: u16,
		pub dest_y: u16,
	}

	pub struct SetInputFocus: Request(42) {
		#[metabyte]
		pub revert_to: Option<RevertTo>,

		pub focus: Option<InputFocus>,
		pub time: Time,
	}

	pub struct GetInputFocus: Request(43) -> GetInputFocusReply;

	pub struct GetInputFocusReply: Reply for GetInputFocus {
		#[metabyte]
		pub revert_to: RevertTo,
		#[sequence]
		pub sequence: u16,

		pub focus: Option<InputFocus>,
		[_; ..],
	}

	pub struct QueryKeymap: Request(44) -> QueryKeymapReply;

	pub struct QueryKeymapReply: Reply for QueryKeymap {
		#[sequence]
		pub sequence: u16,

		pub keys: [u8; 32],
	}

	pub struct OpenFont: Request(45) {
		pub font_id: Font,
		let name_len: u16 = name => name.len() as u16,
		[_; 2],

		#[context(name_len => *name_len as usize)]
		pub name: String8,
		[_; ..],
	}

	pub struct CloseFont: Request(46) {
		pub font: Font,
	}

	pub struct QueryFont: Request(47) -> QueryFontReply {
		pub font: Fontable,
	}

	pub struct QueryFontReply: Reply for QueryFont {
		#[sequence]
		pub sequence: u16,

		pub min_bounds: CharInfo,
		[_; 4],

		pub max_bounds: CharInfo,
		[_; 4],

		pub min_char_or_byte2: u16,
		pub max_char_or_byte2: u16,
		let properties_len: u16 = properties => properties.len() as u16,
		pub draw_direction: DrawDirection,
		pub min_byte1: u8,
		pub max_byte1: u8,
		pub all_chars_exist: bool,
		pub font_ascent: i16,
		pub font_descent: i16,
		let charinfos_len: u32 = charinfos => charinfos.len() as u32,
		#[context(properties_len => *properties_len as usize)]
		pub properties: Vec<FontProperty>,
		#[context(charinfos_len => *charinfos_len as usize)]
		pub charinfos: Vec<CharInfo>,
	}

	pub struct QueryTextExtents: Request(48) -> QueryTextExtentsReply {
		#[metabyte]
		pub odd_length: bool,

		pub font: Fontable,
		#[context(self::length => (*length as usize) - 8)]
		pub string: String16,
		[_; ..],
	}

	pub struct QueryTextExtentsReply: Reply for QueryTextExtents {
		#[metabyte]
		pub draw_direction: DrawDirection,
		#[sequence]
		pub sequence: u16,

		pub font_ascent: i16,
		pub font_descent: i16,
		pub overall_ascent: i16,
		pub overall_descent: i16,
		pub overall_width: i32,
		pub overall_left: i32,
		pub overall_right: i32,
		[_; ..],
	}

	pub struct ListFonts: Request(49) -> ListFontsReply {
		pub max_names: u16,
		let pattern_len: u16 = pattern => pattern.len() as u16,
		#[context(pattern_len => *pattern_len as usize)]
		pub pattern: String8,
		[_; ..],
	}

	pub struct ListFontsReply: Reply for ListFonts {
		#[sequence]
		pub sequence: u16,

		let names_len: u16 = names => names.len() as u16,
		[_; 22],

		#[context(names_len => *names_len as usize)]
		pub names: Vec<LenString8>,
		[_; ..],
	}

	// ListFontsWithInfo has a special format for its reply that needs to be
	// done manually, so both the request and the reply are contained within the
	// `mod list_fonts_with_info;` module.

	pub struct SetFontPath: Request(51) {
		let path_len: u16 = path => path.len() as u16,
		[_; 2],

		#[context(path_len => *path_len as usize)]
		pub path: Vec<LenString8>,
		[_; ..],
	}

	// GetFontPath has a special format for its request. Both the request and
	// the reply are done manually and can be found in the `mod get_font_path;`
	// module.

	pub struct CreatePixmap: Request(53) {
		#[metabyte]
		pub depth: u8,

		pub pixmap_id: Pixmap,
		pub drawable: Drawable,
		pub width: u16,
		pub height: u16,
	}

	pub struct FreePixmap: Request(54) {
		pub pixmap: Pixmap,
	}

	pub struct CreateGraphicsContext: Request(55) {
		pub context_id: GraphicsContext,
		pub drawable: Drawable,
		pub value_mask: GraphicsContextMask,
		// TODO: context attribute
		pub values: Vec<GraphicsContextValue>,
	}

	pub struct ChangeGraphicsContext: Request(56) {
		pub context: GraphicsContext,
		pub value_mask: GraphicsContextMask,
		// TODO: context attribute
		pub values: Vec<GraphicsContextValue>,
	}

	pub struct CopyGraphicsContext: Request(57) {
		pub source: GraphicsContext,
		pub destination: GraphicsContext,
		pub value_mask: GraphicsContextMask,
	}

	pub struct SetDashes: Request(58) {
		pub context: GraphicsContext,
		pub dash_offset: u16,
		let dashes_len: u16 = dashes => dashes.len() as u16,
		#[context(dashes_len => *dashes_len as usize)]
		pub dashes: Vec<u8>,
		[_; ..],
	}

	pub struct SetClipRectangles: Request(59) {
		#[metabyte]
		pub ordering: Ordering,

		pub context: GraphicsContext,
		pub clip_x_origin: i16,
		pub clip_y_origin: i16,
		// TODO: context attribute
		pub rectangles: Vec<Rectangle>,
	}

	pub struct FreeGraphicsContext: Request(60) {
		pub context: GraphicsContext,
	}

	pub struct ClearArea: Request(61) {
		#[metabyte]
		pub exposures: bool,

		pub target_window: Window,
		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,
	}

	pub struct CopyArea: Request(62) {
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

	pub struct CopyPlane: Request(63) {
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

	pub struct PolyPoint: Request(64) {
		#[metabyte]
		pub coordinate_mode: CoordinateMode,

		pub drawable: Drawable,
		pub context: GraphicsContext,
		// TODO: context attribute
		pub points: Vec<(i16, i16)>,
	}

	pub struct PolyLine: Request(65) {
		#[metabyte]
		pub coordinate_mode: CoordinateMode,

		pub drawable: Drawable,
		pub context: GraphicsContext,
		// TODO: context attribute
		pub points: Vec<(i16, i16)>,
	}

	pub struct PolySegment: Request(66) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		// TODO: context attribute
		pub segments: Vec<Segment>,
	}

	pub struct PolyRectangle: Request(67) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		// TODO: context attribute
		pub rectangles: Vec<Rectangle>,
	}

	pub struct PolyArc: Request(68) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		// TODO: context attribute
		pub arcs: Vec<GeomArc>,
	}

	pub struct FillPoly: Request(69) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub shape: Shape,
		pub coordinate_mode: CoordinateMode,
		[_; 2],

		// TODO: context attribute
		pub points: Vec<(i16, i16)>,
	}

	pub struct PolyFillRectangle: Request(70) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		// TODO: context attribute
		pub rectangles: Vec<Rectangle>,
	}

	pub struct PolyFillArc: Request(71) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		// TODO: context attribute
		pub arcs: Vec<GeomArc>,
	}

	pub struct PutImage: Request(72) {
		#[metabyte]
		pub format: BitmapFormat,

		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub width: u16,
		pub height: u16,
		pub dest_x: i16,
		pub dest_y: i16,
		pub left_padding: u8,
		pub depth: u8,
		[_; 2],

		// TODO: context attribute
		pub data: Vec<u8>,
		[_; ..],
	}

	pub struct GetImage: Request(73) -> GetImageReply {
		#[metabyte]
		pub format: Format,

		pub drawable: Drawable,
		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,
		pub plane_mask: u32,
	}

	pub struct GetImageReply: Reply for GetImage {
		#[metabyte]
		pub depth: u8,
		#[sequence]
		pub sequence: u16,

		pub visual: Option<VisualId>,
		[_; 20],

		// TODO: context attribute
		pub data: Vec<u8>,
		[_; ..],
	}

	pub struct PolyText8: Request(74) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub x: i16,
		pub y: i16,
		pub items: Vec<TextItem8>,
		[_; ..],
	}

	pub struct PolyText16: Request(75) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub x: i16,
		pub y: i16,
		pub items: Vec<TextItem16>,
		[_; ..],
	}

	pub struct ImageText8: Request(76) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub x: i16,
		pub y: i16,
		// TODO: context attribute
		pub string: String8,
		[_; ..],
	}

	pub struct ImageText16: Request(77) {
		pub drawable: Drawable,
		pub context: GraphicsContext,
		pub x: i16,
		pub y: i16,
		// TODO: context attribute
		pub string: String16,
		[_; ..],
	}

	pub struct CreateColormap: Request(78) {
		#[metabyte]
		pub alloc: ColormapAlloc,

		pub colormap_id: Colormap,
		pub window: Window,
		pub visual: VisualId,
	}

	pub struct FreeColormap: Request(79) {
		pub colormap: Colormap,
	}

	pub struct CopyColormapAndFree: Request(80) {
		pub colormap_id: Colormap,
		pub source: Colormap,
	}

	pub struct InstallColormap: Request(81) {
		pub colormap: Colormap,
	}

	pub struct UninstallColormap: Request(82) {
		pub colormap: Colormap,
	}

	pub struct ListInstalledColormaps: Request(73) -> ListInstalledColormapsReply {
		pub target_window: Window,
	}

	pub struct ListInstalledColormapsReply: Reply for ListInstalledColormaps {
		#[sequence]
		pub sequence: u16,

		let colormaps_len: u16 = colormaps => colormaps.len() as u16,
		[_; 22],

		#[context(colormaps_len => *colormaps_len as usize)]
		pub colormaps: Vec<Colormap>,
	}

	pub struct AllocColor: Request(84) -> AllocColorReply {
		pub colormap: Colormap,
		pub color: (u16, u16, u16),
		[_; ..],
	}

	pub struct AllocColorReply: Reply for AllocColor {
		#[sequence]
		pub sequence: u16,

		pub color: (u16, u16, u16),
		[_; 2],

		pub pixel: u32,
		[_; ..],
	}

	pub struct AllocNamedColor: Request(85) -> AllocNamedColorReply {
		pub colormap: Colormap,
		let name_len: u16 = name => name.len() as u16,
		[_; 2],

		#[context(name_len => *name_len as usize)]
		pub name: String8,
		[_; ..],
	}

	pub struct AllocNamedColorReply: Reply for AllocNamedColor {
		#[sequence]
		pub sequence: u16,

		pub pixel: u32,
		pub exact_color: (u16, u16, u16),
		pub visual_color: (u16, u16, u16),
		[_; ..],
	}

	pub struct AllocColorCells: Request(86) -> AllocColorCellsReply {
		#[metabyte]
		pub contiguous: bool,

		pub colormap: Colormap,
		pub num_colors: u16, // TODO: its just called `colors`... is it the number?
		pub planes: u16,
	}

	pub struct AllocColorCellsReply: Reply for AllocColorCells {
		#[sequence]
		pub sequence: u16,

		let pixels_len: u16 = pixels => pixels.len() as u16,
		let masks_len: u16 = masks => masks.len() as u16,
		[_; 20],

		#[context(pixels_len => *pixels_len as usize)]
		pub pixels: Vec<u32>,
		#[context(masks_len => *masks_len as usize)]
		pub masks: Vec<u32>,
	}

	pub struct AllocColorPlanes: Request(87) -> AllocColorPlanesReply {
		#[metabyte]
		pub contiguous: bool,

		pub colormap: Colormap,
		pub num_colors: u16, // TODO: its just called `colors`... is it the number?
		pub colors: (u16, u16, u16),
	}

	pub struct AllocColorPlanesReply: Reply for AllocColorPlanes {
		#[sequence]
		pub sequence: u16,

		let pixels_len: u16 = pixels => pixels.len() as u16,
		[_; 2],

		pub color_mask: (u16, u16, u16),
		[_; 8],

		#[context(pixels_len => *pixels_len as usize)]
		pub pixels: Vec<u32>,
	}

	pub struct FreeColors: Request(88) {
		pub colormap: Colormap,
		pub plane_mask: u32,
		// TODO: context attribute
		pub pixels: Vec<u32>,
	}

	pub struct StoreColors: Request(89) {
		pub colormap: Colormap,
		// TODO: context attribute
		pub items: Vec<ColorItem>,
	}

	pub struct StoreNamedColor: Request(90) {
		#[metabyte]
		pub channel_mask: ColorChannelMask,

		pub colormap: Colormap,
		pub pixel: u32,
		let name_len: u16 = name => name.len() as u16,
		[_; 2],

		#[context(name_len => *name_len as usize)]
		pub name: String8,
		[_; ..],
	}

	// The QueryColorsReply for the QueryColors request uses a special format
	// for its list of colors, and so the reply must be done manually. The
	// reply and request have been put in `mod query_colors;`.

	pub struct LookupColor: Request(92) -> LookupColorReply {
		pub colormap: Colormap,
		let name_len: u16 = name => name.len() as u16,
		[_; 2],

		#[context(name_len => *name_len as usize)]
		pub name: String8,
		[_; ..],
	}

	pub struct LookupColorReply: Reply for LookupColor {
		#[sequence]
		pub sequence: u16,

		pub exact_color: (u16, u16, u16),
		pub visual_color: (u16, u16, u16),
		[_; ..],
	}

	pub struct CreateCursor: Request(93) {
		pub cursor_id: Cursor,
		pub source: Pixmap,
		pub mask: Option<Pixmap>,
		pub foreground_color: (u16, u16, u16),
		pub background_color: (u16, u16, u16),
		pub x: u16,
		pub y: u16,
	}

	pub struct CreateGlyphCursor: Request(94) {
		pub cursor_id: Cursor,
		pub source_font: Font,
		pub mask_font: Option<Font>,
		pub source_char: u16,
		pub mask_char: u16,
		pub foreground_color: (u16, u16, u16),
		pub background_color: (u16, u16, u16),
	}

	pub struct FreeCursor: Request(95) {
		pub cursor: Cursor,
	}

	/// Changes the color of the given `cursor`.
	///
	/// If the `cursor` is currently being displayed on a screen, the change is
	/// visible immediately.
	///
	/// # Errors
	/// - [`Cursor`]
	///
	/// [`Cursor`]: crate::x11::error::Cursor
	pub struct RecolorCursor: Request(96) {
		pub cursor: Cursor,
		/// The tint to apply to the cursor's foreground.
		///
		/// This is in RGB format (i.e. `(red, green, blue)`).
		pub foreground_color: (u16, u16, u16),
		/// The tint to apply to the cursor's background.
		///
		/// This is in RGB format (i.e. `(red, green, blue)`).
		pub background_color: (u16, u16, u16),
	}

	/// Gets the closest ideal size to the given `width` and `height`.
	///
	/// For [`Cursor`], this is the largest size that can be fully displayed
	/// within `width` and `height`. For [`Tile`], this is the size that can be
	/// tiled fastest. For [`Stipple`], this is the size that can be stippled
	/// fastest.
	///
	/// # Errors
	/// - [`Drawable`]
	/// - [`Match`] -- Generated if an [`InputOnly`] [window] is used with the
	///   [`Tile`] or [`Stipple`] classes.
	/// - [`Value`]
	///
	/// # Reply
	/// This request generates a [`QueryBestSizeReply`].
	///
	/// [`Cursor`]: query_best_size::Class::Cursor
	/// [`Tile`]: query_best_size::Class::Tile
	/// [`Stipple`]: query_best_size::Class::Stipple
	/// [`Drawable`]: crate::x11::error::Drawable
	/// [`Match`]: crate::x11::error::Match
	/// [`Value`]: crate::x11::error::Value
	/// [window]: Window
	/// [`InputOnly`]: WindowClass::InputOnly
	pub struct QueryBestSize: Request(97) -> QueryBestSizeReply {
		/// The 'type' of 'best size' being queried.
		#[metabyte]
		pub class: QueryBestSizeClass,

		/// Indicates the desired screen.
		///
		/// For [`Tile`] and [`Stipple`], the `drawable` indicates the screen
		/// and also possibly the window class and depth.
		///
		/// An [`InputOnly`] [`Window`] cannot be used as the drawable for
		/// [`Tile`] or [`Stipple`], else a [`Match`] error occurs.
		///
		/// [`Tile`]: query_best_size::Class::Tile
		/// [`Stipple`]: query_best_size::Class::Stipple
		/// [`InputOnly`]: query_best_size::Class::InputOnly
		pub drawable: Drawable,
		/// The given width to find an ideal size for.
		pub width: u16,
		/// The given height to find an ideal size for.
		pub height: u16,
	}

	/// The reply for the [`QueryBestSize`] request.
	///
	/// This contains the closest ideal size to the `width` and `height` that
	/// was given in the [`QueryBestSize`] request. See the request's docs for
	/// more information.
	pub struct QueryBestSizeReply: Reply for QueryBestSize {
		#[sequence]
		pub sequence: u16,

		/// The width of the ideal size found.
		pub width: u16,
		/// The height of the ideal size found.
		pub height: u16,
		[_; ..],
	}

	pub struct QueryExtension: Request(98) -> QueryExtensionReply {
		let name_len: u16 = name => name.len() as u16,
		[_; 2],

		#[context(name_len => *name_len as usize)]
		pub name: String8,
		[_; ..],
	}

	pub struct QueryExtensionReply: Reply for QueryExtension {
		#[metabyte]
		pub present: bool,
		#[sequence]
		pub sequence: u16,

		pub major_opcode: u8,
		pub first_event: u8,
		pub first_error: u8,
		[_; ..],
	}

	pub struct ListExtensions: Request(99) -> ListExtensionsReply;

	pub struct ListExtensionsReply: Reply for ListExtensions {
		#[metabyte]
		let names_len: u8 = names => names.len() as u8,
		#[sequence]
		pub sequence: u16,

		[_; 24],

		#[context(names_len => *names_len as usize)]
		pub names: Vec<LenString8>,
		[_; ..],
	}

	// The `ChangeKeyboardMapping` and `GetKeyboardMapping` requests, as well as
	// the `GetKeyboardMappingReply`, used a special format for the size of
	// their lists of keysyms, and so have to be done manually. They can be
	// found in the `mod keyboard_mapping;` module.

	// The `ChangeKeyboardControl` request uses a special format for its values
	// list, so it has to be done manually. It can be found in the
	// `mod change_keyboard_control;` module.

	pub struct GetKeyboardControl: Request(103) -> GetKeyboardControlReply;

	pub struct GetKeyboardControlReply: Reply for GetKeyboardControl {
		#[metabyte]
		pub global_auto_repeat: bool,
		#[sequence]
		pub sequence: u16,

		pub led_mask: u32,
		pub key_click_percent: u8,
		pub bell_percent: u8,
		pub bell_pitch: u16,
		pub bell_duration: u16,
		[_; 2],

		pub auto_repeats: [u8; 32],
	}

	pub struct Bell: Request(104) {
		#[metabyte]
		pub percent: i8,
	}

	pub struct ChangePointerControl: Request(105) {
		pub acceleration_numerator: i16,
		pub acceleration_denominator: i16,
		pub threshold: i16,
		pub accelerate: bool,
		pub enable_threshold: bool,
	}

	pub struct GetPointerControl: Request(106) -> GetPointerControlReply;

	pub struct GetPointerControlReply: Reply for GetPointerControl {
		#[sequence]
		pub sequence: u16,

		pub acceleration_numerator: i16,
		pub acceleration_denominator: u16,
		pub threshold: u16,
		[_; ..],
	}

	pub struct SetScreenSaver: Request(107) {
		pub timeout: i16,
		pub interval: i16,
		pub prefer_blanking: Defaultable<bool>,
		pub allow_exposures: Defaultable<bool>,
		[_; ..],
	}

	pub struct GetScreenSaver: Request(108) -> GetScreenSaverReply;

	pub struct GetScreenSaverReply: Reply for GetScreenSaver {
		#[sequence]
		pub sequence: u16,

		pub timeout: i16,
		pub interval: i16,
		pub prefer_blanking: bool,
		pub allow_exposures: bool,
		[_; ..],
	}

	pub struct ChangeHosts: Request(109) {
		#[metabyte]
		pub mode: EditMode,

		pub family: HostFamilyA,
		[_; 1],

		let address_len: u16 = address => address.len() as u16,
		#[context(address_len => *address_len as usize)]
		pub address: Vec<u8>,
		[_; ..],
	}

	pub struct ListHosts: Request(110) -> ListHostsReply;

	pub struct ListHostsReply: Reply for ListHosts {
		#[metabyte]
		pub enabled: bool,
		#[sequence]
		pub sequence: u16,

		let hosts_len: u16 = hosts => hosts.len() as u16,
		[_; 22],

		pub hosts: Vec<Host>,
	}

	pub struct SetAccessControl: Request(111) {
		#[metabyte]
		pub enabled: bool,
	}

	pub struct SetCloseDownMode: Request(112) {
		#[metabyte]
		pub mode: CloseDownMode,
	}

	//pub struct KillClient(113): pub resource: AllTemp<u32>;

	pub struct RotateProperties: Request(114) {
		pub target: Window,
		let properties_len: u16 = properties => properties.len() as u16,
		pub delta: i16,
		#[context(properties_len => *properties_len as usize)]
		pub properties: Vec<Atom>,
	}

	pub struct ForceScreenSaver: Request(115) {
		#[metabyte]
		pub mode: ScreenSaverMode,
	}

	pub struct SetPointerMapping: Request(116) -> SetPointerMappingReply {
		#[metabyte]
		let map_len: u8 = map => map.len() as u16,

		#[context(map_len => *map_len as usize)]
		pub map: Vec<u8>,
		[_; ..],
	}

	pub struct SetPointerMappingReply: Reply for SetPointerMapping<'_> {
		#[metabyte]
		pub status: Status,
		#[sequence]
		pub sequence: u16,

		[_; ..],
	}

	pub struct GetPointerMapping: Request(117) -> GetPointerMappingReply;

	pub struct GetPointerMappingReply: Reply for GetPointerMapping {
		#[metabyte]
		let map_len: u8 = map => map.len() as u8,
		#[sequence]
		pub sequence: u16,

		[_; 24],

		#[context(map_len => *map_len as usize)]
		pub map: Vec<u8>,
		[_; ..],
	}

	// `SetModifierMapping` and `GetModifierMappingReply` both use a special
	// format for the list of keycodes, so the `SetModifierMapping` request,
	// the `GetModifierMapping` request, the `SetModifierMappingReply`, and the
	// `GetModifierMappingReply` messages are contained in the
	// `mod modifier_mappings;` module.

	// The `NoOperation` request uses a unique variable unused bytes length
	// format, so it has to be done manually. It is therefore found in the
	// `mod no_operation;` module.
}

// TODO: tests
