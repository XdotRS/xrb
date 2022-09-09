// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::x11::common::*;
use crate::x11::id::atoms::Atom;
use crate::x11::id::*;

use xrb_proc_macros::{messages, ByteSize, StaticByteSize};

/// A request is a message sent from an X client to the X server.
///
/// A request may have a specific reply associated with it. That reply is
/// indicated by `T`.
pub trait Request<T = ()> {
	/// The major opcode that uniquely identifies this request or extension.
	///
	/// X core protocol requests have unique major opcodes, but each extension
	/// is only assigned one major opcode. Extensions are assigned major opcodes
	/// from 127 through to 255.
	fn major_opcode() -> u8;

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

/// A reply is a message sent from the X server to an X client in response to a
/// request.
///
/// The request associated with a reply is indicated by `T`.
pub trait Reply<T>
where
	T: Request<Self>,
	Self: Sized,
{
	/// The sequence number associated with the request that this reply is for.
	///
	/// Every request on a given connection is assigned a sequence number when
	/// it is sent, starting with one. This sequence number can therefore be
	/// used to keep track of exactly which request generated this reply.
	fn sequence(&self) -> u16;
	/// The major opcode, if any, associated with the request that generated
	/// this reply.
	fn major_opcode(&self) -> Option<u8>;
	/// The minor opcode, if any, associated with the request that generated
	/// this reply.
	fn minor_opcode(&self) -> Option<u8>;
	/// The length of this reply in 4-byte units minus 8.
	///
	/// Every reply always consists of 32 bytes followed by zero or more
	/// additional bytes of data; this method indicates the number of additional
	/// bytes of data within this reply.
	///
	/// |'Actual' length in bytes|`length()`|
	/// |------------------------|----------|
	/// |32                      |0         |
	/// |36                      |1         |
	/// |40                      |2         |
	/// |44                      |3         |
	/// |...                     |...       |
	/// |`32 + 4n`               |`n`       |
	fn length(&self) -> u32;
}

#[derive(StaticByteSize, ByteSize)]
pub enum WindowClass {
	InputOutput = 1,
	InputOnly = 2,
}

// TODO: docs
/// See also: [`AttributeMask`]
///
/// [`AttributeMask`]: crate::x11::common::masks::AttributeMask
#[derive(StaticByteSize, ByteSize)]
pub enum Attribute {
	BackgroundPixmap(Option<Relative<Pixmap>>),
	BackgroundPixel(u32),
	BorderPixmap(Inherit<Pixmap>),
	BorderPixel(u32),
	BitGravity(BitGravity),
	WinGravity(WinGravity),
	BackingStore(BackingStore),
	BackingPlanes(u32),
	BackingPixel(u32),
	OverrideRedirect(bool),
	SaveUnder(bool),
	EventMask(EventMask),
	DoNotPropagateMask(DeviceEventMask),
	Colormap(Inherit<Colormap>),
	Cursor(Option<Cursor>),
}

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
		pub class: Inherit<WindowClass>,
		/// The color depth of the window in bits per pixel.
		///
		/// If the class is not [`InputOnly`], [`CopyFromParent`] will copy the
		/// `depth` from the parent. __If the class is [`InputOnly`], this must
		/// be set to [`CopyFromParent`]__, else a [`Match`] error shall occur.
		///
		/// [`InputOnly`]: WindowClass::InputOnly
		/// [`CopyFromParent`]: Inherit::CopyFromParent
		/// [`Match`]: crate::x11::errors::Match
		pub $depth: Inherit<u8>,
		pub visual: Inherit<VisualId>,
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
}

#[derive(StaticByteSize, ByteSize)]
pub enum MapState {
	Unmapped,
	Unviewable,
	Viewable,
}

#[derive(StaticByteSize, ByteSize)]
pub enum BackingStore {
	NotUseful,
	WhenMapped,
	Always,
}

messages! {
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
}

pub mod change_save_set {
	use xrb_proc_macros::{ByteSize, StaticByteSize};

	#[derive(StaticByteSize, ByteSize)]
	pub enum Mode {
		Insertl,
		Delete,
	}
}

messages! {
	pub struct ChangeSaveSet(6) {
		pub $mode: change_save_set::Mode,
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
		//pub value_mask: ConfigureWindowMask,
		//pub values: &'a [ConfigureWindowValue],
	}
}

pub mod circulate_window {
	use xrb_proc_macros::{ByteSize, StaticByteSize};

	#[derive(StaticByteSize, ByteSize)]
	pub enum Direction {
		RaiseLowest,
		RaiseHighest,
	}
}

messages! {
	pub struct CirculateWindow(13) {
		pub $direction: circulate_window::Direction,
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
}

#[derive(StaticByteSize, ByteSize)]
pub enum GrabMode {
	Synchronous,
	Asynchronous,
}

#[derive(StaticByteSize, ByteSize)]
pub enum GrabStatus {
	Success,
	AlreadyGrabbed,
	InvalidTime,
	NotViewable,
	Frozen,
}

messages! {
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
		pub button: Specificity<Button>,
		(),
		pub modifiers: AnyModifierKeyMask,
	}

	pub struct UngrabButton(29) {
		pub $button: Specificity<Button>,
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
		pub key: Specificity<Keycode>,
		pub pointer_mode: GrabMode,
		pub keyboard_mode: GrabMode,
		[(); 3],
	}

	pub struct UngrabKey(34) {
		pub $key: Specificity<Keycode>,
		pub target_window: Window,
		pub modifiers: AnyModifierKeyMask,
		[(); 2],
	}
}

#[derive(StaticByteSize, ByteSize)]
pub enum AllowEventsMode {
	AsyncPointer,
	SyncPointer,
	ReplayPointer,
	AsyncKeyboard,
	SyncKeyboard,
	ReplayKeyboard,
	AsyncBoth,
	SyncBoth,
}

messages! {
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
		pub focus: Option<Focus>,
		pub time: Time,
	}

	pub struct GetInputFocus(43) -> GetInputFocusReply;

	pub struct GetInputFocusReply for GetInputFocus {
		//pub $revert_to: Option<RevertTo>,
		pub focus: Option<Focus>,
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
}

#[derive(StaticByteSize, ByteSize)]
pub enum DrawDirection {
	LeftToRight,
	RightToLeft,
}

#[derive(StaticByteSize, ByteSize)]
pub struct FontProperty {
	pub name: Atom,
	pub value: u32,
}

#[derive(StaticByteSize, ByteSize)]
pub struct CharInfo {
	pub left_side_bearing: i16,
	pub right_side_bearing: i16,
	pub character_width: i16,
	pub ascent: i16,
	pub descent: i16,
	pub attributes: u16,
}

messages! {
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
		//pub values: &'a [GraphicsContextValue],
	}

	pub struct ChangeGraphicsContext(56) {
		pub context: GraphicsContext,
		pub value_mask: GraphicsContextMask,
		//pub values: &'a [GraphicsContextValue],
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
}

#[derive(StaticByteSize, ByteSize)]
pub enum Ordering {
	Unsorted,
	Ysorted,
	YxSorted,
	YxBanded,
}

messages! {
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
}

#[derive(StaticByteSize, ByteSize)]
pub enum CoordinateMode {
	Origin,
	Previous,
}

#[derive(StaticByteSize, ByteSize)]
pub struct Segment {
	pub start: (i16, i16),
	pub end: (i16, i16),
}

messages! {
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
}

#[derive(StaticByteSize, ByteSize)]
pub enum Shape {
	Complex,
	Nonconvex,
	Convex,
}

messages! {
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
		//pub $format: Bitmap<ImageFormat>,
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
		//pub $format: ImageFormat,
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
}

#[derive(StaticByteSize, ByteSize)]
pub enum ColormapAlloc {
	None,
	All,
}

messages! {
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
		//pub $channel_mask: ColorChannelMask,
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
}

pub mod query_best_size {
	use xrb_proc_macros::{ByteSize, StaticByteSize};

	/// The 'type' of 'best size' being queried in a [`QueryBestSize`] request.
	///
	/// [`QueryBestSize`]: super::QueryBestSize
	#[derive(StaticByteSize, ByteSize)]
	pub enum Class {
		Cursor,
		Tile,
		Stipple,
	}
}

messages! {
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
		pub $class: query_best_size::Class,
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
		pub width: u16,
		pub height: u16,
	}

	/// The reply for the [`QueryBestSize`] request.
	///
	/// This contains the closest ideal size to the `width` and `height` that
	/// was given in the [`QueryBestSize`] request. See the request's docs for
	/// more information.
	pub struct QueryBestSizeReply for QueryBestSize<'_> {
		pub width: u16,
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
}

#[derive(StaticByteSize /*ByteSize*/)]
pub enum OrDefault<T> {
	Default,
	Some(T),
}

messages! {
	pub struct SetScreenSaver(107) {
		pub timeout: i16,
		pub interval: i16,
		pub prefer_blanking: OrDefault<bool>,
		pub allow_exposures: OrDefault<bool>,
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
}

pub mod change_hosts {
	use xrb_proc_macros::{ByteSize, StaticByteSize};

	#[derive(StaticByteSize, ByteSize)]
	pub enum Mode {
		Insert,
		Delete,
	}

	#[derive(StaticByteSize, ByteSize)]
	pub enum HostFamily {
		Internet,
		Decnet,
		Chaos,
	}
}

messages! {
	pub struct ChangeHosts<'a>(109) {
		pub $mode: change_hosts::Mode,
		pub family: HostFamily,
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
}

pub mod set_close_down_mode {
	use xrb_proc_macros::{ByteSize, StaticByteSize};

	#[derive(StaticByteSize, ByteSize)]
	pub enum Mode {
		Destroy,
		RetainPermanent,
		RetainTemporary,
	}
}

messages! {
	pub struct SetCloseDownMode(112): pub $mode: set_close_down_mode::Mode;

	//pub struct KillClient(113): pub resource: AllTemp<u32>;

	pub struct RotateProperties<'a>(114) {
		pub target: Window,
		#properties: u16,
		pub delta: i16,
		pub properties: &'a [Atom],
	}
}

pub mod force_screen_saver {
	use xrb_proc_macros::{ByteSize, StaticByteSize};

	#[derive(StaticByteSize, ByteSize)]
	pub enum Mode {
		Reset,
		Activate,
	}
}

messages! {
	pub struct ForceScreenSaver(115): pub $mode: force_screen_saver::Mode;
}

pub mod set_pointer_mapping {
	use xrb_proc_macros::{ByteSize, StaticByteSize};

	#[derive(StaticByteSize, ByteSize)]
	pub enum Status {
		Success,
		Busy,
	}
}

messages! {
	pub struct SetPointerMapping<'a>(116) -> SetPointerMappingReply {
		$#map: u8,
		pub map: &'a [u8],
		[(); {map}],
	}

	pub struct SetPointerMappingReply for SetPointerMapping<'_> {
		pub $status: set_pointer_mapping::Status,
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
