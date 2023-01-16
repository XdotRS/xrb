// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(let_chains)]
#![feature(if_let_guard)]
#![allow(rustdoc::private_intra_doc_links)]

mod attribute;
mod definition;
mod derive;
mod element;
mod ext;
mod source;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

pub(crate) use definition::*;
use derive::*;
pub(crate) use ext::*;
pub(crate) use source::*;

#[proc_macro_derive(new)]
pub fn derive_new(item: TokenStream) -> TokenStream {
	let item = parse_macro_input!(item as DeriveInput);

	let fields = match &item.data {
		Data::Struct(r#struct) => &r#struct.fields,
		Data::Enum(_) | Data::Union(_) => unimplemented!(),
	};

	let ident = &item.ident;
	let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

	let args = args(fields);
	let cons = pat_cons(fields);

	quote! (
		#[automatically_derived]
		impl #impl_generics #ident #type_generics #where_clause {
			#[doc = concat!("Returns a new `", stringify!(#ident), "`.")]
			#[must_use]
			pub const fn new(#args) -> Self {
				Self #cons
			}
		}
	)
	.into()
}

#[proc_macro_derive(unwrap)]
pub fn derive_unwrap(item: TokenStream) -> TokenStream {
	let item = parse_macro_input!(item as DeriveInput);

	let fields = match &item.data {
		Data::Struct(r#struct) => &r#struct.fields,
		Data::Enum(_) | Data::Union(_) => unimplemented!(),
	};

	let ident = &item.ident;
	let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

	let r#return = unwrap_return(fields);
	let pat = pat_cons(fields);
	let names = names(fields);

	let return_comment = match fields {
		Fields::Named(FieldsNamed { named: fields, .. })
		| Fields::Unnamed(FieldsUnnamed {
			unnamed: fields, ..
		}) => {
			if fields.is_empty() {
				quote!("`()`.")
			} else if fields.len() == 1 {
				quote!("its field.")
			} else {
				quote!("a tuple of its fields.")
			}
		},

		Fields::Unit => quote!("`()`."),
	};

	quote! (
		#[automatically_derived]
		impl #impl_generics #ident #type_generics #where_clause {
			#[doc = concat!("Unwraps `self`, returning ", #return_comment)]
			#[must_use]
			pub const fn unwrap(self) -> (#r#return) {
				let Self #pat = self;

				(#names)
			}
		}
	)
	.into()
}

// Potential idea: source attribute to use a source to serialize a field...?
#[proc_macro_derive(Writable, attributes(no_discrim, hide))]
pub fn derive_writable(item: TokenStream) -> TokenStream {
	let item = parse_macro_input!(item as DeriveInput);

	let ident = &item.ident;
	// TODO: add generic bounds
	let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

	let writes = derive_writes(&item.attrs, &item.data);

	quote!(
		#[automatically_derived]
		impl #impl_generics ::xrbk::Writable for #ident #type_generics #where_clause {
			fn write_to(
				&self,
				buf: &mut impl ::xrbk::BufMut,
			) -> Result<(), ::xrbk::WriteError> {
				#writes

				Ok(())
			}
		}
	)
	.into()
}

// TODO: context attribute support
#[proc_macro_derive(Readable, attributes(no_discrim, hide, context))]
pub fn derive_readable(item: TokenStream) -> TokenStream {
	let item = parse_macro_input!(item as DeriveInput);

	let ident = &item.ident;
	// TODO: add generic bounds
	let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

	let reads = derive_reads(&item.attrs, &item.data);

	quote!(
		#[automatically_derived]
		impl #impl_generics ::xrbk::Readable for #ident #type_generics #where_clause {
			fn read_from(
				buf: &mut impl ::xrbk::Buf,
			) -> Result<Self, ::xrbk::ReadError> {
				#reads
			}
		}
	)
	.into()
}

#[proc_macro_derive(X11Size, attributes(no_discrim, hide))]
pub fn derive_x11_size(item: TokenStream) -> TokenStream {
	let item = parse_macro_input!(item as DeriveInput);

	let ident = &item.ident;
	// TODO: add generic bounds
	let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

	let x11_size = derive_x11_sizes(&item.attrs, &item.data);

	quote!(
		#[automatically_derived]
		impl #impl_generics ::xrbk::X11Size for #ident #type_generics #where_clause {
			fn x11_size(&self) -> usize {
				#x11_size
			}
		}
	)
	.into()
}

#[proc_macro_derive(ConstantX11Size, attributes(no_discrim, hide))]
pub fn derive_constant_x11_size(item: TokenStream) -> TokenStream {
	let item = parse_macro_input!(item as DeriveInput);

	let ident = &item.ident;
	// TODO: add generic bounds
	let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

	let x11_sizes = derive_constant_x11_sizes(&item.attrs, &item.data);

	quote!(
		#[automatically_derived]
		impl #impl_generics ::xrbk::ConstantX11Size for #ident #type_generics #where_clause {
			const X11_SIZE: usize = {
				#x11_sizes
			};
		}
	)
	.into()
}

/// Derive XRB-related traits for structs and enums.
///
/// > **<sup>Syntax</sup>**\
/// > _`derive_xrb!`_ :\
/// > &nbsp;&nbsp; _Definition_<sup>\*</sup>
/// >
/// > _Definition_ :\
/// > &nbsp;&nbsp; &nbsp;&nbsp; _Struct_\
/// > &nbsp;&nbsp; | _Enum_\
/// > &nbsp;&nbsp; | _Request_\
/// > &nbsp;&nbsp; | _Reply_\
/// > &nbsp;&nbsp; | _Event_\
/// > &nbsp;&nbsp; | [_Item_][^other-items]
/// >
/// > [^other-items]: Except [_Struct_]s and [_Enumeration_]s.
/// >
/// > [_Item_]: https://doc.rust-lang.org/reference/items.html
/// > [_Struct_]: https://doc.rust-lang.org/reference/items/structs.html
/// > [_Enumeration_]: https://doc.rust-lang.org/reference/items/enumerations.html
/// >
/// > _Struct_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> [_Visibility_]<sup>?</sup>
/// > _StructMetadata_\
/// > &nbsp;&nbsp; _StructlikeContent_
/// >
/// > _StructMetadata_ :\
/// > &nbsp;&nbsp; `struct` [IDENTIFIER] [_GenericParams_]<sup>?</sup>
/// >
/// > _Request_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> [_Visibility_]<sup>?</sup>
/// > _StructMetadata_\
/// > &nbsp;&nbsp; `:` `Request` _Opcodes_ _ReplyType_<sup>?</sup>\
/// > &nbsp;&nbsp; _StructlikeContent_
/// >
/// > _Opcodes_ :\
/// > &nbsp;&nbsp; `(` [_Expression_] ( `,` [_Expression_] )<sup>?</sup> `)`
/// >
/// > _ReplyType_ :\
/// > &nbsp;&nbsp; `->` [_Type_]
/// >
/// > _Reply_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> [_Visibility_]<sup>?</sup>
/// > _StructMetadata_\
/// > &nbsp;&nbsp; `:` `Reply` _RequestType_\
/// > &nbsp;&nbsp; _StructlikeContent_
/// >
/// > _RequestType_ :\
/// > &nbsp;&nbsp; `for` [_Type_]
/// >
/// > _Event_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> [_Visibility_]<sup>?</sup>
/// > _StructMetadata_\
/// > &nbsp;&nbsp; `:` `Event` `(` [_Expression_] `)`\
/// > &nbsp;&nbsp; _StructlikeContent_
/// >
/// > _Enum_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> [_Visibility_]<sup>?</sup>
/// > _EnumMetadata_\
/// > &nbsp;&nbsp; `{` _Variants_ `}`
/// >
/// > _EnumMetadata_ :\
/// > &nbsp;&nbsp; `enum` [IDENTIFIER] [_GenericParams_]<sup>?</sup>
/// > [_WhereClause_]<sup>?</sup>
/// >
/// > _Variants_ :\
/// > &nbsp;&nbsp; _Variant_ ( `,` _Variant_ )<sup>\*</sup> `,`<sup>?</sup>
/// >
/// > _Variant_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> [IDENTIFIER] _Content_
/// > _Discriminant_<sup>?</sup>
/// >
/// > _Discriminant_ :\
/// > &nbsp;&nbsp; `=` [_Expression_]
/// >
/// > _StructlikeContent_ :\
/// > &nbsp;&nbsp; &nbsp;&nbsp; _RegularStructlikeContent_\
/// > &nbsp;&nbsp; | _TupleStructlikeContent_\
/// > &nbsp;&nbsp; | _UnitStructlikeContent_
/// >
/// > _RegularStructlikeContent_ :\
/// > &nbsp;&nbsp; [_WhereClause_]<sup>?</sup> _RegularContent_
/// >
/// > _TupleStructlikeContent_ :\
/// > &nbsp;&nbsp; _TupleContent_ [_WhereClause_]<sup>?</sup> `;`
/// >
/// > _UnitStructlikeContent_ :\
/// > &nbsp;&nbsp; [_WhereClause_]<sup>?</sup> `;`
/// >
/// > _Content_ :\
/// > &nbsp;&nbsp; ( _RegularContent_ | _TupleContent_ )<sup>?</sup>
/// >
/// > _RegularContent_ :\
/// > &nbsp;&nbsp; `{` _NamedElement_<sup>\*</sup> `}`
/// >
/// > _TupleContent_ :\
/// > &nbsp;&nbsp; `(` _UnnamedElement_<sup>\*</sup> `)`
/// >
/// > _NamedElement_ :\
/// > &nbsp;&nbsp; _NamedField_ | _XrbkElement_
/// >
/// > _UnnamedElement_ :\
/// > &nbsp;&nbsp; _UnnamedField_ | _XrbkElement_
/// >
/// > _XrbkElement_ :\
/// > &nbsp;&nbsp; &nbsp;&nbsp; _LetElement_\
/// > &nbsp;&nbsp; | _SingleUnusedElement_\
/// > &nbsp;&nbsp; | _ArrayUnusedElement_
/// >
/// > _NamedField_ :\
/// > &nbsp;&nbsp; ( [_OuterAttribute_]\
/// > &nbsp;&nbsp; | _ContextAttribute_[^attr-once]\
/// > &nbsp;&nbsp; | _MetabyteAttribute_[^attr-once]\
/// > &nbsp;&nbsp; | _SequenceAttribute_[^attr-once][^sequence]\
/// > &nbsp;&nbsp; | _HideAttribute_[^attr-once] )<sup>\*</sup>\
/// > &nbsp;&nbsp; [_Visibility_]<sup>?</sup> [IDENTIFIER] `:` [_Type_]
/// >
/// > _UnnamedField_ :\
/// > &nbsp;&nbsp; ( [_OuterAttribute_]\
/// > &nbsp;&nbsp; | _ContextAttribute_[^attr-once]\
/// > &nbsp;&nbsp; | _MetabyteAttribute_[^attr-once]\
/// > &nbsp;&nbsp; | _SequenceAttribute_[^attr-once][^sequence]\
/// > &nbsp;&nbsp; | _HideAttribute_[^attr-once] )<sup>\*</sup>\
/// > &nbsp;&nbsp; [_Visibility_]<sup>?</sup> [_Type_]
/// >
/// > _LetElement_ :\
/// > &nbsp;&nbsp; ( [_OuterAttribute_] | _ContextAttribute_[^attr-once] |
/// > _MetabyteAttribute_[^attr-once] )<sup>\*</sup>\
/// > &nbsp;&nbsp; `let` [IDENTIFIER] `:` [_Type_] `=` _Source_
/// >
/// > _SingleUnusedElement_ :\
/// > &nbsp;&nbsp; _MetabyteAttribute_<sup>?</sup> `_`
/// >
/// > _ArrayUnusedElement_ :\
/// > &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> `[` `_` `;` _UnusedContent_
/// > `]`
/// >
/// > _UnusedContent_ :\
/// > &nbsp;&nbsp; `..` | _Source_
/// >
/// > [^attr-once]: *ContextAttribute*s, *MetabyteAttribute*s, and
/// > *SequenceAttribute*s may not be used more than once per element.
/// >
/// > [^sequence]: *SequenceAttribute*s may only be used on fields in replies
/// > and events.
/// >
/// > _ContextAttribute_ :\
/// > &nbsp;&nbsp; `#` `[` `context` _Context_ `]`
/// >
/// > _Context_ :\
/// > &nbsp;&nbsp; &nbsp;&nbsp; ( `=` _Source_ )\
/// > &nbsp;&nbsp; | ( `(` _Source_ `)` )\
/// > &nbsp;&nbsp; | ( `{` _Source_ `}` )\
/// > &nbsp;&nbsp; | ( `[` _Source_ `]` )
/// >
/// > _MetabyteAttribute_ :\
/// > &nbsp;&nbsp; `#` `[` `metabyte` `]`
/// >
/// > _SequenceAttribute_ :\
/// > &nbsp;&nbsp; `#` `[` `sequence` `]`
/// >
/// > _HideAttribute_ :\
/// > &nbsp;&nbsp; `#` `[` `hide` `(` _HiddenTraits_ `)` `]`
/// >
/// > _HiddenTraits_ :\
/// > &nbsp;&nbsp; _HiddenTrait_[^hidden-traits] ( `,` _HiddenTrait_[^hidden-traits] )<sup>\*</sup>
/// >
/// > _HiddenTrait_ :\
/// > &nbsp;&nbsp; &nbsp;&nbsp; `Readable` \
/// > &nbsp;&nbsp; | `Writable` \
/// > &nbsp;&nbsp; | `X11Size` \
/// > &nbsp;&nbsp; | `PartialEq` \
/// > &nbsp;&nbsp; | `Hash` \
/// >
/// > [^hidden-traits]: *HideAttribute*s may only specify traits listed in *HiddenTraits*, any
/// > other traits will have no effects.
/// >
/// > _Source_ :\
/// > &nbsp;&nbsp; ( _SourceArgs_ `=>` )<sup>?</sup> [_Expression_]
/// >
/// > _SourceArgs_ :\
/// > &nbsp;&nbsp; _SourceArg_ ( `,` _SourceArg_ )<sup>\*</sup> `,`<sup>?</sup>
/// >
/// > _SourceArg_ :\
/// > &nbsp;&nbsp; [IDENTIFIER][^validity] |
/// > _SourceLengthArg_[^length-arg-once][^length-arg]
/// >
/// > [^length-arg-once]: *SourceLengthArg*s may not be used more than once per
/// > _SourceArgs_.
/// >
/// > [^length-arg]: *SourceLengthArg*s may only be used in requests and
/// > replies.
/// >
/// > [^validity]: Which identifiers are valid for use as source arguments
/// > depends on where the source is used. See [`Source`] for more information.
/// >
/// > _SourceLengthArg_ :\
/// > &nbsp;&nbsp;&nbsp; `self` `::` `length`
/// >
/// > [_OuterAttribute_]: https://doc.rust-lang.org/reference/attributes.html
/// > [_Visibility_]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
/// > [_GenericParams_]: https://doc.rust-lang.org/reference/items/generics.html
/// > [_WhereClause_]: https://doc.rust-lang.org/reference/items/generics.html#where-clauses
/// > [IDENTIFIER]: https://doc.rust-lang.org/reference/identifiers.html
/// > [_Expression_]: https://doc.rust-lang.org/reference/expressions.html
/// > [_Type_]: https://doc.rust-lang.org/reference/types.html
#[proc_macro]
pub fn derive_xrb(input: TokenStream) -> TokenStream {
	let definitions = parse_macro_input!(input as Definitions);

	let expanded = definitions.into_token_stream();

	expanded.into()
}
