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
pub struct Iter<'a> {
	iter: Option<PuncIter<'a, ElementsItem>>,

	metabyte_element: Option<&'a Element>,
	sequence_element: Option<&'a Element>,

	minor_opcode_element: Option<&'a Element>,
	major_opcode_element: Option<&'a Element>,
	error_data_element: Option<&'a Element>,
}

/// An owning iterator over [`Element`]s.
pub struct IntoIter {
	into_iter: Option<PuncIntoIter<ElementsItem>>,

	metabyte_element: Option<Element>,
	sequence_element: Option<Element>,

	minor_opcode_element: Option<Element>,
	major_opcode_element: Option<Element>,
	error_data_element: Option<Element>,
}

/// A mutably borrowing iterator over [`Element`]s.
pub struct IterMut<'a> {
	iter_mut: Option<PuncIterMut<'a, ElementsItem>>,

	metabyte_element: Option<&'a mut Element>,
	sequence_element: Option<&'a mut Element>,

	minor_opcode_element: Option<&'a mut Element>,
	major_opcode_element: Option<&'a mut Element>,
	error_data_element: Option<&'a mut Element>,
}

/// A borrowing iterator over pairs of [`Element`]s and possible commas.
pub struct Pairs<'a> {
	pairs: Option<PuncPairs<'a, ElementsItem, Token![,]>>,

	metabyte_element: Option<&'a Element>,
	sequence_element: Option<&'a Element>,

	minor_opcode_element: Option<&'a Element>,
	major_opcode_element: Option<&'a Element>,
	error_data_element: Option<&'a Element>,
}

/// An owning iterator over pairs of [`Element`]s and possible commas.
pub struct IntoPairs {
	into_pairs: Option<PuncIntoPairs<ElementsItem, Token![,]>>,

	metabyte_element: Option<Element>,
	sequence_element: Option<Element>,

	minor_opcode_element: Option<Element>,
	major_opcode_element: Option<Element>,
	error_data_element: Option<Element>,
}

/// A mutably borrowing iterator over pairs of [`Element`]s and possible commas.
pub struct PairsMut<'a> {
	pairs_mut: Option<PuncPairsMut<'a, ElementsItem, Token![,]>>,

	metabyte_element: Option<&'a mut Element>,
	sequence_element: Option<&'a mut Element>,

	minor_opcode_element: Option<&'a mut Element>,
	major_opcode_element: Option<&'a mut Element>,
	error_data_element: Option<&'a mut Element>,
}

// impl Iterator {{{

impl<'a> Iterator for Iter<'a> {
	type Item = &'a Element;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(iter) = &mut self.iter {
			match iter.next()? {
				ElementsItem::Element(element) => Some(element),

				ElementsItem::Metabyte => self.metabyte_element,
				ElementsItem::Sequence => self.sequence_element,

				ElementsItem::MinorOpcode => self.minor_opcode_element,
				ElementsItem::MajorOpcode => self.major_opcode_element,
				ElementsItem::ErrorData => self.error_data_element,
			}
		} else {
			None
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		if let Some(iter) = &self.iter {
			iter.size_hint()
		} else {
			(0, Some(0))
		}
	}
}

impl Iterator for IntoIter {
	type Item = Element;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(into_iter) = &mut self.into_iter {
			match into_iter.next()? {
				ElementsItem::Element(element) => Some(element),

				ElementsItem::Metabyte => self.metabyte_element.take(),
				ElementsItem::Sequence => self.sequence_element.take(),

				ElementsItem::MinorOpcode => self.minor_opcode_element.take(),
				ElementsItem::MajorOpcode => self.major_opcode_element.take(),
				ElementsItem::ErrorData => self.error_data_element.take(),
			}
		} else {
			None
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		if let Some(into_iter) = &self.into_iter {
			into_iter.size_hint()
		} else {
			(0, Some(0))
		}
	}
}

impl<'a> Iterator for IterMut<'a> {
	type Item = &'a mut Element;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(iter_mut) = &mut self.iter_mut {
			match iter_mut.next()? {
				ElementsItem::Element(element) => Some(element),

				ElementsItem::Metabyte => self.metabyte_element.take(),
				ElementsItem::Sequence => self.sequence_element.take(),

				ElementsItem::MinorOpcode => self.minor_opcode_element.take(),
				ElementsItem::MajorOpcode => self.major_opcode_element.take(),
				ElementsItem::ErrorData => self.error_data_element.take(),
			}
		} else {
			None
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		if let Some(iter_mut) = &self.iter_mut {
			iter_mut.size_hint()
		} else {
			(0, Some(0))
		}
	}
}

impl<'a> Iterator for Pairs<'a> {
	type Item = (&'a Element, Option<&'a Token![,]>);

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(pairs) = &mut self.pairs {
			let (item, comma) = match pairs.next()? {
				Pair::Punctuated(item, comma) => (item, Some(comma)),
				Pair::End(item) => (item, None),
			};

			let element = match item {
				ElementsItem::Element(element) => Some(element),

				ElementsItem::Metabyte => self.metabyte_element.take(),
				ElementsItem::Sequence => self.sequence_element.take(),

				ElementsItem::MinorOpcode => self.minor_opcode_element.take(),
				ElementsItem::MajorOpcode => self.major_opcode_element.take(),
				ElementsItem::ErrorData => self.error_data_element.take(),
			};

			element.map(|element| (element, comma))
		} else {
			None
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		if let Some(pairs) = &self.pairs {
			pairs.size_hint()
		} else {
			(0, Some(0))
		}
	}
}

impl Iterator for IntoPairs {
	type Item = (Element, Option<Token![,]>);

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(into_pairs) = &mut self.into_pairs {
			let (item, comma) = match into_pairs.next()? {
				Pair::Punctuated(item, comma) => (item, Some(comma)),
				Pair::End(item) => (item, None),
			};

			let element = match item {
				ElementsItem::Element(element) => Some(element),

				ElementsItem::Metabyte => self.metabyte_element.take(),
				ElementsItem::Sequence => self.sequence_element.take(),

				ElementsItem::MinorOpcode => self.minor_opcode_element.take(),
				ElementsItem::MajorOpcode => self.major_opcode_element.take(),
				ElementsItem::ErrorData => self.error_data_element.take(),
			};

			element.map(|element| (element, comma))
		} else {
			None
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		if let Some(into_pairs) = &self.into_pairs {
			into_pairs.size_hint()
		} else {
			(0, Some(0))
		}
	}
}

impl<'a> Iterator for PairsMut<'a> {
	type Item = (&'a mut Element, Option<&'a mut Token![,]>);

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(pairs_mut) = &mut self.pairs_mut {
			let (item, comma) = match pairs_mut.next()? {
				Pair::Punctuated(item, comma) => (item, Some(comma)),
				Pair::End(item) => (item, None),
			};

			let element = match item {
				ElementsItem::Element(element) => Some(element),

				ElementsItem::Metabyte => self.metabyte_element.take(),
				ElementsItem::Sequence => self.sequence_element.take(),

				ElementsItem::MinorOpcode => self.minor_opcode_element.take(),
				ElementsItem::MajorOpcode => self.major_opcode_element.take(),
				ElementsItem::ErrorData => self.error_data_element.take(),
			};

			element.map(|element| (element, comma))
		} else {
			None
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		if let Some(pairs_mut) = &self.pairs_mut {
			pairs_mut.size_hint()
		} else {
			(0, Some(0))
		}
	}
}

// }}} with_none {{{

impl Iter<'_> {
	fn with_none() -> Self {
		Self {
			iter: None,

			metabyte_element: None,
			sequence_element: None,

			minor_opcode_element: None,
			major_opcode_element: None,
			error_data_element: None,
		}
	}
}

impl IntoIter {
	fn with_none() -> Self {
		Self {
			into_iter: None,

			metabyte_element: None,
			sequence_element: None,

			minor_opcode_element: None,
			major_opcode_element: None,
			error_data_element: None,
		}
	}
}

impl IterMut<'_> {
	fn with_none() -> Self {
		Self {
			iter_mut: None,

			metabyte_element: None,
			sequence_element: None,

			minor_opcode_element: None,
			major_opcode_element: None,
			error_data_element: None,
		}
	}
}

impl Pairs<'_> {
	fn with_none() -> Self {
		Self {
			pairs: None,

			metabyte_element: None,
			sequence_element: None,

			minor_opcode_element: None,
			major_opcode_element: None,
			error_data_element: None,
		}
	}
}

impl IntoPairs {
	fn with_none() -> Self {
		Self {
			into_pairs: None,

			metabyte_element: None,
			sequence_element: None,

			minor_opcode_element: None,
			major_opcode_element: None,
			error_data_element: None,
		}
	}
}

impl PairsMut<'_> {
	fn with_none() -> Self {
		Self {
			pairs_mut: None,

			metabyte_element: None,
			sequence_element: None,

			minor_opcode_element: None,
			major_opcode_element: None,
			error_data_element: None,
		}
	}
}

// }}} impl IntoIterator {{{

impl<'a> IntoIterator for &'a Elements {
	type Item = &'a Element;
	type IntoIter = Iter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		Iter {
			iter: Some(self.elements.iter()),

			metabyte_element: self.metabyte_element.as_ref(),
			sequence_element: self.sequence_element.as_ref(),

			minor_opcode_element: self.minor_opcode_element.as_ref(),
			major_opcode_element: self.major_opcode_element.as_ref(),
			error_data_element: self.error_data_element.as_ref(),
		}
	}
}

impl IntoIterator for Elements {
	type Item = Element;
	type IntoIter = IntoIter;

	/// Creates an owning iterator over [`Element`]s.
	fn into_iter(self) -> Self::IntoIter {
		IntoIter {
			into_iter: Some(self.elements.into_iter()),

			metabyte_element: self.metabyte_element,
			sequence_element: self.sequence_element,

			minor_opcode_element: self.minor_opcode_element,
			major_opcode_element: self.major_opcode_element,
			error_data_element: self.error_data_element,
		}
	}
}

impl<'a> IntoIterator for &'a mut Elements {
	type Item = &'a mut Element;
	type IntoIter = IterMut<'a>;

	fn into_iter(self) -> Self::IntoIter {
		IterMut {
			iter_mut: Some(self.elements.iter_mut()),

			metabyte_element: self.metabyte_element.as_mut(),
			sequence_element: self.sequence_element.as_mut(),

			minor_opcode_element: self.minor_opcode_element.as_mut(),
			major_opcode_element: self.major_opcode_element.as_mut(),
			error_data_element: self.error_data_element.as_mut(),
		}
	}
}

macro_rules! impl_content {
	($content:ty) => {
		impl<'a> IntoIterator for &'a $content {
			type Item = &'a Element;
			type IntoIter = Iter<'a>;

			fn into_iter(self) -> Self::IntoIter {
				self.elements.iter()
			}
		}

		impl IntoIterator for $content {
			type Item = Element;
			type IntoIter = IntoIter;

			/// Creates an owning iterator over [`Element`]s.
			fn into_iter(self) -> Self::IntoIter {
				self.elements.into_iter()
			}
		}

		impl<'a> IntoIterator for &'a mut $content {
			type Item = &'a mut Element;
			type IntoIter = IterMut<'a>;

			fn into_iter(self) -> Self::IntoIter {
				self.elements.iter_mut()
			}
		}

		impl<'a> $content {
			/// Creates a borrowing iterator over [`Element`]s.
			pub fn iter(&'a self) -> Iter<'a> {
				self.into_iter()
			}

			/// Creates a mutably borrowing iterator over [`Element`]s.
			pub fn iter_mut(&'a mut self) -> IterMut<'a> {
				self.into_iter()
			}

			/// Creates a borrowing iterator over pairs of [`Elements`]s and possible
			/// accompanying commas.
			pub fn pairs(&'a self) -> Pairs<'a> {
				self.elements.pairs()
			}

			/// Creates an owning iterator over pairs of [`Element`]s and possible
			/// accompanying commas.
			pub fn into_pairs(self) -> IntoPairs {
				self.elements.into_pairs()
			}

			/// Creates a mutably borrowing iterator over pairs of [`Element`]s and
			/// possible accompanying commas.
			pub fn pairs_mut(&'a mut self) -> PairsMut<'a> {
				self.elements.pairs_mut()
			}
		}
	};
}

impl_content!(RegularContent);
impl_content!(TupleContent);

impl<'a> IntoIterator for &'a Content {
	type Item = &'a Element;
	type IntoIter = Iter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		match self {
			Content::Regular(content) => content.iter(),
			Content::Tuple(content) => content.iter(),
			Content::Unit => Iter::with_none(),
		}
	}
}

impl IntoIterator for Content {
	type Item = Element;
	type IntoIter = IntoIter;

	/// Creates an owning iterator over any [`Element`]s contained within.
	fn into_iter(self) -> Self::IntoIter {
		match self {
			Content::Regular(content) => content.into_iter(),
			Content::Tuple(content) => content.into_iter(),
			Content::Unit => IntoIter::with_none(),
		}
	}
}

impl<'a> IntoIterator for &'a mut Content {
	type Item = &'a mut Element;
	type IntoIter = IterMut<'a>;

	fn into_iter(self) -> Self::IntoIter {
		match self {
			Content::Regular(content) => content.iter_mut(),
			Content::Tuple(content) => content.iter_mut(),
			Content::Unit => IterMut::with_none(),
		}
	}
}

impl<'a> IntoIterator for &'a StructlikeContent {
	type Item = &'a Element;
	type IntoIter = Iter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		match self {
			StructlikeContent::Regular { content, .. } => content.iter(),
			StructlikeContent::Tuple { content, .. } => content.iter(),
			StructlikeContent::Unit { .. } => Iter::with_none(),
		}
	}
}

impl IntoIterator for StructlikeContent {
	type Item = Element;
	type IntoIter = IntoIter;

	/// Creates an owning iterator over any [`Element`]s contained within.
	fn into_iter(self) -> Self::IntoIter {
		match self {
			StructlikeContent::Regular { content, .. } => content.into_iter(),
			StructlikeContent::Tuple { content, .. } => content.into_iter(),
			StructlikeContent::Unit { .. } => IntoIter::with_none(),
		}
	}
}

impl<'a> IntoIterator for &'a mut StructlikeContent {
	type Item = &'a mut Element;
	type IntoIter = IterMut<'a>;

	fn into_iter(self) -> Self::IntoIter {
		match self {
			StructlikeContent::Regular { content, .. } => content.iter_mut(),
			StructlikeContent::Tuple { content, .. } => content.iter_mut(),
			StructlikeContent::Unit { .. } => IterMut::with_none(),
		}
	}
}

// }}} impl iter methods {{{

impl<'a> Elements {
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
		Pairs {
			pairs: Some(self.elements.pairs()),

			metabyte_element: self.metabyte_element.as_ref(),
			sequence_element: self.sequence_element.as_ref(),

			minor_opcode_element: self.minor_opcode_element.as_ref(),
			major_opcode_element: self.major_opcode_element.as_ref(),
			error_data_element: self.error_data_element.as_ref(),
		}
	}

	/// Creates an owning iterator over pairs of [`Element`]s and possible
	/// accompanying commas.
	pub fn into_pairs(self) -> IntoPairs {
		IntoPairs {
			into_pairs: Some(self.elements.into_pairs()),

			metabyte_element: self.metabyte_element,
			sequence_element: self.sequence_element,

			minor_opcode_element: self.minor_opcode_element,
			major_opcode_element: self.major_opcode_element,
			error_data_element: self.error_data_element,
		}
	}

	/// Creates a mutably borrowing iterator over pairs of [`Element`]s and
	/// possible accompanying commas.
	pub fn pairs_mut(&'a mut self) -> PairsMut<'a> {
		PairsMut {
			pairs_mut: Some(self.elements.pairs_mut()),

			metabyte_element: self.metabyte_element.as_mut(),
			sequence_element: self.sequence_element.as_mut(),

			minor_opcode_element: self.minor_opcode_element.as_mut(),
			major_opcode_element: self.major_opcode_element.as_mut(),
			error_data_element: self.error_data_element.as_mut(),
		}
	}
}

impl<'a> Content {
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
		match self {
			Content::Regular(content) => content.pairs(),
			Content::Tuple(content) => content.pairs(),
			Content::Unit => Pairs::with_none(),
		}
	}

	/// Creates an owning iterator over pairs of any [`Element`]s contained
	/// within and possible accompanying commas.
	pub fn into_pairs(self) -> IntoPairs {
		match self {
			Content::Regular(content) => content.into_pairs(),
			Content::Tuple(content) => content.into_pairs(),
			Content::Unit => IntoPairs::with_none(),
		}
	}

	/// Creates a mutably borrowing iterator over pairs of any [`Element`]s
	/// contained within and possible accompanying commas.
	pub fn pairs_mut(&'a mut self) -> PairsMut<'a> {
		match self {
			Content::Regular(content) => content.pairs_mut(),
			Content::Tuple(content) => content.pairs_mut(),
			Content::Unit => PairsMut::with_none(),
		}
	}
}

impl<'a> StructlikeContent {
	/// Creates a borrowing iterator over [`Element`]s contained within.
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
		match self {
			StructlikeContent::Regular { content, .. } => content.pairs(),
			StructlikeContent::Tuple { content, .. } => content.pairs(),
			StructlikeContent::Unit { .. } => Pairs::with_none(),
		}
	}

	/// Creates an owning iterator over pairs of any [`Element`]s contained
	/// within and possible accompanying commas.
	pub fn into_pairs(self) -> IntoPairs {
		match self {
			StructlikeContent::Regular { content, .. } => content.into_pairs(),
			StructlikeContent::Tuple { content, .. } => content.into_pairs(),
			StructlikeContent::Unit { .. } => IntoPairs::with_none(),
		}
	}

	/// Creates a mutably borrowing iterator over pairs of any [`Element`]s
	/// contained within and possible accompanying commas.
	pub fn pairs_mut(&'a mut self) -> PairsMut<'a> {
		match self {
			StructlikeContent::Regular { content, .. } => content.pairs_mut(),
			StructlikeContent::Tuple { content, .. } => content.pairs_mut(),
			StructlikeContent::Unit { .. } => PairsMut::with_none(),
		}
	}
}

// }}}
