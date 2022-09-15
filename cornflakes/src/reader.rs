use crate::{Readable, ReadableWithSize, ReadableWithLength};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadError {
	#[error("not enough bytes remaining (expected {expected:?}, found {found:?})")]
	#[allow(dead_code)]
	NotEnoughRemaining { expected: usize, found: usize },
}

// Much of the following code is based on the [bytes] crate. This functionality
// is implemented here so that it can better interface with `cornflakes`'
// 'ecosystem'. Use [bytes] if you are not specifically making use of
// `cornflakes`' features.
//
// [bytes]: https://github.com/tokio-rs/bytes/

/// Generates an implementation for reading a type with a fixed size in bytes.
macro_rules! read_impl {
	($this:ident, $ty:tt::$conversion:tt) => {{
		// Create a temporary buffer with a length equal to the fixed size of
		// `$ty` in bytes.
		let mut temp_buf = [0; (std::mem::size_of::<$ty>())];
		// Fill `temp_buf` with bytes from `self`, advancing the internal cursor
		// by `temp_buf.len()` in doing so.
		$this.copy_to_slice(&mut temp_buf)?;
		// Convert the bytes with the `$conversion` method.
		$ty::$conversion(temp_buf)
	}};
}

/// Returns an error if `$remaining < $expected`.
macro_rules! check_capacity {
	($remaining:expr, $expected:expr) => {
		if $remaining < $expected {
			return Err(ReadError::NotEnoughRemaining {
				expected: $expected,
				found: $remaining,
			});
		}
	};
}

/// Provides utilities to read data from buffers of bytes.
///
/// This trait is based on the implementation of the `Buf` trait in [bytes]. Its
/// purpose in `cornflakes` is simply to provide utilities more specifically
/// catered towards `cornflakes`' 'ecosystem'. Specifically, it works well with
/// [`ReadBytes`], [`ReadList`], and [`ReadSized`], and it provides methods for
/// reading data with the native endianness.
///
/// If the above functionality provided by `cornflakes` is not useful to you,
/// you may wish to use [bytes] instead.
///
/// [bytes]: https://github.com/tokio-rs/bytes/
#[doc(notable_trait)]
pub trait Reader {
	/// Reads a [`Readable`] type.
	fn read<T>(&mut self) -> Result<T, ReadError>
	where
		T: Readable,
		Self: Sized,
	{
		T::read_from(self)
	}

	/// Reads a [`ReadableWithSize`] type with the given number of bytes.
	fn read_with_size<T>(&mut self, num_bytes: usize) -> Result<T, ReadError>
	where
		T: ReadableWithSize,
		Self: Sized,
	{
		T::read_from_with_count(self, num_bytes)
	}

	/// Reads a [`WritableWithLength`] list of values with the given `length`.
	fn read_list<T>(&mut self, length: usize) -> Result<T, ReadError>
	where
		T: ReadableWithLength,
		Self: Sized,
	{
		T::read_from_with_length(self, length)
	}

	/// Returns a <code>[Vec]<[u8]></code> with a length of `num_bytes` bytes
	/// read from `self`.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// `num_bytes` is greater than [`remaining()`].
	///
	/// [`remaining()`]: Reader::remaining
	fn read_bytes(&mut self, num_bytes: usize) -> Result<Vec<u8>, ReadError> {
		let mut bytes = vec![0; num_bytes];
		self.copy_to_slice(&mut bytes)?;

		Ok(bytes)
	}

	/// Returns the number of bytes remaining from the current position until
	/// the end of the `Reader`.
	fn remaining(&self) -> usize;

	/// Returns whether the number of bytes [`remaining()`] in this `Reader` is
	/// greater than zero.
	fn has_remaining(&self) -> bool {
		self.remaining() > 0
	}

	/// Returns a chunk of data starting at the current position and less than
	/// or equal to the number of [`remaining()`] bytes.
	///
	/// This method allows for a `Reader` to be stored non-contiguously; that
	/// is, the `chunk()` of data returned is not necessarily all of the bytes
	/// [`remaining()`] in the `Reader`.
	///
	/// This method may return a slice of zero bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn chunk(&self) -> &[u8];

	/// Advances the internal cursor of this [`Reader`] by `num_bytes`.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// `num_bytes` is greater than [`remaining()`].
	///
	/// # Implementation notes
	/// This method should not panic. Return an appropriate [`ReadError`]
	/// instead.
	///
	/// [`remaining()`]: Reader::remaining
	fn advance(&mut self, num_bytes: usize) -> Result<(), ReadError>;

	/// Copies bytes from `self` to fill `slice`.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// `slice.len()` is greater than [`remaining()`].
	///
	/// [`remaining()`]: Reader::remaining
	fn copy_to_slice(&mut self, slice: &mut [u8]) -> Result<(), ReadError> {
		// If there are not enough bytes [`remaining()`] in `self` to fill the
		// `slice`, we return a `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), slice.len());

		// Keep track of the position in the `slice` of bytes. This is because
		// the `Reader` may be stored as multiple chunks of data: we need to
		// keep writing every chunk until we've filled `slice`, so we need to
		// know how much we've already written.
		let mut offset = 0;

		while offset < slice.len() {
			let num_bytes;

			unsafe {
				// Get a chunk of data stored for this `Reader`.
				let bytes = self.chunk();
				// Set `num_bytes` to the smaller of:
				// - the number of bytes we got in the chunk
				// - the number of bytes left to write
				//
				// We are going to write this many bytes, so we either need to
				// write to the end of the chunk, or to the end of the slice,
				// depending on which comes first.
				num_bytes = std::cmp::min(bytes.len(), slice.len() - offset);

				// Copy `num_bytes` number of bytes to the `slice`.
				std::ptr::copy_nonoverlapping(
					bytes.as_ptr(),
					slice[offset..].as_mut_ptr(),
					num_bytes,
				);

				// Increase the `offset` by the `num_bytes` copied.
				offset += num_bytes;
			}

			// Advance the internal cursor by the number of bytes copied.
			self.advance(num_bytes)?;
		}

		Ok(())
	}

	/// Reads a boolean value from one byte.
	///
	/// Advances the internal cursor by `1` byte.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is `0` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_bool(&mut self) -> Result<bool, ReadError> {
		Ok(self.read_u8()? != 0)
	}

	/// Reads an 8-bit unsigned integer.
	///
	/// Advances the internal cursor by `1` byte.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is `0` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_u8(&mut self) -> Result<u8, ReadError> {
		// If there is not at least 1 byte remaining, then we can't read a
		// `u8` value, so we return a `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 1);

		// Since a byte is just a `u8` value, we can simply take the first byte.
		// There is no endianness to consider, because there is only one way to
		// arrange a single byte.
		let val = self.chunk()[0];
		self.advance(1)?;

		Ok(val)
	}

	/// Reads an 8-bit signed integer.
	///
	/// Advances the internal cursor by `1` byte.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is `0` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_i8(&mut self) -> Result<i8, ReadError> {
		// If there is not at least 1 byte remaining, then we can't read an
		// `i8` value, so we return a `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 1);

		// Since a byte is just a `u8` value, and we can cast a `u8` value to an
		// `i8` value with no cost, we can simply return the first byte `as i8`.
		//
		// Since an `i8` value is a single byte, and there is only one way to
		// order a single byte, we don't have to consider endianness.
		let val = self.chunk()[0] as i8;
		self.advance(1)?;

		Ok(val)
	}

	// Native-endian {{{

	/// Reads a 16-bit unsigned integer with the native endianness.
	///
	/// Advances the internal cursor by `2` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `2` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_u16_ne(&mut self) -> Result<u16, ReadError> {
		// If there are less than 2 bytes left in `self` then there are not
		// enough bytes to read a `u16` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 2);

		Ok(read_impl!(self, u16::from_ne_bytes))
	}

	/// Reads a 16-bit signed integer with the native endianness.
	///
	/// Advances the internal cursor by `2` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `2` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_i16_ne(&mut self) -> Result<i16, ReadError> {
		// If there are less than 2 bytes left in `self` then there are not
		// enough bytes to read an `i16` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 2);

		Ok(read_impl!(self, i16::from_ne_bytes))
	}

	/// Reads a 32-bit unsigned integer with the native endianness.
	///
	/// Advances the internal cursor by `4` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `4` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_u32_ne(&mut self) -> Result<u32, ReadError> {
		// If there are less than 4 bytes left in `self` then there are not
		// enough bytes to read a `u32` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 4);

		Ok(read_impl!(self, u32::from_ne_bytes))
	}

	/// Reads a 32-bit signed integer with the native endianness.
	///
	/// Advances the internal cursor by `4` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `4` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_i32_ne(&mut self) -> Result<i32, ReadError> {
		// If there are less than 4 bytes left in `self` then there are not
		// enough bytes to read an `i32` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 4);

		Ok(read_impl!(self, i32::from_ne_bytes))
	}

	/// Reads a 64-bit unsigned integer with the native endianness.
	///
	/// Advances the internal cursor by `8` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `8` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_u64_ne(&mut self) -> Result<u64, ReadError> {
		// If there are less than 8 bytes left in `self` then there are not
		// enough bytes to read a `u64` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 8);

		Ok(read_impl!(self, u64::from_ne_bytes))
	}

	/// Reads a 64-bit signed integer with the native endianness.
	///
	/// Advances the internal cursor by `8` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `8` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_i64_ne(&mut self) -> Result<i64, ReadError> {
		// If there are less than 8 bytes left in `self` then there are not
		// enough bytes to read an `i64` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 8);

		Ok(read_impl!(self, i64::from_ne_bytes))
	}

	/// Reads a 128-bit unsigned integer with the native endianness.
	///
	/// Advances the internal cursor by `16` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `16` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_u128_ne(&mut self) -> Result<u128, ReadError> {
		// If there are less than 16 bytes left in `self` then there are not
		// enough bytes to read a `u128` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 16);

		Ok(read_impl!(self, u128::from_ne_bytes))
	}

	/// Reads a 128-bit signed integer with the native endianness.
	///
	/// Advances the internal cursor by `16` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `16` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_i128_ne(&mut self) -> Result<i128, ReadError> {
		// If there are less than 16 bytes left in `self` then there are not
		// enough bytes to read an `i128` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 16);

		Ok(read_impl!(self, i128::from_ne_bytes))
	}

	/// Reads a 32-bit floating point number with the native endianness.
	///
	/// Advances the internal cursor by `4` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `4` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_f32_ne(&mut self) -> Result<f32, ReadError> {
		// If there are less than 4 bytes left in `self` then there are not
		// enough bytes to read an `f32` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 4);

		Ok(read_impl!(self, f32::from_ne_bytes))
	}

	/// Reads a 64-bit floating point number with the native endianness.
	///
	/// Advances the internal cursor by `8` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `8` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_f64_ne(&mut self) -> Result<f64, ReadError> {
		// If there are less than 8 bytes left in `self` then there are not
		// enough bytes to read an `f64` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 8);

		Ok(read_impl!(self, f64::from_ne_bytes))
	}

	// }}}

	// Big-endian {{{

	/// Reads a 16-bit unsigned integer with big endianness.
	///
	/// Advances the internal cursor by `2` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `2` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_u16_be(&mut self) -> Result<u16, ReadError> {
		// If there are less than 2 bytes left in `self` then there are not
		// enough bytes to read a `u16` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 2);

		Ok(read_impl!(self, u16::from_be_bytes))
	}

	/// Reads a 16-bit signed integer with big endianness.
	///
	/// Advances the internal cursor by `2` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `2` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_i16_be(&mut self) -> Result<i16, ReadError> {
		// If there are less than 2 bytes left in `self` then there are not
		// enough bytes to read an `i16` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 2);

		Ok(read_impl!(self, i16::from_be_bytes))
	}

	/// Reads a 32-bit unsigned integer with big endianness.
	///
	/// Advances the internal cursor by `4` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `4` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_u32_be(&mut self) -> Result<u32, ReadError> {
		// If there are less than 4 bytes left in `self` then there are not
		// enough bytes to read a `u32` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 4);

		Ok(read_impl!(self, u32::from_be_bytes))
	}

	/// Reads a 32-bit signed integer with big endianness.
	///
	/// Advances the internal cursor by `4` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `4` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_i32_be(&mut self) -> Result<i32, ReadError> {
		// If there are less than 4 bytes left in `self` then there are not
		// enough bytes to read an `i32` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 4);

		Ok(read_impl!(self, i32::from_be_bytes))
	}

	/// Reads a 64-bit unsigned integer with big endianness.
	///
	/// Advances the internal cursor by `8` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `8` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_u64_be(&mut self) -> Result<u64, ReadError> {
		// If there are less than 8 bytes left in `self` then there are not
		// enough bytes to read a `u64` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 8);

		Ok(read_impl!(self, u64::from_be_bytes))
	}

	/// Reads a 64-bit signed integer with big endianness.
	///
	/// Advances the internal cursor by `8` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `8` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_i64_be(&mut self) -> Result<i64, ReadError> {
		// If there are less than 8 bytes left in `self` then there are not
		// enough bytes to read an `i64` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 8);

		Ok(read_impl!(self, i64::from_be_bytes))
	}

	/// Reads a 128-bit unsigned integer with big endianness.
	///
	/// Advances the internal cursor by `16` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `16` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_u128_be(&mut self) -> Result<u128, ReadError> {
		// If there are less than 16 bytes left in `self` then there are not
		// enough bytes to read a `u128` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 16);

		Ok(read_impl!(self, u128::from_be_bytes))
	}

	/// Reads a 128-bit signed integer with big endianness.
	///
	/// Advances the internal cursor by `16` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `16` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_i128_be(&mut self) -> Result<i128, ReadError> {
		// If there are less than 16 bytes left in `self` then there are not
		// enough bytes to read an `i128` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 16);

		Ok(read_impl!(self, i128::from_be_bytes))
	}

	/// Reads a 32-bit floating point number with big endianness.
	///
	/// Advances the internal cursor by `4` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `4` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_f32_be(&mut self) -> Result<f32, ReadError> {
		// If there are less than 4 bytes left in `self` then there are not
		// enough bytes to read an `f32` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 4);

		Ok(read_impl!(self, f32::from_be_bytes))
	}

	/// Reads a 64-bit floating point number with big endianness.
	///
	/// Advances the internal cursor by `8` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `8` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_f64_be(&mut self) -> Result<f64, ReadError> {
		// If there are less than 8 bytes left in `self` then there are not
		// enough bytes to read an `f64` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 8);

		Ok(read_impl!(self, f64::from_ne_bytes))
	}

	// }}}

	// Little-endian {{{

	/// Reads a 16-bit unsigned integer with little endianness.
	///
	/// Advances the internal cursor by `2` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `2` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_u16_le(&mut self) -> Result<u16, ReadError> {
		// If there are less than 2 bytes left in `self` then there are not
		// enough bytes to read a `u16` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 2);

		Ok(read_impl!(self, u16::from_le_bytes))
	}

	/// Reads a 16-bit signed integer with little endianness.
	///
	/// Advances the internal cursor by `2` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `2` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_i16_le(&mut self) -> Result<i16, ReadError> {
		// If there are less than 2 bytes left in `self` then there are not
		// enough bytes to read an `i16` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 2);

		Ok(read_impl!(self, i16::from_le_bytes))
	}

	/// Reads a 32-bit unsigned integer with little endianness.
	///
	/// Advances the internal cursor by `4` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `4` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_u32_le(&mut self) -> Result<u32, ReadError> {
		// If there are less than 4 bytes left in `self` then there are not
		// enough bytes to read a `u32` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 4);

		Ok(read_impl!(self, u32::from_le_bytes))
	}

	/// Reads a 32-bit signed integer with little endianness.
	///
	/// Advances the internal cursor by `4` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `4` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_i32_le(&mut self) -> Result<i32, ReadError> {
		// If there are less than 4 bytes left in `self` then there are not
		// enough bytes to read an `i32` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 4);

		Ok(read_impl!(self, i32::from_le_bytes))
	}

	/// Reads a 64-bit unsigned integer with little endianness.
	///
	/// Advances the internal cursor by `8` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `8` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_u64_le(&mut self) -> Result<u64, ReadError> {
		// If there are less than 8 bytes left in `self` then there are not
		// enough bytes to read a `u64` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 8);

		Ok(read_impl!(self, u64::from_le_bytes))
	}

	/// Reads a 64-bit signed integer with little endianness.
	///
	/// Advances the internal cursor by `8` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `8` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_i64_le(&mut self) -> Result<i64, ReadError> {
		// If there are less than 8 bytes left in `self` then there are not
		// enough bytes to read an `i64` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 8);

		Ok(read_impl!(self, i64::from_le_bytes))
	}

	/// Reads a 128-bit unsigned integer with little endianness.
	///
	/// Advances the internal cursor by `16` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `16` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_u128_le(&mut self) -> Result<u128, ReadError> {
		// If there are less than 16 bytes left in `self` then there are not
		// enough bytes to read a `u128` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 16);

		Ok(read_impl!(self, u128::from_le_bytes))
	}

	/// Reads a 128-bit signed integer with little endianness.
	///
	/// Advances the internal cursor by `16` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `16` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_i128_le(&mut self) -> Result<i128, ReadError> {
		// If there are less than 16 bytes left in `self` then there are not
		// enough bytes to read an `i128` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 16);

		Ok(read_impl!(self, i128::from_le_bytes))
	}

	/// Reads a 32-bit floating point number with little endianness.
	///
	/// Advances the internal cursor by `4` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `4` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_f32_le(&mut self) -> Result<f32, ReadError> {
		// If there are less than 4 bytes left in `self` then there are not
		// enough bytes to read an `f32` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 4);

		Ok(read_impl!(self, f32::from_le_bytes))
	}

	/// Reads a 64-bit floating point number with little endianness.
	///
	/// Advances the internal cursor by `8` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `8` bytes.
	///
	/// [`remaining()`]: Reader::remaining
	fn read_f64_le(&mut self) -> Result<f64, ReadError> {
		// If there are less than 8 bytes left in `self` then there are not
		// enough bytes to read an `f64` value, so we return a
		// `NotEnoughRemaining` error.
		check_capacity!(self.remaining(), 8);

		Ok(read_impl!(self, f64::from_le_bytes))
	}

	// }}}
}

impl Reader for &[u8] {
	fn remaining(&self) -> usize {
		self.len()
	}

	fn chunk(&self) -> &[u8] {
		self
	}

	fn advance(&mut self, num_bytes: usize) -> Result<(), ReadError> {
		// Since a slice is a reference to a slice of a list, we can advance by
		// `num_bytes` by setting `self` to start `num_bytes` later:
		*self = &self[num_bytes..];

		Ok(())
	}
}

// This function is unused, but it asserts that `Reader` is object safe; that
// is, it ensures the Rust compiler will generate an error if `Reader` is ever
// made _object unsafe_ by accident, which means that it cannot be used with the
// `dyn` keyword.
fn _assert_reader_object_safety(_reader: &dyn Reader) {}
