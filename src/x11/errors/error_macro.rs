// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Automatically generates structs for X protocol errors.
macro_rules! errors {
	(
		$(
			$(#[$attr:meta])* // attributes
			$vis:vis struct $Name:ident($code:expr) { // pub struct Error(0) {
				$($bad_data:ident)? // bad_res_id (optional: extra data)
			}
		)+
	) => {
		$(
			#[derive(thiserror::Error, Debug)]
			$(#[$attr])* // attributes
			$vis struct $Name { // pub struct Error {
				sequence: u16,
				$(pub $bad_data: u32,)? // pub bad_res_id: u32, (optional)
				minor_opcode: u16,
				major_opcode: u8,
			}

			impl $crate::x11::errors::Xerror for $Name { // impl Xerror for Error {
				fn code(&self) -> u8 { $code } //  fn code(&self) -> u8 { 0 }
				fn sequence(&self) -> u16 { self.sequence }
				fn minor_opcode(&self) -> u16 { self.minor_opcode }
				fn major_opcode(&self) -> u8 { self.major_opcode }
			}

			impl $crate::rw::Serialize for $Name { // impl Serialize for Error {
				fn serialize(self) -> $crate::rw::WriteResult<Vec<u8>> {
					let mut bytes = bytes::BytesMut::new();

					// Padding
					<u8 as $crate::rw::WriteValue>::write_1b_to(0, &mut bytes)?;

					// Error code. This is literally just `$code.write_1b_to(&mut bytes)?;`.
					<u8 as $crate::rw::WriteValue>::write_1b_to($code, &mut bytes)?;
					// Sequence. This is literally just `self.sequence.write_2b_to(&mut bytes)?;`.
					<u16 as $crate::rw::WriteValue>::write_2b_to(self.sequence, &mut bytes)?;

					// TODO: What to do when $bad_data is missing? Still need
					//       padding... do we need to duplicate all this macro
					//       logic for the case where no $bad_data is given?
					//
					// `self.bad_res_id.write_4b_to(&mut bytes)?;` (optional)
					$(<u32 as $crate::rw::WriteValue>::write_4b_to(self.$bad_data, &mut bytes)?;)?

					// Minor opcode. `self.minor_opcode.write_2b_to(&mut bytes)?;`
					<u16 as $crate::rw::WriteValue>::write_2b_to(self.minor_opcode, &mut bytes)?;
					// Major opcode. `self.major_opcode.write_1b_to(&mut bytes)?;`
					<u8 as $crate::rw::WriteValue>::write_1b_to(self.major_opcode, &mut bytes)?;

					// Trailing 21 unused bytes. `bytes.put_bytes(0, 21);`
					<bytes::BytesMut as bytes::BufMut>::put_bytes(&mut bytes, 0u8, 21);

					Ok(bytes.to_vec())
				}
			}

			impl $crate::rw::Deserialize for $Name { // impl Deserialize for Error
				fn deserialize(buf: &mut impl bytes::Buf) -> $crate::rw::ReadResult<Self> {
					// Skip error and error code; not used for deserialization
					// if we already know what to deserialize.
					buf.advance(2);

					// Sequence. `let sequence = u16::read_2b_from(buf)?;`
					let sequence = <u16 as $crate::rw::ReadValue>::read_2b_from(buf)?;

					// TODO: What to do when $bad_value is missing? Still need
					//       to advance...
					//
					// `let bad_res_id = u32::read_4b_from(buf)?;`
					$(let $bad_data = <u32 as $crate::rw::ReadValue>::read_4b_from(buf)?;)?

					// Minor opcode. `let minor_opcode = u16::read_2b_from(buf)?;`
					let minor_opcode = <u16 as $crate::rw::ReadValue>::read_2b_from(buf)?;
					// Major opcode. `let major_opcode = u8::read_1b_from(buf)?;`
					let major_opcode = <u8 as $crate::rw::ReadValue>::read_1b_from(buf)?;

					// Skip trailing 21 unused bytes.
					buf.advance(21);

					Ok(Self {
						sequence,
						$($bad_data,)? // optional
						minor_opcode,
						major_opcode,
					})
				}
			}
		)+
	};
}
