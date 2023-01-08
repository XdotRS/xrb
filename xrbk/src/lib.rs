// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// We need specialization to implement DataSize for types with generics like
// Option<T>
#![allow(incomplete_features)]
#![feature(specialization)]
// Deny the following clippy lints to enforce them:
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
// Warn for these lints, rather than denying them.
#![warn(clippy::use_self)]
// Warn for pedantic & cargo lints. They are allowed completely by default.
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
// Continue to allow these though.
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::module_name_repetitions)]

//! # XRBK
//!
//! The XRB Kit, a collection of traits and types to help with
//! (de)serialization of types in XRB.

use std::error::Error;

use num::Zero;
use thiserror::Error;

pub type ReadResult<T> = Result<T, ReadError>;
pub type WriteResult = Result<(), WriteError>;

pub use bytes::{Buf, BufMut};

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ReadError {
	#[error("unrecognized variant discriminant: {0}")]
	UnrecognizedDiscriminant(u8),

	#[error("{0}")]
	Other(Box<dyn Error>),
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum WriteError {
	#[error("{0}")]
	Other(Box<dyn Error>),
}

mod readable;
mod writable;
mod x11_size;

/// Gives the type size in bytes.
/// The size can vary depending on the quantity of data it contains
pub trait X11Size {
	/// Returns the size of `self` when serialized according to the X11
	/// protocol, measured in bytes.
	fn x11_size(&self) -> usize;
}

/// Defines the constant size in bytes of a type when serialized according to
/// the X11 protocol.
///
/// [`X11Size`] must be implemented to return the same `X11_SIZE`:
/// ```
/// # use xrbk::{ConstantX11Size, X11Size};
/// # struct MyStruct;
/// #
/// # impl ConstantX11Size for MyStruct {
/// #     const X11_SIZE: usize = 5;
/// # }
/// #
/// impl X11Size for MyStruct {
///     fn x11_size(&self) -> usize {
///         Self::X11_SIZE
///     }
/// }
/// ```
pub trait ConstantX11Size: X11Size {
	/// The size of this type when serialized according to the the X11 protocol,
	/// measured in bytes.
	const X11_SIZE: usize;
}

/// Reads a type from bytes.
pub trait Readable: X11Size {
	/// Reads [`Self`] from a [`Buf`] of bytes.
	///
	/// # Errors
	///
	/// - [`ReadError::UnrecognizedDiscriminant`]: The value encountered is not
	///   matching any enum's variants discriminant.
	/// - [`ReadError::Other`]: Any other error when parsing.
	///
	/// [`Buf`]: Buf
	fn read_from(reader: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized;
}

/// Allows the reading of a type from bytes given some additional
/// [`Context`](Self::Context).
pub trait ReadableWithContext: X11Size {
	/// The type of context with which this type can be read from bytes.
	///
	/// For example, this might be `usize` for some collection, where that
	/// `usize` context represents the length of the list with which to read.
	type Context;

	/// Reads [`Self`] from a [`Buf`] of bytes, given some additional
	/// [`Context`](Self::Context).
	///
	/// # Errors
	///
	/// - [`ReadError::UnrecognizedDiscriminant`]: The value encountered is not
	///   matching any enum's variants discriminant.
	/// - [`ReadError::Other`]: Any other error when parsing.
	///
	/// [`Buf`]: Buf
	fn read_with(reader: &mut impl Buf, context: &Self::Context) -> ReadResult<Self>
	where
		Self: Sized;
}

/// Allows a type to be written as bytes.
pub trait Writable: X11Size {
	/// Writes [`self`](Self) as bytes to a [`BufMut`].
	///
	/// # Errors
	///
	/// Returns a [`WriteError`] if it was not able to properly write to the
	/// given `reader`.
	///
	/// [`BufMut`]: BufMut
	fn write_to(&self, writer: &mut impl BufMut) -> WriteResult;
}

// TODO: see if this is actually a good way to do things
pub trait Wrapper: ConstantX11Size {
	type WrappedType: Writable + Readable + ConstantX11Size;

	fn wrap(val: Self::WrappedType) -> Self;
	fn unwrap(&self) -> &Self::WrappedType;
}

impl<T: Wrapper> Readable for Option<T>
where
	T::WrappedType: Zero,
{
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(match T::WrappedType::read_from(buf)? {
			x if x.is_zero() => None,
			val => Some(T::wrap(val)),
		})
	}
}

impl<T: Wrapper> Writable for Option<T>
where
	T::WrappedType: Zero,
{
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		match self {
			None => T::WrappedType::zero().write_to(buf)?,
			Some(val) => val.unwrap().write_to(buf)?,
		}

		Ok(())
	}
}

// This function is unused, but writing it here asserts that these traits are
// _object safe_; that is, that the Rust compiler will generate an error if any
// of these traits are accidentally made _object unsafe_, which means that they
// cannot be used with the `dyn` keyword.
fn _assert_object_safety(
	_data_size: &dyn X11Size,
	_readable: &dyn Readable,
	_contextual_readable: &dyn ReadableWithContext<Context = ()>,
	//_writable: &dyn Writable,
) {
}
