// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[macro_export]
macro_rules! error {
	(
		$(#[$outer:meta])*
		$vis:vis struct $Error:ident: Error($code:expr);

		$($t:tt)*
	) => {
		$crate::error! { // TODO: this actually needs to be its own implementation with data: 0u8
			$(#[$outer])*
			$vis struct $Error: Error($code) -> u32;

			$($t)*
		}
	};

	(
		$(#[$outer:meta])*
		$vis:vis struct $Error:ident: Error($code:expr) -> (
			$(#[$inner:meta])*
			$T:ty
		);

		$($t:tt)*
	) => {
		$(#[$outer])*
		$vis struct $Error {
			sequence_num: u16,
			minor_opcode: u16,
			major_opcode: u8,
			data: $T,
		}

		impl $crate::proto::messages::errors::Error<$T> for $Error {
			fn error_code() -> u8 {
				$code
			}

			fn sequence_num(&self) -> u16 {
				self.sequence_num
			}

			fn minor_opcode(&self) -> u16 {
				self.minor_opcode
			}

			fn major_opcode(&self) -> u8 {
				self.major_opcode
			}

			$(#[$inner])*
			fn data(&self) -> $T {
				self.data
			}

			fn new(sequence_num: u16, minor_opcode: u16, major_opcode: u8, data: $T) -> Self {
				Self {
					sequence_num,
					minor_opcode,
					major_opcode,
					data,
				}
			}
		}

		$crate::error!($($t)*);
	};
	(
		$(#[$outer:meta])*
		$vis:vis struct $Error:ident: Error($code:expr) -> $T:ty;

		$($t:tt)*
	) => {
		$crate::error! {
			$(#[$outer])*
			$vis struct $Error: Error($code) -> (
				$T
			);

			$($t)*
		}
	};
	() => {};
}
