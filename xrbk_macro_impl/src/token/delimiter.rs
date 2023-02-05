// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::error::ExpectedButFound;
use chumsky::prelude::*;
use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};

pub struct NormalBrackets {
	pub open_span: Span,
	pub span: Span,
	pub close_span: Span,
}
pub struct CurlyBrackets {
	pub open_span: Span,
	pub span: Span,
	pub close_span: Span,
}
pub struct SquareBrackets {
	pub open_span: Span,
	pub span: Span,
	pub close_span: Span,
}

pub enum Group {
	/// Tokens surrounded by `(` and `)`.
	NormalBrackets(NormalBracketsGroup),
	/// Tokens surrounded by `{` and `}`.
	CurlyBrackets(CurlyBracketsGroup),
	/// Tokens surrounded by `[` and `]`.
	SquareBrackets(SquareBracketsGroup),
	/// Tokens surrounded by 'invisible' delimiters.
	NoDelimiters(NoDelimitersGroup),
}

/// Tokens surrounded by `(` and `)`.
pub struct NormalBracketsGroup {
	pub open_span: Span,
	pub close_span: Span,

	pub tokens_span: Span,
	pub tokens: TokenStream,
}
/// Tokens surrounded by `{` and `}`.
pub struct CurlyBracketsGroup {
	pub open_span: Span,
	pub close_span: Span,

	pub tokens_span: Span,
	pub tokens: TokenStream,
}
/// Tokens surrounded by `[` and `]`.
pub struct SquareBracketsGroup {
	pub open_span: Span,
	pub close_span: Span,

	pub tokens_span: Span,
	pub tokens: TokenStream,
}
/// Tokens surrounded by 'invisible' delimiters.
///
/// This still has the semantic effect of delimiters, but with no tokens to
/// represent those delimiters.
pub struct NoDelimitersGroup {
	pub open_span: Span,
	pub close_span: Span,

	pub tokens_span: Span,
	pub tokens: TokenStream,
}

impl NormalBracketsGroup {
	pub fn parser(
	) -> impl Parser<TokenTree, NormalBracketsGroup, Error = ExpectedButFound<TokenTree>> {
		filter_map(|span, token| match token {
			TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
				Ok(NormalBracketsGroup {
					open_span: group.span_open(),
					close_span: group.span_close(),

					tokens_span: group.span(),
					tokens: group.stream(),
				})
			},

			_ => Err(
				ExpectedButFound::expected_input_found(span, [], Some(token))
					.with_label("expected tokens delimited by `(` and `)`".into()),
			),
		})
	}
}

impl CurlyBracketsGroup {
	pub fn parser(
	) -> impl Parser<TokenTree, CurlyBracketsGroup, Error = ExpectedButFound<TokenTree>> {
		filter_map(|span, token| match token {
			TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
				Ok(CurlyBracketsGroup {
					open_span: group.span_open(),
					close_span: group.span_close(),

					tokens_span: group.span(),
					tokens: group.stream(),
				})
			},

			_ => Err(
				ExpectedButFound::expected_input_found(span, [], Some(token))
					.with_label("expected tokens delimited by `{` and `}`".into()),
			),
		})
	}
}

impl SquareBracketsGroup {
	pub fn parser(
	) -> impl Parser<TokenTree, SquareBracketsGroup, Error = ExpectedButFound<TokenTree>> {
		filter_map(|span, token| match token {
			TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
				Ok(SquareBracketsGroup {
					open_span: group.span_open(),
					close_span: group.span_close(),

					tokens_span: group.span(),
					tokens: group.stream(),
				})
			},

			_ => Err(
				ExpectedButFound::expected_input_found(span, [], Some(token))
					.with_label("expected tokens delimited by `[` and `]`".into()),
			),
		})
	}
}

impl NoDelimitersGroup {
	pub fn parser() -> impl Parser<TokenTree, NoDelimitersGroup, Error = ExpectedButFound<TokenTree>>
	{
		filter_map(|span, token| match token {
			TokenTree::Group(group) if group.delimiter() == Delimiter::None => {
				Ok(NoDelimitersGroup {
					open_span: group.span_open(),
					close_span: group.span_close(),

					tokens_span: group.span(),
					tokens: group.stream(),
				})
			},

			_ => Err(ExpectedButFound::expected_input_found(
				span,
				[],
				Some(token),
			)),
		})
	}
}
