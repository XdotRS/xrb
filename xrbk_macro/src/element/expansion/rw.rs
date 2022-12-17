// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use crate::{definition::DefinitionType, TsExt};

impl Element {
	pub fn serialize(&self, tokens: &mut TokenStream2, definition_type: DefinitionType) {
		match self {
			Self::Field(field) => field.serialize(tokens),
			Self::Let(r#let) => r#let.serialize(tokens),

			Self::SingleUnused(unused) => unused.serialize(tokens),
			Self::ArrayUnused(unused) => unused.serialize(tokens, definition_type),
		}
	}

	pub fn deserialize(&self, tokens: &mut TokenStream2, definition_type: DefinitionType) {
		match self {
			Self::Field(field) => field.deserialize(tokens),
			Self::Let(r#let) => r#let.deserialize(tokens),

			Self::SingleUnused(unused) => unused.deserialize(tokens),
			Self::ArrayUnused(unused) => unused.deserialize(tokens, definition_type),
		}
	}

	pub fn add_datasize_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Field(field) => field.add_datasize_tokens(tokens),
			Self::Let(r#let) => r#let.add_datasize_tokens(tokens),

			Self::SingleUnused(unused) => unused.add_datasize_tokens(tokens),
			Self::ArrayUnused(unused) => unused.add_datasize_tokens(tokens),
		}
	}
}

// Field {{{

impl Field {
	pub fn serialize(&self, tokens: &mut TokenStream2) {
		let formatted = &self.formatted;
		let r#type = &self.r#type;

		tokens.append_tokens(|| {
			quote!(
				<#r#type as cornflakes::Writable>::write_to(#formatted, buf)?;
			)
		});
	}

	pub fn deserialize(&self, tokens: &mut TokenStream2) {
		let formatted = &self.formatted;
		let r#type = &self.r#type;

		match &self.context_attribute {
			Some(ContextAttribute { context, .. }) => {
				context
					.source()
					.function_to_tokens(tokens, None, formatted, r#type);

				let function_call = TokenStream2::with_tokens(|tokens| {
					context.source().call_to_tokens(tokens, formatted);
				});

				tokens.append_tokens(|| {
					quote!(
						let #formatted = <#r#type as cornflakes::ContextualReadable>::read_with(
							buf,
							#function_call,
						)?;
					)
				});
			},

			None => {
				tokens.append_tokens(|| {
					quote!(
						let #formatted = <#r#type as cornflakes::Readable>::read_from(buf)?;
					)
				});
			},
		}
	}

	pub fn add_datasize_tokens(&self, tokens: &mut TokenStream2) {
		tokens.append_tokens(|| {
			let r#type = &self.r#type;
			let formatted = &self.formatted;

			quote!(
				datasize += <#r#type as cornflakes::DataSize>::data_size(#formatted);
			)
		});
	}
}

// }}} Let {{{

impl Let {
	pub fn serialize(&self, tokens: &mut TokenStream2) {
		let formatted = &self.formatted;
		let r#type = &self.r#type;

		self.source
			.function_to_tokens(tokens, Some(&self.attributes), formatted, &self.r#type);

		let function_call = TokenStream2::with_tokens(|tokens| {
			self.source.call_to_tokens(tokens, formatted);
		});

		tokens.append_tokens(|| {
			quote!(
				let #formatted = #function_call;

				<#r#type as cornflakes::Writable>::write_to(#formatted, buf)?;
			)
		});
	}

	pub fn deserialize(&self, tokens: &mut TokenStream2) {
		let formatted = &self.formatted;
		let r#type = &self.r#type;

		match &self.context_attribute {
			Some(ContextAttribute { context, .. }) => {
				context
					.source()
					.function_to_tokens(tokens, None, formatted, r#type);

				let function_call = TokenStream2::with_tokens(|tokens| {
					context.source().call_to_tokens(tokens, formatted);
				});

				tokens.append_tokens(|| {
					quote!(
						let #formatted = <#r#type as cornflakes::ContextualReadable>::read_with(
							buf,
							#function_call,
						)?;
					)
				});
			},

			None => {
				tokens.append_tokens(|| {
					quote!(
						let #formatted = <#r#type as cornflakes::Readable>::read_from(buf)?;
					)
				});
			},
		}
	}

	pub fn add_datasize_tokens(&self, tokens: &mut TokenStream2) {
		let r#type = &self.r#type;
		let formatted = &self.formatted;

		tokens.append_tokens(|| {
			quote!(
				datasize += <#r#type as cornflakes::DataSize>::data_size(#formatted);
			)
		});
	}
}

// }}} Single unused byte {{{

impl SingleUnused {
	pub fn serialize(&self, tokens: &mut TokenStream2) {
		tokens.append_tokens(|| {
			quote!(
				buf.put_u8(0);
			)
		});
	}

	pub fn deserialize(&self, tokens: &mut TokenStream2) {
		tokens.append_tokens(|| {
			quote!(
				buf.advance(1);
			)
		});
	}

	pub fn add_datasize_tokens(&self, tokens: &mut TokenStream2) {
		tokens.append_tokens(|| {
			quote!(
				datasize += 1;
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
				source.function_to_tokens(
					tokens,
					Some(&self.attributes),
					formatted,
					&Type::Verbatim(quote!(usize)),
				);

				tokens.append_tokens(|| {
					quote!(
						let #formatted = #formatted();
					)
				});
			},
		}
	}

	pub fn serialize(&self, tokens: &mut TokenStream2, definition_type: DefinitionType) {
		let formatted = &self.formatted;

		self.r#impl(tokens, definition_type);

		tokens.append_tokens(|| {
			quote!(
				buf.put_bytes(0u8, #formatted);
			)
		});
	}

	pub fn deserialize(&self, tokens: &mut TokenStream2, definition_type: DefinitionType) {
		let formatted = &self.formatted;

		self.r#impl(tokens, definition_type);

		tokens.append_tokens(|| {
			quote!(
				buf.advance(#formatted);
			)
		})
	}

	pub fn add_datasize_tokens(&self, tokens: &mut TokenStream2) {
		let formatted = &self.formatted;

		tokens.append_tokens(|| {
			quote!(
				datasize += #formatted;
			)
		});
	}
}

// }}}
