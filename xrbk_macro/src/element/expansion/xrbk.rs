// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use quote::quote_spanned;
use syn::spanned::Spanned;

use super::*;
use crate::{definition::DefinitionType, TsExt};

impl Element {
	pub fn write_tokens(&self, tokens: &mut TokenStream2, definition_type: DefinitionType) {
		match self {
			Self::Field(field) => {
				if !field.is_ignoring_trait(format_ident!("Writable")) {
					field.write_tokens(tokens)
				}
			},
			Self::Let(r#let) => r#let.write_tokens(tokens),

			Self::SingleUnused(unused) => unused.write_tokens(tokens),
			Self::ArrayUnused(unused) => unused.write_tokens(tokens, definition_type),
		}
	}

	pub fn x11_size_tokens(&self, tokens: &mut TokenStream2, definition_type: DefinitionType) {
		match self {
			Self::Field(field) => {
				if !field.is_ignoring_trait(format_ident!("X11Size")) {
					field.x11_size_tokens(tokens)
				}
			},
			Self::Let(r#let) => r#let.x11_size_tokens(tokens),

			Self::SingleUnused(unused) => unused.x11_size_tokens(tokens),
			Self::ArrayUnused(unused) => unused.x11_size_tokens(tokens, definition_type),
		}
	}

	pub fn read_tokens(&self, tokens: &mut TokenStream2, definition_type: DefinitionType) {
		match self {
			Self::Field(field) => {
				if !field.is_ignoring_trait(format_ident!("Readable"))
					|| field.context_attribute.is_some()
				{
					field.read_tokens(tokens)
				}
			},
			Self::Let(r#let) => r#let.read_tokens(tokens),

			Self::SingleUnused(unused) => unused.read_tokens(tokens),
			Self::ArrayUnused(unused) => unused.read_tokens(tokens, definition_type),
		}
	}

	pub fn add_x11_size_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Field(field) => {
				if !field.is_ignoring_trait(format_ident!("X11Size")) {
					field.add_x11_size_tokens(tokens)
				}
			},
			Self::Let(r#let) => r#let.add_x11_size_tokens(tokens),

			Self::SingleUnused(unused) => unused.add_x11_size_tokens(tokens),
			Self::ArrayUnused(unused) => unused.add_x11_size_tokens(tokens),
		}
	}

	pub fn partial_eq_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Field(field) => {
				if !field.is_ignoring_trait(format_ident!("PartialEq")) {
					field.add_partial_eq_tokens(tokens)
				}
			},
			_ => (),
		}
	}

	pub fn hash_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Field(field) => {
				if !field.is_ignoring_trait(format_ident!("Hash")) {
					field.add_hash_tokens(tokens)
				}
			},
			_ => (),
		}
	}
}

// Field {{{

impl Field {
	pub fn write_tokens(&self, tokens: &mut TokenStream2) {
		let formatted = &self.formatted;
		let r#type = &self.r#type;

		tokens.append_tokens({
			let r#type = quote_spanned!(r#type.span()=>
				<#r#type as ::xrbk::Writable>
			);

			quote_spanned!(self.span()=>
				#r#type::write_to(&#formatted, buf)?;
			)
		});
	}

	pub fn x11_size_tokens(&self, tokens: &mut TokenStream2) {
		self.add_x11_size_tokens(tokens);
	}

	pub fn read_tokens(&self, tokens: &mut TokenStream2) {
		let formatted = &self.formatted;
		let r#type = &self.r#type;

		match &self.context_attribute {
			Some(ContextAttribute { context, .. }) => {
				context.source().function_to_tokens(
					tokens,
					None,
					formatted,
					quote_spanned!(r#type.span()=>
						<#r#type as ::xrbk::ReadableWithContext>::Context
					),
				);

				let function_call = TokenStream2::with_tokens(|tokens| {
					context.source().call_to_tokens(tokens, formatted);
				});

				tokens.append_tokens({
					let r#type = quote_spanned!(r#type.span()=>
						<#r#type as ::xrbk::ReadableWithContext>
					);

					quote_spanned!(self.span()=>
						let #formatted = #r#type::read_with(
							buf,
							&#function_call,
						)?;
					)
				});
			},

			None => {
				tokens.append_tokens({
					let r#type = quote_spanned!(r#type.span()=>
						<#r#type as ::xrbk::Readable>
					);

					quote_spanned!(self.span()=>
						let #formatted = #r#type::read_from(buf)?;
					)
				});
			},
		}
	}

	pub fn add_x11_size_tokens(&self, tokens: &mut TokenStream2) {
		tokens.append_tokens({
			let r#type = &self.r#type;
			let formatted = &self.formatted;

			quote_spanned!(self.span()=>
				size += <#r#type as ::xrbk::X11Size>::x11_size(&#formatted);
			)
		});
	}

	pub fn add_partial_eq_tokens(&self, tokens: &mut TokenStream2) {
		tokens.append_tokens({
			let ident = &self.id;

			quote_spanned!(self.span()=>
				&& self.#ident == other.#ident
			)
		});
	}

	pub fn add_hash_tokens(&self, tokens: &mut TokenStream2) {
		tokens.append_tokens({
			let ident = &self.id;

			quote_spanned!(self.span()=>
				::core::hash::Hash::hash(&self.#ident, state);
			)
		});
	}
}

// }}} Let {{{

impl Let {
	pub fn write_tokens(&self, tokens: &mut TokenStream2) {
		let formatted = &self.formatted;
		let r#type = &self.r#type;

		self.source.function_to_tokens(
			tokens,
			Some(&self.attributes),
			formatted,
			r#type.into_token_stream(),
		);

		let function_call = TokenStream2::with_tokens(|tokens| {
			self.source.call_to_tokens(tokens, formatted);
		});

		tokens.append_tokens({
			quote_spanned!(formatted.span()=>
				let #formatted = #function_call;
			)
		});
		tokens.append_tokens({
			let r#type = quote_spanned!(r#type.span()=>
				<#r#type as ::xrbk::Writable>
			);

			quote_spanned!(self.span()=>
				#r#type::write_to(&#formatted, buf)?;
			)
		});
	}

	pub fn x11_size_tokens(&self, tokens: &mut TokenStream2) {
		let formatted = &self.formatted;

		self.source.function_to_tokens(
			tokens,
			Some(&self.attributes),
			formatted,
			self.r#type.to_token_stream(),
		);

		let function_call = TokenStream2::with_tokens(|tokens| {
			self.source.call_to_tokens(tokens, formatted);
		});

		tokens.append_tokens({
			quote_spanned!(self.span()=>
				let #formatted = #function_call;
			)
		});

		self.add_x11_size_tokens(tokens);
	}

	pub fn read_tokens(&self, tokens: &mut TokenStream2) {
		let formatted = &self.formatted;
		let r#type = &self.r#type;

		match &self.context_attribute {
			Some(ContextAttribute { context, .. }) => {
				context.source().function_to_tokens(
					tokens,
					None,
					formatted,
					quote_spanned!(r#type.span()=>
						<#r#type as ::xrbk::ReadableWithContext>::Context
					),
				);

				let function_call = TokenStream2::with_tokens(|tokens| {
					context.source().call_to_tokens(tokens, formatted);
				});

				tokens.append_tokens({
					let r#type = quote_spanned!(r#type.span()=>
						<#r#type as ::xrbk::ReadableWithContext>
					);

					quote_spanned!(self.span()=>
						let #formatted = #r#type::read_with(
							buf,
							#function_call,
						)?;
					)
				});
			},

			None => {
				tokens.append_tokens({
					let r#type = quote_spanned!(r#type.span()=>
						<#r#type as ::xrbk::Readable>
					);

					quote_spanned!(self.span()=>
						let #formatted = #r#type::read_from(buf)?;
					)
				});
			},
		}
	}

	pub fn add_x11_size_tokens(&self, tokens: &mut TokenStream2) {
		let r#type = &self.r#type;
		let formatted = &self.formatted;

		tokens.append_tokens({
			quote_spanned!(self.span()=>
				size += <#r#type as ::xrbk::X11Size>::x11_size(&#formatted);
			)
		});
	}
}

// }}} Single unused byte {{{

impl SingleUnused {
	pub fn write_tokens(&self, tokens: &mut TokenStream2) {
		tokens.append_tokens({
			quote_spanned!(self.span()=>
				buf.put_u8(0);
			)
		});
	}

	pub fn x11_size_tokens(&self, tokens: &mut TokenStream2) {
		self.add_x11_size_tokens(tokens);
	}

	pub fn read_tokens(&self, tokens: &mut TokenStream2) {
		tokens.append_tokens({
			quote_spanned!(self.span()=>
				buf.advance(1);
			)
		});
	}

	pub fn add_x11_size_tokens(&self, tokens: &mut TokenStream2) {
		tokens.append_tokens({
			quote_spanned!(self.span()=>
				size += 1;
			)
		});
	}
}

// }}} Array-type unused bytes {{{

impl ArrayUnused {
	fn r#impl(&self, tokens: &mut TokenStream2, definition_type: DefinitionType) {
		let formatted = &self.formatted;

		match &self.content {
			UnusedContent::Infer { last_element, .. } => {
				tokens.append_tokens(match definition_type.min_length() {
					Some(min_length) if *last_element => {
						quote_spanned!(self.span()=>
							let #formatted = if size < #min_length {
								#min_length - size
							} else {
								(4 - (size % 4)) % 4
							};
						)
					},

					_ => {
						quote_spanned!(self.span()=>
							let #formatted = (4 - (size % 4)) % 4;
						)
					},
				});
			},

			UnusedContent::Source(source) => {
				source.function_to_tokens(tokens, Some(&self.attributes), formatted, quote!(usize));

				tokens.append_tokens({
					quote_spanned!(self.span()=>
						let #formatted = #formatted();
					)
				});
			},
		}
	}

	pub fn write_tokens(&self, tokens: &mut TokenStream2, definition_type: DefinitionType) {
		let formatted = &self.formatted;

		self.r#impl(tokens, definition_type);

		tokens.append_tokens({
			quote_spanned!(self.span()=>
				buf.put_bytes(0u8, #formatted);
			)
		});
	}

	pub fn x11_size_tokens(&self, tokens: &mut TokenStream2, definition_type: DefinitionType) {
		self.r#impl(tokens, definition_type);
		self.add_x11_size_tokens(tokens);
	}

	pub fn read_tokens(&self, tokens: &mut TokenStream2, definition_type: DefinitionType) {
		let formatted = &self.formatted;

		self.r#impl(tokens, definition_type);

		tokens.append_tokens({
			quote_spanned!(self.span()=>
				buf.advance(#formatted);
			)
		})
	}

	pub fn add_x11_size_tokens(&self, tokens: &mut TokenStream2) {
		let formatted = &self.formatted;

		tokens.append_tokens({
			quote_spanned!(self.span()=>
				size += #formatted;
			)
		});
	}
}

// }}}
