// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod expansion;
mod parsing;

use std::collections::HashMap;
use syn::{punctuated::Punctuated, Expr, Ident, Pat, Token, Type};

pub type IdentMap<'a> = &'a HashMap<String, Type>;
pub type IdentMapMut<'a> = &'a mut HashMap<String, Type>;

/// A non-[SourceLengthArg] argument for a [`Source`].
///
/// > **<sup>Syntax</sup>**\
/// > _SourceArg_ :\
/// > &nbsp;&nbsp; [IDENTIFIER][^validity] ( `:` [_Pattern_] )<sup>?</sup>
/// >
/// > [IDENTIFIER]: https://doc.rust-lang.org/reference/identifiers.html
/// > [^validity]: See [Argument name validity] for which identifiers are
/// > allowed.
/// >
/// > [Argument name validity]: ../Source.html#argument-name-validity
/// >
/// > [_Pattern_]: https://doc.rust-lang.org/reference/patterns.html
pub struct SourceArg {
	pub ident: Ident,
	pub pattern: Option<(Token![:], Pat)>,

	pub r#type: Option<Type>,
	pub formatted: Option<Ident>,
}

/// A [`Source`] argument referring to the length of a [`Request`] or [`Reply`].
///
/// > **<sup>Syntax</sup>**\
/// > _SourceLengthArg_ :\
/// > &nbsp;&nbsp; `self` `::` `length`
///
/// [`Request`]: crate::definition::Request
/// [`Reply`]: crate::definition::Reply
pub struct SourceLengthArg {
	pub self_token: Token![self],
	pub double_colon_token: Token![::],
	pub length_token: Ident,
}

/// Arguments for a [`Source`].
///
/// > **<sup>Syntax</sup>**\
/// > _SourceArgs_ :\
/// > &nbsp;&nbsp; _Arg_ ( `,` _Arg_ )<sup>\*</sup> `,`<sup>?</sup>
/// >
/// > _Arg_ :\
/// > &nbsp;&nbsp; [_SourceArg_] | [_SourceLengthArg_][^usage]
/// >
/// > [^usage]: [_SourceLengthArg_]s may only be used within [`Request`]s and
/// > [`Reply`]s, and they may be used no more than once per _SourceArgs_.
///
/// [_SourceArg_]: SourceArg
/// [_SourceLengthArg_]: SourceLengthArg
/// [`Request`]: crate::definition::Request
/// [`Reply`]: crate::definition::Reply
pub struct SourceArgs {
	pub args: Punctuated<SourceArg, Token![,]>,
	pub length_arg: Option<(SourceLengthArg, Type)>,
}

/// An inline function.
///
/// > **<sup>Syntax</sup>**\
/// > _Source_ :\
/// > &nbsp;&nbsp; ( [_SourceArgs_] `=>` )<sup>?</sup> [_Expression_]
/// >
/// > [_SourceArgs_] :\
/// > &nbsp;&nbsp; [_Arg_] ( `,` [_Arg_] )<sup>\*</sup> `,`<sup>?</sup>
/// >
/// > [_Arg_] :\
/// > &nbsp;&nbsp; [_SourceArg_] | [_SourceLengthArg_]
/// >
/// > [_SourceArg_] :\
/// > &nbsp;&nbsp; [IDENTIFIER] ( `:` [_Pattern_] )<sup>?</sup>
/// >
/// > [_SourceLengthArg_] :\
/// > &nbsp;&nbsp; `self` `::` `length`
/// >
/// > [_SourceArgs_]: SourceArgs
/// > [_Arg_]: SourceArgs
/// > [_SourceArg_]: SourceArg
/// > [_SourceLengthArg_]: SourceLengthArg
/// >
/// > [_Expression_]: https://doc.rust-lang.org/reference/expressions.html
/// > [IDENTIFIER]: https://doc.rust-lang.org/reference/identifiers.html
/// > [_Pattern_]: https://doc.rust-lang.org/reference/patterns.html
///
/// `Source`s have optional arguments separated by commas and followed by `=>`.
/// These arguments are taken by reference. If there are no arguments for a
/// given `Source`, the `=>` is omitted.
///
/// `Source`s are converted into typical `fn`s.
///
/// `Source`s are used in [`ContextAttribute`]s to provide context when reading
/// a [`Field`] or [`Let`] element, [`Let`] elements to write their values, and
/// in [`ArrayUnused`] bytes elements with [`UnusedContent::Source`]s to
/// determine the number of unused bytes to skip.
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
/// # Argument patterns
/// A `Source`'s arguments may optionally specify a pattern by adding a `:` and
/// that pattern after the argument's name. This pattern is used in the `Source`
/// function's parameters to destructure the argument; it does not affect what
/// type of value is provided for that argument when the function is called.
///
/// For example, take the following struct containing a [`Let`] element which
/// uses a `Source` (assume `width` and `height` are of type `u32`):
/// ```ignore
/// # extern crate cornflakes;
/// use xrbk_macro::derive_xrb;
/// use xrb::Rectangle;
///
/// derive_xrb! {
///     pub struct Courtyard {
///         pub shape: Rectangle,
///
///         let area: u32 = shape: Rectangle {
///             width,
///             height,
///             ..
///         } => width * height,
///
///         [_; 2],
///     }
/// }
/// ```
/// `area` would have a `Source` function like so:
/// ```
/// # struct Rectangle { width: u32, height: u32, x: i32, y: i32 }
/// # let shape = Rectangle { width: 30, height: 30, x: 10, y: 15 };
/// #
/// fn area(Rectangle { width, height, .. }: &Rectangle) -> u32 {
///     width * height
/// }
///
/// let area = area(&shape);
/// ```
///
/// Note that using the name `shape` was necessary to retrieve the type of
/// the argument from the definition the [`Let`] element is contained within,
/// even though it doesn't appear in the generated `Source` function. It is also
/// used when calling the `Source` function as the argument that is passed.
///
/// Also note that it is not possible[^pattern-use-name] to use the name of the
/// argument in the `Source`'s expression if a pattern is provided: that pattern
/// is used to destructure the argument.
///
/// [^pattern-use-name]: Unless that pattern simply repeats the same name, like
/// `shape: shape`.
///
/// # Length arguments
/// Additionally, in a [`Request`] or a [`Reply`], a special argument referring
/// to the length of the message (in units of 4 bytes, as defined in the X11
/// protocol, and offset by 8 units in the case of replies) may be used:
/// `self::length`. This special syntax may be used in any `Source` within that
/// [`Request`] or [`Reply`].
///
/// # Examples
/// ```ignore
/// # extern crate cornflakes;
/// # extern crate xrb;
/// #
/// use xrbk_macro::derive_xrb;
/// use xrb::{String8, Atom};
///
/// derive_xrb! {
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
///     pub struct InternAtomReply: Reply for InternAtom {
///         #[sequence]
///         pub sequence: u16,
///
///         pub atom: Option<Atom>,
///         [_; ..],
///     }
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
/// Neither the second [`ArrayUnused`] bytes element in the [`Request`] nor the
/// [`ArrayUnused`] bytes element within its [`Reply`] use a `Source`: the
/// `[_; ..]` syntax is a special syntax for [`ArrayUnused`] bytes elements to
/// infer the number of unused bytes. It generates no function.
///
/// ## Length arguments
/// ```ignore
/// # extern crate cornflakes;
/// # extern crate xrb;
/// #
/// use xrbk_macro::derive_xrb;
/// use xrb::{GraphicsContext, Rectangle, Point};
///
/// derive_xrb! {
///     pub enum Ordering {
///         Unsorted,
///         YSorted,
///         YxSorted,
///         YxBanded,
///     }
///
///     pub struct SetClipRectangles: Request(59) {
///         #[metabyte]
///         pub ordering: Ordering,
///
///         pub context: GraphicsContext,
///         pub clip_origin: Point,
///
///         #[context(self::length => {
///             (
///                 // Each length unit is four bytes.
///                 (length * 4)
///                 // The request header is four bytes.
///                 - 4
///                 // The elements before the `rectangles` field are eight bytes.
///                 - 8
///             )
///             // Each rectangle is eight bytes.
///             / 8
///         })]
///         pub rectangles: Vec<Rectangle>,
///     }
/// }
/// ```
///
/// [`ArrayUnused`]: crate::element::ArrayUnused
/// [`UnusedContent::Source`]: crate::element::UnusedContent::Source
/// [`ContextAttribute`]: crate::attribute::ContextAttribute
/// [`Let`]: crate::element::Let
/// [`Request`]: crate::definition::Request
/// [`Reply`]: crate::definition::Reply
/// [`Field`]: crate::element::Field
/// [`cornflakes::ContextualReadable::Context`]: https://docs.rs/cornflakes/latest/cornflakes/trait.ContextualReadable.html#associatedtype.Context
pub struct Source {
	/// Optional arguments for the `Source` function, followed by a `=>`.
	pub args: Option<(SourceArgs, Token![=>])>,
	/// The body of the `Source` function: its expression.
	pub expr: Expr,
}
