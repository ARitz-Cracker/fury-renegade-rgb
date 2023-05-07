use std::{time::Duration, thread};

use i2cdev::{linux::LinuxI2CDevice, core::I2CDevice};

pub(crate) trait HeadBangingI2CDevice {
	fn force_smbus_read_byte_data(&mut self, register: u8) -> std::result::Result<u8, std::io::Error>;
	fn force_smbus_write_byte_data(&mut self, register: u8, value: u8) -> std::result::Result<(), std::io::Error>;
}
impl HeadBangingI2CDevice for LinuxI2CDevice {
	fn force_smbus_write_byte_data(&mut self, register: u8, value: u8) -> std::result::Result<(), std::io::Error> {
		let mut retry_count = 1;
		loop {
			match self.smbus_write_byte_data(register, value) {
				Ok(_) => {
					return Ok(());
				},
				Err(err) => {
					let io_err: std::io::Error = err.into();
					if io_err.raw_os_error() == Some(6) {
						// ENXIO
						thread::sleep(Duration::from_millis(retry_count));
						retry_count <<= 1;
						// println!("retry write {} to {}", value, register);
						if retry_count > 10000 {
							return Err(io_err);
						}
						continue;
					}
					return Err(io_err);
				},
			}
		}
	}
	fn force_smbus_read_byte_data(&mut self, register: u8) -> std::result::Result<u8, std::io::Error> {
		let mut retry_count = 1;
		loop {
			match self.smbus_read_byte_data(register) {
				Ok(val) => {
					return Ok(val)
				},
				Err(err) => {
					let io_err: std::io::Error = err.into();
					if io_err.raw_os_error() == Some(6) {
						// ENXIO
						thread::sleep(Duration::from_millis(retry_count));
						retry_count <<= 1;
						if retry_count > 10000 {
							return Err(io_err);
						}
						continue;
					}
					// println!("{:#?}", io_err.kind());
					return Err(io_err);
				},
			}
		}
	}
}
