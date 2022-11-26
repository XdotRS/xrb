// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{Item, ItemWithId, Items};
use syn::punctuated::{IntoIter as PuncIntoIter, Iter as PuncIter};

/// An iterator over borrowed values of type `(`[`ItemId`]`, `[`Item`]`)`.
///
/// [`ItemId`]: super::ItemId
/// [`Item`]: Item
pub struct Pairs<'a>(Option<PuncIter<'a, ItemWithId>>);
/// An iterator over owned values of type `(`[`ItemId`]`, `[`Item`]`)`.
///
/// [`ItemId`]: super::ItemId
/// [`Item`]: Item
pub struct IntoPairs(Option<PuncIntoIter<ItemWithId>>);

/// An iterator over borrowed values of type [`Item`].
///
/// [`Item`]: Item
pub struct Iter<'a>(Pairs<'a>);
/// An iterator over owned values of type [`Item`].
///
/// [`Item`]: Item
pub struct IntoIter(IntoPairs);

impl<'a> Iterator for Pairs<'a> {
	type Item = &'a ItemWithId;

	fn next(&mut self) -> Option<Self::Item> {
		if let Self(Some(iter)) = self {
			// If `self` contains an iterator, return that iterator's `next()`.
			iter.next()
		} else {
			// Otherwise, if `self` does not contain an iterator, return
			// `None`.
			None
		}
	}
}

impl Iterator for IntoPairs {
	type Item = ItemWithId;

	fn next(&mut self) -> Option<Self::Item> {
		if let Self(Some(into_iter)) = self {
			// If `self` contains an iterator, return that iterator's `next()`.
			into_iter.next()
		} else {
			// Otherwise, if `self` does not contain an iterator, return
			// `None`.
			None
		}
	}
}

impl<'a> Iterator for Iter<'a> {
	type Item = &'a Item;

	fn next(&mut self) -> Option<Self::Item> {
		let Self(pairs) = self;

		pairs.next().map(|(_, item)| item)
	}
}

impl Iterator for IntoIter {
	type Item = Item;

	fn next(&mut self) -> Option<Self::Item> {
		let Self(into_pairs) = self;

		into_pairs.next().map(|(_, item)| item)
	}
}

impl<'a> Pairs<'a> {
	/// Creates a new borrowing iterator of [`Pairs`].
	fn new(iter: Option<PuncIter<'a, ItemWithId>>) -> Self {
		Self(iter)
	}
}

impl IntoPairs {
	/// Creates a new owning iterator of [`Pairs`].
	fn new(into_iter: Option<PuncIntoIter<ItemWithId>>) -> Self {
		Self(into_iter)
	}
}

impl<'a> Iter<'a> {
	/// Creates a new borrowing iterator of [`Item`]s.
	///
	/// [`Item`]: Item
	fn new(iter: Option<PuncIter<'a, ItemWithId>>) -> Self {
		Self(Pairs::new(iter))
	}
}

impl IntoIter {
	/// Creates a new owning iterator of [`Item`]s.
	///
	/// [`Item`]: Item
	fn new(into_iter: Option<PuncIntoIter<ItemWithId>>) -> Self {
		Self(IntoPairs::new(into_iter))
	}
}

impl<'a> IntoIterator for &'a Items {
	type Item = &'a Item;
	type IntoIter = Iter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		// If `self` contains items, use those items' iterator.
		Iter::new(match &self {
			Items::Named { items, .. } => Some(items.iter()),
			Items::Unnamed { items, .. } => Some(items.iter()),
			Items::Unit => None,
		})
	}
}

impl IntoIterator for Items {
	type Item = Item;
	type IntoIter = IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		// If `self` contains items, use those items' iterator.
		IntoIter::new(match self {
			Items::Named { items, .. } => Some(items.into_iter()),
			Items::Unnamed { items, .. } => Some(items.into_iter()),
			Items::Unit => None,
		})
	}
}

impl Items {
	/// Creates a borrowing iterator over values of type [`Item`].
	///
	/// [`Item`]: Item
	pub fn iter(&self) -> Iter {
		self.into_iter()
	}

	/// Creates a borrowing iterator over values of type `(`[`ItemId`]`, `[`Item`]`)`.
	///
	/// [`ItemId`]: super::ItemId
	/// [`Item`]: Item
	pub fn pairs(&self) -> Pairs {
		Pairs::new(match self {
			Items::Named { items, .. } => Some(items.iter()),
			Items::Unnamed { items, .. } => Some(items.iter()),
			Items::Unit => None,
		})
	}

	/// Creates an owning iterator over values of type `(`[`ItemId`]`, `[`Item`]`)`.
	///
	/// [`ItemId`]: super::ItemId
	/// [`Item`]: Item
	pub fn into_pairs(self) -> IntoPairs {
		IntoPairs::new(match self {
			Items::Named { items, .. } => Some(items.into_iter()),
			Items::Unnamed { items, .. } => Some(items.into_iter()),
			Items::Unit => None,
		})
	}
}
