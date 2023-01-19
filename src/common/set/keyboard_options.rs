// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{Keycode, Toggle, ToggleOrDefault};
use xrbk::{
	Buf,
	BufMut,
	ConstantX11Size,
	ReadError::UnrecognizedDiscriminant,
	ReadResult,
	Readable,
	Writable,
	WriteResult,
	X11Size,
};
use xrbk_macro::{ConstantX11Size, Readable, Writable, X11Size};

use bitflags::bitflags;

bitflags! {
	#[derive(Default, X11Size, Readable, ConstantX11Size, Writable)]
	pub struct KeyboardOptionMask: u32 {
		const KEY_CLICK_PERCENT = 0x0000_0001;

		const BELL_PERCENT = 0x0000_0002;
		const BELL_PITCH = 0x0000_0004;
		const BELL_DURATION = 0x0000_0008;

		const LED = 0x0000_0010;
		const LED_MODE = 0x0000_0020;

		const KEY = 0x0000_0040;

		const AUTO_REPEAT_MODE = 0x0000_0080;
	}
}

pub struct KeyboardOptions {
	x11_size: usize,

	mask: KeyboardOptionMask,

	// Represents an `i8` value.
	key_click_percent: Option<i32>,

	// Represents an `i8` value.
	bell_percent: Option<i32>,
	// Represents an `i16` value.
	bell_pitch: Option<i32>,
	// Represents an `i16` value.
	bell_duration: Option<i32>,

	// Represents a `u8` value.
	led: Option<u32>,
	led_mode: Option<__Toggle>,

	key: Option<__Keycode>,

	auto_repeat_mode: Option<__ToggleOrDefault>,
}

impl KeyboardOptions {
	pub fn new(
		key_click_percent: Option<i8>, bell_percent: Option<i8>, bell_pitch: Option<i16>,
		bell_duration: Option<i16>, led: Option<u8>, led_mode: Option<Toggle>,
		key: Option<Keycode>, auto_repeat_mode: Option<ToggleOrDefault>,
	) -> Self {
		let mut x11_size = 0;
		let mut mask = KeyboardOptionMask::empty();

		if key_click_percent.is_some() {
			x11_size += i32::X11_SIZE;
			mask |= KeyboardOptionMask::KEY_CLICK_PERCENT;
		}

		if bell_percent.is_some() {
			x11_size += i32::X11_SIZE;
			mask |= KeyboardOptionMask::BELL_PERCENT;
		}
		if bell_pitch.is_some() {
			x11_size += i32::X11_SIZE;
			mask |= KeyboardOptionMask::BELL_PITCH;
		}
		if bell_duration.is_some() {
			x11_size += i32::X11_SIZE;
			mask |= KeyboardOptionMask::BELL_DURATION;
		}

		if led.is_some() {
			x11_size += u32::X11_SIZE;
			mask |= KeyboardOptionMask::LED;
		}
		if led_mode.is_some() {
			x11_size += __Toggle::X11_SIZE;
			mask |= KeyboardOptionMask::LED_MODE;
		}

		if key.is_some() {
			x11_size += __Keycode::X11_SIZE;
			mask |= KeyboardOptionMask::KEY;
		}

		if auto_repeat_mode.is_some() {
			x11_size += __ToggleOrDefault::X11_SIZE;
			mask |= KeyboardOptionMask::AUTO_REPEAT_MODE;
		}

		Self {
			x11_size,

			mask,

			key_click_percent: key_click_percent.map(std::convert::Into::into),

			bell_percent: bell_percent.map(std::convert::Into::into),
			bell_pitch: bell_pitch.map(std::convert::Into::into),
			bell_duration: bell_duration.map(std::convert::Into::into),

			led: led.map(std::convert::Into::into),
			led_mode: led_mode.map(__Toggle),

			key: key.map(__Keycode),

			auto_repeat_mode: auto_repeat_mode.map(__ToggleOrDefault),
		}
	}

	pub fn key_click_percent(&self) -> Option<i8> {
		self.key_click_percent.map(|key_click_percent| {
			key_click_percent
				.try_into()
				.expect("must fit into i8; represents an i8 value")
		})
	}

	pub fn bell_percent(&self) -> Option<i8> {
		self.bell_percent.map(|bell_percent| {
			bell_percent
				.try_into()
				.expect("must fit into i8; represents an i8 value")
		})
	}
	pub fn bell_pitch(&self) -> Option<i16> {
		self.bell_pitch.map(|bell_pitch| {
			bell_pitch
				.try_into()
				.expect("must fit into i16; represents an i16 value")
		})
	}
	pub fn bell_duration(&self) -> Option<i16> {
		self.bell_duration.map(|bell_duration| {
			bell_duration
				.try_into()
				.expect("must fit into i16; represents an i16 value")
		})
	}

	pub fn led(&self) -> Option<u8> {
		self.led.map(|led| {
			led.try_into()
				.expect("must fit into u8; represents a u8 value")
		})
	}
	pub fn led_mode(&self) -> Option<&Toggle> {
		self.led_mode.as_ref().map(|__Toggle(toggle)| toggle)
	}

	pub fn key(&self) -> Option<&Keycode> {
		self.key.as_ref().map(|__Keycode(keycode)| keycode)
	}

	pub fn auto_repeat_mode(&self) -> Option<&ToggleOrDefault> {
		self.auto_repeat_mode
			.as_ref()
			.map(|__ToggleOrDefault(toggle_or_default)| toggle_or_default)
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
		let mut x11_size = 0;
		let mask = KeyboardOptionMask::read_from(buf)?;

		let key_click_percent = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(KeyboardOptionMask::KEY_CLICK_PERCENT),
		)?;

		let bell_percent = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(KeyboardOptionMask::BELL_PERCENT),
		)?;
		let bell_pitch = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(KeyboardOptionMask::BELL_PITCH),
		)?;
		let bell_duration = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(KeyboardOptionMask::BELL_DURATION),
		)?;

		let led =
			super::read_set_value(buf, &mut x11_size, mask.contains(KeyboardOptionMask::LED))?;
		let led_mode = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(KeyboardOptionMask::LED_MODE),
		)?;

		let key =
			super::read_set_value(buf, &mut x11_size, mask.contains(KeyboardOptionMask::KEY))?;

		let auto_repeat_mode = super::read_set_value(
			buf,
			&mut x11_size,
			mask.contains(KeyboardOptionMask::AUTO_REPEAT_MODE),
		)?;

		Ok(Self {
			x11_size,
			mask,

			key_click_percent,

			bell_percent,
			bell_pitch,
			bell_duration,

			led,
			led_mode,

			key,

			auto_repeat_mode,
		})
	}
}

impl Writable for KeyboardOptions {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		self.mask.write_to(buf)?;

		if let Some(key_click_percent) = &self.key_click_percent {
			key_click_percent.write_to(buf)?;
		}

		if let Some(bell_percent) = &self.bell_percent {
			bell_percent.write_to(buf)?;
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

		if let Some(key) = &self.key {
			key.write_to(buf)?;
		}

		if let Some(auto_repeat_mode) = &self.auto_repeat_mode {
			auto_repeat_mode.write_to(buf)?;
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct __Toggle(Toggle);

impl ConstantX11Size for __Toggle {
	const X11_SIZE: usize = 4;
}

impl X11Size for __Toggle {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for __Toggle {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		Ok(Self(match buf.get_u32() {
			discrim if discrim == 0 => Toggle::Disabled,
			discrim if discrim == 1 => Toggle::Enabled,

			other_discrim => return Err(UnrecognizedDiscriminant(other_discrim as usize)),
		}))
	}
}

impl Writable for __Toggle {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		let Self(toggle) = self;

		match toggle {
			Toggle::Disabled => buf.put_u32(0),
			Toggle::Enabled => buf.put_u32(1),
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
