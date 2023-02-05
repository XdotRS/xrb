// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::error::ExpectedButFound;
use chumsky::prelude::*;
use proc_macro2::{Punct, Spacing, Span, TokenTree};

/// This macro is very much inspired by `syn`'s `Token!` macro.
#[macro_export]
#[allow(non_snake_case)]
macro_rules! Punct {
	(=) => {
		$crate::token::punct::SingleEquals
	};
	(==) => {
		$crate::token::punct::DoubleEquals
	};

	(+) => {
		$crate::token::punct::Plus
	};
	(-) => {
		$crate::token::punct::Minus
	};

	(+=) => {
		$crate::token::punct::PlusEquals
	};
	(-=) => {
		$crate::token::punct::MinusEquals
	};

	(*) => {
		$crate::token::punct::Star
	};
	(/) => {
		$crate::token::punct::Slash
	};

	(*=) => {
		$crate::token::punct::StarEquals
	};
	(/=) => {
		$crate::token::punct::SlashEquals
	};

	(%) => {
		$crate::token::punct::Percent
	};
	(^) => {
		$crate::token::punct::Caret
	};
	(!) => {
		$crate::token::punct::Exclamation
	};

	(%=) => {
		$crate::token::punct::PercentEquals
	};
	(^=) => {
		$crate::token::punct::CaretEquals
	};
	(!=) => {
		$crate::token::punct::ExclamationEquals
	};

	(&) => {
		$crate::token::punct::SingleAnd
	};
	(|) => {
		$crate::token::punct::SingleOr
	};

	(&&) => {
		$crate::token::punct::DoubleAnd
	};
	(||) => {
		$crate::token::punct::DoubleOr
	};

	(&=) => {
		$crate::token::punct::SingleAndEquals
	};
	(|=) => {
		$crate::token::punct::SingleOrEquals
	};

	(<) => {
		$crate::token::punct::SingleLeftArrowBracket
	};
	(>) => {
		$crate::token::punct::SingleRightArrowBracket
	};

	(<=) => {
		$crate::token::punct::SingleLeftArrowBracketEquals
	};
	(>=) => {
		$crate::token::punct::SingleRightArrowBracketEquals
	};

	(<<) => {
		$crate::token::punct::DoubleLeftArrowBracket
	};
	(>>) => {
		$crate::token::punct::DoubleRightArrowBracket
	};

	(<<=) => {
		$crate::token::punct::DoubleLeftArrowBracketEquals
	};
	(>>=) => {
		$crate::token::punct::DoubleRightArrowBracketEquals
	};

	(@) => {
		$crate::token::punct::At
	};
	(_) => {
		$crate::token::punct::Underscore
	};

	(.) => {
		$crate::token::punct::SingleDot
	};
	(..) => {
		$crate::token::punct::DoubleDot
	};
	(...) => {
		$crate::token::punct::TripleDot
	};

	(..=) => {
		$crate::token::punct::DoubleDotEquals
	};

	(,) => {
		$crate::token::punct::Comma
	};
	(;) => {
		$crate::token::punct::Semicolon
	};

	(:) => {
		$crate::token::punct::SingleColon
	};
	(::) => {
		$crate::token::punct::DoubleColon
	};

	(->) => {
		$crate::token::punct::MinusRightArrowBracket
	};
	(=>) => {
		$crate::token::punct::EqualsRightArrowBracket
	};

	(#) => {
		$crate::token::punct::Hash
	};
	($) => {
		$crate::token::punct::Dollar
	};
	(?) => {
		$crate::token::punct::Question
	};
	(~) => {
		$crate::token::punct::Tilde
	};
}

pub enum Punctuation {
	SingleEquals(Punct![=]),
	DoubleEquals(Punct![==]),

	Plus(Punct![+]),
	Minus(Punct![-]),

	PlusEquals(Punct![+=]),
	MinusEquals(Punct![-=]),

	Star(Punct![*]),
	Slash(Punct![/]),

	StarEquals(Punct![*=]),
	SlashEquals(Punct![/=]),

	Percent(Punct![%]),
	Caret(Punct![^]),
	Exclamation(Punct![!]),

	PercentEquals(Punct![%=]),
	CaretEquals(Punct![^=]),
	ExclamationEquals(Punct![!=]),

	SingleAnd(Punct![&]),
	SingleOr(Punct![|]),

	SingleAndEquals(Punct![&=]),
	SingleOrEquals(Punct![|=]),

	DoubleAnd(Punct![&&]),
	DoubleOr(Punct![||]),

	SingleLeftArrowBracket(Punct![<]),
	SingleRightArrowBracket(Punct![>]),

	SingleLeftArrowBracketEquals(Punct![<=]),
	SingleRightArrowBracketEquals(Punct![>=]),

	DoubleLeftArrowBracket(Punct![<<]),
	DoubleRightArrowBracket(Punct![>>]),

	DoubleLeftArrowBracketEquals(Punct![<<=]),
	DoubleRightArrowBracketEquals(Punct![>>=]),

	At(Punct![@]),
	Underscore(Punct![_]),

	SingleDot(Punct![.]),
	DoubleDot(Punct![..]),
	TripleDot(Punct![...]),

	DoubleDotEquals(Punct![..=]),

	Comma(Punct![,]),
	Semicolon(Punct![;]),

	SingleColon(Punct![:]),
	DoubleColon(Punct![::]),

	MinusRightArrowBracket(Punct![->]),
	EqualsRightArrowBracket(Punct![=>]),

	Hash(Punct![#]),
	Dollar(Punct![$]),
	Question(Punct![?]),
	Tilde(Punct![~]),
}

/// `=`
pub struct SingleEquals(pub Span);
/// `==`
pub struct DoubleEquals(pub Span, pub Span);

/// `+`
pub struct Plus(pub Span);
/// `-`
pub struct Minus(pub Span);

/// `+=`
pub struct PlusEquals(pub Span, pub Span);
/// `-=`
pub struct MinusEquals(pub Span, pub Span);

/// `*`
pub struct Star(pub Span);
/// `/`
pub struct Slash(pub Span);

/// `*=`
pub struct StarEquals(pub Span, pub Span);
/// `/=`
pub struct SlashEquals(pub Span, pub Span);

/// `%`
pub struct Percent(pub Span);
/// `^`
pub struct Caret(pub Span);
/// `!`
pub struct Exclamation(pub Span);

/// `%=`
pub struct PercentEquals(pub Span, pub Span);
/// `^=`
pub struct CaretEquals(pub Span, pub Span);
/// `!=`
pub struct ExclamationEquals(pub Span, pub Span);

/// `&`
pub struct SingleAnd(pub Span);
/// `|`
pub struct SingleOr(pub Span);

/// `&=`
pub struct SingleAndEquals(pub Span, pub Span);
/// `|=`
pub struct SingleOrEquals(pub Span, pub Span);

/// `&&`
pub struct DoubleAnd(pub Span, pub Span);
/// `||`
pub struct DoubleOr(pub Span, pub Span);

/// `<`
pub struct SingleLeftArrowBracket(pub Span);
/// `>`
pub struct SingleRightArrowBracket(pub Span);

/// `<=`
pub struct SingleLeftArrowBracketEquals(pub Span, pub Span);
/// `>=`
pub struct SingleRightArrowBracketEquals(pub Span, pub Span);

/// `<<`
pub struct DoubleLeftArrowBracket(pub Span, pub Span);
/// `>>`
pub struct DoubleRightArrowBracket(pub Span, pub Span);

/// `<<=`
pub struct DoubleLeftArrowBracketEquals(pub Span, pub Span, pub Span);
/// `>>=`
pub struct DoubleRightArrowBracketEquals(pub Span, pub Span, pub Span);

/// `@`
pub struct At(pub Span);
/// `_`
pub struct Underscore(pub Span);

/// `.`
pub struct SingleDot(pub Span);
/// `..`
pub struct DoubleDot(pub Span, pub Span);
/// `...`
pub struct TripleDot(pub Span, pub Span, pub Span);

/// `..=`
pub struct DoubleDotEquals(pub Span, pub Span, pub Span);

/// `,`
pub struct Comma(pub Span);
/// `;`
pub struct Semicolon(pub Span);

/// `:`
pub struct SingleColon(pub Span);
/// `::`
pub struct DoubleColon(pub Span, pub Span);

/// `->`
pub struct MinusRightArrowBracket(pub Span, pub Span);
/// `=>`
pub struct EqualsRightArrowBracket(pub Span, pub Span);

/// `#`
pub struct Hash(pub Span);
/// `$`
pub struct Dollar(pub Span);
/// `?`
pub struct Question(pub Span);
/// `~`
pub struct Tilde(pub Span);

fn joint(char: char) -> impl Parser<TokenTree, Punct, Error = ExpectedButFound<TokenTree>> {
	filter_map(move |span, token| match token {
		TokenTree::Punct(punct) if punct.spacing() == Spacing::Joint && punct.as_char() == char => {
			Ok(punct)
		},

		_ => Err(ExpectedButFound::expected_input_found(
			span,
			[Some(TokenTree::Punct(Punct::new(char, Spacing::Joint)))],
			Some(token),
		)),
	})
}

fn alone(char: char) -> impl Parser<TokenTree, Punct, Error = ExpectedButFound<TokenTree>> {
	filter_map(move |span, token| match token {
		TokenTree::Punct(punct) if punct.spacing() == Spacing::Alone && punct.as_char() == char => {
			Ok(punct)
		},

		_ => Err(ExpectedButFound::expected_input_found(
			span,
			[Some(TokenTree::Punct(Punct::new(char, Spacing::Alone)))],
			Some(token),
		)),
	})
}

impl Punct![=] {
	pub fn parser() -> impl Parser<TokenTree, Punct![=], Error = ExpectedButFound<TokenTree>> {
		alone('=').map(|p| Punct![=](p.span()))
	}
}
impl Punct![==] {
	pub fn parser() -> impl Parser<TokenTree, Punct![==], Error = ExpectedButFound<TokenTree>> {
		joint('=')
			.then(alone('='))
			.map(|(p1, p2)| Punct![==](p1.span(), p2.span()))
	}
}

impl Punct![+] {
	pub fn parser() -> impl Parser<TokenTree, Punct![+], Error = ExpectedButFound<TokenTree>> {
		alone('+').map(|p| Punct![+](p.span()))
	}
}
impl Punct![-] {
	pub fn parser() -> impl Parser<TokenTree, Punct![-], Error = ExpectedButFound<TokenTree>> {
		alone('-').map(|p| Punct![-](p.span()))
	}
}

impl Punct![+=] {
	pub fn parser() -> impl Parser<TokenTree, Punct![+=], Error = ExpectedButFound<TokenTree>> {
		joint('+')
			.then(alone('='))
			.map(|(p1, p2)| Punct![+=](p1.span(), p2.span()))
	}
}
impl Punct![-=] {
	pub fn parser() -> impl Parser<TokenTree, Punct![-=], Error = ExpectedButFound<TokenTree>> {
		joint('-')
			.then(alone('='))
			.map(|(p1, p2)| Punct![-=](p1.span(), p2.span()))
	}
}

impl Punct![*] {
	pub fn parser() -> impl Parser<TokenTree, Punct![*], Error = ExpectedButFound<TokenTree>> {
		alone('*').map(|p| Punct![*](p.span()))
	}
}
impl Punct![/] {
	pub fn parser() -> impl Parser<TokenTree, Punct![/], Error = ExpectedButFound<TokenTree>> {
		alone('/').map(|p| Punct![/](p.span()))
	}
}

impl Punct![*=] {
	pub fn parser() -> impl Parser<TokenTree, Punct![*=], Error = ExpectedButFound<TokenTree>> {
		joint('*')
			.then(alone('='))
			.map(|(p1, p2)| Punct![*=](p1.span(), p2.span()))
	}
}
impl Punct![/=] {
	pub fn parser() -> impl Parser<TokenTree, Punct![/=], Error = ExpectedButFound<TokenTree>> {
		joint('/')
			.then(alone('='))
			.map(|(p1, p2)| Punct![/=](p1.span(), p2.span()))
	}
}

impl Punct![%] {
	pub fn parser() -> impl Parser<TokenTree, Punct![%], Error = ExpectedButFound<TokenTree>> {
		alone('%').map(|p| Punct![%](p.span()))
	}
}
impl Punct![^] {
	pub fn parser() -> impl Parser<TokenTree, Punct![^], Error = ExpectedButFound<TokenTree>> {
		alone('^').map(|p| Punct![^](p.span()))
	}
}
impl Punct![!] {
	pub fn parser() -> impl Parser<TokenTree, Punct![!], Error = ExpectedButFound<TokenTree>> {
		alone('!').map(|p| Punct![!](p.span()))
	}
}

impl Punct![%=] {
	pub fn parser() -> impl Parser<TokenTree, Punct![%=], Error = ExpectedButFound<TokenTree>> {
		joint('%')
			.then(alone('='))
			.map(|(p1, p2)| Punct![%=](p1.span(), p2.span()))
	}
}
impl Punct![^=] {
	pub fn parser() -> impl Parser<TokenTree, Punct![^=], Error = ExpectedButFound<TokenTree>> {
		joint('^')
			.then(alone('='))
			.map(|(p1, p2)| Punct![^=](p1.span(), p2.span()))
	}
}
impl Punct![!=] {
	pub fn parser() -> impl Parser<TokenTree, Punct![!=], Error = ExpectedButFound<TokenTree>> {
		joint('!')
			.then(alone('='))
			.map(|(p1, p2)| Punct![!=](p1.span(), p2.span()))
	}
}

impl Punct![&] {
	pub fn parser() -> impl Parser<TokenTree, Punct![&], Error = ExpectedButFound<TokenTree>> {
		alone('&').map(|p| Punct![&](p.span()))
	}
}
impl Punct![|] {
	pub fn parser() -> impl Parser<TokenTree, Punct![|], Error = ExpectedButFound<TokenTree>> {
		alone('|').map(|p| Punct![|](p.span()))
	}
}

impl Punct![&=] {
	pub fn parser() -> impl Parser<TokenTree, Punct![&=], Error = ExpectedButFound<TokenTree>> {
		joint('&')
			.then(alone('='))
			.map(|(p1, p2)| Punct![&=](p1.span(), p2.span()))
	}
}
impl Punct![|=] {
	pub fn parser() -> impl Parser<TokenTree, Punct![|=], Error = ExpectedButFound<TokenTree>> {
		joint('|')
			.then(alone('='))
			.map(|(p1, p2)| Punct![|=](p1.span(), p2.span()))
	}
}

impl Punct![&&] {
	pub fn parser() -> impl Parser<TokenTree, Punct![&&], Error = ExpectedButFound<TokenTree>> {
		joint('&')
			.then(alone('&'))
			.map(|(p1, p2)| Punct![&&](p1.span(), p2.span()))
	}
}
impl Punct![||] {
	pub fn parser() -> impl Parser<TokenTree, Punct![||], Error = ExpectedButFound<TokenTree>> {
		joint('|')
			.then(alone('|'))
			.map(|(p1, p2)| Punct![||](p1.span(), p2.span()))
	}
}

impl Punct![<] {
	pub fn parser() -> impl Parser<TokenTree, Punct![<], Error = ExpectedButFound<TokenTree>> {
		alone('<').map(|p| Punct![<](p.span()))
	}
}
impl Punct![>] {
	pub fn parser() -> impl Parser<TokenTree, Punct![>], Error = ExpectedButFound<TokenTree>> {
		alone('>').map(|p| Punct![>](p.span()))
	}
}

impl Punct![<=] {
	pub fn parser() -> impl Parser<TokenTree, Punct![<=], Error = ExpectedButFound<TokenTree>> {
		joint('<')
			.then(alone('='))
			.map(|(p1, p2)| Punct![<=](p1.span(), p2.span()))
	}
}
impl Punct![>=] {
	pub fn parser() -> impl Parser<TokenTree, Punct![>=], Error = ExpectedButFound<TokenTree>> {
		joint('>')
			.then(alone('='))
			.map(|(p1, p2)| Punct![>=](p1.span(), p2.span()))
	}
}

impl Punct![<<] {
	pub fn parser() -> impl Parser<TokenTree, Punct![<<], Error = ExpectedButFound<TokenTree>> {
		joint('<')
			.then(alone('<'))
			.map(|(p1, p2)| Punct![<<](p1.span(), p2.span()))
	}
}
impl Punct![>>] {
	pub fn parser() -> impl Parser<TokenTree, Punct![>>], Error = ExpectedButFound<TokenTree>> {
		joint('>')
			.then(alone('>'))
			.map(|(p1, p2)| Punct![>>](p1.span(), p2.span()))
	}
}

impl Punct![<<=] {
	pub fn parser() -> impl Parser<TokenTree, Punct![<<=], Error = ExpectedButFound<TokenTree>> {
		joint('<')
			.then(joint('<'))
			.then(alone('='))
			.map(|((p1, p2), p3)| Punct![<<=](p1.span(), p2.span(), p3.span()))
	}
}
impl Punct![>>=] {
	pub fn parser() -> impl Parser<TokenTree, Punct![>>=], Error = ExpectedButFound<TokenTree>> {
		joint('>')
			.then(joint('>'))
			.then(alone('='))
			.map(|((p1, p2), p3)| Punct![>>=](p1.span(), p2.span(), p3.span()))
	}
}

impl Punct![@] {
	pub fn parser() -> impl Parser<TokenTree, Punct![@], Error = ExpectedButFound<TokenTree>> {
		alone('@').map(|p| Punct![@](p.span()))
	}
}
impl Punct![_] {
	pub fn parser() -> impl Parser<TokenTree, Punct![_], Error = ExpectedButFound<TokenTree>> {
		alone('_').map(|p| Punct![_](p.span()))
	}
}

impl Punct![.] {
	pub fn parser() -> impl Parser<TokenTree, Punct![.], Error = ExpectedButFound<TokenTree>> {
		alone('.').map(|p| Punct![.](p.span()))
	}
}
impl Punct![..] {
	pub fn parser() -> impl Parser<TokenTree, Punct![..], Error = ExpectedButFound<TokenTree>> {
		joint('.')
			.then(alone('.'))
			.map(|(p1, p2)| Punct![..](p1.span(), p2.span()))
	}
}
impl Punct![...] {
	pub fn parser() -> impl Parser<TokenTree, Punct![...], Error = ExpectedButFound<TokenTree>> {
		joint('.')
			.then(joint('.'))
			.then(alone('.'))
			.map(|((p1, p2), p3)| Punct![...](p1.span(), p2.span(), p3.span()))
	}
}

impl Punct![..=] {
	pub fn parser() -> impl Parser<TokenTree, Punct![..=], Error = ExpectedButFound<TokenTree>> {
		joint('.')
			.then(joint('.'))
			.then(alone('='))
			.map(|((p1, p2), p3)| Punct![..=](p1.span(), p2.span(), p3.span()))
	}
}

impl Punct![,] {
	pub fn parser() -> impl Parser<TokenTree, Punct![,], Error = ExpectedButFound<TokenTree>> {
		alone(',').map(|p| Punct![,](p.span()))
	}
}
impl Punct![;] {
	pub fn parser() -> impl Parser<TokenTree, Punct![;], Error = ExpectedButFound<TokenTree>> {
		alone(';').map(|p| Punct![;](p.span()))
	}
}

impl Punct![:] {
	pub fn parser() -> impl Parser<TokenTree, Punct![:], Error = ExpectedButFound<TokenTree>> {
		alone(':').map(|p| Punct![:](p.span()))
	}
}
impl Punct![::] {
	pub fn parser() -> impl Parser<TokenTree, Punct![::], Error = ExpectedButFound<TokenTree>> {
		joint(':')
			.then(alone(':'))
			.map(|(p1, p2)| Punct![::](p1.span(), p2.span()))
	}
}

impl Punct![->] {
	pub fn parser() -> impl Parser<TokenTree, Punct![->], Error = ExpectedButFound<TokenTree>> {
		joint('-')
			.then(alone('>'))
			.map(|(p1, p2)| Punct![->](p1.span(), p2.span()))
	}
}
impl Punct![=>] {
	pub fn parser() -> impl Parser<TokenTree, Punct![=>], Error = ExpectedButFound<TokenTree>> {
		joint('=')
			.then(alone('>'))
			.map(|(p1, p2)| Punct![=>](p1.span(), p2.span()))
	}
}

impl Punct![#] {
	pub fn parser() -> impl Parser<TokenTree, Punct![#], Error = ExpectedButFound<TokenTree>> {
		alone('#').map(|p| Punct![#](p.span()))
	}
}
impl Punct![$] {
	pub fn parser() -> impl Parser<TokenTree, Punct![$], Error = ExpectedButFound<TokenTree>> {
		alone('$').map(|p| Punct![$](p.span()))
	}
}
impl Punct![?] {
	pub fn parser() -> impl Parser<TokenTree, Punct![?], Error = ExpectedButFound<TokenTree>> {
		alone('?').map(|p| Punct![?](p.span()))
	}
}
impl Punct![~] {
	pub fn tilde() -> impl Parser<TokenTree, Punct![~], Error = ExpectedButFound<TokenTree>> {
		alone('~').map(|p| Punct![~](p.span()))
	}
}
