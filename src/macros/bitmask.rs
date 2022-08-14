// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Creates a bitmask enum that can be serialized, deserialized, and compared.
///
/// Implements [`Bitmask`](crate::util::Bitmask), [`Serialize`](crate::Serialize),
/// [`Deserialize`](crate::Deserialize), [`PartialEq`], [`PartialOrd`], [`Eq`], [`Ord`], [`Clone`],
/// and [`Copy`].
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
/// An enum of the variants will be generated, and [`Bitmask<u16>`](crate::util::Bitmask<u16>)
/// will be implemented for the enum according to the provided values.
///
/// Implementations of [`Serialize`](crate::Serialize), [`Deserialize`](crate::Deserialize),
/// [`PartialEq<Self>`]` for `[`Self`], [`PartialEq<Self>`]` for `[`u16`],
/// [`PartialEq<u16>`]` for `[`Self`], [`PartialOrd<Self>`]` for `[`Self`],
/// [`PartialOrd<Self>`]` for `[`u16`], [`PartialOrd<u16>`]` for `[`Self`], [Eq], and [Ord] will
/// also be generated.
#[macro_export]
macro_rules! bitmask {
	(
		$(#[$outer:meta])* // attributes/docs
		$vis:vis enum $Mask:ident: Bitmask<$T:ty> { // pub enum Mask -> Type {
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
		impl crate::util::Bitmask<$T> for $Mask {
			// Docs for `mask(&self) -> T`
			crate::doc!(concat!("Gets the bitmask value associated with this `",
				stringify!($Mask), "`.

# Implementation:
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
				fn mask(&self) -> $T { // pub fn mask(&self) -> Type {
					match self {
						$(Self::$Variant => $value),+ // Self::Variant => value,
					}
				}
			);

			// Docs for `match_mask(mask: T) -> Option<Self>`
			crate::doc!(concat!("Gets the exactly matching `",
				stringify!($Mask), "` variant for the given bitmask.

# Implementation:
```rust
match mask {",
				$(
					concat!("
    ", stringify!($value), " => Some(Self::", stringify!($Variant), "),"
					)
				),+, "
    _ => None,
}
```"
				),
				fn match_mask(mask: $T) -> Option<Self> {
					match mask {
						$($value => Some(Self::$Variant),)+ // value => Some(Self::Variant)
						_ => None,
					}
				}
			);

			// Docs for `from_mask(mask: T) -> Vec<Self>`
			crate::doc!(concat!("Returns a [`Vec`] of all matching `",
				stringify!($Mask), "` variants for the given bitmask.

# Implementation:
```rust
let variants = vec![",
				$(
					concat!("
    Self::", stringify!($Variant), ","
					)
				),+, "
];

variants.iter().filter(|variant| {
    // Filter the variants by those which have their bitmask value's bits set in the given bitmask.
    variant.mask() & mask == variant.mask()
}).map(|variant| *variant).collect()
```"
				),
				fn from_mask(mask: $T) -> Vec<Self> {
					vec![$(Self::$Variant),+] // Given a [`Vec`] of all possible variants...
						.iter().filter_map(|variant| {
							// Filter the variants by those whose masks are contained in the given
							// mask. A bitwise & will return only the bits that are both 1;
							// therefore, using bitwise & on the variant's mask and the given mask
							// will equal the variant's mask if the variant's mask is set.
							(variant.mask() & mask == variant.mask()).then(|| *variant)
						}).collect()
				}
			);
		}

		///////////////////////////////////////////////////////
		// [`Serialize`] and [`Deserialize`] implementations //
		///////////////////////////////////////////////////////

		impl crate::Serialize for $Mask {
			fn write(self, buf: &mut impl bytes::BufMut) {
				match self {
					$(Self::$Variant => $value.write(buf),)+ // Self::Variant => value.write(buf),
				}
			}
		}

		impl crate::Deserialize for $Mask {
			fn read(buf: &mut impl bytes::Buf) -> Self {
				match <$T>::read(buf) {
					$($value => Self::$Variant,)+ // value => Self::Variant,
					_ => panic!(concat!( // panic!("tried to read Mask from Buf but no matching...
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

		// Since we cannot be sure that `crate::util::Bitmask<$T>` is in scope, we can't actually
		// use `self.mask()` for these equality implementations. That's why `self.mask()`'s
		// definition (`match self { $(Self::$Variant => $value,)+ }`) is repeated here so often.

		impl Eq for $Mask {}

		impl PartialEq<Self> for $Mask {
			fn eq(&self, other: &Self) -> bool {
				// Get `self`'s mask value.
				let mask = match self {
					$(Self::$Variant => $value,)+ // Self::Variant => value,
				};

				// Get `other`'s mask value.
				let other_mask = match other {
					$(Self::$Variant => $value,)+ // Self::Variant => value,
				};

				// Compare them.
				mask == other_mask
			}
		}

		impl PartialEq<$T> for $Mask {
			fn eq(&self, other: &$T) -> bool {
				other == &match self { // other == &self.mask()
					$(Self::$Variant => $value,)+
				}
			}
		}

		impl PartialEq<$Mask> for $T {
			fn eq(&self, other: &$Mask) -> bool {
				self == &match other { // self == &other.mask()
					$($Mask::$Variant => $value,)+
				}
			}
		}

		impl Ord for $Mask {
			fn cmp(&self, other: &Self) -> std::cmp::Ordering {
				// self.mask().cmp(&other.mask())
				match self {
					$(Self::$Variant => $value,)+
				}.cmp(&match other {
					$(Self::$Variant => $value,)+
				})
			}
		}

		impl PartialOrd<Self> for $Mask {
			fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
				// self.mask().partial_cmp(&other.mask())
				match self {
					$(Self::$Variant => $value,)+
				}.partial_cmp(&match other {
					$(Self::$Variant => $value,)+
				})
			}
		}

		impl PartialOrd<$T> for $Mask {
			fn partial_cmp(&self, other: &$T) -> Option<std::cmp::Ordering> {
				// self.mask().partial_cmp(other)
				match self {
					$(Self::$Variant => $value,)+
				}.partial_cmp(other)
			}
		}

		impl PartialOrd<$Mask> for $T {
			fn partial_cmp(&self, other: &$Mask) -> Option<std::cmp::Ordering> {
				// self.partial_cmp(&other.mask())
				self.partial_cmp(&match other {
					$($Mask::$Variant => $value,)+
				})
			}
		}

		/////////////////////////////////////////////
		// Repeat for any more bitmask definitions //
		/////////////////////////////////////////////
		crate::bitmask! {
			$($t)*
		}
	};
	() => {};
}
