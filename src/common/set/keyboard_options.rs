// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{Keycode, ToggleOrDefault};
use std::fmt::{Display, Formatter};
use xrbk::{
	Buf,
	BufMut,
	ConstantX11Size,
	ReadError,
	ReadError::UnrecognizedDiscriminant,
	ReadResult,
	Readable,
	Writable,
	WriteResult,
	X11Size,
};
use xrbk_macro::{ConstantX11Size, Readable, Writable, X11Size};

use bitflags::bitflags;
use thiserror::Error;

/// An error generated when a negative value is given for a [`Pitch`] or
/// [`Duration`].
///
/// This error is not generated for [`Pitch::new`] and [`Duration::new`] if the
/// value is `-1` - for those functions, `-1` has a special meaning of `Reset`.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Error)]
#[error("negative values are not allowed, found {0}")]
pub struct NegativeValue(i16);

/// An error generated when a [`Pitch::Reset`] or [`Duration::Reset`] value is
/// attempted to be unwrapped.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Error)]
#[error("tried to unwrap the value of a Reset value")]
pub struct ResetValue;

/// A value representing a non-negative percentage.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Percentage {
	/// Resets the percentage to the default percentage.
	Reset,

	/// Represents a percentage.
	///
	/// The wrapped percentage value can be accessed with [`unwrap()`].
	///
	/// [`unwrap()`]: Percentage::unwrap
	Percent(u8),
}

/// An error generated when a [`Percentage`] is attempted to be created with a
/// `value < 0` or `value > 100`.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Error)]
#[error("percentages must satisfy 0 <= percent <= 100")]
pub struct PercentOutOfBounds;

impl Percentage {
	/// Creates a new `Percentage` from the given `value`.
	///
	/// If `value == -1`, this creates [`Percentage::Reset`]. If
	/// `value >= 0` and `value <= 100`, this creates a [`Percentage::Percent`]
	/// with the given value.
	///
	/// # Errors
	/// If `value < -1` or `value > 100`, this generates a
	/// [`PercentOutOfBounds` error].
	///
	/// [`PercentOutOfBounds` error]: PercentOutOfBounds
	pub fn new(value: i8) -> Result<Self, PercentOutOfBounds> {
		match value {
			reset if reset == -1 => Ok(Self::Reset),

			other => match u8::try_from(other) {
				Ok(percent) if (0..=100).contains(&percent) => Ok(Self::Percent(percent)),
				_ => Err(PercentOutOfBounds),
			},
		}
	}

	/// Creates a new [`Percentage::Reset`].
	#[must_use]
	pub const fn new_reset() -> Self {
		Self::Reset
	}

	/// Creates a new [`Percentage::Percent`] with the given `percentage`.
	///
	/// # Errors
	/// Generates a [`PercentOutOfBoundsU8` error] if `percentage > 100`.
	///
	/// [`PercentOutOfBoundsU8` error]: PercentOutOfBoundsU8
	pub const fn new_percent(percentage: u8) -> Result<Self, PercentOutOfBounds> {
		match percentage {
			percent if percent <= 100 => Ok(Self::Percent(percent)),
			_ => Err(PercentOutOfBounds),
		}
	}

	/// Returns the percentage value wrapped by [`Percentage::Percent`].
	///
	/// # Errors
	/// Generates a [`ResetValue` error] if called on a [`Percentage::Reset`].
	///
	/// [`ResetValue` error]: ResetValue
	pub const fn unwrap(self) -> Result<u8, ResetValue> {
		match self {
			Self::Reset => Err(ResetValue),
			Self::Percent(percent) => Ok(percent),
		}
	}
}

impl Display for Percentage {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Reset => write!(f, "default percentage"),
			Self::Percent(percent) => write!(f, "{percent}%"),
		}
	}
}

/// A value representing a non-negative pitch measured in `Hz`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Pitch {
	/// Resets the pitch to the default pitch.
	Reset,

	/// Represents a pitch value, measured in `Hz`.
	///
	/// The wrapped pitch value can be accessed with [`unwrap()`].
	///
	/// [`unwrap()`]: Pitch::unwrap
	Hz(u8),
}

impl Pitch {
	/// Creates a new `Pitch` from the given `value`.
	///
	/// If `value == -1`, this creates [`Pitch::Reset`]. If `value >= 0`, this
	/// creates a [`Pitch::Hz`] with the given value, measured in hertz.
	///
	/// # Errors
	/// If `value < -1`, this generates a [`NegativeValue` error].
	///
	/// [`NegativeValue` error]: NegativeValue
	pub fn new(value: i16) -> Result<Self, NegativeValue> {
		match value {
			reset if reset == -1 => Ok(Self::Reset),

			other => {
				u8::try_from(other).map_or(Err(NegativeValue(other)), |pitch| Ok(Self::Hz(pitch)))
			},
		}
	}

	/// Creates a new [`Pitch::Reset`].
	#[must_use]
	pub const fn new_reset() -> Self {
		Self::Reset
	}

	/// Creates a new [`Pitch::Hz`] with the specified pitch, measured in hertz.
	#[must_use]
	pub const fn new_pitch(pitch: u8) -> Self {
		Self::Hz(pitch)
	}

	/// Returns the pitch wrapped by [`Pitch::Hz`].
	///
	/// # Errors
	/// Generates a [`ResetValue` error] if called on a [`Pitch::Reset`].
	///
	/// [`ResetValue` error]: ResetValue
	pub const fn unwrap(self) -> Result<u8, ResetValue> {
		match self {
			Self::Reset => Err(ResetValue),
			Self::Hz(pitch) => Ok(pitch),
		}
	}
}

impl Display for Pitch {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Reset => write!(f, "default pitch"),
			Self::Hz(pitch) => write!(f, "{pitch} Hz"),
		}
	}
}

/// A value representing a non-negative duration measured in milliseconds.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Duration {
	/// Resets the duration to the default duration.
	Reset,

	/// Represents a duration, measured in milliseconds.
	///
	/// The wrapped duration can be accessed with [`unwrap()`].
	///
	/// [`unwrap()`]: Duration::unwrap
	Ms(u8),
}

impl Duration {
	/// Creates a new `Duration` from the given `value`.
	///
	/// If `value == -1`, this creates [`Duration::Reset`]. If `value >= 0`,
	/// this creates a [`Duration::Ms`] with the given value, measured in
	/// milliseconds.
	///
	/// # Errors
	/// If `value < -1`, this generates a [`NegativeValue` error].
	///
	/// [`NegativeValue` error]: NegativeValue
	pub fn new(value: i16) -> Result<Self, NegativeValue> {
		match value {
			reset if reset == -1 => Ok(Self::Reset),

			other => u8::try_from(other)
				.map_or(Err(NegativeValue(other)), |duration| Ok(Self::Ms(duration))),
		}
	}

	/// Creates a new [`Duration::Reset`].
	#[must_use]
	pub const fn new_reset() -> Self {
		Self::Reset
	}

	/// Creates a new [`Duration::Ms`] with the specified duration, measured in
	/// milliseconds.
	#[must_use]
	pub const fn new_duration(duration: u8) -> Self {
		Self::Ms(duration)
	}

	/// Returns the duration wrapped by [`Duration::Ms`].
	///
	/// # Errors
	/// Generates a [`ResetValue` error] if called on a [`Duration::Reset`].
	///
	/// [`ResetValue` error]: ResetValue
	pub const fn unwrap(self) -> Result<u8, ResetValue> {
		match self {
			Self::Reset => Err(ResetValue),
			Self::Ms(duration) => Ok(duration),
		}
	}
}

impl Display for Duration {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Reset => write!(f, "default duration"),
			Self::Ms(duration) => write!(f, "{duration} ms"),
		}
	}
}

/// An LED on the keyboard.
///
/// LEDs are numbered starting at one, to a maximum of 32.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Led(u8);

/// Errors generated if the LED number was not in the range `1..=32`.
#[derive(Debug, Error)]
pub enum LedError {
	/// LEDs are numbered starting at one. An LED cannot be zero.
	#[error("keyboard LEDs are numbered from 1 to a maximum of 32, found 0")]
	Zero,

	/// LED numbers cannot be greater than 32.
	#[error("keyboard LEDs a re numbered from 1 to a maximum of 32, found {0}")]
	TooHigh(u8),
}

impl Led {
	/// Creates a new LED number.
	///
	/// LEDs are numbered from one to a maximum of 32.
	///
	/// # Errors
	/// Returns an [`LedError::Zero`] if the given `number == 0`, and an
	/// [`LedError::TooHigh`] if the given `number > 32`.
	pub const fn new(number: u8) -> Result<Self, LedError> {
		match number {
			zero if zero == 0 => Err(LedError::Zero),
			high if high > 32 => Err(LedError::TooHigh(high)),

			led => Ok(Self(led)),
		}
	}

	/// Unwraps the wrapped LED number.
	#[must_use]
	pub const fn unwrap(self) -> u8 {
		let Self(led) = self;

		led
	}
}

/// Whether LEDs are turned on or off.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum LedMode {
	/// The LED(s) is/are turned on.
	Off,
	/// The LED(s) is/are turned off.
	On,
}

/// A set of options which control various aspects of the keyboard.
///
/// This set is used in the [`ChangeKeyboardControl` request].
///
/// [`ChangeKeyboardControl` request]: crate::x11::request::ChangeKeyboardControl
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct KeyboardOptions {
	x11_size: usize,

	mask: KeyboardOptionsMask,

	key_click_volume: Option<__Percentage>,

	bell_volume: Option<__Percentage>,
	bell_pitch: Option<__Pitch>,
	bell_duration: Option<__Duration>,

	led: Option<__Led>,
	led_mode: Option<__LedMode>,

	auto_repeated_key: Option<__Keycode>,

	auto_repeat_mode: Option<__ToggleOrDefault>,
}

impl KeyboardOptions {
	/// Returns a new [`KeyboardOptionsBuilder`] with which a `KeyboardOptions`
	/// set can be created.
	#[must_use]
	pub const fn builder() -> KeyboardOptionsBuilder {
		KeyboardOptionsBuilder::new()
	}
}

/// A builder used to construct a new [`KeyboardOptions` set].
///
/// All configuration options start as [`None`], and can be configured with the
/// methods on this builder. When the builder is configured, [`build()`] can be
/// used to construct the resulting [`KeyboardOptions`].
///
/// [`build()`]: KeyboardOptionsBuilder::build
/// [`KeyboardOptions` set]: KeyboardOptions
#[derive(Clone, Default, Debug, Hash, PartialEq, Eq)]
pub struct KeyboardOptionsBuilder {
	x11_size: usize,

	mask: KeyboardOptionsMask,

	key_click_volume: Option<Percentage>,

	bell_volume: Option<Percentage>,
	bell_pitch: Option<Pitch>,
	bell_duration: Option<Duration>,

	led: Option<Led>,
	led_mode: Option<LedMode>,

	auto_repeated_key: Option<Keycode>,

	auto_repeat_mode: Option<ToggleOrDefault>,
}

impl KeyboardOptionsBuilder {
	/// Creates a new `KeyboardOptionsBuilder`.
	///
	/// All configuration options start as [`None`], and can be configured with
	/// the other methods on this builder. When the builder is configured,
	/// [`build()`] can be used to build the resulting [`KeyboardOptions`].
	///
	/// [`build()`]: KeyboardOptionsBuilder::build
	#[must_use]
	pub const fn new() -> Self {
		Self {
			x11_size: KeyboardOptionsMask::X11_SIZE,

			mask: KeyboardOptionsMask::empty(),

			key_click_volume: None,

			bell_volume: None,
			bell_pitch: None,
			bell_duration: None,

			led: None,
			led_mode: None,

			auto_repeated_key: None,

			auto_repeat_mode: None,
		}
	}

	/// Constructs the resulting [`KeyboardOptions` set] with the configured
	/// options.
	///
	/// [`KeyboardOptions` set]: KeyboardOptions
	#[must_use]
	pub fn build(self) -> KeyboardOptions {
		KeyboardOptions {
			x11_size: self.x11_size,

			mask: self.mask,

			key_click_volume: self.key_click_volume.map(__Percentage),

			bell_volume: self.bell_volume.map(__Percentage),
			bell_pitch: self.bell_pitch.map(__Pitch),
			bell_duration: self.bell_duration.map(__Duration),

			led: self.led.map(__Led),
			led_mode: self.led_mode.map(__LedMode),

			auto_repeated_key: self.auto_repeated_key.map(__Keycode),

			auto_repeat_mode: self.auto_repeat_mode.map(__ToggleOrDefault),
		}
	}
}

impl KeyboardOptionsBuilder {
	/// Configures the volume for key clicks as a [percentage] from 0% to 100%.
	///
	/// See [`KeyboardOptions::key_click_volume`] for more information.
	///
	/// [percentage]: Percentage
	pub fn key_click_volume(&mut self, key_click_volume: Percentage) -> &mut Self {
		if self.key_click_volume.is_none() {
			self.x11_size += 4;
		}

		self.key_click_volume = Some(key_click_volume);
		self.mask |= KeyboardOptionsMask::KEY_CLICK_VOLUME;

		self
	}

	/// Configures the volume of the bell as a [percentage] from 0% to 100%.
	///
	/// See [`KeyboardOptions::bell_volume`] for more information.
	///
	/// [percentage]: Percentage
	pub fn bell_volume(&mut self, bell_volume: Percentage) -> &mut Self {
		if self.bell_volume.is_none() {
			self.x11_size += 4;
		}

		self.bell_volume = Some(bell_volume);
		self.mask |= KeyboardOptionsMask::BELL_VOLUME;

		self
	}
	/// Configures the [pitch] of the bell, measured in hertz.
	///
	/// See [`KeyboardOptions::bell_pitch`] for more information.
	///
	/// [pitch]: Pitch
	pub fn bell_pitch(&mut self, bell_pitch: Pitch) -> &mut Self {
		if self.bell_pitch.is_none() {
			self.x11_size += 4;
		}

		self.bell_pitch = Some(bell_pitch);
		self.mask |= KeyboardOptionsMask::BELL_PITCH;

		self
	}
	/// Configures the [duration] of the bell, measured in milliseconds.
	///
	/// See [`KeyboardOptions::bell_duration`] for more information.
	///
	/// [duration]: Duration
	pub fn bell_duration(&mut self, bell_duration: Duration) -> &mut Self {
		if self.bell_duration.is_none() {
			self.x11_size += 4;
		}

		self.bell_duration = Some(bell_duration);
		self.mask |= KeyboardOptionsMask::BELL_DURATION;

		self
	}

	/// Configures the [LED] which the [configured LED mode] applies to.
	///
	/// See [`KeyboardOptions::led`] for more information.
	///
	/// # Errors
	/// This causes a [`Match` error] to be generated when sent in a
	/// [`ChangeKeyboardControl` request] if it is configured but [`led_mode`]
	/// is not.
	///
	/// [`led_mode`]: KeyboardOptionsBuilder::led_mode
	/// [configured LED mode]: KeyboardOptionsBuilder::led_mode
	/// [LED]: Led
	/// [`Match` error]: crate::x11::error::Match
	/// [`ChangeKeyboardControl` request]: crate::x11::request::ChangeKeyboardControl
	pub fn led(&mut self, led: Led) -> &mut Self {
		if self.led.is_none() {
			self.x11_size += 4;
		}

		self.led = Some(led);
		self.mask |= KeyboardOptionsMask::LED;

		self
	}
	/// Configures the state of the [configured LED mode], if any, otherwise all
	/// [LEDs].
	///
	/// See [`KeyboardOptions::led_mode`] for more information.
	///
	/// [configured LED mode]: KeyboardOptionsBuilder::led_mode
	/// [LEDs]: Led
	pub fn led_mode(&mut self, led_mode: LedMode) -> &mut Self {
		if self.led_mode.is_none() {
			self.x11_size += 4;
		}

		self.led_mode = Some(led_mode);
		self.mask |= KeyboardOptionsMask::LED_MODE;

		self
	}

	/// Configures the key which the [`auto_repeat_mode`] applies to.
	///
	/// See [`KeyboardOptions::auto_repeated_key`] for more information.
	///
	/// # Errors
	/// This causes a [`Match` error] to be generated when sent in a
	/// [`ChangeKeyboardControl` request] if it is configured but
	/// [`auto_repeat_mode`] is not.
	///
	/// [`auto_repeat_mode`]: KeyboardOptionsBuilder::auto_repeat_mode
	///
	/// [`Match` error]: crate::x11::error::Match
	/// [`ChangeKeyboardControl` request]: crate::x11::request::ChangeKeyboardControl
	pub fn auto_repeated_key(&mut self, key: Keycode) -> &mut Self {
		if self.auto_repeated_key.is_none() {
			self.x11_size += 4;
		}

		self.auto_repeated_key = Some(key);
		self.mask |= KeyboardOptionsMask::AUTO_REPEATED_KEY;

		self
	}

	/// Configures whether auto repeat mode is enabled.
	///
	/// See [`KeyboardOptions::auto_repeat_mode`] for more information.
	pub fn auto_repeat_mode(&mut self, auto_repeat_mode: ToggleOrDefault) -> &mut Self {
		if self.auto_repeat_mode.is_none() {
			self.x11_size += 4;
		}

		self.auto_repeat_mode = Some(auto_repeat_mode);
		self.mask |= KeyboardOptionsMask::AUTO_REPEAT_MODE;

		self
	}
}

impl KeyboardOptions {
	/// The volume of key clicks which is configured.
	///
	/// The volume is represented as a [percentage] from 0% to 100%.
	///
	/// [percentage]: Percentage
	#[must_use]
	pub fn key_click_volume(&self) -> Option<&Percentage> {
		self.key_click_volume
			.as_ref()
			.map(|__Percentage(percentage)| percentage)
	}

	/// The volume of the bell which is configured.
	///
	/// A bell generator connected to the console is treated as if it is part of
	/// the keyboard.
	///
	/// The volume is represented as a [percentage] from 0% to 100%.
	///
	/// [percentage]: Percentage
	#[must_use]
	pub fn bell_volume(&self) -> Option<&Percentage> {
		self.bell_volume
			.as_ref()
			.map(|__Percentage(percentage)| percentage)
	}
	/// The [pitch] of the bell which is configured.
	///
	/// The [pitch] is measured in hertz.
	///
	/// [pitch]: Pitch
	#[must_use]
	pub fn bell_pitch(&self) -> Option<&Pitch> {
		self.bell_pitch.as_ref().map(|__Pitch(pitch)| pitch)
	}
	/// The [duration] of the bell which is configured.
	///
	/// The [duration] is measured in milliseconds.
	///
	/// [duration]: Duration
	#[must_use]
	pub fn bell_duration(&self) -> Option<&Duration> {
		self.bell_duration
			.as_ref()
			.map(|__Duration(duration)| duration)
	}

	/// The [LED] that the [`led_mode`] applies to that is configured.
	///
	/// [LED]: Led
	/// [`led_mode`]: KeyboardOptions::led_mode
	#[must_use]
	pub fn led(&self) -> Option<&Led> {
		self.led.as_ref().map(|__Led(led)| led)
	}
	/// Whether the relevant keyboard [LED(s)][LED] are turned on or turned off.
	///
	/// If [`led`] is configured, this applies to that specific [LED]. If
	/// [`led`] is not configured, this applies to all [LEDs][LED].
	///
	/// [`led`]: KeyboardOptions::led
	/// [LED]: Led
	#[must_use]
	pub fn led_mode(&self) -> Option<&LedMode> {
		self.led_mode.as_ref().map(|__LedMode(mode)| mode)
	}

	/// The specific key which the [`auto_repeat_mode`] applies to.
	///
	/// [`auto_repeat_mode`]: KeyboardOptions::auto_repeat_mode
	#[must_use]
	pub fn auto_repeated_key(&self) -> Option<&Keycode> {
		self.auto_repeated_key.as_ref().map(|__Keycode(key)| key)
	}

	/// Whether the relevant keys have their auto repeats applied.
	///
	/// If [`auto_repeated_key`] is configured, this affects the auto repeat
	/// mode of that key in particular.
	///
	/// If no [`auto_repeated_key`] is configured, this affects the _global_
	/// auto repeat mode. If the global auto repeat mode is disabled, no keys
	/// have their auto repeat enabled. If it is enabled, keys which has auto
	/// repeat enabled are repeated.
	///
	/// [`auto_repeated_key`]: KeyboardOptions::auto_repeated_key
	#[must_use]
	pub fn auto_repeat_mode(&self) -> Option<&ToggleOrDefault> {
		self.auto_repeat_mode
			.as_ref()
			.map(|__ToggleOrDefault(toggle_or_default)| toggle_or_default)
	}
}

bitflags! {
	/// A mask of configured options for the keyboard.
	///
	/// This mask is used in the [`KeyboardOptions` set].
	///
	/// [`KeyboardOptions` set]: KeyboardOptions
	#[derive(Default, X11Size, Readable, ConstantX11Size, Writable)]
	pub struct KeyboardOptionsMask: u32 {
		/// Whether the [volume of key clicks] is configured.
		///
		/// See [`KeyboardOptions::key_click_volume`] for more information.
		///
		/// [volume of key clicks]: KeyboardOptions::key_click_volume
		const KEY_CLICK_VOLUME = 0x0000_0001;

		/// Whether the [volume of the bell] is configured.
		///
		/// See [`KeyboardOptions::bell_volume`] for more information.
		///
		/// [volume of the bell]: KeyboardOptions::bell_volume
		const BELL_VOLUME = 0x0000_0002;
		/// Whether the [pitch of the bell] is configured.
		///
		/// See [`KeyboardOptions::bell_pitch`] for more information.
		///
		/// [pitch of the bell]: KeyboardOptions::bell_pitch
		const BELL_PITCH = 0x0000_0004;
		/// Whether the [duration of the bell] is configured.
		///
		/// See [`KeyboardOptions::bell_duration`] for more information.
		///
		/// [duration of the bell]: KeyboardOptions::bell_duration
		const BELL_DURATION = 0x0000_0008;

		/// Whether the [LED] which the [`led_mode`] applies to is configured.
		///
		/// See [`KeyboardOptions::led`] for more information.
		///
		/// [LED]: Led
		/// [`led_mode`]: KeyboardOptions::led_mode
		const LED = 0x0000_0010;
		/// Whether the state of the relevant [LEDs] is configured.
		///
		/// See [`KeyboardOptions::led_mode`] for more information.
		///
		/// [LEDs]: Led
		const LED_MODE = 0x0000_0020;

		/// Whether the specific key which the [`auto_repeat_mode`] applies to
		/// is configured.
		///
		/// See [`KeyboardOptions::auto_repeat_mode`] for more information.
		///
		/// [`auto_repeat_mode`]: KeyboardOptions::auto_repeat_mode
		const AUTO_REPEATED_KEY = 0x0000_0040;

		/// Whether [`auto_repeat_mode`] is configured for the relevant keys.
		///
		/// See [`KeyboardOptions::auto_repeat_mode`] for more information.
		///
		/// [`auto_repeat_mode`]: KeyboardOptions::auto_repeat_mode
		const AUTO_REPEAT_MODE = 0x0000_0080;
	}
}

impl X11Size for KeyboardOptions {
	fn x11_size(&self) -> usize {
		self.x11_size
	}
}

impl Readable for KeyboardOptions {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		let mask = KeyboardOptionsMask::read_from(buf)?;
		let mut x11_size = mask.x11_size();

		let key_click_volume = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(KeyboardOptionsMask::KEY_CLICK_VOLUME),
		)?;

		let bell_volume = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(KeyboardOptionsMask::BELL_VOLUME),
		)?;
		let bell_pitch = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(KeyboardOptionsMask::BELL_PITCH),
		)?;
		let bell_duration = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(KeyboardOptionsMask::BELL_DURATION),
		)?;

		let led =
			super::read_set_value(buf, &mut x11_size, mask.contains(KeyboardOptionsMask::LED))?;
		let led_mode = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(KeyboardOptionsMask::LED_MODE),
		)?;

		let auto_repeated_key = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(KeyboardOptionsMask::AUTO_REPEATED_KEY),
		)?;

		let auto_repeat_mode = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(KeyboardOptionsMask::AUTO_REPEAT_MODE),
		)?;

		Ok(Self {
			x11_size,
			mask,

			key_click_volume,

			bell_volume,
			bell_pitch,
			bell_duration,

			led,
			led_mode,

			auto_repeated_key,

			auto_repeat_mode,
		})
	}
}

impl Writable for KeyboardOptions {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		self.mask.write_to(buf)?;

		if let Some(key_click_volume) = &self.key_click_volume {
			key_click_volume.write_to(buf)?;
		}

		if let Some(bell_volume) = &self.bell_volume {
			bell_volume.write_to(buf)?;
		}
		if let Some(bell_pitch) = &self.bell_pitch {
			bell_pitch.write_to(buf)?;
		}
		if let Some(bell_duration) = &self.bell_duration {
			bell_duration.write_to(buf)?;
		}

		if let Some(led) = &self.led {
			led.write_to(buf)?;
		}
		if let Some(led_mode) = &self.led_mode {
			led_mode.write_to(buf)?;
		}

		if let Some(auto_repeated_key) = &self.auto_repeated_key {
			auto_repeated_key.write_to(buf)?;
		}

		if let Some(auto_repeat_mode) = &self.auto_repeat_mode {
			auto_repeat_mode.write_to(buf)?;
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __Percentage(Percentage);

impl ConstantX11Size for __Percentage {
	const X11_SIZE: usize = 4;
}

impl X11Size for __Percentage {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __Percentage {
	#[allow(
		clippy::cast_possible_truncation,
		reason = "truncation is intended behavior"
	)]
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_i32() {
			reset if reset == -1 => Percentage::Reset,

			other => match u8::try_from(other) {
				Ok(percent) if (0..=100).contains(&percent) => Percentage::Percent(percent),
				_ => return Err(ReadError::Other(Box::new(PercentOutOfBounds))),
			},
		}))
	}
}

impl Writable for __Percentage {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(percentage) = self;

		match percentage {
			Percentage::Reset => buf.put_i32(-1),
			Percentage::Percent(percent) => buf.put_i32(i32::from(*percent)),
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __Pitch(Pitch);

impl ConstantX11Size for __Pitch {
	const X11_SIZE: usize = 4;
}

impl X11Size for __Pitch {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __Pitch {
	#[allow(
		clippy::cast_possible_truncation,
		reason = "truncation is intended behavior"
	)]
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_i32() {
			reset if reset == -1 => Pitch::Reset,

			other => match u8::try_from(other) {
				Ok(pitch) => Pitch::Hz(pitch),
				_ => return Err(ReadError::Other(Box::new(NegativeValue(other as i16)))),
			},
		}))
	}
}

impl Writable for __Pitch {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(pitch) = self;

		match pitch {
			Pitch::Reset => buf.put_i32(-1),
			Pitch::Hz(pitch) => buf.put_i32(i32::from(*pitch)),
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __Duration(Duration);

impl ConstantX11Size for __Duration {
	const X11_SIZE: usize = 4;
}

impl X11Size for __Duration {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __Duration {
	#[allow(
		clippy::cast_possible_truncation,
		reason = "truncation is intended behavior"
	)]
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_i32() {
			reset if reset == -1 => Duration::Reset,

			other => match u8::try_from(other) {
				Ok(duration) => Duration::Ms(duration),
				_ => return Err(ReadError::Other(Box::new(NegativeValue(other as i16)))),
			},
		}))
	}
}

impl Writable for __Duration {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(duration) = self;

		match duration {
			Duration::Reset => buf.put_i32(-1),
			Duration::Ms(duration) => buf.put_i32(i32::from(*duration)),
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __Led(Led);

impl ConstantX11Size for __Led {
	const X11_SIZE: usize = 4;
}

impl X11Size for __Led {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __Led {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match u8::try_from(buf.get_u32()) {
			Ok(zero) if zero == 0 => return Err(ReadError::Other(Box::new(LedError::Zero))),
			Ok(high) if high > 32 => {
				return Err(ReadError::Other(Box::new(LedError::TooHigh(high))))
			},

			Ok(led) => Led(led),

			Err(error) => return Err(ReadError::FailedConversion(Box::new(error))),
		}))
	}
}

impl Writable for __Led {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(Led(u8)) = self;

		u32::from(*u8).write_to(buf)?;

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __LedMode(LedMode);

impl ConstantX11Size for __LedMode {
	const X11_SIZE: usize = 4;
}

impl X11Size for __LedMode {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __LedMode {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_u32() {
			off if off == 0 => LedMode::Off,
			on if on == 1 => LedMode::On,

			other_discrim => return Err(UnrecognizedDiscriminant(other_discrim as usize)),
		}))
	}
}

impl Writable for __LedMode {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(led_mode) = self;

		match led_mode {
			LedMode::Off => buf.put_u32(0),
			LedMode::On => buf.put_u32(1),
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __Keycode(Keycode);

impl ConstantX11Size for __Keycode {
	const X11_SIZE: usize = 4;
}

impl X11Size for __Keycode {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __Keycode {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(Keycode::new(
			buf.get_u32()
				.try_into()
				.expect("must fit into u8; represents u8 value"),
		)))
	}
}

impl Writable for __Keycode {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(Keycode(keycode)) = self;

		buf.put_u32((*keycode).into());

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __ToggleOrDefault(ToggleOrDefault);

impl ConstantX11Size for __ToggleOrDefault {
	const X11_SIZE: usize = 4;
}

impl X11Size for __ToggleOrDefault {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __ToggleOrDefault {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_u32() {
			discrim if discrim == 0 => ToggleOrDefault::Disabled,
			discrim if discrim == 1 => ToggleOrDefault::Enabled,

			discrim if discrim == 2 => ToggleOrDefault::Default,

			other_discrim => return Err(UnrecognizedDiscriminant(other_discrim as usize)),
		}))
	}
}

impl Writable for __ToggleOrDefault {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(toggle_or_default) = self;

		match toggle_or_default {
			ToggleOrDefault::Disabled => buf.put_u32(0),
			ToggleOrDefault::Enabled => buf.put_u32(1),

			ToggleOrDefault::Default => buf.put_u32(2),
		}

		Ok(())
	}
}
