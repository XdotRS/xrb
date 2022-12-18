// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod expansion;
mod parsing;

use std::collections::HashMap;
use syn::{punctuated::Punctuated, Expr, Ident, Token, Type};

pub type IdentMap<'a> = &'a HashMap<String, Type>;
pub type IdentMapMut<'a> = &'a mut HashMap<String, Type>;

pub struct Arg {
	pub ident: Ident,
	pub r#type: Option<Type>,

	pub formatted: Ident,
}

pub struct LengthArg {
	pub self_token: Token![self],
	pub double_colon_token: Token![::],
	pub length_token: Ident,

	pub formatted: Ident,
}

pub struct Args {
	pub args: Punctuated<Arg, Token![,]>,
	pub length_arg: Option<(LengthArg, Type)>,
}

/// An inline function to determine a particular value.
///
/// `Source`s have optional arguments separated by commas and followed by `=>`.
/// These arguments are taken by reference. If there are no arguments for a
/// given `Source`, the `=>` is omitted.
///
/// # Argument name validity
/// The names of a `Source`'s arguments considered valid varies depending on
/// where the `Source` is being used.
///
/// The `Source` used in a [`Let`] element may use arguments corresponding to
/// any [`Let`] element defined _before_ that [`Let`] element, as well as the
/// names of _any_ [`Field`]s (whether they are defined before or after the
/// [`Let`] element).
///
/// The `Source`s used in an [`ArrayUnused`] bytes element with
/// [`UnusedContent::Source`] and in [`ContextAttribute`]s may use arguments
/// corresponding to any [`Let`] element or [`Field`] defined _before_ the
/// `Source`. Note that in this case, [`Field`]s defined after the
/// [`ArrayUnused`] bytes element or [`ContextAttribute`] *may not* be used as
/// arguments to the `Source`. This is because these `Source`s are called during
/// deserialization, when the [`Field`]s following the sources have not yet been
/// deserialized.
///
/// Additionally, in a [`Request`] or a [`Reply`], a special argument referring
/// to the length of the message (in units of 4 bytes, as defined in the X11
/// protocol, and offset by 8 units in the case of replies) may be used:
/// `self::length`. This special syntax may be used in any `Source` within that
/// [`Request`] or [`Reply`].
///
/// # Examples
/// ```
/// # extern crate cornflakes;
/// # extern crate xrb;
/// #
/// use xrbk_macro::define;
/// use xrb::String8;
///
/// define! {
///     // ... snippet ...
///
///     pub struct InternAtom: Request(16) -> InternAtomReply {
///         #[metabyte]
///         pub only_if_exists: bool,
///
///         let name_len: u16 = name => name.len() as u16,
///         [_; 2],
///
///         #[context(name_len => *name_len as usize)]
///         pub name: String8,
///         [_; ..],
///     }
///
///     // ...snippet...
///     # pub struct InternAtomReply: Reply for InternAtom { [_; ..] }
/// }
/// ```
/// In this example, `Source` syntax is used three times within the [`Request`].
///
/// The first is in the `name_len` [`Let`] element: the `Source` takes the
/// `name` field (defined _after_ the `Source` in this case) as an argument, and
/// has `name.len() as u16` as the body of the `Source` function.
///
/// The second is in the [`ArrayUnused`] bytes element directly following that
/// [`Let`] element. In this case, the `Source` does not use any arguments, and
/// so the `=>` is omitted. The body of this `Source` function simply returns
/// `2` (of type `usize`, as is the case for all [`UnusedContent::Source`]s).
///
/// The third `Source` appears in the [`ContextAttribute`] for the `name` field.
/// This `Source` takes the `name_len` [`Let`] element defined earlier as an
/// argument, and has `*name_len as usize` as the body of its function. The
/// return type of [`ContextAttribute`] sources, like this one, is given by the
/// [`cornflakes::ContextualReadable::Context`] associated type, which happens
/// to be `usize` for a `String8`.
///
/// The second [`ArrayUnused`] bytes element in this example _does not_ use a
/// `Source`: the `[_; ..]` syntax is a special syntax for [`ArrayUnused`] bytes
/// elements to infer the number of unused bytes. It does not generate a
/// `Source` function.
///
/// [`ArrayUnused`]: crate::element::ArrayUnused
/// [`UnusedContent::Source`]: crate::element::UnusedContent::Source
/// [`ContextAttribute`]: crate::attribute::ContextAttribute
/// [`Let`]: crate::element::Let
/// [`Request`]: crate::definition::Request
/// [`Reply`]: crate::definition::Reply
/// [`Field`]: crate::element::Field
/// [`cornflakes::ContextualReadable::Context`]: https://docs.rs/cornflakes/0.0.5/cornflakes/trait.ContextualReadable.html#associatedtype.Context
pub struct Source {
	/// Optional arguments for the `Source` function, followed by a `=>`.
	pub args: Option<(Args, Token![=>])>,
	/// The body of the `Source` function: its expression.
	pub expr: Expr,
}
