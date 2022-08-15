// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{BitAnd, BitOr};

use crate::{Deserialize, Serialize};

/// A trait implemented by bitmask enums to provide conversions between a variant and its bitmask.
///
/// Use [`bitmask!`](crate::bitmask) to define bitmask enums implementing this trait.
pub trait Bitmask<T>
where
	Self: Sized + Serialize + Deserialize,
	T: BitAnd + BitOr,
{
	/// Gets the bitmask value associated with this bitmask variant.
	fn mask(&self) -> T;

	/// Gets the exactly matching bitmask variant for the given bitmask.
	///
	/// By 'exactly matching', this means that only the matching mask bit can be set. Use
	/// [`from_mask(mask: T) -> Vec<Self>`](Bitmask::from_mask) to get a [`Vec`] of all matching
	/// bitmask variants for the given mask.
	fn match_mask(mask: T) -> Option<Self>;

	/// Returns a [`Vec`] of all matching bitmask variants for the given bitmask.
	fn from_mask(mask: T) -> Vec<Self>;
}
