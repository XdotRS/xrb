// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chumsky::Error;
use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use std::{
	fmt,
	fmt::{Display, Formatter},
	ops::Range,
};

#[derive(Clone)]
pub struct Span(pub proc_macro2::Span);

impl From<proc_macro2::Span> for Span {
	fn from(span: proc_macro2::Span) -> Self {
		Self(span)
	}
}

impl From<Span> for proc_macro2::Span {
	fn from(Span(span): Span) -> Self {
		span
	}
}

impl chumsky::Span for Span {
	#[cfg(not(procmacro2_semver_exempt))]
	type Context = ();
	#[cfg(procmacro2_semver_exempt)]
	type Context = proc_macro2::SourceFile;

	type Offset = proc_macro2::LineColumn;

	fn new(_: Self::Context, _: Range<Self::Offset>) -> Self {
		unimplemented!("creating a new procmacro2::Span is not allowed")
	}

	#[cfg(not(procmacro2_semver_exempt))]
	fn context(&self) {}
	#[cfg(procmacro2_semver_exempt)]
	fn context(Self(span): &Self) -> proc_macro2::SourceFile {
		span.source_file()
	}

	fn start(&self) -> Self::Offset {
		self.0.start()
	}

	fn end(&self) -> Self::Offset {
		self.0.end()
	}
}

pub enum Level {
	Error,
	Warning,
}

pub struct Diagnostic {
	pub span: proc_macro2::Span,

	pub level: Level,
	pub message: String,
}

pub struct ExpectedButFound<T: Display> {
	pub span: Span,

	pub expected: Vec<Option<T>>,
	pub found: Option<T>,

	pub label: Option<String>,
}

impl<T: Display> Error<T> for ExpectedButFound<T> {
	type Span = Span;
	type Label = String;

	fn expected_input_found<Iter: IntoIterator<Item = Option<T>>>(
		span: Self::Span, expected: Iter, found: Option<T>,
	) -> Self {
		Self {
			span,

			expected: expected.into_iter().collect(),
			found,

			label: None,
		}
	}

	fn with_label(self, label: String) -> Self {
		Self {
			label: Some(label),
			..self
		}
	}

	fn merge(mut self, mut other: Self) -> Self {
		self.expected.append(&mut other.expected);

		self
	}
}

impl<T: Display> Display for ExpectedButFound<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let expect = |element: &Option<T>| match element {
			Some(element) => format!("`{}`", element),
			None => format!("end of input"),
		};

		match self.expected.len() {
			0 => {},

			1 => write!(
				f,
				"expected {}, but ",
				expect(self.expected.first().unwrap())
			)?,

			2 => write!(
				f,
				"expected {} or {}, but ",
				expect(self.expected.first().unwrap()),
				expect(self.expected.get(1).unwrap())
			)?,

			_ => write!(
				f,
				"expected {}, or {}, but ",
				self.expected[..(self.expected.len() - 1)]
					.iter()
					.map(|element| expect(element))
					.collect::<Vec<String>>()
					.join(", "),
				expect(self.expected.last().unwrap())
			)?,
		}

		write!(f, "found {}", expect(&self.found))?;

		Ok(())
	}
}

impl<T: Display> From<ExpectedButFound<T>> for Diagnostic {
	fn from(diagnostic: ExpectedButFound<T>) -> Self {
		Self {
			span: diagnostic.span.0,
			level: Level::Error,
			message: diagnostic.to_string(),
		}
	}
}

impl<T: Display> ToTokens for ExpectedButFound<T> {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let message = self.to_string();

		quote_spanned!(self.span.0=> compile_error!(stringify!(#message));).to_tokens(tokens);
	}
}

impl ToTokens for Diagnostic {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let message = &self.message;

		quote_spanned!(self.span=> compile_error!(stringify!(#message));).to_tokens(tokens);
	}
}
