// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[macro_export]
/// Implements [`WriteValue`](crate::WriteValue) and a `mask` method for a
/// request values enum.
macro_rules! values {
	(
		$(
			$(#[$outer:meta])* // attributes
			$vis:vis enum $Value:ident<$Mask:ty> { // pub enum Value<Mask> {
				$(
					$(#[$inner:meta])* // variant attributes
					$Variant:ident($type:ty): $mask:ident // Variant(u32): VARIANT
				),+$(,)? // comma separated, with optional final comma
			}
		)+
	) => {
		$(
			$(#[$outer])* // attributes
			$vis enum $Value { // pub enum Value {
				$(
					$(#[$inner])* // variant attributes
					$Variant($type) // Variant(u32)
				),+
			}

			impl $Value {
				/// Get the value mask associated with this field.
				pub fn mask(&self) -> $Mask {
					match self {
						$(
							// Self::Variant(_) => Mask::VARIANT
							Self::$Variant(_) => <$Mask>::$mask
						),+
					}
				}
			}

			impl $crate::rw::WriteValue for $Value { // impl WriteValue for Value {
				// fn write_1b(self) -> WriteResult<u8> {
				fn write_1b(self) -> $crate::rw::WriteResult<u8> {
					match self {
						$(
							// Self::Variant(val) => val.write_1b()
							Self::$Variant(val) =>
								<$type as $crate::rw::WriteValue>::write_1b(val)
						),+
					}
				}

				// fn write_2b(self) -> WriteResult<u16> {
				fn write_2b(self) -> $crate::rw::WriteResult<u16> {
					match self {
						$(
							// Self::Variant(val) => val.write_2b()
							Self::$Variant(val) =>
								<$type as $crate::rw::WriteValue>::write_2b(val)
						),+
					}
				}

				// fn write_4b(self) -> WriteResult<u32> {
				fn write_4b(self) -> $crate::rw::WriteResult<u32> {
					match self {
						$(
							// Self::Variant(val) => val.write_4b()
							Self::$Variant(val) =>
								<$type as $crate::rw::WriteValue>::write_4b(val)
						),+
					}
				}
			}
		)+
	};
}

#[macro_export]
/// Implements simple requests and their serialization.
///
/// The second byte of a request's header can be used for one of three things:
/// the minor opcode, if this is an extension's request, an additiona data byte,
/// if the request has an additional one-byte data field to fit in, or nothing
/// at all.
///
/// The following syntax can be used to specify a minor opcode:
/// ```rust
/// requests! {
///     pub struct MyExtensionRequest(134,7)[2] {
///         pub window(4): Window,
///     }
/// }
/// ```
/// Where the `7` specifies the minor opcode. If the request is to have no minor
/// opcode, the minor opcode is omitted as follows:
/// ```rust
/// requests! {
///     pub struct DestroyWindow(4)[2] {
///         pub window(4): Window,
///     }
/// }
/// ```
/// Alternatively, provided that no minor opcode is specified, the request can
/// contain an additional data byte in place of a minor opcode:
/// ```rust
/// requests! {
///     pub struct ChangeSaveSet(6)[2] {
///         pub mode: Mode,
///         pub window(4): Window,
///     }
/// }
/// ```
/// Note that the data byte for the header shall be the first field, and it
/// shall not specify its length in bytes. All other fields, if any, shall
/// specify their length in bytes with `name(len)`.
///
/// Note that the preceding examples define the length of the entire request
/// __in units of 4 bytes, _not_ individual bytes__. The length of the request is given by
/// `[2]` in these examples - the request length shall be given enclosed in
/// square brackets (`[` and `]`), after the major (and minor, if given)
/// opcode(s). The length of individual fields, however, is indeed measured in
/// individual bytes. A `u32` value is 4 bytes, so a [`u32`] field will have
/// `(4)` following its name. The length of an individual field may only be one
/// of `1`, `2`, or `4`.
///
/// If there is a reply associated with a request, it can be specified by
/// appending its type within `<` and `>` as follows:
/// ```rust
/// requests! {
///     pub struct GetWindowAttributes<GetWindowAttributesReply>(3)[2] {
///         pub window(4): Window,
///     }
/// }
/// ```
/// This will mean the [`Request`](crate::Request) trait is implemented as
/// `Request<GetWindowAttributesReply>`, rather than the default of `Request<()>`.
///
/// Additionally, due to how common this layout of request fields is, a
/// shorthand can be used to specify a single `window` field:
/// ```rust
/// requests! {
///     pub struct MapWindow(8);
/// }
/// ```
/// This shorthand is equivalent to:
/// ```rust
/// requests! {
///     pub struct MapWindow(8)[2] {
///         pub window(4): Window,
///     }
/// }
/// ```
macro_rules! requests {
	(
		// pub struct Request<Reply>(1,3);
		$vis:vis struct $Request:ident$(<$reply:ty>)?($major:expr,$minor:expr);

		// Other request definitions.
		$($t:tt)*
	) => {
		$vis struct $Request { // pub struct Request {
			/// The target window of this request.
			pub window: $crate::Window, // pub window: Window,
		}

		// impl Request<Reply> for Request {
		impl $crate::Request$(<$reply>)? for $Request {
			fn opcode() -> u8 { $major }

			fn minor_opcode() -> Option<u8> {
				Some($minor)
			}

			fn length(&self) -> u16 { 2 }
		}

		// impl Serialize for Request {
		impl $crate::rw::Serialize for $Request {
			// fn serialize(self) -> WriteResult<Vec<u8>> {
			fn serialize(self) -> $crate::rw::WriteResult<Vec<u8>> {
				let mut bytes = vec![];

				// Header {{{

				// Major opcode
				<u8 as $crate::rw::WriteValue>::write_1b_to($major, &mut bytes)?;

				// Minor opcode
				<u8 as $crate::rw::WriteValue>::write_1b_to($minor, &mut bytes)?;

				// Length
				<u16 as $crate::rw::WriteValue>::write_2b_to(2u16, &mut bytes)?;

				// }}}

				// `window`
				<$crate::Window as $crate::rw::WriteValue>::write_4b_to(
					self.window,
					&mut bytes
				)?;

				Ok(bytes)
			}
		}
	};
	(
		// pub struct Request<Reply>(1);
		$vis:vis struct $Request:ident$(<$reply:ty>)?($major:expr);

		// Other request definitions.
		$($t:tt)*
	) => {
		$vis struct $Request { // pub struct Request {
			/// The target window of this request.
			pub window: $crate::Window, // pub window: Window,
		}

		// impl Request<Reply> for Request {
		impl $crate::Request$(<$reply>)? for $Request {
			fn opcode() -> u8 { $major }

			fn minor_opcode() -> Option<u8> {
				None
			}

			fn length(&self) -> u16 { 2 }
		}

		// impl Serialize for Request {
		impl $crate::rw::Serialize for $Request {
			// fn serialize(self) -> WriteResult<Vec<u8>> {
			fn serialize(self) -> $crate::rw::WriteResult<Vec<u8>> {
				let mut bytes = vec![];

				// Header {{{

				// Major opcode
				<u8 as $crate::rw::WriteValue>::write_1b_to($major, &mut bytes)?;

				// Empty byte
				<u8 as $crate::rw::WriteValue>::write_1b_to(0u8, &mut bytes)?;

				// Length
				<u16 as $crate::rw::WriteValue>::write_2b_to(2u16, &mut bytes)?;

				// }}}

				// `window`
				<$crate::Window as $crate::rw::WriteValue>::write_4b_to(
					self.window,
					&mut bytes
				)?;

				Ok(bytes)
			}
		}
	};
	(
		// pub struct Request<Reply>(1,3)[2] {
		$vis:vis struct $Request:ident$(<$reply:ty>)?($major:expr,$minor:expr)[$len:expr] {
			$(
				// These are additional fields after the header. They specify
				// the length in bytes that they will be written as.

				$(#[$field_attr:meta])* // field attributes
				pub $field:ident($field_len:expr): $field_ty:ty // pub window(4): Window,
			),*$(,)? // optional final comma
		}

		// Other request definitions.
		$($t:tt)*
	) => {
		$vis struct $Request { // pub struct Request {
			$(
				$(#[$field_attr])* // field attributes
				pub $field: $field_ty // pub window: Window,
			),*
		}

		// impl Request<Reply> for Request {
		impl $crate::Request$(<$reply>)? for $Request {
			// major opcode
			fn opcode() -> u8 { $major }

			// minor opcode
			fn minor_opcode() -> Option<u8> {
				Some($minor)
			}

			// length
			fn length(&self) -> u16 { $len }
		}

		// impl Serialize for Request {
		impl $crate::rw::Serialize for $Request {
			// fn serialize(self) -> WriteResult<Vec<u8>> {
			fn serialize(self) -> $crate::rw::WriteResult<Vec<u8>> {
				let mut bytes = vec![];

				// Header {{{

				// Major opcode
				// Self::opcode().write_1b_to(&mut bytes)?;
				<u8 as $crate::rw::WriteValue>::write_1b_to(
					<Self as $crate::Request$(<$reply>)?>::opcode(),
					&mut bytes
				)?;

				// Minor opcode
				// Self::minor_opcode().write_1b_to(&mut bytes)?;
				<u8 as $crate::rw::WriteValue>::write_1b_to(
					<Self as $crate::Request$(<$reply>)?>::minor_opcode().unwrap(),
					&mut bytes
				)?;

				// Length
				// self.length().write_2b_to(&mut bytes)?;
				<u16 as $crate::rw::WriteValue>::write_2b_to(
					<Self as $crate::Request$(<$reply>)?>::length(&self),
					&mut bytes
				)?;

				// }}}

				// Fields {{{

				$(
					match $field_len { // match 4 {
						1 => {
							// If the byte length of this field is 1, write this
							// field as 1 byte.

							// self.window.write_1b_to(&mut bytes)?;
							<$field_ty as $crate::rw::WriteValue>::write_1b_to(
								self.$field,
								&mut bytes
							)?;
						}
						2 => {
							// If the byte length of this field is 2, write this
							// field as 2 bytes.

							// self.window.write_2b_to(&mut bytes)?;
							<$field_ty as $crate::rw::WriteValue>::write_2b_to(
								self.$field,
								&mut bytes
							)?;
						}
						4 => {
							// If the byte length of this field is 4, write this
							// field as 4 bytes.

							// self.window.write_4b_to(&mut bytes)?;
							<$field_ty as $crate::rw::WriteValue>::write_4b_to(
								self.$field,
								&mut bytes
							)?;
						}
						// If another byte length was provided, then there was
						// a mistake. We panic.
						_ => panic!("incorrect field length given in `requests!` macro")
					}
				)*

				// }}}

				Ok(bytes)
			}
		}

		// Repeat for any other requests given.
		$crate::requests!($($t)*);
	};
	(
		// pub struct Request<Reply>(1)[2] {
		$vis:vis struct $Request:ident$(<$reply:ty>)?($major:expr)[$len:expr] {
			// This is for a data byte that sits in the header. It only does
			// anything if there is no minor opcode specified, and is
			// optional.

			$(#[$data_attr:meta])* // data attributes
			pub $data:ident: $data_ty:ty, // pub [mode]: Mode,

			$(
				// These are additional fields after the header. They specify
				// the length in bytes that they will be written as.

				$(#[$field_attr:meta])* // field attributes
				pub $field:ident($field_len:expr): $field_ty:ty // pub window(4): Window,
			),*$(,)? // optional final comma
		}

		// Other request definitions.
		$($t:tt)*
	) => {
		$vis struct $Request { // pub struct Request {
			$(#[$data_attr])* // data attributes
			pub $data: $data_ty, // pub mode: Mode,

			$(
				$(#[$field_attr])* // field attributes
				pub $field: $field_ty // pub window: Window,
			),*
		}

		// impl Request<Reply> for Request {
		impl $crate::Request$(<$reply>)? for $Request {
			// major opcode
			fn opcode() -> u8 { $major }

			// minor opcode
			fn minor_opcode() -> Option<u8> {
				None
			}

			// length
			fn length(&self) -> u16 { $len }
		}

		// impl Serialize for Request {
		impl $crate::rw::Serialize for $Request {
			// fn serialize(self) -> WriteResult<Vec<u8>> {
			fn serialize(self) -> $crate::rw::WriteResult<Vec<u8>> {
				let mut bytes = vec![];

				// Header {{{

				// Major opcode
				// Self::opcode().write_1b_to(&mut bytes)?;
				<u8 as $crate::rw::WriteValue>::write_1b_to(
					<Self as $crate::Request$(<$reply>)?>::opcode(),
					&mut bytes
				)?;

				// Data byte
				// self.$data.write_1b_to(&mut bytes)?;
				<u8 as $crate::rw::WriteValue>::write_1b_to(
					self.$data
					&mut bytes
				)?;

				// Length
				// self.length().write_2b_to(&mut bytes)?;
				<u16 as $crate::rw::WriteValue>::write_2b_to(
					<Self as $crate::Request$(<$reply>)?>::length(&self),
					&mut bytes
				)?;

				// }}}

				// Fields {{{

				$(
					match $field_len { // match 4 {
						1 => {
							// If the byte length of this field is 1, write this
							// field as 1 byte.

							// self.window.write_1b_to(&mut bytes)?;
							<$field_ty as $crate::rw::WriteValue>::write_1b_to(
								self.$field,
								&mut bytes
							)?;
						}
						2 => {
							// If the byte length of this field is 2, write this
							// field as 2 bytes.

							// self.window.write_2b_to(&mut bytes)?;
							<$field_ty as $crate::rw::WriteValue>::write_2b_to(
								self.$field,
								&mut bytes
							)?;
						}
						4 => {
							// If the byte length of this field is 4, write this
							// field as 4 bytes.

							// self.window.write_4b_to(&mut bytes)?;
							<$field_ty as $crate::rw::WriteValue>::write_4b_to(
								self.$field,
								&mut bytes
							)?;
						}
						// If another byte length was provided, then there was
						// a mistake. We panic.
						_ => panic!("incorrect field length given in `requests!` macro")
					}
				)*

				// }}}

				Ok(bytes)
			}
		}

		// Repeat for any other requests given.
		$crate::requests!($($t)*);
	};
	(
		// pub struct Request<Reply>(1)[2] {
		$vis:vis struct $Request:ident$(<$reply:ty>)?($major:expr)[$len:expr] {
			$(
				// These are additional fields after the header. They specify
				// the length in bytes that they will be written as.

				$(#[$field_attr:meta])* // field attributes
				pub $field:ident($field_len:expr): $field_ty:ty // pub window(4): Window,
			),*$(,)? // optional final comma
		}

		// Other request definitions.
		$($t:tt)*
	) => {
		$vis struct $Request { // pub struct Request {
			$(
				$(#[$field_attr])* // field attributes
				pub $field: $field_ty // pub window: Window,
			),*
		}

		// impl Request<Reply> for Request {
		impl $crate::Request$(<$reply>)? for $Request {
			// major opcode
			fn opcode() -> u8 { $major }

			// minor opcode
			fn minor_opcode() -> Option<u8> {
				None
			}

			// length
			fn length(&self) -> u16 { $len }
		}

		// impl Serialize for Request {
		impl $crate::rw::Serialize for $Request {
			// fn serialize(self) -> WriteResult<Vec<u8>> {
			fn serialize(self) -> $crate::rw::WriteResult<Vec<u8>> {
				let mut bytes = vec![];

				// Header {{{

				// Major opcode
				// Self::opcode().write_1b_to(&mut bytes)?;
				<u8 as $crate::rw::WriteValue>::write_1b_to(
					<Self as $crate::Request$(<$reply>)?>::opcode(),
					&mut bytes
				)?;

				// Empty byte
				// 0u8.write_1b_to(&mut bytes)?;
				<u8 as $crate::rw::WriteValue>::write_1b_to(0u8, &mut bytes)?;

				// Length
				// self.length().write_2b_to(&mut bytes)?;
				<u16 as $crate::rw::WriteValue>::write_2b_to(
					<Self as $crate::Request$(<$reply>)?>::length(&self),
					&mut bytes
				)?;

				// }}}

				// Fields {{{

				$(
					match $field_len { // match 4 {
						1 => {
							// If the byte length of this field is 1, write this
							// field as 1 byte.

							// self.window.write_1b_to(&mut bytes)?;
							<$field_ty as $crate::rw::WriteValue>::write_1b_to(
								self.$field,
								&mut bytes
							)?;
						}
						2 => {
							// If the byte length of this field is 2, write this
							// field as 2 bytes.

							// self.window.write_2b_to(&mut bytes)?;
							<$field_ty as $crate::rw::WriteValue>::write_2b_to(
								self.$field,
								&mut bytes
							)?;
						}
						4 => {
							// If the byte length of this field is 4, write this
							// field as 4 bytes.

							// self.window.write_4b_to(&mut bytes)?;
							<$field_ty as $crate::rw::WriteValue>::write_4b_to(
								self.$field,
								&mut bytes
							)?;
						}
						// If another byte length was provided, then there was
						// a mistake. We panic.
						_ => panic!("incorrect field length given in `requests!` macro")
					}
				)*

				// }}}

				Ok(bytes)
			}
		}

		// Repeat for any other requests given.
		$crate::requests!($($t)*);
	};
	() => {};
}
