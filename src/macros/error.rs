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
		crate::error! { // TODO: this actually needs to be its own implementation with data: 0u8
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

		impl crate::proto::messages::errors::Error<$T> for $Error {
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

		impl crate::Deserialize for $Error {
			fn read(buf: &mut impl bytes::Buf) -> Self {
				// buf has been advanced once to know that this is an error
				// and a second time to know it is _this_ error (from `error_code()`)

				let sequence_num = u16::read(buf);
				let data = <$T>::read(buf); // we are relying on $T being exactly 4 bytes here!
				let minor_opcode = u16::read(buf);
				let major_opcode = u8::read(buf);

				buf.advance(21); // an error is 32 bytes: advance the further 21 bytes needed

				Self {
					sequence_num,
					minor_opcode,
					major_opcode,
					data,
				}
			}
		}

		impl crate::Serialize for $Error {
			fn write(self, buf: &mut impl bytes::BufMut) {
				0u8.write(buf); // first byte of `0` means this is an error
				$code.write(buf); // second byte gives which error this is by the error code

				self.sequence_num.write(buf);
				self.data.write(buf); // we are relying on $T being exactly 4 bytes here!
				self.minor_opcode.write(buf);
				self.major_opcode.write(buf);

				buf.put_bytes(0u8, 21); // put the remaining 21 bytes of padding to meet 32 bytes length
			}
		}

		crate::error!($($t)*);
	};
	(
		$(#[$outer:meta])*
		$vis:vis struct $Error:ident: Error($code:expr) -> $T:ty;

		$($t:tt)*
	) => {
		crate::error! {
			$(#[$outer])*
			$vis struct $Error: Error($code) -> (
				$T
			);

			$($t)*
		}
	};
	() => {};
}
