// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// An internal-use macro for the main recursion of [`predefined_atoms`].
///
/// This is needed so that the first identifier can be treated differently and be set to 1.
/// This is because every second element is set to the first plus 1; the first is never defined
/// in [`predef_atoms_recurse`].
#[macro_export(crate)]
macro_rules! predef_atoms_recurse {
	// Base case (only two atoms):
	(
        $a:ident, // capture `$a` (`$a` has already been assigned)

        $(#[$b_atrib:meta])* // capture `$b`'s attributes (includes doc comments)
        $b:ident // capture `$b`
    ) => {
		#[allow(dead_code)]
        $(#[$b_atrib])* // apply `$b`'s attributes (includes doc comments)
		pub const $b: crate::Atom = $a + 1; // define `$b` in terms of `$a`
	};
	// Recursion (three or more atoms):
	(
        $a:ident, // capture `$a` (`$a` has already been assigned)

        $(#[$b_atrib:meta])* // capture `$b`'s attributes (includes doc comments)
        $b:ident, // capture `$b`

        $( // capture one or more tail elements
            $(#[$tail_atrib:meta])* // capture `$tail`s' attributes (includes doc comments)
            $tail:ident // capture `$tail`s
        ),+
    ) => {
        #[allow(dead_code)]
        $(#[$b_atrib])* // apply `$b`'s attributes
		pub const $b: crate::Atom = $a + 1; // define `$b` in terms of `$a`

		crate::predef_atoms_recurse!( // recurse
            $b,

            $(
                $(#[$tail_atrib])*
                $tail
            ),+
        );
	}
}

/// Defines constants for [Atom](crate::Atom)s predefined in the X11 protocol, starting at index
/// `1u8`.
#[macro_export(crate)]
macro_rules! predefined_atoms {
    (
        $(#[$a_atrib:meta])* // capture `$a`'s attributes (includes doc comments)
        $a:ident, // capture `$a`

        $( // recursively capture tail elements:
            $(#[$tail_atrib:meta])* // their attributes (includes doc comments)
            $tail:ident // tail elements themselves
        ),+
    ) => {
        #[allow(dead_code)]
        $(#[$a_atrib])* // apply `$a`'s attributes
        pub const $a: crate::Atom = 1; // define `$a` as 1

        crate::predef_atoms_recurse!( // recurse
            $a,

            $(
                $(#[$tail_atrib])*
                $tail
            ),+
        );
    };
}
