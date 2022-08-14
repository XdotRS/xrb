// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Defines a series of sequential constants, starting at `1`.
///
/// # Example
/// Consider the following syntax:
/// ```rust
/// predefine!(
/// 	#[allow(dead_code)]
/// 	for Atom {
/// 	    STRUCTURE_NOTIFY,
/// 	    SUBSTRUCTURE_NOTIFY,
/// 	    /// If you are creating a window manager, you need to register for substructure
/// 	    /// redirection on the root window.
///  	   SUBSTRUCTURE_REDIRECT,
/// 	}
/// );
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
#[macro_export]
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

		crate::predefine! {
			$T, $a, $($universal_atribs)*$(,
				$(#[$tail_atribs])*
				$tail
			)*
		}

		crate::predefine! {
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

		crate::predefine! {
			$T, $a, $($universal_atribs)*$(,
				$(#[$tail_atribs])*
				$tail
			)*
		}
	};
	($T:ty, $a:ident, $($universal_atribs:meta)*) => {};
	() => {};
}
