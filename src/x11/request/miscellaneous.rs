// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Requests] defined in the [core X11 protocol] that relate to [atoms],
//! properties, selections, and sending [events].
//!
//! [Requests] are messages sent from an X client to the X server.
//!
//! [atoms]: Atom
//! [events]: Event
//! [Requests]: crate::message::Request
//! [core X11 protocol]: crate::x11

extern crate self as xrb;

use xrbk::{
	pad,
	Buf,
	BufMut,
	ConstantX11Size,
	ReadError,
	ReadError::UnrecognizedDiscriminant,
	ReadResult,
	ReadableWithContext,
	Wrap,
	Writable,
	WriteResult,
	X11Size,
};
use xrbk_macro::{derive_xrb, Readable, Writable, X11Size};

use crate::{
	message::Event,
	x11::{error, reply},
	Any,
	Atom,
	CurrentableTime,
	DestinationWindow,
	EventMask,
	String8,
	Window,
};

macro_rules! request_error {
	(
		$(#[$meta:meta])*
		$vis:vis enum $Name:ident for $Request:ty {
			$($($Error:ident),+$(,)?)?
		}
	) => {
		#[doc = concat!(
			"An [error](crate::message::Error) generated because of a failed [`",
			stringify!($Request),
			"` request](",
			stringify!($Request),
			")."
		)]
		#[doc = ""]
		$(#[$meta])*
		$vis enum $Name {
			$($(
				#[doc = concat!(
					"A [`",
					stringify!($Error),
					"` error](error::",
					stringify!($Error),
					")."
				)]
				$Error(error::$Error)
			),+)?
		}
	};
}

derive_xrb! {
	/// A [request] that returns the [atom] with the given `name`.
	///
	/// If `no_creation` is false and an [atom] by the specified `name` does not
	/// already exist, a new [atom] will be created and then returned. If an
	/// [atom] by the specified `name` already exists, that [atom] will be
	/// returned.
	///
	/// # Replies
	/// This [request] generates a [`GetAtom` reply].
	///
	/// [atom]: Atom
	/// [request]: crate::message::Request
	///
	/// [`GetAtom` reply]: reply::GetAtom
	#[doc(alias("InternAtom", "CreateAtom"))]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct GetAtom: Request(16, error::Value) -> reply::GetAtom {
		#[metabyte]
		/// Whether the X server should avoid creating a new [atom] for an
		/// unrecognised `name`.
		///
		/// If this is `true`, the X server won't create a new [atom] for a
		/// `name` which doesn't already refer to an [atom]. If it is `false`,
		/// the X server will create a new [atom] for the given `name`.
		///
		/// [atom]: Atom
		#[doc(alias = "only_if_exists")]
		pub no_creation: bool,

		// Encodes the length of `name`.
		#[allow(clippy::cast_possible_truncation)]
		let name_len: u16 = name => name.len() as u16,
		[_; 2],

		/// The name of the [atom] to either create or retrieve.
		///
		/// If an [atom] by this name does not already exist and `no_creation`
		/// is `false`, a new [atom] with this name will be created and
		/// returned.
		///
		/// If an [atom] by this name already exists, that [atom] will be
		/// returned.
		///
		/// [atom]: Atom
		#[context(name_len => usize::from(*name_len))]
		pub name: String8,
		[_; name => pad(name)],
	}

	/// A [request] that returns the name of the given [atom].
	///
	/// # Replies
	/// This [request] generates a [`GetAtomName` reply].
	///
	/// # Errors
	/// An [`Atom` error] is generated if the `target` does not refer to a
	/// defined [atom].
	///
	/// [atom]: Atom
	/// [request]: crate::message::Request
	///
	/// [`GetAtomName` reply]: reply::GetAtomName
	///
	/// [`Atom` error]: error::Atom
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct GetAtomName: Request(17, error::Atom) -> reply::GetAtomName {
		/// The [atom] for which this [request] gets its name.
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		/// [request]: crate::message::Request
		///
		/// [`Atom` error]: error::Atom
		#[doc(alias = "atom")]
		pub target: Atom,
	}
}

request_error! {
	pub enum ModifyPropertyError for ModifyProperty {
		Atom,
		Match,
		Value,
		Window,
	}
}

/// Whether a property is [replaced], [prepended] to a [window]'s list of
/// properties, or [appended] to the [window]'s list of properties.
///
/// [replaced]: ModifyPropertyMode::Replace
/// [prepended]: ModifyPropertyMode::Prepend
/// [appended]: ModifyPropertyMode::Append
///
/// [window]: Window
#[doc(alias = "ChangePropertyMode")]
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum ModifyPropertyMode {
	/// The property replaces an existing property; the previous value is
	/// discarded.
	Replace,

	/// The property is prepended to the list of properties.
	Prepend,

	/// The property is appended to the list of properties.
	Append,
}

/// Whether a [`DataList`] is formatted as a list of `i8` values, `i16` values,
/// or `i32` values.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum DataFormat {
	/// The list is formatted as `i8` values.
	I8 = 8,
	/// The list is formatted as `i16` values.
	I16 = 16,
	/// The list is formatted as `i32` values.
	I32 = 32,
}

impl ConstantX11Size for DataFormat {
	const X11_SIZE: usize = 1;
}

impl Wrap for DataFormat {
	type Integer = u8;
}

impl TryFrom<u8> for DataFormat {
	type Error = ReadError;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			i8 if i8 == 8 => Ok(Self::I8),
			i16 if i16 == 16 => Ok(Self::I16),
			i32 if i32 == 32 => Ok(Self::I32),

			other => Err(UnrecognizedDiscriminant(usize::from(other))),
		}
	}
}

impl From<DataFormat> for u8 {
	fn from(format: DataFormat) -> Self {
		match format {
			DataFormat::I8 => 8,
			DataFormat::I16 => 16,
			DataFormat::I32 => 32,
		}
	}
}

/// A list of either `i8` values, `i16` values, or `i32` values.
///
/// This represents uninterpreted 'raw' data.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum DataList {
	/// A list of `i8` values.
	I8(Vec<i8>),
	/// A list of `i16` values.
	I16(Vec<i16>),
	/// A list of `i32` values.
	I32(Vec<i32>),
}

impl DataList {
	/// The length of the data.
	///
	/// This is how many values there are - not the number of bytes.
	#[must_use]
	pub fn len(&self) -> usize {
		match self {
			Self::I8(list) => list.len(),
			Self::I16(list) => list.len(),
			Self::I32(list) => list.len(),
		}
	}

	/// Whether the `DataList` is empty.
	#[must_use]
	pub fn is_empty(&self) -> bool {
		match self {
			Self::I8(list) => list.is_empty(),
			Self::I16(list) => list.is_empty(),
			Self::I32(list) => list.is_empty(),
		}
	}
}

impl X11Size for DataList {
	fn x11_size(&self) -> usize {
		match self {
			Self::I8(list) => list.x11_size(),
			Self::I16(list) => list.x11_size(),
			Self::I32(list) => list.x11_size(),
		}
	}
}

impl ReadableWithContext for DataList {
	type Context = (DataFormat, u32);

	fn read_with(buf: &mut impl Buf, (format, length): &(DataFormat, u32)) -> ReadResult<Self>
	where
		Self: Sized,
	{
		let length = &(*length as usize);

		Ok(match format {
			DataFormat::I8 => Self::I8(<Vec<i8>>::read_with(buf, length)?),
			DataFormat::I16 => Self::I16(<Vec<i16>>::read_with(buf, length)?),
			DataFormat::I32 => Self::I32(<Vec<i32>>::read_with(buf, length)?),
		})
	}
}

impl Writable for DataList {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		match self {
			Self::I8(list) => list.write_to(buf)?,
			Self::I16(list) => list.write_to(buf)?,
			Self::I32(list) => list.write_to(buf)?,
		}

		Ok(())
	}
}

derive_xrb! {
	/// A [request] that modifies the given `property` for the [window].
	///
	/// A [`Property` event] is generated on the `target` [window].
	///
	/// # Errors
	/// A [`Window` error] is generated if `target` does not refer to a defined
	/// [window].
	///
	/// An [`Atom` error] is generated if either `property` or `type` do not
	/// refer to defined [windows][window].
	///
	/// If the `modify_mode` is [`Prepend`] or [`Append`], the `type` and
	/// `format` must match that of the existing property's value, else a
	/// [`Match` error] is generated.
	///
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [`Prepend`]: ModifyPropertyMode::Prepend
	/// [`Append`]: ModifyPropertyMode::Append
	///
	/// [`Property` event]: crate::x11::event::Property
	///
	/// [`Window` error]: error::Window
	/// [`Atom` error]: error::Atom
	/// [`Match` error]: error::Match
	#[doc(alias = "ChangeProperty")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct ModifyProperty: Request(18, ModifyPropertyError) {
		#[metabyte]
		/// The way in which the property is modified.
		///
		/// If the mode is [`Replace`], the previous property value is
		/// discarded.
		///
		/// If the mode is [`Prepend`], the data is prepended to the existing
		/// data. If the mode is [`Append`], the data is appended to the
		/// existing data.
		///
		/// # Errors
		/// If the mode is [`Prepend`] or [`Append`], the `type` and `format`
		/// must match that of the existing property's value, else a
		/// [`Match` error] is generated.
		///
		/// [window]: Window
		///
		/// [`Replace`]: ModifyPropertyMode::Replace
		/// [`Prepend`]: ModifyPropertyMode::Prepend
		/// [`Append`]: ModifyPropertyMode::Append
		///
		/// [`Match` error]: error::Match
		#[doc(alias = "mode")]
		pub modify_mode: ModifyPropertyMode,

		/// The [window] which the `property` is modified for.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,

		/// The property which is modified.
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		///
		/// [`Atom` error]: error::Atom
		pub property: Atom,
		/// The type of the property's data.
		///
		/// For example, if the property is of type [`Window`], then this would
		/// be [`atom::WINDOW`].
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		/// [`atom::WINDOW`]: crate::atom::WINDOW
		///
		/// [`Atom` error]: error::Atom
		pub r#type: Atom,

		// Whether the `data` is formatted as `i8` values, `i16` values, or
		// `i32` values.
		let format: DataFormat = data => match data {
			DataList::I8(_) => DataFormat::I8,
			DataList::I16(_) => DataFormat::I16,
			DataList::I32(_) => DataFormat::I32,
		},
		[_; 3],

		// The length of `data` in number of values (i.e., an `i32` value is
		// counted as one, rather than the number of bytes).
		#[allow(clippy::cast_possible_truncation)]
		let data_len: u32 = data => data.len() as u32,

		/// The property's value.
		///
		/// See [`DataList`] for information on the format of this data.
		#[context(format, data_len => (*format, *data_len))]
		pub data: DataList,
	}
}

request_error! {
	pub enum DeletePropertyError for DeleteProperty {
		Atom,
		Window,
	}
}

derive_xrb! {
	/// A [request] that removes the given `property` from a [window].
	///
	/// If the `property` does not exist on the [window], this [request] has no
	/// effect. Otherwise, a [`Property` event] is generated on the [window].
	///
	/// # Errors
	/// A [`Window` error] is generated if the `target` does not refer to a
	/// defined [window].
	///
	/// An [`Atom` error] is generated if the `property` does not refer to a
	/// defined [atom].
	///
	/// [window]: Window
	/// [atom]: Atom
	/// [request]: crate::message::Request
	///
	/// [`Property` event]: crate::x11::event::Property
	///
	/// [`Window` error]: error::Window
	/// [`Atom` error]: error::Atom
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct DeleteProperty: Request(19, DeletePropertyError) {
		/// The [window] for which this [request] removes the `property`.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		/// [request]: crate::message::Request
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,

		/// The property which is to be removed from the `target` [window].
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		/// [window]: Window
		///
		/// [`Atom` error]: error::Atom
		pub property: Atom,
	}
}

request_error! {
	pub enum GetPropertyError for GetProperty {
		Atom,
		Value,
		Window,
	}
}

derive_xrb! {
	/// A [request] that gets the value of the given `property` on the given
	/// [window].
	///
	/// # Replies
	/// This [request] generates a [`GetProperty` reply].
	///
	/// # Errors
	/// A [`Window` error] is generated if `target` does not refer to a defined
	/// [window].
	///
	/// An [`Atom` error] is generated if either `property` or `type` do not
	/// refer to defined [atoms].
	///
	/// [window]: Window
	/// [request]: crate::message::Request
	/// [atoms]: Atom
	///
	/// [`GetProperty` reply]: reply::GetProperty
	///
	/// [`Window` error]: error::Window
	/// [`Atom` error]: error::Atom
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct GetProperty: Request(20, GetPropertyError) -> reply::GetProperty {
		/// Whether the `property` should be deleted from the `target` [window].
		///
		/// If the `type` matches the `property`'s actual type (or is [`Any`]),
		/// the property is removed from the [window]. Otherwise, this is
		/// ignored.
		///
		/// [window]: Window
		///
		/// [`Any`]: Any::Any
		#[metabyte]
		pub delete: bool,

		/// The [window] on which the requested `property` is found.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,

		/// The property for which this [request] gets its value.
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		/// [request]: crate::message::Request
		pub property: Atom,
		/// The property type to filter the [window]'s properties by.
		///
		/// This specifies that specifically a `property` of this type is
		/// requested. If the type does not match, the value is not provided in
		/// [the reply].
		///
		/// [window]: Window
		///
		/// [the reply]: reply::GetProperty
		pub r#type: Any<Atom>,

		/// The offset of the value of the `property` that is requested in
		/// 4-byte units.
		///
		/// This offset is multiplied by 4 when applied to the start of the
		/// `property`'s data.
		#[doc(alias = "long_offset")]
		pub offset: u32,
		/// The length of the value of the `property` that is requested in
		/// 4-byte units.
		///
		/// This length is multiplied by 4 and added to the `offset` to find the
		/// endpoint of the value that is requested.
		#[doc(alias = "long_length")]
		pub length: u32,
	}

	/// A [request] that returns the list of properties defined for the given
	/// [window].
	///
	/// # Replies
	/// This [request] generates a [`ListProperties` reply].
	///
	/// # Errors
	/// A [`Window` error] is generated if `target` does not refer to a defined
	/// [window].
	///
	/// [window]: Window
	/// [request]: crate::message::Request
	///
	/// [`ListProperties` reply]: reply::ListProperties
	///
	/// [`Window` error]: error::Window
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct ListProperties: Request(21, error::Window) -> reply::ListProperties {
		/// The [window] for which this [request] returns its properties.
		///
		/// # Errors
		/// A [`Window` error] is returned if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		/// [request]: crate::message::Request
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "window")]
		pub target: Window,
	}
}

request_error! {
	pub enum SetSelectionOwnerError for SetSelectionOwner {
		Atom,
		Window,
	}
}

derive_xrb! {
	/// A [request] that changes the owner of the given selection.
	///
	/// If the `new_owner` is different to the previous owner of the selection,
	/// and the previous owner was not [`None`], then a [`SelectionClear` event]
	/// is sent to the previous owner.
	///
	/// If the given `time` is earlier than the [time] of the previous owner
	/// change or is later than the X server's [current time], this [request]
	/// has no effect.
	///
	/// # Errors
	/// A [`Window` error] is generated if `owner` is [`Some`] but does not
	/// refer to a defined [window].
	///
	/// An [`Atom` error] is generated if `selection` does not refer to a
	/// defined [atom].
	///
	/// [window]: Window
	/// [atom]: Atom
	/// [time]: crate::Timestamp
	/// [request]: crate::message::Request
	///
	/// [current time]: CurrentableTime::CurrentTime
	///
	/// [`SelectionClear` event]: crate::x11::event::SelectionClear
	///
	/// [`Window` error]: error::Window
	/// [`Atom` error]: error::Atom
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct SetSelectionOwner: Request(22, SetSelectionOwnerError) {
		/// Sets the new owner of the `selection`.
		///
		/// [`None`] specifies that the `selection` is to have no owner.
		///
		/// # Errors
		/// A [`Window` error] is generated if this is [`Some`] but does not
		/// refer to a defined [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		#[doc(alias = "owner")]
		pub new_owner: Option<Window>,
		/// The selection for which this [request] changes its owner.
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		/// [request]: crate::message::Request
		///
		/// [`Atom` error]: error::Atom
		pub selection: Atom,

		/// The [time] at which this change is recorded to occur at.
		///
		/// If this [time] is earlier than the server's current 'last-change'
		/// [time] for the selection's owner, or this [time] is later than the
		/// server's [current time], this [request] has no effect.
		///
		/// [time]: crate::Timestamp
		/// [current time]: CurrentableTime::CurrentTime
		/// [request]: crate::message::Request
		pub time: CurrentableTime,
	}

	/// A [request] that returns the owner of a given selection.
	///
	/// # Replies
	/// This [request] generates a [`GetSelectionOwner` reply].
	///
	/// # Errors
	/// An [`Atom` error] is generated if `target` does not refer to a defined
	/// [atom].
	///
	/// [atom]: Atom
	/// [request]: crate::message::Request
	///
	/// [`GetSelectionOwner` reply]: reply::GetSelectionOwner
	///
	/// [`Atom` error]: error::Atom
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct GetSelectionOwner: Request(23) -> reply::GetSelectionOwner {
		/// The selection for which this [request] returns its owner.
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		/// [request]: crate::message::Request
		///
		/// [`Atom` error]: error::Atom
		pub target: Atom,
	}
}

request_error! {
	pub enum ConvertSelectionError for ConvertSelection {
		Atom,
		Window,
	}
}

derive_xrb! {
	/// A [request] that asks the given selection's owner to convert it to the
	/// given `target_type`.
	///
	/// # Errors
	/// A [`Window` error] is generated if `requester` does not refer to a
	/// defined [window].
	///
	/// An [`Atom` error] is generated if any `selection`, `target_type`, or
	/// `property` do not refer to defined [atoms].
	///
	/// [window]: Window
	/// [atoms]: Atom
	/// [request]: crate::message::Request
	///
	/// [`Window` error]: error::Window
	/// [`Atom` error]: error::Atom
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct ConvertSelection: Request(24, ConvertSelectionError) {
		/// Your [window] which is requesting this conversion.
		///
		/// # Errors
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [`Window` error]: error::Window
		pub requester: Window,

		/// The selection which this [request] asks to be converted.
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		/// [request]: crate::message::Request
		///
		/// [`Atom` error]: error::Atom
		pub selection: Atom,

		/// The type which the selection should be converted into.
		///
		/// # Errors
		/// An [`Atom` error] is generated if this does not refer to a defined
		/// [atom].
		///
		/// [atom]: Atom
		///
		/// [`Atom` error]: error::Atom
		pub target_type: Atom,
		pub property: Option<Atom>,

		/// The [time] at which this conversion is recorded as having taken
		/// place.
		///
		/// [time]: crate::Timestamp
		pub time: CurrentableTime,
	}
}

request_error! {
	pub enum SendEventError for SendEvent {
		Value,
		Window,
	}
}

derive_xrb! {
	/// A [request] that sends the given [event] to the given [window].
	///
	/// If the `event_mask` is empty, the [event] is sent to the client that
	/// created the [window] - if that client no longer exists, the [event] is
	/// not sent.
	///
	/// If `propagate` is `false`, the [event] is sent to every client selecting
	/// any of the [events][event] indicated in the `event_mask`.
	///
	/// If `propagate` is `true` and no clients have selected any of the
	/// [events][event] indicated in the `event_mask` on the [window], the
	/// [event] is sent to the closest ancestor [window] of the [window] which
	/// some client has selected at least one of the indicated [events][event]
	/// for (provided no [windows][window] between the original destination and
	/// the closest ancestor have that [event] in their
	/// [`do_not_propagate_mask`]). The [event] is sent to every client
	/// selecting any of the [events][event] indicated in the `event_mask` on
	/// the final destination.
	///
	/// Active grabs are ignored for this [request].
	///
	/// # Errors
	/// A [`Window` error] is generated if the `destination` is [`DestinationWindow::Other`] and the
	/// specified [window] is not defined.
	///
	/// [window]: Window
	/// [event]: Event
	/// [request]: crate::message::Request
	///
	/// [`do_not_propagate_mask`]: crate::set::Attributes::do_not_propagate_mask
	///
	/// [`Window` error]: error::Window
	// FIXME: this requires that the event is absolutely 32 bytes, which is
	//        currently not bounded.
	//
	// This feature would be nice for this:
	// <https://github.com/rust-lang/rust/issues/92827>
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct SendEvent<E: Event + ConstantX11Size>: Request(25, SendEventError) {
		/// Whether the `event` should be propagated to the closest appropriate
		/// ancestor, if necessary.
		///
		/// That is, whether the `event` should be propagated to the closest
		/// ancestor of the `destination` [window] which some client has
		/// selected any of the [events] indicated in the `event_mask` on if no
		/// clients have selected any of the [events] in the `event_mask` on the
		/// `destination` [window].
		///
		/// [window]: Window
		/// [events]: Event
		#[metabyte]
		pub propagate: bool,

		/// The destination [window] for the `event`.
		///
		/// [window]: Window
		pub destination: DestinationWindow,

		/// The mask of [events][event] which should be selected for the [event]
		/// to be sent to the selecting clients.
		///
		/// [event]: Event
		pub event_mask: EventMask,

		/// The [event] that is sent.
		///
		/// [event]: Event
		pub event: E,
	}
}
