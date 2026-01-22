use std::{path::Path, str::FromStr};

use color_eyre::eyre::Result;
use i2cdev::linux::LinuxI2CDevice;

use crate::{error::FuryControllerError, headbang::HeadBangingI2CDevice, types::Colour};

pub(crate) struct MultiRamController {
	bus: LinuxI2CDevice,
	sticks: Vec<u16>,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub(crate) enum PatternStyle {
	/// Always shows custom colour
	Solid = 0x00,
	/// The unicorn barf we all know and love, no custimization options have any effect
	Rainbow = 0x01,
	/// A dot scans from top to bottom, then settles on the center
	Scan = 0x02,
	/// Fades in and out
	Breathe = 0x03,
	/// Only fades in, most interesting with the colour cycle
	Fade = 0x04,
	/// Wipes from the bottom to top
	Stripe = 0x05,
	/// Trailing light, bottom to top
	Trail = 0x06,
	/// Electrical pattern, not unlike a plasma ball. Looks best with 4 sticks
	Lightning = 0x07,
	/// Counts down from 9 to 0 repeatedly. Looks best with 4 sticks
	Countdown = 0x08,
	/// Fire pattern, no custimization options have any effect, looks best with 4 sticks
	Fire = 0x09,
	/// Sprikles random colours around the ram, non-customizable, looks best with 4 sticks
	Sparkles = 0x0a,
	/// Writes "F" on the sticks, then "U", then "R", then "Y". Looks best with 4 sticks.
	Fury = 0x0b,
}
impl FromStr for PatternStyle {
	type Err = FuryControllerError;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		match s {
			"solid" => Ok(Self::Solid),
			"rainbow" => Ok(Self::Rainbow),
			"scan" => Ok(Self::Scan),
			"breathe" => Ok(Self::Breathe),
			"fade" => Ok(Self::Fade),
			"stripe" => Ok(Self::Stripe),
			"trail" => Ok(Self::Trail),
			"lightning" => Ok(Self::Lightning),
			"countdown" => Ok(Self::Countdown),
			"fire" => Ok(Self::Fire),
			"sparkles" => Ok(Self::Sparkles),
			"fury" => Ok(Self::Fury),

			_ => Err(FuryControllerError::UnknownPatternStyle(s.to_string())),
		}
	}
}
impl MultiRamController {
	pub fn new<P: AsRef<Path>>(path: P, sticks: Vec<u16>) -> Result<Self> {
		Ok(MultiRamController {
			bus: LinuxI2CDevice::new(path, sticks[0])?,
			sticks,
		})
	}
	#[inline]
	fn write_to_all(&mut self, register: u8, value: u8) -> Result<()> {
		for address in self.sticks.iter() {
			self.bus.set_slave_address(*address)?;
			self.bus.force_smbus_write_byte_data(register, value)?;
		}
		Ok(())
	}
	#[inline]
	fn start_command(&mut self) -> Result<()> {
		self.write_to_all(0x08, 0x53)?;
		Ok(())
	}
	#[inline]
	fn end_command(&mut self) -> Result<()> {
		self.write_to_all(0x08, 0x44)?;
		Ok(())
	}
	pub fn noop(&mut self) -> Result<()> {
		for address in self.sticks.iter() {
			self.bus.set_slave_address(*address)?;
			self.bus.force_smbus_write_byte_data(0x08, 0x53)?;
			// no idea why these are read, they seemingly always return 0x5c
			core::hint::black_box(self.bus.force_smbus_read_byte_data(0x05)?);
			core::hint::black_box(self.bus.force_smbus_read_byte_data(0x06)?);
			core::hint::black_box(self.bus.force_smbus_read_byte_data(0x26)?);
			self.bus.force_smbus_write_byte_data(0x08, 0x44)?;
		}
		Ok(())
	}
	pub fn sync_timings(&mut self) -> Result<()> {
		self.start_command()?;
		for (i, address) in self.sticks.iter().enumerate() {
			let sync_offset = self.sticks.len() as u8 - i as u8 - 1;
			self.bus.set_slave_address(*address)?;
			self.bus.force_smbus_write_byte_data(0x0b, sync_offset)?;
		}
		self.end_command()?;
		Ok(())
	}
	/// Default value is 100, 100, 100
	pub fn set_rgb_brightness_percent(
		&mut self,
		r_brightness_percent: u8,
		g_brightness_percent: u8,
		b_brightness_percent: u8,
	) -> Result<()> {
		self.start_command()?;
		self.write_to_all(0x2d, r_brightness_percent)?;
		self.write_to_all(0x2e, g_brightness_percent)?;
		self.write_to_all(0x2f, b_brightness_percent)?;
		self.end_command()?;
		Ok(())
	}
	/// Default value is 100
	pub fn set_brightness_percent(&mut self, brightness_percent: u8) -> Result<()> {
		self.start_command()?;
		self.write_to_all(0x20, brightness_percent)?;
		self.end_command()?;
		Ok(())
	}
	/// Default value is 0
	pub fn set_pattern_start_offset(&mut self, raw_offset: u8) -> Result<()> {
		self.start_command()?;
		self.write_to_all(0x0d, raw_offset)?;
		self.end_command()?;
		Ok(())
	}
	/// Default value is 1, all should be set to the same value
	pub fn set_pattern_repeat_delay(&mut self, raw_delay: u8) -> Result<()> {
		self.start_command()?;
		self.write_to_all(0x27, raw_delay)?;
		self.end_command()?;
		Ok(())
	}
	pub fn set_pattern(&mut self, pattern: PatternStyle, colorus: &[Colour]) -> Result<()> {
		if colorus.len() > 0x0b {
			return Err(color_eyre::eyre::eyre!(
				"Cannot set more than 11 colours, got {}",
				colorus.len()
			));
		}
		self.start_command()?;
		self.write_to_all(0x09, pattern as u8)?;
		self.write_to_all(0x30, colorus.len() as u8)?;
		for (i, colour) in colorus.iter().enumerate() {
			self.write_to_all(0x31 + (i as u8 * 3), colour.red)?;
			self.write_to_all(0x32 + (i as u8 * 3), colour.green)?;
			self.write_to_all(0x33 + (i as u8 * 3), colour.blue)?;
		}
		self.end_command()?;
		Ok(())
	}
	pub fn reset(&mut self) -> Result<()> {
		self.noop()?;
		self.noop()?;
		self.sync_timings()?;
		self.set_brightness_percent(100)?;
		self.set_rgb_brightness_percent(100, 100, 100)?;
		self.set_pattern_start_offset(0)?;
		self.set_pattern_repeat_delay(1)?;
		self.set_pattern(PatternStyle::Rainbow, &[Colour::default()])?;
		self.sync_timings()?;
		Ok(())
	}
}
