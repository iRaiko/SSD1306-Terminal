use std::fs::{File, OpenOptions};
use std::os::raw::{c_int, c_ulong, c_void};
use std::os::unix::io::{RawFd, AsRawFd};
use std::io::Result;

/// I2C struct with basic functions and initialization for the ssd1306 screen
pub struct I2c
{
    file: File,
    slave_adress: u8,
}

impl I2c
{
    /// Create a new I2C instance from the dev file and slave adress
    pub fn new(file: &str, slave_adress: u8) -> Result<I2c>
    {
        let file = OpenOptions::new().write(true).read(true).open(&file)?;

        // Linux needs a raw FD for ioctl call
        let rawfd = file.as_raw_fd();

        if unsafe { ioctl(rawfd, 0x0703, slave_adress as *mut c_void) } < 0
        {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error setting the file as i2c file"))
        }

        Ok( I2c { file, slave_adress })
    }

    /// Write data to the device file
    pub fn write_data(&mut self, data: &[u8]) -> Result<usize>
    {
        let mut buffer = vec![0x40];
        buffer.extend_from_slice(data);
        self.write(&buffer)
    }

    /// Write a command to the device file
    pub fn write_command(&mut self, command: u8) -> Result<usize>
    {
        self.write(&[0x00, command])
    }

    /// Write to the device file
    fn write(&mut self, data: &[u8]) -> Result<usize>
    {
        self.write(data)
    }
}

extern "C"
{
    fn ioctl(fd: RawFd, request: c_ulong, v: *mut c_void) -> c_int;
}


