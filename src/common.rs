// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate self as xrb;

mod id;
mod mask;
mod string;
mod values;
mod wrappers;

pub use id::*;
pub use mask::*;
pub use string::*;
pub use values::*;
pub use wrappers::*;

use xrbk_macro::derive_xrb;

derive_xrb! {
	/// A resource ID referring to either a [`Window`] or a [`Pixmap`].
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Drawable(u32);

	impl From<Window> for Drawable {
		fn from(window: Window) -> Self {
			let Window(id) = window;
			Self(id)
		}
	}

	impl From<Pixmap> for Drawable {
		fn from(pixmap: Pixmap) -> Self {
			let Pixmap(id) = pixmap;
		Self(id)
		}
	}

	/// A resource ID referring to a particular window resource.
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Window(u32);

	impl From<Drawable> for Window {
		fn from(drawable: Drawable) -> Self {
			let Drawable(id) = drawable;
			Self(id)
		}
	}

	/// A resource ID referring to a particular pixmap resource.
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Pixmap(u32);

	impl From<Drawable> for Pixmap {
		fn from(drawable: Drawable) -> Self {
			let Drawable(id) = drawable;
			Self(id)
		}
	}

	/// A resource ID referring to a particular cursor resource.
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Cursor(u32);

	/// A resource ID referring to either a [`Font`] or a [`GraphicsContext`].
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Fontable(u32);

	impl From<Font> for Fontable {
		fn from(font: Font) -> Self {
			let Font(id) = font;
			Self(id)
		}
	}

	impl From<GraphicsContext> for Fontable {
		fn from(context: GraphicsContext) -> Self {
			let GraphicsContext(id) = context;
			Self(id)
		}
	}

	/// A resource ID referring to a particular font resource.
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Font(u32);

	impl From<Fontable> for Font {
		fn from(fontable: Fontable) -> Self {
			let Fontable(id) = fontable;
			Self(id)
		}
	}

	/// A resource ID referring to a particular graphics context resource.
	///
	/// Information relating to graphics output is stored in a graphics
	/// context such as foreground pixel, background pixel, line width,
	/// clipping region, etc. A graphics context can only be used with
	/// [`Drawable`]s that have the same `root` and `depth` as the
	/// `GraphicsContext`.
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct GraphicsContext(u32);

	impl From<Fontable> for GraphicsContext {
		fn from(fontable: Fontable) -> Self {
			let Fontable(id) = fontable;
			Self(id)
		}
	}

	/// A resource ID referring to a particular colormap resource.
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Colormap(u32);

	/// A unique ID corresponding to a string name.
	///
	/// `Atom`s are used to identify properties, types, and selections.
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Atom(u32);

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct VisualId(u32);

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum BitGravity {
		Forget,
		Static,
		NorthWest,
		North,
		NorthEast,
		West,
		Center,
		East,
		SouthWest,
		South,
		SouthEast,
	}

	impl Default for BitGravity {
		fn default() -> Self {
			Self::NorthWest
		}
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum WinGravity {
		Unmap,
		Static,
		NorthWest,
		North,
		NorthEast,
		West,
		Center,
		East,
		SouthWest,
		South,
		SouthEast,
	}

	impl Default for WinGravity {
		fn default() -> Self {
			Self::NorthWest
		}
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Keysym(u32);
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Keycode(u8);
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Button(u8);
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Timestamp(u32);

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Char8(u8);
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Char16(u8, u8);

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Point {
		x: i16,
		y: i16,
	}

	/// A rectangle with coordinates and dimensions.
	///
	/// The coordinates are those of the upper-left corner of the rectangle. The
	/// units for the coordinates and dimensions are not specified.
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Rectangle {
		/// The `x` coordinate of the upper left corner of the `Rectangle`.
		x: i16,
		/// The `y` coordinate of the upper left corner of the `Rectangle`.
		y: i16,
		/// The `width` of the `Rectangle`.
		width: u16,
		/// The `height` of the `Rectangle`.
		height: u16,
	}

	impl Default for Rectangle {
		fn default() -> Self {
			Self {
				x: 0,
				y: 0,
				width: 1,
				height: 1,
			}
		}
	}

	/// An arc (the geometry kind) with coordinates, dimensions, and angles.
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Arc {
		x: i16,
		y: i16,
		width: u16,
		height: u16,

		/// Specifies the start of the `Arc`.
		///
		/// The angle is measured in degrees scaled by 64. Positive indicates
		/// counterclockwise motion and negative indicates clockwise motion.
		/// The angle is measured relative to the three-o'clock position from
		/// the center of the rectangle.
		start_angle: i16,
		/// Specifies the extent of the `Arc` relative to the `start_angle`.
		///
		/// The angle is measured in degrees scaled by 64. If greater than 360
		/// degrees, this angle is truncated to 360 degrees.
		end_angle: i16,
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum HostFamily {
		Internet,
		Decnet,
		Chaos,
		ServerInterpreted = 5,
		InternetV6,
	}

	impl Default for HostFamily {
		fn default() -> Self {
			Self::ServerInterpreted
		}
	}

	#[derive(Clone, Eq, PartialEq, Hash, Debug)]
	pub struct Host {
		pub family: HostFamily,
		_,

		#[allow(clippy::cast_possible_truncation)]
		let address_len: u16 = address => address.len() as u16,

		#[context(address_len => *address_len as usize)]
		pub address: Vec<u8>,
		[_; ..],
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum Status {
		Success,
		Busy,
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum ScreenSaverMode {
		Reset,
		Activate,
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum CloseDownMode {
		Destroy,
		RetainPermanent,
		RetainTemporary,
	}

	/// The 'type' of 'best size' being queried in a [`QueryBestSize`] request.
	///
	/// [`QueryBestSize`]: super::QueryBestSize
	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum QueryBestSizeClass {
		Cursor,
		Tile,
		Stipple,
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum ColormapAlloc {
		None,
		All,
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum Shape {
		Complex,
		Nonconvex,
		Convex,
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum CoordinateMode {
		Origin,
		Previous,
	}

	impl Default for CoordinateMode {
		fn default() -> Self {
			Self::Origin
		}
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
	pub struct Segment {
		pub start: (i16, i16),
		pub end: (i16, i16),
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum Ordering {
		Unsorted,
		Ysorted,
		YxSorted,
		YxBanded,
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum DrawDirection {
		LeftToRight,
		RightToLeft,
	}

	impl Default for DrawDirection {
		fn default() -> Self {
			Self::LeftToRight
		}
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct FontProperty {
		pub name: Atom,
		pub value: u32,
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub struct CharInfo {
		pub left_side_bearing: i16,
		pub right_side_bearing: i16,
		pub character_width: i16,
		pub ascent: i16,
		pub descent: i16,
		pub attributes: u16,
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
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

	impl Default for AllowEventsMode {
		fn default() -> Self {
			Self::AsyncBoth
		}
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum HostFamilyA {
		Internet,
		Decnet,
		Chaos,
	}

	impl Default for HostFamilyA {
		fn default() -> Self {
			Self::Internet
		}
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum GrabMode {
		Synchronous,
		Asynchronous,
	}

	impl Default for GrabMode {
		fn default() -> Self {
			Self::Asynchronous
		}
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum GrabStatus {
		Success,
		AlreadyGrabbed,
		InvalidTime,
		NotViewable,
		Frozen,
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum CirculateDirection {
		RaiseLowest,
		RaiseHighest,
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum EditMode {
		Insert,
		Delete,
	}

	impl Default for EditMode {
		fn default() -> Self {
			Self::Insert
		}
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum Format {
		XyPixmap = 1,
		Zpixmap = 2,
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum StackMode {
		Above,
		Below,
		TopIf,
		Bottomif,
		Opposite,
	}

	impl Default for StackMode {
		fn default() -> Self {
			Self::Above
		}
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum MapState {
		Unmapped,
		Unviewable,
		Viewable,
	}

	impl Default for MapState {
		fn default() -> Self {
			Self::Unmapped
		}
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum BackingStore {
		NotUseful,
		WhenMapped,
		Always,
	}

	#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
	pub enum WindowClass {
		InputOutput = 1,
		InputOnly = 2,
	}

	impl Default for WindowClass {
		fn default() -> Self {
			Self::InputOutput
		}
	}

	/// An identifier representing the concept of all possible keys.
	///
	/// The difference between a `Keysym` and a [`Keycode`] is that the `Keysym`
	/// universally represents the concept of any particular key, while the
	/// [`Keycode`] refers to the specific position of a key on the user's keyboard,
	/// as interpreted by the device drivers.
	///
	/// For example, the concept of an `F13` key always exists as a `Keysym`, even
	/// if there is no such key represented by a [`Keycode`] for the actual keyboard
	/// currently in use.
	// pub type Keysym = u32;
	/// An identifier for the location of a key as interepreted by OS drivers.
	///
	/// The difference between a `Keycode` and a [`Keysym`] is that the `Keycode`
	/// refers to the specific position of a key on the user's keyboard, as
	/// interpreted by the device drivers, while the [`Keysym`] universally
	/// represents the concept of any particular key.
	///
	/// For example, the concept of an `F13` key always exists as a [`Keysym`], even
	/// if there is no such key represented by a `Keycode` for the actual keyboard
	/// currently in use.
	// pub type Keycode = u8;
	/// A button on the mouse.
	///
	/// For example, button 1 is the primary mouse button, commonly found on the
	/// left of a mouse.
	// pub type Button = u8;

	// pub type Timestamp = u32;

	/// Specifies how to pick the window to revert focus to when the current
	/// window is unmapped.
	//
	// Would this be better as a `Parent` unit struct and a type alias for
	// `Option<InputFocus<Parent>>`? Did it like this so that you don't have to do:
	// ```
	// Some(InputFocus::Specific(Parent))
	// ```
	// and can instead do:
	// ```
	// RevertTo::Parent
	// ```
	pub enum RevertTo {
		/// Revert the focus to none at all.
		///
		/// It is recommended to avoid setting this: it might lead to behavior you
		/// don't expect. Only set this as the [`RevertTo`] if you know the
		/// potential consequences.
		None,
		// TODO: What is this?
		PointerRoot,
		/// Revert the focus to the parent of the window.
		///
		/// This is the recommended [`RevertTo`] option for most cases.
		Parent,
	}

	impl Default for RevertTo {
		fn default() -> Self {
			Self::Parent
		}
	}

	/// The destination for an [`Event`] in a [`SendEvent`] request.
	///
	/// This is the window that the event will be sent to.
	pub enum Destination {
		/// The [`Window`] the pointer is currently within.
		PointerWindow,
		/// The [`Window`] that currently has input focus.
		InputFocus,
		/// A specific [`Window`].
		Specific(Window),
	}

	impl Default for Destination {
		fn default() -> Self {
			Self::InputFocus
		}
	}
}
