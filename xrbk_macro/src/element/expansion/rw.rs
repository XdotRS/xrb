// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use crate::{element::parsing::DefinitionType, TsExt};

impl ArrayUnused {
	fn r#impl(&self, tokens: &mut TokenStream2, definition_type: DefinitionType) {
		let formatted = &self.id.formatted;

		match &self.content {
			UnusedContent::Infer { last_element, .. } => {
				tokens.append_tokens(|| match definition_type.min_length() {
					Some(min_length) if *last_element => {
						quote!(
							let #formatted = if datasize < #min_length {
								#min_length - datasize
							} else {
								(4 - (datasize % 4)) % 4
							};
						)
					},

					_ => {
						quote!(
							let #formatted = (4 - (datasize % 4)) % 4;
						)
					},
				});
			},

			UnusedContent::Source(source) => {
				source.function_to_tokens(tokens, formatted, &Type::Verbatim(quote!(usize)));

				tokens.append_tokens(|| {
					quote!(
						let #formatted = #formatted();
					)
				});
			},
		}
	}

	pub fn impl_writable(&self, tokens: &mut TokenStream2, definition_type: DefinitionType) {
		let formatted = &self.id.formatted;

		self.r#impl(tokens, definition_type);

		tokens.append_tokens(|| {
			quote!(
				writer.put_bytes(0u8, #formatted);
			)
		});
	}

	pub fn impl_readable(&self, tokens: &mut TokenStream2, definition_type: DefinitionType) {
		let formatted = &self.id.formatted;

		self.r#impl(tokens, definition_type);

		tokens.append_tokens(|| {
			quote!(
				reader.advance(#formatted);
			)
		})
	}
}
