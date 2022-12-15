// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use syn::punctuated::{
	IntoIter as PuncIntoIter,
	IntoPairs as PuncIntoPairs,
	Iter as PuncIter,
	IterMut as PuncIterMut,
	Pair,
	Pairs as PuncPairs,
	PairsMut as PuncPairsMut,
};

/// A borrowing iterator over [`Element`]s.
pub struct Iter<'a>(Option<PuncIter<'a, Element<'a>>>);
/// An owning iterator over [`Element`]s.
pub struct IntoIter<'a>(Option<PuncIntoIter<Element<'a>>>);
/// A mutably borrowing iterator over [`Element`]s.
pub struct IterMut<'a>(Option<PuncIterMut<'a, Element<'a>>>);

/// A borrowing iterator over pairs of [`Element`]s and possible commas.
pub struct Pairs<'a>(Option<PuncPairs<'a, Element<'a>, Token![,]>>);
/// An owning iterator over pairs of [`Element`]s and possible commas.
pub struct IntoPairs<'a>(Option<PuncIntoPairs<Element<'a>, Token![,]>>);
/// A mutably borrowing iterator over pairs of [`Element`]s and possible commas.
pub struct PairsMut<'a>(Option<PuncPairsMut<'a, Element<'a>, Token![,]>>);

// impl Iterator {{{

impl<'a> Iterator for Iter<'a> {
	type Item = &'a Element<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if let Self(Some(iter)) = self {
			iter.next()
		} else {
			None
		}
	}
}

impl<'a> Iterator for IntoIter<'a> {
	type Item = Element<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if let Self(Some(into_iter)) = self {
			into_iter.next()
		} else {
			None
		}
	}
}

impl<'a> Iterator for IterMut<'a> {
	type Item = &'a mut Element<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if let Self(Some(iter_mut)) = self {
			iter_mut.next()
		} else {
			None
		}
	}
}

impl<'a> Iterator for Pairs<'a> {
	type Item = (&'a Element<'a>, Option<&'a Token![,]>);

	fn next(&mut self) -> Option<Self::Item> {
		if let Self(Some(pairs)) = self {
			Some(match pairs.next()? {
				Pair::Punctuated(element, comma) => (element, Some(comma)),
				Pair::End(element) => (element, None),
			})
		} else {
			None
		}
	}
}

impl<'a> Iterator for IntoPairs<'a> {
	type Item = (Element<'a>, Option<Token![,]>);

	fn next(&mut self) -> Option<Self::Item> {
		if let Self(Some(into_pairs)) = self {
			Some(match into_pairs.next()? {
				Pair::Punctuated(element, comma) => (element, Some(comma)),
				Pair::End(element) => (element, None),
			})
		} else {
			None
		}
	}
}

impl<'a> Iterator for PairsMut<'a> {
	type Item = (&'a mut Element<'a>, Option<&'a mut Token![,]>);

	fn next(&mut self) -> Option<Self::Item> {
		if let Self(Some(pairs_mut)) = self {
			Some(match pairs_mut.next()? {
				Pair::Punctuated(element, comma) => (element, Some(comma)),
				Pair::End(element) => (element, None),
			})
		} else {
			None
		}
	}
}

// }}} constructors {{{

impl<'a> Iter<'a> {
	fn new(iter: Option<PuncIter<'a, Element<'a>>>) -> Self {
		Self(iter)
	}
}

impl IntoIter<'_> {
	fn new(into_iter: Option<PuncIntoIter<Element<'_>>>) -> Self {
		Self(into_iter)
	}
}

impl<'a> IterMut<'a> {
	fn new(iter_mut: Option<PuncIterMut<'a, Element<'a>>>) -> Self {
		Self(iter_mut)
	}
}

impl<'a> Pairs<'a> {
	fn new(pairs: Option<PuncPairs<'a, Element<'a>, Token![,]>>) -> Self {
		Self(pairs)
	}
}

impl IntoPairs<'_> {
	fn new(into_pairs: Option<PuncIntoPairs<Element<'_>, Token![,]>>) -> Self {
		Self(into_pairs)
	}
}

impl<'a> PairsMut<'a> {
	fn new(pairs_mut: Option<PuncPairsMut<'a, Element<'a>, Token![,]>>) -> Self {
		Self(pairs_mut)
	}
}

// }}} impl IntoIterator {{{

impl<'a> IntoIterator for &'a Elements<'a> {
	type IntoIter = Iter<'a>;
	type Item = &'a Element<'a>;

	fn into_iter(self) -> Self::IntoIter {
		Iter::new(Some(self.elements.iter()))
	}
}

impl<'a> IntoIterator for Elements<'a> {
	type IntoIter = IntoIter<'a>;
	type Item = Element<'a>;

	fn into_iter(self) -> Self::IntoIter {
		IntoIter::new(Some(self.elements.into_iter()))
	}
}

impl<'a> IntoIterator for &'a mut Elements<'a> {
	type IntoIter = IterMut<'a>;
	type Item = &'a mut Element<'a>;

	fn into_iter(self) -> Self::IntoIter {
		IterMut::new(Some(self.elements.iter_mut()))
	}
}

impl<'a> IntoIterator for &'a Content<'a> {
	type IntoIter = Iter<'a>;
	type Item = &'a Element<'a>;

	/// Creates an owning iterator over [`Element`]s.
	fn into_iter(self) -> Self::IntoIter {
		Iter::new(match self {
			Content::Struct {
				elements: Elements { elements, .. },
				..
			} => Some(elements.iter()),

			Content::Tuple {
				elements: Elements { elements, .. },
				..
			} => Some(elements.iter()),

			Content::Unit => None,
		})
	}
}

impl<'a> IntoIterator for Content<'a> {
	type IntoIter = IntoIter<'a>;
	type Item = Element<'a>;

	/// Creates an owning iterator over any [`Element`]s contained within.
	fn into_iter(self) -> Self::IntoIter {
		IntoIter::new(match self {
			Content::Struct {
				elements: Elements { elements, .. },
				..
			} => Some(elements.into_iter()),

			Content::Tuple {
				elements: Elements { elements, .. },
				..
			} => Some(elements.into_iter()),

			Content::Unit => None,
		})
	}
}

impl<'a> IntoIterator for &'a mut Content<'a> {
	type IntoIter = IterMut<'a>;
	type Item = &'a mut Element<'a>;

	fn into_iter(self) -> Self::IntoIter {
		IterMut::new(match self {
			Content::Struct {
				elements: Elements { elements, .. },
				..
			} => Some(elements.iter_mut()),

			Content::Tuple {
				elements: Elements { elements, .. },
				..
			} => Some(elements.iter_mut()),

			Content::Unit => None,
		})
	}
}

// }}} impl iter methods {{{

impl<'a> Elements<'a> {
	/// Creates a borrowing iterator over [`Element`]s.
	pub fn iter(&'a self) -> Iter<'a> {
		self.into_iter()
	}

	/// Creates a mutably borrowing iterator over [`Element`]s.
	pub fn iter_mut(&'a mut self) -> IterMut<'a> {
		self.into_iter()
	}

	/// Creates a borrowing iterator over pairs of [`Element`]s and possible
	/// accompanying commas.
	pub fn pairs(&'a self) -> Pairs<'a> {
		Pairs::new(Some(self.elements.pairs()))
	}

	/// Creates an owning iterator over pairs of [`Element`]s and possible
	/// accompanying commas.
	pub fn into_pairs(self) -> IntoPairs<'a> {
		IntoPairs::new(Some(self.elements.into_pairs()))
	}

	/// Creates a mutably borrowing iterator over pairs of [`Element`]s and
	/// possible accompanying commas.
	pub fn pairs_mut(&'a mut self) -> PairsMut<'a> {
		PairsMut::new(Some(self.elements.pairs_mut()))
	}
}

impl<'a> Content<'a> {
	/// Creates a borrowing iterator over any [`Element`]s contained within.
	pub fn iter(&'a self) -> Iter<'a> {
		self.into_iter()
	}

	/// Creates a mutably borrowing iterator over any [`Element`]s contained
	/// within.
	pub fn iter_mut(&'a mut self) -> IterMut<'a> {
		self.into_iter()
	}

	/// Creates a borrowing iterator over pairs of any [`Element`]s contained
	/// within and possible accompanying commas.
	pub fn pairs(&'a self) -> Pairs<'a> {
		Pairs::new(match self {
			Content::Struct {
				elements: Elements { elements, .. },
				..
			} => Some(elements.pairs()),

			Content::Tuple {
				elements: Elements { elements, .. },
				..
			} => Some(elements.pairs()),

			Content::Unit => None,
		})
	}

	/// Creates an owning iterator over pairs of any [`Element`]s contained
	/// within and possible accompanying commas.
	pub fn into_pairs(self) -> IntoPairs<'a> {
		IntoPairs::new(match self {
			Content::Struct {
				elements: Elements { elements, .. },
				..
			} => Some(elements.into_pairs()),

			Content::Tuple {
				elements: Elements { elements, .. },
				..
			} => Some(elements.into_pairs()),

			Content::Unit => None,
		})
	}

	/// Creates a mutably borrowing iterator over pairs of any [`Element`]s
	/// contained within and possible accompanying commas.
	pub fn pairs_mut(&'a mut self) -> PairsMut<'a> {
		PairsMut::new(match self {
			Content::Struct {
				elements: Elements { elements, .. },
				..
			} => Some(elements.pairs_mut()),

			Content::Tuple {
				elements: Elements { elements, .. },
				..
			} => Some(elements.pairs_mut()),

			Content::Unit => None,
		})
	}
}

// }}}
