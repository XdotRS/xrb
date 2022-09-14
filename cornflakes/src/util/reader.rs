use crate::rw::{ReadBytes, ReadList, ReadSized};
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
macro_rules! reader_impl {
	($this:ident, $ty:tt::$conversion:tt) => {{
		// Create a temporary buffer with a length equal to the fixed size of
		// `$ty` in bytes.
		let mut temp_buf = [0; (core::mem::size_of::<$ty>())];
		// Copy `temp_buf.len()` many bytes into `temp_buf`, advancing in the
		// process.
		$this.copy_to_slice(&mut temp_buf)?;
		// Convert the bytes with the `$conversion` method.
		$ty::$conversion(temp_buf)
	}};
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
pub trait Reader {
	/// Reads a [`ReadBytes`]-implementing type.
	fn read<T>(&mut self) -> Result<T, ReadError>
	where
		T: ReadBytes,
		Self: Sized,
	{
		T::read(self)
	}

	/// Reads a [`ReadBytes`]-implementing type from the given number of bytes.
	fn read_with_size<T>(&mut self, num_bytes: usize) -> Result<T, ReadError>
	where
		T: ReadSized,
		Self: Sized,
	{
		T::read_with_size(num_bytes, self)
	}

	/// Reads a [`ReadList`]-implementing list of values with the given
	/// `length`.
	fn read_list<T>(&mut self, length: usize) -> Result<T, ReadError>
	where
		T: ReadList,
		Self: Sized,
	{
		T::read(length, self)
	}

	/// Returns the number of bytes remaining between the current position and
	/// the end of the reader.
	fn remaining(&self) -> usize;

	/// Returns a slice, beginning at the current position, of between `0` and
	/// [`remaining()`] bytes.
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
	fn copy_to_slice(&mut self, slice: &mut [u8]) -> Result<(), ReadError> {
		// If there are not enough bytes [`remaining()`] in `self` to fill the
		// `slice`, we return a `NotEnoughRemaining` error.
		if self.remaining() < slice.len() {
			return Err(ReadError::NotEnoughRemaining {
				expected: slice.len(),
				found: self.remaining(),
			});
		}

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

	/// Reads an 8-bit unsigned integer.
	///
	/// Advances the internal cursor by `1` byte.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is `0` bytes.
	fn read_u8(&mut self) -> Result<u8, ReadError> {
		// If there is not at least 1 byte remaining, then we can't read a
		// `u8` value, so we return a `NotEnoughRemaining` error.
		if self.remaining() == 0 {
			return Err(ReadError::NotEnoughRemaining {
				expected: 1,
				found: 0,
			});
		}

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
	fn read_i8(&mut self) -> Result<i8, ReadError> {
		// If there is not at least 1 byte remaining, then we can't read an
		// `i8` value, so we return a `NotEnoughRemaining` error.
		if self.remaining() < 1 {
			return Err(ReadError::NotEnoughRemaining {
				expected: 1,
				found: 0,
			});
		}

		// Since a byte is just a `u8` value, and we can cast a `u8` value to an
		// `i8` value with no cost, we can simply return the first byte `as i8`.
		//
		// Since an `i8` value is a single byte, and there is only one way to
		// order a single byte, we don't have to consider endianness.
		let val = self.chunk()[0] as i8;
		self.advance(1)?;

		Ok(val)
	}

	/// Reads a 16-bit unsigned integer with the native endianness.
	///
	/// Advances the internal cursor by `2` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `2` bytes.
	fn read_u16_ne(&mut self) -> Result<u16, ReadError> {
		// If there are less than 2 bytes left in `self` then there are not
		// enough bytes to read a `u16` value, so we return a
		// `NotEnoughRemaining` error.
		if self.remaining() < 2 {
			return Err(ReadError::NotEnoughRemaining {
				expected: 2,
				found: self.remaining(),
			});
		}

		Ok(reader_impl!(self, u16::from_ne_bytes))
	}

	/// Reads a 16-bit signed integer with the native endianness.
	///
	/// Advances the internal cursor by `2` bytes.
	///
	/// # Errors
	/// This method returns a [`ReadError::NotEnoughRemaining`] error if
	/// [`remaining()`] is less than `2` bytes.
	fn read_i16_ne(&mut self) -> Result<i16, ReadError> {
		// If there are less than 2 bytes left in `self` then there are not
		// enough bytes to read an `i16` value, so we return a
		// `NotEnoughRemaining` error.
		if self.remaining() < 2 {
			return Err(ReadError::NotEnoughRemaining {
				expected: 2,
				found: self.remaining(),
			});
		}

		Ok(reader_impl!(self, i16::from_ne_bytes))
	}
}
