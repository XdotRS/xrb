// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Defines a series of sequential constants, starting at `1`.
///
/// # Example
/// Consider the following syntax:
/// ```rust
/// predefine!(for Atom {
///     STRUCTURE_NOTIFY,
///     SUBSTRUCTURE_NOTIFY,
///     /// If you are creating a window manager, you need to register for substructure
///     /// redirection on the root window.
///     SUBSTRUCTURE_REDIRECT,
/// });
/// ```
/// This will generate the following code:
/// ```rust
/// #[allow(dead_code)]
/// pub const STRUCTURE_NOTIFY: Atom = 1;
/// #[allow(dead_code)]
/// pub const SUBSTRUCTURE_NOTIFY: Atom = STRUCTURE_NOTIFY + 1;
/// /// If you are creating a window manager, you need to register for substructure redirection on
/// /// the root window.
/// #[allow(dead_code)]
/// pub const SUBSTRUCTURE_REDIRECT: Atom = SUBSTRUCTURE_NOTIFY + 1;
/// ```
#[macro_export(crate)]
macro_rules! predefine {
	(
		$(#[$universal_atribs:meta])*
		for $T:ty {
			$(#[$a_atribs:meta])*
			$a:ident$(,
				$(#[$tail_atribs:meta])*
				$tail:ident
			)*
		}

		$($t:tt)*
	) => {
		$(#[$a_atribs])*
		$(#[$universal_atribs])*
		pub const $a: $T = 1;

		predefine! {
			$T, $a, $($universal_atribs)*$(,
				$(#[$tail_atribs])*
				$tail
			)*
		}

		predefine! {
			$($t)*
		}
	};
	(
		$T:ty, $previous:ident, $($universal_atribs:meta)*,

		$(#[$a_atribs:meta])*
		$a:ident$(,
			$(#[$tail_atribs:meta])*
			$tail:ident
		)*
	) => {
		$(#[$a_atribs])*
		$(#[$universal_atribs])*
		pub const $a: $T = $previous + 1;

		predefine! {
			$T, $a, $($universal_atribs)*$(,
				$(#[$tail_atribs])*
				$tail
			)*
		}
	};
	($T:ty, $a:ident, $($universal_atribs:meta)*) => {};
	() => {};
}

/// Generates a doc comment for the given tokens.
///
/// # Example
/// Consider the following syntax:
/// ```rust
/// doc! {
///     "This is a doc comment.",
/// 	const TEXT: &str = "Hello, world!";
/// }
/// ```
/// This will generate:
/// ```rust
/// /// This is a doc comment.
/// const TEXT: &str = "Hello, world!";
/// ```
#[macro_export(crate)]
macro_rules! doc {
    ($x:expr, $($t:tt)+) => {
        #[doc = $x]
        $($t)+
    };
}

/// Creates a bitmask enum implementing [`Serialize`](crate::Serialize) and
/// [`Deserialize`](crate::Deserialize).
///
/// # Example
/// Consider the following syntax:
/// ```rust
/// bitmask! {
///     /// A mask of keys and buttons.
/// 	pub enum KeyButtonMask -> u16 {
///         Shift => 0x0001,
///         Lock => 0x0002,
///         Control => 0x0004,
///         Mod1 => 0x0008,
///         Mod2 => 0x0010,
///         Mod3 => 0x0020,
///         /// Super key.
///         Mod4 => 0x0040,
///         Mod5 => 0x0080,
///         /// Primary mouse button.
///         Button1 => 0x0100,
///         Button2 => 0x0200,
///         Button3 => 0x0400,
///         Button4 => 0x0800,
///         Button5 => 0x1000,
///     }
/// }
/// ```
/// The following enum will be generated:
/// ```rust
/// /// A mask of keys and buttons.
/// #[derive(Copy, Clone)]
/// pub enum KeyButtonMask {
///     Shift,
///     Lock,
///     Control,
///     Mod1,
///     Mod2,
///     Mod3,
///     /// Super key.
///     Mod4,
///     Mod5,
///     /// Primary mouse button.
///     Button1,
///     Button2,
///     Button3,
///     Button4,
///     Button5,
/// }
/// ```
/// With the following implementation to get the masks:
/// ```rust
/// impl KeyButtonMask {
///     /// Gets the bitmask associated with this `KeyButtonMask`.
///     ///
///     /// ```rust
///     /// match self {
///     ///     Self::Shift => 0x0001,
///     ///     Self::Lock => 0x0002,
///     ///     Self::Control => 0x0004,
///     ///     Self::Mod1 => 0x0008,
///     ///     Self::Mod2 => 0x0010,
///     ///     Self::Mod3 => 0x0020,
///     ///     Self::Mod4 => 0x0040,
///     ///     Self::Mod5 => 0x0080,
///     ///     Self::Button1 => 0x0100,
///     ///     Self::Button2 => 0x0200,
///     ///     Self::Button3 => 0x0400,
///     ///     Self::Button4 => 0x0800,
///     ///     Self::Button5 => 0x1000,
///     /// }
///     /// ```
///     pub fn mask(&self) -> u16 {
///         match self {
///             Self::Shift => 0x0001,
///             Self::Lock => 0x0002,
///             Self::Control => 0x0004,
///             Self::Mod1 => 0x0008,
///             Self::Mod2 => 0x0010,
///             Self::Mod3 => 0x0020,
///             Self::Mod4 => 0x0040,
///             Self::Mod5 => 0x0080,
///             Self::Button1 => 0x0100,
///             Self::Button2 => 0x0200,
///             Self::Button3 => 0x0400,
///             Self::Button4 => 0x0800,
///             Self::Button5 => 0x1000,
///         }
///     }
/// }
/// ```
/// And the following implementations for [`Serialize`](crate::Serialize) and
/// [`Deserialize`](crate::Deserialize):
/// ```rust
/// impl crate::Serialize for KeyButtonMask {
///     fn write(self, buf: &mut impl bytes::BufMut) {
///         self.mask().write(buf);
///     }
/// }
///
/// impl crate::Deserialize for KeyButtonMask {
///     fn read(buf: &mut impl bytes::Buf) -> Self {
///         match u16::read(buf) {
///             0x0001 => Self::Shift,
///             0x0002 => Self::Lock,
///             0x0004 => Self::Control,
///             0x0008 => Self::Mod1,
///             0x0010 => Self::Mod2,
///             0x0020 => Self::Mod3,
///             0x0040 => Self::Mod4,
///             0x0080 => Self::Mod5,
///             0x0100 => Self::Button1,
///             0x0200 => Self::Button2,
///             0x0400 => Self::Button3,
///             0x0800 => Self::Button4,
///             0x1000 => Self::Button5,
///             _ => panic!(
///                 "tried to read KeyButtonMask from Buf but no matching variant was found"
///             ),
///         }
///     }
/// }
/// ```
/// Implementations of [PartialEq] for `Self` and `Self`, `Self` and [`u16`], [`u16`] and `Self`,
/// [PartialOrd] for `Self` and `Self`, `Self` and [`u16`], [`u16`] and `Self`, [Eq], and [Ord]
/// will also be generated.
#[macro_export(crate)]
macro_rules! bitmask {
	(
		$(#[$outer:meta])* // attributes/docs
		$vis:vis enum $Mask:ident -> $T:ty { // pub enum Mask -> Type {
			$(
				$(#[$inner:meta])* // variant attributes/docs
				$Variant:ident => $value:expr // Variant => value
			),+$(,)*
		}

		$($t:tt)* // more bitmask definitions
	) => {
		////////////////
		// $Mask enum //
		////////////////
		$(#[$outer])* // attributes/docs
		#[derive(Copy, Clone)]
		$vis enum $Mask { // pub enum Mask {
			$(
				$(#[$inner])* // variant attributes/docs
				$Variant // Variant
			),+
		}

		////////////
		// mask() //
		////////////
		impl $Mask {
			// Docs for `mask()` (messy, I know).
			crate::doc!(concat!("Gets the bitmask associated with this `",
				stringify!($Mask), "`.

```rust
match self {",
				$(
					concat!("
    Self::", stringify!($Variant), " => ", stringify!($value), ","
					)
				),+, "
}
```"
			),
				pub fn mask(&self) -> $T { // pub fn mask(&self) -> Type {
					match self {
						$(Self::$Variant => $value),+ // Self::Variant => value,
					}
				}
			);
		}

		///////////////////////////////////////////////////////
		// [`Serialize`] and [`Deserialize`] implementations //
		///////////////////////////////////////////////////////

		impl crate::Serialize for $Mask {
			fn write(self, buf: &mut impl bytes::BufMut) {
				self.mask().write(buf);
			}
		}

		impl crate::Deserialize for $Mask {
			fn read(buf: &mut impl bytes::Buf) -> Self {
				match <$T>::read(buf) {
					$($value => Self::$Variant,)+ // value => Self::Variant,
					_ => panic!(concat!(
						"tried to read ",
						stringify!($Mask),
						" from Buf but no matching variant was found")
					)
				}
			}
		}

		/////////////////////////////////
		// Equality: compare the masks //
		/////////////////////////////////

		impl Eq for $Mask {}

		impl PartialEq<Self> for $Mask {
			fn eq(&self, other: &Self) -> bool {
				self.mask() == other.mask()
			}
		}

		impl PartialEq<$T> for $Mask {
			fn eq(&self, other: &$T) -> bool {
				&self.mask() == other
			}
		}

		impl PartialEq<$Mask> for $T {
			fn eq(&self, other: &$Mask) -> bool {
				self == &other.mask()
			}
		}

		impl Ord for $Mask {
			fn cmp(&self, other: &Self) -> std::cmp::Ordering {
				self.mask().cmp(&other.mask())
			}
		}

		impl PartialOrd<Self> for $Mask {
			fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
				self.mask().partial_cmp(&other.mask())
			}
		}

		impl PartialOrd<$T> for $Mask {
			fn partial_cmp(&self, other: &$T) -> Option<std::cmp::Ordering> {
				self.mask().partial_cmp(other)
			}
		}

		impl PartialOrd<$Mask> for $T {
			fn partial_cmp(&self, other: &$Mask) -> Option<std::cmp::Ordering> {
				self.partial_cmp(&other.mask())
			}
		}

		/////////////////////////////////////////////
		// Repeat for any more bitmask definitions //
		/////////////////////////////////////////////
		bitmask! {
			$($t)*
		}
	};
	() => {};
}
