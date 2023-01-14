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

use std::{
	cmp,
	cmp::Ordering,
	convert::Infallible,
	fmt::{Debug, Display},
	mem::size_of,
};

use num_traits::Zero;
use thiserror::Error;

pub type ReadResult<T> = Result<T, ReadError>;
pub type WriteResult = Result<(), WriteError>;

pub use bytes::{Buf, BufMut};

trait DebugDisplay: Debug + Display {}
impl<T: Debug + Display> DebugDisplay for T {}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ReadError {
	#[error("unrecognized variant discriminant: {0}")]
	UnrecognizedDiscriminant(usize),

	#[error("a conversion failed: {0}")]
	FailedConversion(Box<dyn DebugDisplay>),
	#[error("{0}")]
	Other(Box<dyn DebugDisplay>),
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum WriteError {
	#[error("a conversion failed: {0}")]
	FailedConversion(Box<dyn DebugDisplay>),
	#[error("{0}")]
	Other(Box<dyn DebugDisplay>),
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

/// Trait for enums wrapping either a specific value of type [`Wrapped`], or one
/// of a discrete number of equally sized alternative unit variants.
///
/// A type implementing [`Wrapper`] must be an enum with one variant that has a
/// single tuple field of type [`Wrapped`], and zero or more unit variants (i.e.
/// variants with no fields) to serve as 'alternatives'.
///
/// A type implementing [`Wrapper`] is read by comparing a value of
/// [`Wrapped::Integer`] to each unit variant's discriminant. If it matches the
/// discriminant of a unit variant, it is read as that unit variant. If all unit
/// variants fail to match, it falls back to the tuple variant wrapping the
/// [`Wrapped`] type, constructing it with the use of
/// <code>[From]<[Wrapped::Integer]></code>.
///
/// [wraps]: Wrap
/// [`Wrapped`]: Self::Wrapped
/// [`Wrapped::Integer`]: Wrap::Integer
/// [Wrapped::Integer]: Wrap::Integer
///
/// # Examples
/// For example, this trait is implemented for <code>[Option]<T></code>, where
/// `T` becomes the [`Wrapped`] associated type. [`Option`] is an enum
/// containing two variants: one, <code>[Some]\(T)</code>, which [wraps] the
/// [`Wrapped`] type, and another, [`None`], which is a unit variant which has
/// an equivalent size to <code>[Some]\(T)</code>.
///
/// The following is an example of a `Maybe<T>` type equivalent to
/// <code>[Option]<T></code>.
/// ```
/// use xrbk::{
///     ConstantX11Size,
///     Readable,
///     Wrap,
///     Wrapper,
///     Buf,
///     ReadResult,
///     X11Size,
///     Writable,
///     WriteResult,
///     BufMut,
/// };
///
/// enum Maybe<T> {
///     Nothing,
///     Just(T),
/// }
///
/// impl<T: Wrap> Wrapper for Maybe<T> {
///     type Wrapped = T;
/// }
///
/// impl<T: Wrap> ConstantX11Size for Maybe<T> {
///     const X11_SIZE: usize = T::Integer::X11_SIZE;
/// }
///
/// impl<T: Wrap> X11Size for Maybe<T> {
///     fn x11_size(&self) -> usize {
///         Self::X11_SIZE
///     }
/// }
///
/// impl<T: Wrap> Readable for Maybe<T> {
///     fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
///     where
///         Self: Sized,
///     {
///         Ok(match <Self::Wrapped::Integer>::read_from(buf)? {
///             discrim if usize::from(discrim) == 0 => Self::Nothing,
///             val => Self::Just(val),
///         })
///     }
/// }
///
/// impl<T: Wrap> Writable for Maybe<T> {
///     fn write_to(&self, writer: &mut impl BufMut) -> WriteResult {
///         match self {
///             Self::Nothing => <Self::Wrapped::Integer>::from(0usize).write_to(buf)?,
///             Self::Just(val) => <Self::Wrapped::Integer>::from(val).write_to(buf)?,
///         }
///
///         Ok(())
///     }
/// }
/// ```
pub trait Wrapper: ConstantX11Size {
	/// The type wrapped in the value-containing tuple variant of this
	/// `Wrapper`.
	///
	/// See [`Wrapper`] docs for more information.
	type Wrapped: Wrap;
}

/// A trait implemented for types which 'wrap' some primitive integer type.
///
/// The purpose of this trait is for use with [`Wrapper`]s which may either have
/// a specific value (implementing `Wrap`), or one out of one or more possible
/// discrete alternatives, where these alternatives' discriminants use the
/// [`Integer`] associated type defined by `Wrap`.
///
/// The <code>[TryFrom]<[usize]></code> and <code>[Into]<[usize]></code> bounds
/// on [`Integer`] are arbitrarily chosen for the sake of easily comparing the
/// [`Integer`] value against the discriminants of other [`Wrapper`] variants.
///
/// [`Integer`]: Self::Integer
///
/// # Examples
/// For example, take the following [`Wrapper`]:
/// ```
/// use xrbk::{ConstantX11Size, Wrap, Wrapper, X11Size};
///
/// // The #[repr(u8)] attribute here is required because of the enum's layout
/// // in Rust - this is not the same layout as the Wrapper trait describes for
/// // the X11 protocol format.
/// #[repr(u8)]
/// pub enum Inheritable<T: Wrap> {
///     CopyFromParent,
///     Other(T),
/// }
///
/// impl<T: Wrap> Wrapper for Inheritable<T> {
///     type Wrapped = T;
/// }
///
/// impl<T: Wrap> ConstantX11Size for Inheritable<T> {
///     const X11_SIZE: usize = T::Integer::X11_SIZE;
/// }
///
/// // ... implementations of X11Size, Readable, Writable not shown ...
/// #
/// # impl<T: Wrap> X11Size for Inheritable<T> {
/// #     fn x11_size(&self) -> usize {
/// #         Self::X11_SIZE
/// #     }
/// # }
/// ```
/// It makes use of the `Wrap` trait so that it can have a value of a generic
/// type `T` and choose the appropriate integer type to encode the discriminant
/// of its discrete alternative `CopyFromParent`.
pub trait Wrap: TryFrom<Self::Integer> + Into<Self::Integer> + ConstantX11Size {
	type Integer: Copy + TryFrom<usize> + Into<usize> + ConstantX11Size + Readable + Writable;

	/// Referencing this associated `const` causes a compilation error if
	/// `Self::X11_SIZE` does not equal `Self::Integer::X11_SIZE`.
	const WRAPS_X11_SIZE: () = {
		assert!(
			Self::X11_SIZE == Self::Integer::X11_SIZE,
			"Wrap-implementing types must have an equal X11_SIZE to their Integer"
		);
	};
}

impl<T: Wrap> Wrapper for Option<T> {
	type Wrapped = T;
}

impl<T: Wrap> Readable for Option<T> {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(match <T::Integer>::read_from(buf).unwrap() {
			discrim if discrim.into() == 0_usize => None,
			value => Some(match T::try_from(value) {
				Ok(value) => value,
				Err(error) => return Err(ReadError::FailedConversion(Box::new(error))),
			}),
		})
	}
}

impl<T: Wrap> Writable for Option<T> {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		match self {
			None => match T::Integer::try_from(0_usize) {
				Ok(val) => val,
				Err(error) => return Err(WriteError::FailedConversion(Box::new(error))),
			}
			.write_to(buf)?,

			Some(val) => <&T::Integer as From<T>>::from(val).write_to(buf)?,
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
