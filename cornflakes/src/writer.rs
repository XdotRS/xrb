use crate::rw::{ByteCount, Writable, WritableWithCount};
use crate::util::reader::Reader;

use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WriteError {
	#[error("not enough remaining capacity (required {required:?} bytes, found {found:?} bytes)")]
	#[allow(dead_code)]
	CapacityTooLow { required: usize, found: usize },
}

/// Returns an error if `$remaining < $expected`.
macro_rules! check_capacity {
	($remaining:expr, $required:expr) => {
		if $remaining < $required {
			return Err(WriteError::CapacityTooLow {
				required: $required,
				found: $remaining,
			});
		}
	};
}

/// Returns a boxed error if `$remaining < $expected`.
macro_rules! check_capacity_boxed {
	($remaining:expr, $required:expr) => {
		if $remaining < $required {
			return Err(Box::new(WriteError::CapacityTooLow {
				required: $required,
				found: $remaining,
			}));
		}
	};
}

#[doc(notable_trait)]
pub trait Writer {
	/// Writes a [`Writable`] type as bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if
	/// [`writable::byte_count()`] is greater than [`remaining()`].
	///
	/// [`writable::byte_count()`]: ByteCount::byte_count()
	fn write<T>(&mut self, writable: T) -> Result<(), WriteError>
	where
		T: Writable + ByteCount,
		Self: Sized,
	{
		writable.write_to(self)
	}

	/// Writes a [`WritableWithCount`] type as bytes with the given byte count.
	fn write_with_count<T>(&mut self, writable: T, num_bytes: usize) -> Result<(), WriteError>
	where
		T: WritableWithCount,
		Self: Sized,
	{
		writable.write_to_with_count(self, num_bytes)
	}

	/// Writes the bytes of a [`Reader`] to `self`.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if
	/// `reader.remaining()` is greater than [`self.remaining()`].
	///
	/// [`self.remaining()`]: Writer::remaining
	fn write_from_reader<T: Reader>(&mut self, mut reader: T) -> Result<(), Box<dyn Error>>
	where
		Self: Sized,
	{
		// If there are not enough bytes `remaining()` in `self` to fit
		// `reader`, we return a `CapacityTooLow` error.
		check_capacity_boxed!(self.remaining(), reader.remaining());

		while reader.has_remaining() {
			let num_bytes;

			unsafe {
				// Get a chunk of data stored within the `reader`.
				let reader_chunk = reader.chunk();
				// Get a mutable chunk to write data to from `self`.
				let writer_chunk = self.chunk();
				// Set `num_bytes` to the smaller of:
				// - the number of bytes we got in the chunk from `reader`
				// - the number of bytes we got in the chunk from `self`
				//
				// We are going to copy this many bytes from `reader` to `self`.
				// We can't write more bytes than fit in our own chunk, nor can
				// we write more bytes than we can read from `reader`'s chunk,
				// which is why we take the `min`.
				num_bytes = std::cmp::min(reader_chunk.len(), writer_chunk.len());

				// Copy `num_bytes` number of bytes from the `reader` to `self`.
				std::ptr::copy_nonoverlapping(
					reader_chunk.as_ptr(),
					writer_chunk.as_mut_ptr() as *mut u8,
					num_bytes,
				);
			}

			// Advance the `reader`'s internal cursor by the `num_bytes` copied.
			reader.advance(num_bytes)?;
			unsafe {
				// Advance `self`'s internal cursor by the `num_bytes` copied.
				self.advance(num_bytes)?;
			}
		}

		Ok(())
	}

	/// Returns the number of bytes remaining from the current position until
	/// the current end of the `Writer`.
	///
	/// The number of bytes `remaining()` may increase if the `Writer` allocates
	/// more memory. This represents only the remaining number of bytes at this
	/// point in time.
	fn remaining(&self) -> usize;

	/// Returns whether there is more than zero bytes remaining in  `self`.
	fn has_remaining(&self) -> bool {
		self.remaining() > 0
	}

	/// Returns a mutable chunk of data starting at the current position and
	/// less than or equal to the number of [`remaining()`] bytes.
	///
	/// This method allows for a `Writer` to be stored non-contiguously; that
	/// is, the `chunk()` of data returned is not necessarily all of the bytes
	/// [`remaining()`] in the `Writer`.
	///
	/// This method may return a slice of zero bytes _if and only if_ the number
	/// of bytes [`remaining()`] is `0`.
	///
	/// [`remaining()`]: Writer::remaining
	fn chunk(&mut self) -> &mut [u8];

	/// Advances the internal cursor of this [`Writer`] by `num_bytes`.
	///
	/// # Unsafe
	/// This method is unsafe because there is no guarantee that the bytes being
	/// advanced over have been initialized.
	unsafe fn advance(&mut self, num_bytes: usize) -> Result<(), WriteError>;

	/// Writes a slice of `bytes` to `self`.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if
	/// `slice.len()` is greater than [`self.remaining()`].
	///
	/// [`self.remaining()`]: Writer::remaining
	fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), WriteError> {
		check_capacity!(self.remaining(), bytes.len());

		let mut offset = 0;

		while offset < bytes.len() {
			let num_bytes;

			unsafe {
				// Get a chunk from `self` to write data to.
				let chunk = self.chunk();
				// Set `num_bytes` to the smaller of:
				// - the number of bytes we got in the chunk
				// - the number of bytes left to write
				//
				// We are going to write this many bytes, so we either need to
				// write to the end of the chunk, or to the e nd of the slice,
				// depending on which comes first.
				num_bytes = std::cmp::min(chunk.len(), bytes.len() - offset);

				// Copy `num_bytes` number of bytes from `bytes` to `self`.
				std::ptr::copy_nonoverlapping(
					bytes[offset..].as_ptr(),
					chunk.as_mut_ptr() as *mut u8,
					num_bytes,
				);

				// Increase the `offset` by the `num_bytes` copied.
				offset += num_bytes;

				// Advance the internal cursor by the number of bytes written.
				self.advance(num_bytes)?;
			}
		}

		Ok(())
	}

	/// Writes a boolean value as one byte.
	///
	/// Advances the internal cursor by `1` byte.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// `0` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_bool(&mut self, val: bool) -> Result<(), WriteError> {
		self.write_u8(val as u8)
	}

	/// Writes an 8-bit unsigned integer to `self`.
	///
	/// Advances the internal cursor by `1` byte.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// `0` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_u8(&mut self, val: u8) -> Result<(), WriteError> {
		// Since a byte is just a `u8` value, we can simply write the `val`
		// directly by putting it in a slice and using the existing
		// `self.write_slice(slice)` method. There is no endianness to consider,
		// because there is only one way to arrange a single byte.
		let val = [val];
		self.write_bytes(&val)?;

		Ok(())
	}

	/// Writes an 8-bit signed integer to `self`.
	///
	/// Advances the internal cursor by `1` byte.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// `0` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_i8(&mut self, val: i8) -> Result<(), WriteError> {
		// Since a byte is just a `u8` value, and we can cast an `i8` value to a
		// `u8` value, we can simply write the `val as u8` directly by putting
		// it in a slice and using the existing `self.write_slice(slice)`
		// method. There is no endianness to consider, because there is only one
		// way to arrange a single byte.
		let val = [val as u8];
		self.write_bytes(&val)?;

		Ok(())
	}

	// Native-endian {{{

	/// Writes a 16-bit unsigned integer to `self` with the native endianness.
	///
	/// Advances the internal cursor by `2` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `2` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_u16_ne(&mut self, val: u16) -> Result<(), WriteError> {
		self.write_bytes(&val.to_ne_bytes())
	}

	/// Writes a 16-bit signed integer to `self` with the native endianness.
	///
	/// Advances the internal cursor by `2` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `2` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_i16_ne(&mut self, val: i16) -> Result<(), WriteError> {
		self.write_bytes(&val.to_ne_bytes())
	}

	/// Writes a 32-bit unsigned integer to `self` with the native endianness.
	///
	/// Advances the internal cursor by `4` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `4` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_u32_ne(&mut self, val: u32) -> Result<(), WriteError> {
		self.write_bytes(&val.to_ne_bytes())
	}

	/// Writes a 32-bit signed integer to `self` with the native endianness.
	///
	/// Advances the internal cursor by `4` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `4` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_i32_ne(&mut self, val: i32) -> Result<(), WriteError> {
		self.write_bytes(&val.to_ne_bytes())
	}

	/// Writes a 64-bit unsigned integer to `self` with the native endianness.
	///
	/// Advances the internal cursor by `8` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `8` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_u64_ne(&mut self, val: u64) -> Result<(), WriteError> {
		self.write_bytes(&val.to_ne_bytes())
	}

	/// Writes a 64-bit signed integer to `self` with the native endianness.
	///
	/// Advances the internal cursor by `8` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `8` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_i64_ne(&mut self, val: i64) -> Result<(), WriteError> {
		self.write_bytes(&val.to_ne_bytes())
	}

	/// Writes a 128-bit unsigned integer to `self` with the native endianness.
	///
	/// Advances the internal cursor by `16` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `16` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_u128_ne(&mut self, val: u128) -> Result<(), WriteError> {
		self.write_bytes(&val.to_ne_bytes())
	}

	/// Writes a 128-bit signed integer to `self` with the native endianness.
	///
	/// Advances the internal cursor by `16` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `16` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_i128_ne(&mut self, val: i128) -> Result<(), WriteError> {
		self.write_bytes(&val.to_ne_bytes())
	}

	// }}}

	// Big-endian {{{

	/// Writes a 16-bit unsigned integer to `self` with big endianness.
	///
	/// Advances the internal cursor by `2` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `2` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_u16_be(&mut self, val: u16) -> Result<(), WriteError> {
		self.write_bytes(&val.to_be_bytes())
	}

	/// Writes a 16-bit signed integer to `self` with big endianness.
	///
	/// Advances the internal cursor by `2` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `2` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_i16_be(&mut self, val: i16) -> Result<(), WriteError> {
		self.write_bytes(&val.to_be_bytes())
	}

	/// Writes a 32-bit unsigned integer to `self` with big endianness.
	///
	/// Advances the internal cursor by `4` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `4` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_u32_be(&mut self, val: u32) -> Result<(), WriteError> {
		self.write_bytes(&val.to_be_bytes())
	}

	/// Writes a 32-bit signed integer to `self` with big endianness.
	///
	/// Advances the internal cursor by `4` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `4` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_i32_be(&mut self, val: i32) -> Result<(), WriteError> {
		self.write_bytes(&val.to_be_bytes())
	}

	/// Writes a 64-bit unsigned integer to `self` with big endianness.
	///
	/// Advances the internal cursor by `8` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `8` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_u64_be(&mut self, val: u64) -> Result<(), WriteError> {
		self.write_bytes(&val.to_be_bytes())
	}

	/// Writes a 64-bit signed integer to `self` with big endianness.
	///
	/// Advances the internal cursor by `8` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `8` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_i64_be(&mut self, val: i64) -> Result<(), WriteError> {
		self.write_bytes(&val.to_be_bytes())
	}

	/// Writes a 128-bit unsigned integer to `self` with big endianness.
	///
	/// Advances the internal cursor by `16` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `16` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_u128_be(&mut self, val: u128) -> Result<(), WriteError> {
		self.write_bytes(&val.to_be_bytes())
	}

	/// Writes a 128-bit signed integer to `self` with big endianness.
	///
	/// Advances the internal cursor by `16` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `16` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_i128_be(&mut self, val: i128) -> Result<(), WriteError> {
		self.write_bytes(&val.to_be_bytes())
	}

	// }}}

	// Little-endian {{{

	/// Writes a 16-bit unsigned integer to `self` with little endianness.
	///
	/// Advances the internal cursor by `2` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `2` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_u16_le(&mut self, val: u16) -> Result<(), WriteError> {
		self.write_bytes(&val.to_le_bytes())
	}

	/// Writes a 16-bit signed integer to `self` with little endianness.
	///
	/// Advances the internal cursor by `2` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `2` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_i16_le(&mut self, val: i16) -> Result<(), WriteError> {
		self.write_bytes(&val.to_le_bytes())
	}

	/// Writes a 32-bit unsigned integer to `self` with little endianness.
	///
	/// Advances the internal cursor by `4` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `4` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_u32_le(&mut self, val: u32) -> Result<(), WriteError> {
		self.write_bytes(&val.to_le_bytes())
	}

	/// Writes a 32-bit signed integer to `self` with little endianness.
	///
	/// Advances the internal cursor by `4` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `4` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_i32_le(&mut self, val: i32) -> Result<(), WriteError> {
		self.write_bytes(&val.to_le_bytes())
	}

	/// Writes a 64-bit unsigned integer to `self` with little endianness.
	///
	/// Advances the internal cursor by `8` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `8` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_u64_le(&mut self, val: u64) -> Result<(), WriteError> {
		self.write_bytes(&val.to_le_bytes())
	}

	/// Writes a 64-bit signed integer to `self` with little endianness.
	///
	/// Advances the internal cursor by `8` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `8` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_i64_le(&mut self, val: i64) -> Result<(), WriteError> {
		self.write_bytes(&val.to_le_bytes())
	}

	/// Writes a 128-bit unsigned integer to `self` with little endianness.
	///
	/// Advances the internal cursor by `16` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `16` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_u128_le(&mut self, val: u128) -> Result<(), WriteError> {
		self.write_bytes(&val.to_le_bytes())
	}

	/// Writes a 128-bit signed integer to `self` with little endianness.
	///
	/// Advances the internal cursor by `16` bytes.
	///
	/// # Errors
	/// This method returns a [`WriteError::CapacityTooLow`] error if there are
	/// less than `16` bytes [`remaining()`].
	///
	/// [`remaining()`]: Writer::remaining
	fn write_i128_le(&mut self, val: i128) -> Result<(), WriteError> {
		self.write_bytes(&val.to_le_bytes())
	}

	// }}}
}

// This function is unused, but it asserts that `Writer` is object safe; that
// is, it ensures the Rust compiler will generate an error if `Writer` is ever
// made _object unsafe_ by accident, which means that it cannot be used with the
// `dyn` keyword.
fn _assert_writer_object_safety(_writer: &dyn Writer) {}
