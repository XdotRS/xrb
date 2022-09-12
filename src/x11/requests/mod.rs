// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::x11::*;
use xrb_proc_macros::messages;

messages! {
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
	/// [Alloc]: crate::x11::errors::Alloc
	/// [Colormap]: crate::x11::errors::Colormap
	/// [Cursor]: crate::x11::errors::Cursor
	/// [IdChoice]: crate::x11::errors::IdChoice
	/// [Match]: crate::x11::errors::Match
	/// [Pixmap]: crate::x11::errors::Pixmap
	/// [Value]: crate::x11::errors::Value
	/// [Window]: crate::x11::errors::Window
	pub struct CreateWindow<'a>(1) {
		/// The resource ID given to the window.
		pub window_id: Window,
		/// The parent of which the window will be created as a child of.
		pub parent: Window,
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
		/// [`Match`]: crate::x11::errors::Match
		pub class: Inheritable<WindowClass>,
		/// The color depth of the window in bits per pixel.
		///
		/// If the class is not [`InputOnly`], [`CopyFromParent`] will copy the
		/// `depth` from the parent. __If the class is [`InputOnly`], this must
		/// be set to [`CopyFromParent`]__, else a [`Match`] error shall occur.
		///
		/// [`InputOnly`]: WindowClass::InputOnly
		/// [`CopyFromParent`]: Inherit::CopyFromParent
		/// [`Match`]: crate::x11::errors::Match
		pub $depth: Inheritable<u8>,
		pub visual: Inheritable<VisualId>,
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
		pub value_mask: AttributeMask,
		/// A list of [window attributes] that are to configured for the window.
		///
		/// [window attributes]: Attribute
		pub values: &'a [Attribute], // Window is a placeholder until WinAttr is done
	}

	pub struct ChangeWindowAttributes<'a>(2) {
		pub target: Window,
		pub value_mask: AttributeMask,
		pub values: &'a [Attribute],
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
		pub map_state: MapState,
		pub override_redirect: bool,
		pub colormap: Option<Colormap>,
		pub all_event_masks: EventMask,
		pub your_event_mask: EventMask,
		pub do_not_propagate_mask: DeviceEventMask,
		[(); 2],
	}

	pub struct DestroyWindow(4): pub target: Window;
	pub struct DestroySubwindows(5): pub target: Window;

	pub struct ChangeSaveSet(6) {
		pub $mode: EditMode,
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

	pub struct ConfigureWindow<'a>(12) {
		pub target: Window,
		pub value_mask: ConfigureWindowMask,
		pub values: &'a [ConfigureWindowValue],
	}

	pub struct CirculateWindow(13) {
		pub $direction: CirculateDirection,
		pub target: Window,
	}

	pub struct GetGeometry(14) -> GetGeometryReply: pub target: Box<dyn Drawable>;

	pub struct GetGeometryReply for GetGeometry {
		pub $depth: u8,
		pub root: Window,
		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,
		pub border_width: u16,
		[(); 10],
	}

	pub struct QueryTree(15) -> QueryTreeReply: pub target: Window;

	pub struct QueryTreeReply for QueryTree {
		pub root: Window,
		pub parent: Option<Window>,
		#children: u16,
		[(); 14],
		pub children: Vec<Window>,
	}

	pub struct InternAtom(16) -> InternAtomReply {
		pub $only_if_exists: bool,
		#name: u16,
		[(); 2],
		pub name: String8,
		[(); {name}],
	}

	pub struct InternAtomReply for InternAtom {
		pub atom: Option<Atom>,
		[(); 20],
	}

	pub struct GetAtomName(17) -> GetAtomNameReply: pub atom: Atom;

	pub struct GetAtomNameReply for GetAtomName {
		#name: u16,
		[(); 22],
		pub name: String8,
		[(); {name}],
	}

	// The property requests (`ChangeProperty(18)`, `DeleteProperty(19)`,
	// `GetProperty(20)`, and `ListProperties(21)`) are special cases and need
	// to be defined manually. You can find them in `mod properties;`.

	pub struct SetSelectionOwner(22) {
		pub $owner: Option<Window>,
		pub selection: Atom,
		pub time: Time,
	}

	pub struct GetSelectionOwner(23) -> GetSelectionOwnerReply {
		pub selection: Atom,
	}

	pub struct GetSelectionOwnerReply for GetSelectionOwner {
		pub owner: Option<Window>,
		[(); 20],
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
		//pub event: Box<dyn Event>,
	}

	pub struct GrabPointer(26) -> GrabPointerReply {
		pub $owner_events: bool,
		pub target_window: Window,
		pub event_mask: PointerEventMask,
		pub pointer_mode: GrabMode,
		pub keyboard_mode: GrabMode,
		pub confine_to: Option<Window>,
		pub cursor_override: Option<Cursor>,
		pub time: Time,
	}

	pub struct GrabPointerReply for GrabPointer {
		pub $status: GrabStatus,
		[(); 24],
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
		pub button: Any<Button>,
		(),
		pub modifiers: AnyModifierKeyMask,
	}

	pub struct UngrabButton(29) {
		pub $button: Any<Button>,
		pub target_window: Window,
		[(); 2],
	}

	pub struct ChangeActivePointerGrab(30) {
		pub cursor_override: Option<Cursor>,
		pub time: Time,
		pub event_mask: PointerEventMask,
		[(); 2],
	}

	pub struct GrabKeyboard(31) -> GrabKeyboardReply {
		pub $owner_events: bool,
		pub target_window: Window,
		pub time: Time,
		pub pointer_mode: GrabMode,
		pub keyboard_mode: GrabMode,
		[(); 2],
	}

	pub struct GrabKeyboardReply for GrabKeyboard {
		pub $status: GrabStatus,
		[(); 24],
	}

	pub struct UngrabKeyboard(32): pub time: Time;

	pub struct GrabKey(33) {
		pub $owner_events: bool,
		pub target_window: Window,
		pub modifiers: AnyModifierKeyMask,
		pub key: Any<Keycode>,
		pub pointer_mode: GrabMode,
		pub keyboard_mode: GrabMode,
		[(); 3],
	}

	pub struct UngrabKey(34) {
		pub $key: Any<Keycode>,
		pub target_window: Window,
		pub modifiers: AnyModifierKeyMask,
		[(); 2],
	}

	pub struct AllowEvents(35) {
		pub $mode: AllowEventsMode,
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
		pub mask: ModifierMask,
		[(); 6],
	}

	pub struct GetMotionEvents(39) -> GetMotionEventsReply {
		pub target: Window,
		pub start: Time,
		pub stop: Time,
	}

	pub struct GetMotionEventsReply for GetMotionEvents {
		#events: u32,
		[(); 20],
		pub events: Vec<(Timestamp, (i16, i16))>,
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
		[(); 16],
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
		//pub $revert_to: Option<RevertTo>,
		pub focus: Option<InputFocus>,
		pub time: Time,
	}

	pub struct GetInputFocus(43) -> GetInputFocusReply;

	pub struct GetInputFocusReply for GetInputFocus {
		pub $revert_to: RevertTo,
		pub focus: Option<InputFocus>,
		[(); 20],
	}

	pub struct QueryKeymap(44) -> QueryKeymapReply;

	pub struct QueryKeymapReply for QueryKeymap {
		pub keys: [u8; 32],
	}

	pub struct OpenFont(45) {
		pub font_id: Font,
		#name: u16,
		[(); 2],
		pub name: String8,
		[(); {name}],
	}

	pub struct CloseFont(46): pub font: Font;

	pub struct QueryFont<'a>(47) -> QueryFontReply: pub font: &'a dyn Fontable;

	pub struct QueryFontReply for QueryFont<'_> {
		pub min_bounds: CharInfo,
		[(); 4],
		pub max_bounds: CharInfo,
		[(); 4],
		pub min_char_or_byte2: u16,
		pub max_char_or_byte2: u16,
		#properties: u16,
		pub draw_direction: DrawDirection,
		pub min_byte1: u8,
		pub max_byte1: u8,
		pub all_chars_exist: bool,
		pub font_ascent: i16,
		pub font_descent: i16,
		#charinfos: u32,
		pub properties: Vec<FontProperty>,
		pub charinfos: Vec<CharInfo>,
	}

	pub struct QueryTextExtents(48) -> QueryTextExtentsReply {
		pub $odd_length: bool,
		pub font: Box<dyn Fontable>,
		pub string: String16,
		[(); {string}],
	}

	pub struct QueryTextExtentsReply for QueryTextExtents {
		pub $draw_direction: DrawDirection,
		pub font_ascent: i16,
		pub font_descent: i16,
		pub overall_ascent: i16,
		pub overall_descent: i16,
		pub overall_width: i32,
		pub overall_left: i32,
		pub overall_right: i32,
		[(); 4],
	}

	pub struct ListFonts(49) -> ListFontsReply {
		pub max_names: u16,
		#pattern: u16,
		pub pattern: String8,
		[(); {pattern}],
	}

	pub struct ListFontsReply for ListFonts {
		#names: u32,
		[(); 22],
		pub names: Vec<LenString8>,
		[(); {names}],
	}

	// ListFontsWithInfo has a special format for its reply that needs to be
	// done manually, so both the request and the reply are contained within the
	// `mod list_fonts_with_info;` module.

	pub struct SetFontPath<'a>(51) {
		#path: u16,
		[(); 2],
		pub path: &'a [LenString8],
		[(); {path}],
	}

	// GetFontPath has a special format for its request. Both the request and
	// the reply are done manually and can be found in the `mod get_font_path;`
	// module.

	pub struct CreatePixmap<'a>(53) {
		pub $depth: u8,
		pub pixmap_id: Pixmap,
		pub drawable: &'a dyn Drawable,
		pub width: u16,
		pub height: u16,
	}

	pub struct FreePixmap(54): pub pixmap: Pixmap;

	pub struct CreateGraphicsContext<'a>(55) {
		pub context_id: GraphicsContext,
		pub drawable: &'a dyn Drawable,
		pub value_mask: GraphicsContextMask,
		pub values: &'a [GraphicsContextValue],
	}

	pub struct ChangeGraphicsContext<'a>(56) {
		pub context: GraphicsContext,
		pub value_mask: GraphicsContextMask,
		pub values: &'a [GraphicsContextValue],
	}

	pub struct CopyGraphicsContext(57) {
		pub source: GraphicsContext,
		pub destination: GraphicsContext,
		pub value_mask: GraphicsContextMask,
	}

	pub struct SetDashes<'a>(58) {
		pub context: GraphicsContext,
		pub dash_offset: u16,
		#dashes: u16,
		pub dashes: &'a [u8],
		[(); {dashes}],
	}

	pub struct SetClipRectangles<'a>(59) {
		pub $ordering: Ordering,
		pub context: GraphicsContext,
		pub clip_x_origin: i16,
		pub clip_y_origin: i16,
		pub rectangles: &'a [Rectangle],
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

	pub struct CopyArea<'a>(62) {
		pub source: &'a dyn Drawable,
		pub destination: &'a dyn Drawable,
		pub context: GraphicsContext,
		pub src_x: i16,
		pub src_y: i16,
		pub dest_x: i16,
		pub dest_y: i16,
		pub width: u16,
		pub height: u16,
	}

	pub struct CopyPlane<'a>(63) {
		pub source: &'a dyn Drawable,
		pub destination: &'a dyn Drawable,
		pub context: GraphicsContext,
		pub src_x: i16,
		pub src_y: i16,
		pub dest_x: i16,
		pub dest_y: i16,
		pub width: u16,
		pub height: u16,
		pub bit_plane: u32,
	}

	pub struct PolyPoint<'a>(64) {
		pub $coordinate_mode: CoordinateMode,
		pub drawable: &'a dyn Drawable,
		pub context: GraphicsContext,
		pub points: &'a [(i16, i16)],
	}

	pub struct PolyLine<'a>(65) {
		pub $coordinate_mode: CoordinateMode,
		pub drawable: &'a dyn Drawable,
		pub context: GraphicsContext,
		pub points: &'a [(i16, i16)],
	}

	pub struct PolySegment<'a>(66) {
		pub drawable: &'a dyn Drawable,
		pub context: GraphicsContext,
		pub segments: &'a [Segment],
	}

	pub struct PolyRectangle<'a>(67) {
		pub drawable: &'a dyn Drawable,
		pub context: GraphicsContext,
		pub rectangles: &'a [Rectangle],
	}

	pub struct PolyArc<'a>(68) {
		pub drawable: &'a dyn Drawable,
		pub context: GraphicsContext,
		pub arcs: &'a [GeomArc],
	}

	pub struct FillPoly<'a>(69) {
		pub drawable: &'a dyn Drawable,
		pub context: GraphicsContext,
		pub shape: Shape,
		pub coordinate_mode: CoordinateMode,
		[(); 2],
		pub points: &'a [(i16, i16)],
	}

	pub struct PolyFillRectangle<'a>(70) {
		pub drawable: &'a dyn Drawable,
		pub context: GraphicsContext,
		pub rectangles: &'a [Rectangle],
	}

	pub struct PolyFillArc<'a>(71) {
		pub drawable: &'a dyn Drawable,
		pub context: GraphicsContext,
		pub arcs: &'a [GeomArc],
	}

	pub struct PutImage<'a>(72) {
		pub $format: BitmapFormat,
		pub drawable: &'a dyn Drawable,
		pub context: GraphicsContext,
		pub width: u16,
		pub height: u16,
		pub dest_x: i16,
		pub dest_y: i16,
		pub left_padding: u8,
		pub depth: u8,
		[(); 2],
		pub data: &'a [u8],
		[(); {data}],
	}

	pub struct GetImage<'a>(73) -> GetImageReply {
		pub $format: Format,
		pub drawable: &'a dyn Drawable,
		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,
		pub plane_mask: u32,
	}

	pub struct GetImageReply for GetImage<'_> {
		pub $depth: u8,
		pub visual: Option<VisualId>,
		[(); 20],
		pub data: Vec<u8>,
		[(); {data}],
	}

	pub struct PolyText8<'a>(74) {
		pub drawable: &'a dyn Drawable,
		pub context: GraphicsContext,
		pub x: i16,
		pub y: i16,
		//pub items: &'a [TextItem8], // TODO: TextItem8 and TextItem16 need to be done separately
		//[(); {items}],
	}

	pub struct PolyText16<'a>(75) {
		pub drawable: &'a dyn Drawable,
		pub context: GraphicsContext,
		pub x: i16,
		pub y: i16,
		//pub items: [TextItem16], // TODO: TextItem8 and TextItem16 need to be done separately
		//[(); {items}],
	}

	pub struct ImageText8<'a>(76) {
		pub drawable: &'a dyn Drawable,
		pub context: GraphicsContext,
		pub x: i16,
		pub y: i16,
		pub string: String8,
		[(); {string}],
	}

	pub struct ImageText16<'a>(77) {
		pub drawable: &'a dyn Drawable,
		pub context: GraphicsContext,
		pub x: i16,
		pub y: i16,
		pub string: String16,
		[(); {string}],
	}

	pub struct CreateColormap(78) {
		pub $alloc: ColormapAlloc,
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
		#colormaps: u16,
		[(); 22],
		pub colormaps: Vec<Colormap>,
	}

	pub struct AllocColor(84) -> AllocColorReply {
		pub colormap: Colormap,
		pub color: (u16, u16, u16),
		[(); 2],
	}

	pub struct AllocColorReply for AllocColor {
		pub color: (u16, u16, u16),
		[(); 2],
		pub pixel: u32,
		[(); 12],
	}

	pub struct AllocNamedColor(85) -> AllocNamedColorReply {
		pub colormap: Colormap,
		#name: u16,
		[(); 2],
		pub name: String8,
		[(); {name}],
	}

	pub struct AllocNamedColorReply for AllocNamedColor {
		pub pixel: u32,
		pub exact_color: (u16, u16, u16),
		pub visual_color: (u16, u16, u16),
		[(); 8],
	}

	pub struct AllocColorCells(86) -> AllocColorCellsReply {
		pub $contiguous: bool,
		pub colormap: Colormap,
		pub num_colors: u16, // TODO: its just called `colors`... is it the number?
		pub planes: u16,
	}

	pub struct AllocColorCellsReply for AllocColorCells {
		#pixels: u16,
		#masks: u16,
		[(); 20],
		pub pixels: Vec<u32>,
		pub masks: Vec<u32>,
	}

	pub struct AllocColorPlanes(87) -> AllocColorPlanesReply {
		pub $contiguous: bool,
		pub colormap: Colormap,
		pub num_colors: u16, // TODO: its just called `colors`... is it the number?
		pub colors: (u16, u16, u16),
	}

	pub struct AllocColorPlanesReply for AllocColorPlanes {
		#pixels: u16,
		[(); 2],
		pub color_mask: (u16, u16, u16),
		[(); 8],
		pub pixels: Vec<u32>,
	}

	pub struct FreeColors<'a>(88) {
		pub colormap: Colormap,
		pub plane_mask: u32,
		pub pixels: &'a [u32],
	}

	pub struct StoreColors(89) {
		pub colormap: Colormap,
		//pub items: [ColorItem], // ColorItems need to be done separately
	}

	pub struct StoreNamedColor(90) {
		pub $channel_mask: ColorChannelMask,
		pub colormap: Colormap,
		pub pixel: u32,
		#name: u16,
		[(); 2],
		pub name: String8,
		[(); {name}],
	}

	// The QueryColorsReply for the QueryColors request uses a special format
	// for its list of colors, and so the reply must be done manually. The
	// reply and request have been put in `mod query_colors;`.

	pub struct LookupColor(92) -> LookupColorReply {
		pub colormap: Colormap,
		#name: u16,
		[(); 2],
		pub name: String8,
		[(); {name}],
	}

	pub struct LookupColorReply for LookupColor {
		pub exact_color: (u16, u16, u16),
		pub visual_color: (u16, u16, u16),
		[(); 12],
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

	/// Changes the color of the given `cursor`.
	///
	/// If the `cursor` is currently being displayed on a screen, the change is
	/// visible immediately.
	///
	/// # Errors
	/// - [`Cursor`]
	///
	/// [`Cursor`]: crate::x11::errors::Cursor
	pub struct RecolorCursor(96) {
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
	/// [`Drawable`]: crate::x11::errors::Drawable
	/// [`Match`]: crate::x11::errors::Match
	/// [`Value`]: crate::x11::errors::Value
	/// [window]: Window
	/// [`InputOnly`]: WindowClass::InputOnly
	pub struct QueryBestSize<'a>(97) -> QueryBestSizeReply {
		/// The 'type' of 'best size' being queried.
		pub $class: QueryBestSizeClass,
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
		pub drawable: &'a dyn Drawable,
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
	pub struct QueryBestSizeReply for QueryBestSize<'_> {
		/// The width of the ideal size found.
		pub width: u16,
		/// The height of the ideal size found.
		pub height: u16,
		[(); 20],
	}

	pub struct QueryExtension(98) -> QueryExtensionReply {
		#name: u16,
		[(); 2],
		pub name: String8,
		[(); {name}],
	}

	pub struct QueryExtensionReply for QueryExtension {
		pub $present: bool,
		pub major_opcode: u8,
		pub first_event: u8,
		pub first_error: u8,
		[(); 20],
	}

	pub struct ListExtensions(99) -> ListExtensionsReply;

	pub struct ListExtensionsReply for ListExtensions {
		$#names: u8,
		[(); 24],
		pub names: Vec<LenString8>,
		[(); {names}],
	}

	// The `ChangeKeyboardMapping` and `GetKeyboardMapping` requests, as well as
	// the `GetKeyboardMappingReply`, used a special format for the size of
	// their lists of keysyms, and so have to be done manually. They can be
	// found in the `mod keyboard_mapping;` module.

	// The `ChangeKeyboardControl` request uses a special format for its values
	// list, so it has to be done manually. It can be found in the
	// `mod change_keyboard_control;` module.

	pub struct GetKeyboardControl(103) -> GetKeyboardControlReply;

	pub struct GetKeyboardControlReply for GetKeyboardControl {
		pub $global_auto_repeat: bool,
		pub led_mask: u32,
		pub key_click_percent: u8,
		pub bell_percent: u8,
		pub bell_pitch: u16,
		pub bell_duration: u16,
		[(); 2],
		pub auto_repeats: [u8; 32],
	}

	pub struct Bell(104): pub $percent: i8;

	pub struct ChangePointerControl(105) {
		pub acceleration_numerator: i16,
		pub acceleration_denominator: i16,
		pub threshold: i16,
		pub accelerate: bool,
		pub enable_threshold: bool,
	}

	pub struct GetPointerControl(106) -> GetPointerControlReply;

	pub struct GetPointerControlReply for GetPointerControl {
		pub acceleration_numerator: i16,
		pub acceleration_denominator: u16,
		pub threshold: u16,
		[(); 18],
	}

	pub struct SetScreenSaver(107) {
		pub timeout: i16,
		pub interval: i16,
		pub prefer_blanking: Defaultable<bool>,
		pub allow_exposures: Defaultable<bool>,
		[(); 2],
	}

	pub struct GetScreenSaver(108) -> GetScreenSaverReply;

	pub struct GetScreenSaverReply for GetScreenSaver {
		pub timeout: i16,
		pub interval: i16,
		pub prefer_blanking: bool,
		pub allow_exposures: bool,
		[(); 18],
	}

	pub struct ChangeHosts<'a>(109) {
		pub $mode: EditMode,
		pub family: HostFamilyA,
		[(); 1],
		#address: u16,
		pub address: &'a [u8],
		[(); {address}],
	}

	pub struct ListHosts(110) -> ListHostsReply;

	pub struct ListHostsReply for ListHosts {
		pub $enabled: bool,
		#hosts: u16,
		[(); 22],
		pub hosts: Vec<Host>,
	}

	pub struct SetAccessControl(111): pub $enabled: bool;

	pub struct SetCloseDownMode(112): pub $mode: CloseDownMode;

	//pub struct KillClient(113): pub resource: AllTemp<u32>;

	pub struct RotateProperties<'a>(114) {
		pub target: Window,
		#properties: u16,
		pub delta: i16,
		pub properties: &'a [Atom],
	}

	pub struct ForceScreenSaver(115): pub $mode: ScreenSaverMode;

	pub struct SetPointerMapping<'a>(116) -> SetPointerMappingReply {
		$#map: u8,
		pub map: &'a [u8],
		[(); {map}],
	}

	pub struct SetPointerMappingReply for SetPointerMapping<'_> {
		pub $status: Status,
		[(); 24],
	}

	pub struct GetPointerMapping(117) -> GetPointerMappingReply;

	pub struct GetPointerMappingReply for GetPointerMapping {
		$#map: u8,
		[(); 24],
		pub map: Vec<u8>,
		[(); {map}],
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

#[cfg(test)]
mod tests {
	use crate::traits::*;
	use super::*;

	#[test]
	fn grab_button_major_opcode_is_correct() {
		assert_eq!(GrabButton::major_opcode(), 28);
	}

	#[test]
	fn convert_selection_length_is_correct() {
		let convert_selection = ConvertSelection {
			requestor: Window::new(0),
			selection: Atom::new(0),
			target: Atom::new(0),
			property: None,
			time: Time::Current,
		};

		assert_eq!(convert_selection.length(), 6);
	}

	#[test]
	fn grab_pointer_length_is_correct() {
		let grab_pointer = GrabPointer {
			owner_events: false,
			target_window: Window::new(0),
			event_mask: PointerEventMask::empty(),
			pointer_mode: GrabMode::Asynchronous,
			keyboard_mode: GrabMode::Asynchronous,
			confine_to: None,
			cursor_override: None,
			time: Time::Current,
		};

		assert_eq!(grab_pointer.length(), 6);
	}

	#[test]
	fn grab_pointer_reply_length_is_correct() {
		let grab_pointer_reply = GrabPointerReply {
			__sequence: 0,
			__major_opcode: None,
			__minor_opcode: None,
			status: GrabStatus::Success,
		};

		assert_eq!(grab_pointer_reply.length(), 0);
	}

	#[test]
	fn grab_server_length_is_correct() {
		let grab_server = GrabServer {};

		assert_eq!(grab_server.length(), 1);
	}
}
