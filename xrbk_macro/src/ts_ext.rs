// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;

pub trait TsExt {
    fn with_tokens<F>(f: F) -> Self
    where
        F: FnMut(&mut Self);
}

impl TsExt for TokenStream2 {
    fn with_tokens<F>(f: F) -> Self
    where
        F: FnMut(&mut Self),
    {
        let mut tokens = Self::new();
        f(&mut tokens);

        tokens
    }
}

macro_rules! with_braces {
    ($expr:expr) => {
        {
            let tokens = <proc_macro2::TokenStream as crate::ts_ext::TsExt>::with_tokens($expr);
            quote!({ #tokens })
        }
    };
}

macro_rules! with_parens {
    ($expr:expr) => {
        {
            let tokens = <proc_macro2::TokenStream as crate::ts_ext::TsExt>::with_tokens($expr);
            quote!(( #tokens ))
        }
    }
}

macro_rules! with_brackets {
    ($expr:expr) => {
        {
            let tokens = <proc_macro2::TokenStream as crate::ts_ext::TsExt>::with_tokens($expr);
            quote!([ #tokens ])
        }
    }
}
