// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Replies] defined in the [core X11 protocol] for
//! [requests that relate to atoms, properties, selections, and sending events].
//!
//! [Replies] are messages sent from the X server to an X client in response to
//! a [request].
//!
//! [Replies]: Reply
//! [request]: crate::message::Request
//! [core X11 protocol]: crate::x11
//!
//! [requests that relate to atoms, properties, selections, and sending events]: request::miscellaneous

extern crate self as xrb;

use derivative::Derivative;

use xrbk::pad;
use xrbk_macro::derive_xrb;

use crate::{
	message::Reply,
	x11::request::{self, DataFormat, DataList},
	Atom,
	String8,
	Window,
};

derive_xrb! {
	/// The [reply] to a [`GetAtom` request].
	///
	/// [reply]: Reply
	///
	/// [`GetAtom` request]: request::GetAtom
	#[doc(alias("InternAtom", "CreateAtom"))]
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetAtom: Reply for request::GetAtom {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// The returned [atom].
		///
		/// If `no_creation` was set to `true` and an [atom] by the given `name`
		/// didn't already exist, this will be [`None`].
		///
		/// [atom]: Atom
		pub atom: Option<Atom>,
		[_; ..],
	}

	/// The [reply] to a [`GetAtomName` request].
	///
	/// [reply]: crate::message
	///
	/// [`GetAtomName` request]: request::GetAtomName
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetAtomName: Reply for request::GetAtomName {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		// The length of `name`.
		#[allow(clippy::cast_possible_truncation)]
		let name_len: u16 = name => name.len() as u16,
		[_; 22],

		/// The name of the [atom].
		///
		/// [atom]: Atom
		#[context(name_len => usize::from(*name_len))]
		pub name: String8,
		[_; name => pad(name)],
	}

	/// The [reply] to a [`GetProperty` request].
	///
	/// [reply]: Reply
	///
	/// [`GetProperty` request]: request::GetProperty
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetProperty: Reply for request::GetProperty {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// Whether the `value` is empty ([`None`]), or made up of `i8` values,
		/// `i16` values, or `i32` values.
		#[metabyte]
		pub format: Option<DataFormat>,

		/// The actual type of the property.
		pub r#type: Option<Atom>,
		/// The number of bytes remaining in the `property`'s data.
		///
		/// If the specified `property` does not exist for the `target`
		/// [window], this is zero.
		///
		/// If the specified `property` exists but its `type` does not match the
		/// specified type, this is the size of the property's data in bytes.
		///
		/// If the specified `property` exists and the type is either [`Any`] or
		/// matches the actual `type` of the property, this is the number of
		/// bytes remaining in the `property`'s data after the end of the
		/// returned `value`.
		///
		/// [window]: Window
		///
		/// [`Any`]: crate::Any::Any
		#[doc(alias = "bytes_after")]
		pub bytes_remaining: u32,

		// The length of `value`.
		#[allow(clippy::cast_possible_truncation)]
		let value_len: u32 = value => value.len() as u32,
		[_; 12],

		/// The property's value.
		///
		/// If `format` is [`None`], this will be [`DataList::I8`], but with an
		/// empty list.
		#[context(format, value_len => (format.unwrap_or(DataFormat::I8), *value_len))]
		pub value: DataList,
	}

	/// The [reply] for a [`ListProperties` request].
	///
	/// [reply]: Reply
	///
	/// [`ListProperties` request]: request::ListProperties
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct ListProperties: Reply for request::ListProperties {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		// The length of `properties`.
		#[allow(clippy::cast_possible_truncation)]
		let properties_len: u16 = properties => properties.len() as u16,
		[_; 22],

		/// The properties defined for the given [window].
		///
		/// [window]: Window
		#[doc(alias = "atoms")]
		#[context(properties_len => usize::from(*properties_len))]
		pub properties: Vec<Atom>,
	}

	/// The [reply] to a [`GetSelectionOwner` request].
	///
	/// [reply]: Reply
	///
	/// [`GetSelectionOwner` request]: request::GetSelectionOwner
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct GetSelectionOwner: Reply for request::GetSelectionOwner {
		/// The sequence number identifying the [request] that generated this
		/// [reply].
		///
		/// See [`Reply::sequence`] for more information.
		///
		/// [request]: crate::message::Request
		/// [reply]: Reply
		///
		/// [`Reply::sequence`]: Reply::sequence
		#[sequence]
		#[derivative(Hash = "ignore", PartialEq = "ignore")]
		pub sequence: u16,

		/// The owner of the given `selection`.
		///
		/// If this is [`None`], then the selection has no owner.
		pub owner: Option<Window>,
		[_; ..],
	}
}
