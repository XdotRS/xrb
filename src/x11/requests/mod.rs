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

use crate::values;

use xrb_proc_macros::request as requests;

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

	#3: pub struct GetWindowAttributes<2> -> GetWindowAttributesReply {
		window: Window[4],
	}

	#4: pub struct DestroyWindow<2> {
		window: Window[4],
	}

	#5: pub struct DestroySubwindows<2> window: Window[4];

	// Just need the `Mode` enum for this one:
	// #6 pub struct ChangeSaveSet<2>(mode: Mode) window: Window[4];

	#7: pub struct ReparentWindow<4> {
		window: Window[4],
		parent: Window[4],
		x: i16[2],
		y: i16[2],
	}

	#8: pub struct MapWindow<2> window: Window[4];
	#9: pub struct MapSubwindows<2> window: Window[4];
	#10: pub struct UnmapWindow<2> window: Window[4];
	#11: pub struct UnmapSubwindows<2> window: Window[4];

	// #12: ConfigureWindow - waiting on algebraic length expressions

	// Just need the `Direction` enum for this one:
	// #13: pub struct CirculateWindow<2>(direction: Direction) window: Window[4];

	#14: pub struct GetGeometry<2> drawable: Drawable[4] -> GetGeometryReply;
	#15: pub struct QueryTree<2> window: Window[4] -> QueryTreeReply;

	// #16: InternAtom - waiting on algebraic length expressions

	#17: pub struct GetAtomName<2> atom: Atom[4] -> GetAtomNameReply;

	// #18: ChangeProperty - waiting on algebraic length expressions

	#19: pub struct DeleteProperty<3> {
		window: Window[4],
		atom: Atom[4],
	}

	#20: pub struct GetProperty<6>(delete: bool) -> GetPropertyReply {
		window: Window[4],
		property: Atom[4],
		property_type: Specificity<Atom>[4],
		long_offset: u32[4],
		long_length: u32[4],
	}

	#21: pub struct ListProperties<2> window: Window[4] -> ListPropertiesReply;

	#22: pub struct SetSelectionOwner<4> {
		owner: Option<Window>[4],
		selection: Atom[4],
		time: Time[4],
	}

	#23: pub struct GetSelectionOwner<2> -> GetSelectionOwnerReply {
		selection: Atom[4],
	}

	#24: pub struct ConvertSelection<6> {
		requestor: Window[4],
		selection: Atom[4],
		target: Atom[4],
		property: Option<Atom>[4],
		time: Time[4],
	}

	// I'm assuming it's best for all requests to be supported by this macro,
	// considering how many are. In that case, we'll want some kinda syntax to
	// show fields referring to _structures_, like so:
	// ```
	// #25: pub struct SendEvent(propagate: bool)<11> {
	//     destination: Destination<Window>[4],
	//     event_mask: EventMask[4],
	//     event: Box<dyn Event>{32},
	// }
	// ```

	// This syntax is fully supported, just need some other types for these
	// fields.
	//
	// #26: pub struct GrabPointer<6>(owner_events: bool) -> GrabPointerReply {
	// 	grab_window: Window[4],
	// 	event_mask: PointerEventMask[2],
	// 	pointer_mode: GrabMode[1],
	// 	keyboard_mode: GrabMode[1],
	// 	confine_to: Option<Window>[4],
	// 	cursor: Option<Cursor>[4],
	// 	time: Time[4],
	// }

	#27: pub struct UngrabPointer<2> time: Time[4];

	// This syntax is fully supported, just need some other types for these
	// fields.
	//
	// #28: pub struct GrabButton<6>(owner_events: bool) {
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

	// #29: pub struct UngrabButton<3>(button: Specificity<Button>) {
	// 	grab_window: Window[4],
	// 	modifiers: Window[2],
	// 	?[2], // this syntax seems to be broken.
	// }

	// #30: pub struct ChangeActivePointerGrab<4> {
	// 	cursor: Option<Cursor>[4],
	// 	time: Time[4],
	// 	event_mask: PointerEventMask[2],
	// 	?[2], // this syntax seems to be broken.
	// }

	// #31: pub struct GrabKeyboard<4>(owner_events: bool) -> GrabKeyboardReply {
	// 	grab_window: Window[4],
	// 	pointer_mode: GrabMode[1],
	// 	keyboard_mode: GrabMode[1],
	// 	?[2], // this syntax seems to be broken.
	// }

	#32: pub struct UngrabKeyboard<2> time: Time[4];

	// #33: pub struct GrabKey<4>(owner_events: bool) {
	//     grab_window: Window[4],
	//     modifiers: KeyMask[2],
	//     key: Specificity<Keycode>[1],
	//     pointer_mode: GrabMode[1],
	//     keyboard_mode: GrabMode[1],
	//     ?[3],
	// }

	// #34: pub struct UngrabKey<3>(key: Specificity<Keycode>) {
	// 	grab_window: Window[4],
	// 	modifiers: KeyMask[2],
	// 	?[2],
	// }

	// #35: pub struct AllowEvents<2>(mode: AllowEventsMode) time: Time[4];

	#36: pub struct GrabServer;
	#37: pub struct UngrabServer;

	#38: pub struct QueryPointer<2> window: Window[4] -> QueryPointerReply;

	#39: pub struct GetMotionEvents<4> -> GetMotionEventsReply {
		window: Window[4],
		start: Time[4],
		stop: Time[4],
	}

	#40: pub struct TranslateCoordinates<4> -> TranslateCoordinatesReply {
		source_window: Window[4],
		destination_window: Window[4],
		source_x: i16[2],
		source_y: i16[2],
	}

	#41: pub struct WarpPointer<6> {
		source_window: Option<Window>[4],
		destination_window: Option<Window>[4],
		source_x: i16[2],
		source_y: i16[2],
		source_width: u16[2],
		source_height: u16[2],
		destination_x: i16[2],
		destination_y: i16[2],
	}

	// #42: pub struct SetInputFocus<3>(revert_to: RevertTo) {
	//     focus: Option<Focus<Window>>[4],
	//     time: Time[4],
	// }

	#43: pub struct GetInputFocus -> GetInputFocusReply;
	#44: pub struct QueryKeymap -> QueryKeymapReply;

	// #45: OpenFont - waiting on algebraic length expressions

	#46: pub struct CloseFont<2> font: Font[4];
	#47: pub struct QueryFont<2> font: Fontable[4] -> QueryFontReply;

	// #48: QueryTextExtents - waiting on algebraic length expressions
}

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
