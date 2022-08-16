// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::serialization::{ReadWriteError, ReadWriteResult, WriteValue};

/// A unique ID referring to a particular resource.
///
/// That resource can be a [`Window`], [`Pixmap`], [`Cursor`], [`Font`], [`GContext`], or
/// [`Colormap`].
///
/// A [ResId] must be unique in terms of those specific resources; another type of ID, such as an
/// [`Atom`](crate::Atom) or [`VisualId`], may have the same value as a [ResId], however. For
/// example, no two [`Window`]s may share the same ID (nor can a [`Window`] and a [`Cursor`], nor a
/// [`Cursor`] and a [`Font`], etc.) but a [`Window`]'s ID may equal a [`VisualId`]. Which type of
/// ID applies can be assumed by context.
///
/// The top three bits of any [ResId] are guaranteed to be zero.
pub trait ResId {
	/// Gets the [ResId] itself.
	fn id(&self) -> u32;
}

#[macro_export]
macro_rules! res_id {
	(
		$(#[$metadata:meta])* // attributes or doc comments
		$vis:vis struct $Id:ident: $Trait:ty; // pub struct Id: Trait;

		$($t:tt)* // recurse: other definitions
	) => {
		crate::res_id! {
			$(#[$metadata])* // attributes or doc comments
			$vis struct $Id; // pub struct Id;

			$($t)* // recurse other definitions
		}

		impl $Trait for $Id {} // impl Trait for Id {}
	};
	(
		$(#[$metadata:meta])* // attributes or doc comments
		$vis:vis struct $Id:ident; // pub struct Id;

		$($t:tt)*
	) => {
		$(#[$metadata])* // attributes or doc comments
		#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
		$vis struct $Id { // pub struct Id {
			pub id: u32,
		}

		impl crate::proto::ids::ResId for $Id { // impl ResId for $Id {
			fn id(&self) -> u32 {
				self.id
			}
		}

		impl crate::serialization::WriteValue for $Id {
			fn write_1b(self) -> crate::serialization::ReadWriteResult<u8> {
				// An ID __must__ remain intact, or it loses all meaning.
				Err(crate::serialization::ReadWriteError::NotEnoughSpace)
			}

			fn write_2b(self) -> crate::serialization::ReadWriteResult<u16> {
				// An ID __must__ remain intact, or it loses all meaning.
				Err(crate::serialization::ReadWriteError::NotEnoughSpace)
			}

			fn write_4b(self) -> crate::serialization::ReadWriteResult<u32> {
				Ok(self.id as u32)
			}
		}

		impl crate::serialization::ReadValue for $Id {
			fn read_1b(_byte: u8) -> crate::serialization::ReadWriteResult<Self> {
				// An ID __must__ remain intact, or it loses all meaning.
				Err(crate::serialization::ReadWriteError::NotEnoughSpace)
			}

			fn read_2b(_bytes: u16) -> crate::serialization::ReadWriteResult<Self> {
				// An ID __must__ remain intact, or it loses all meaning.
				Err(crate::serialization::ReadWriteError::NotEnoughSpace)
			}

			fn read_4b(bytes: u32) -> crate::serialization::ReadWriteResult<Self> {
				Ok(Self { id: bytes })
			}
		}

		crate::res_id! { // recurse other definitions
			$($t)*
		}
	};
	() => {};
}

/// A trait object that represents either a [`Window`] or a [`Pixmap`].
pub trait Drawable: ResId {}
/// A trait object that represents either a [`Font`] or a [`GContext`].
pub trait Fontable: ResId {}

res_id! {
	/// A [resource ID](ResId) referring to a window. Implements [`Drawable`].
	pub struct Window: Drawable;
	/// A [resource ID](ResId) referring to a pixmap. Implements [`Drawable`].
	pub struct Pixmap: Drawable;
	/// A [resource ID](ResId) referring to an appearance of the cursor.
	///
	/// For example, when the cursor hovers over a link it might change appearance to a hand icon.
	/// This ID is a unique identifier that refers to that 'hand appearance'.
	pub struct Cursor;
	/// A [resource ID](ResId) referring to a particular font. Implements [`Fontable`].
	pub struct Font: Fontable;
	// TODO: What is a GContext?
	/// A [resource ID](ResId) referring to a graphics context. Implements [`Fontable`].
	pub struct GContext: Fontable;
	/// A [resource ID](ResId) referring to a colormap.
	///
	/// Colormaps were commonly used at the time the X11 protocol specification was written (in the
	/// late 80s), as memory was a limited resource and only so much color information could be
	/// stored in each pixel. While a number colors may be able to be displayed by the computer,
	/// a 'palette' would have to be defined for an image that would 'map' each of a limited number
	/// of values to a color. Those palettes are called [Colormap]s.
	pub struct Colormap;
}

// TODO: What is a visual? Docs.
pub struct VisualId {
	pub id: u32,
}

impl WriteValue for VisualId {
	fn write_1b(self) -> ReadWriteResult<u8> {
		// An ID __must__ remain intact, or it loses all meaning.
		Err(ReadWriteError::NotEnoughSpace)
	}

	fn write_2b(self) -> ReadWriteResult<u16> {
		// An ID __must__ remain intact, or it loses all meaning.
		Err(ReadWriteError::NotEnoughSpace)
	}

	fn write_4b(self) -> ReadWriteResult<u32> {
		Ok(self.id)
	}
}
