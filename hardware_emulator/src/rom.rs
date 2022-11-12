use std::fs;

use crate::{Result, Error};

pub struct Rom (Box<[u16]>); // ROM memory

impl Rom {
    const ROM_SIZE: usize = 32768;

    pub fn from_file(path : &String) -> Result<Self> {
        let bytes = fs::read(path)?;
        if bytes.len() > Self::ROM_SIZE*2 {
            return Err(Error::RomError("ROM too large".to_string()));
        }
        if bytes.len() % 2 != 0 {
            return Err(Error::RomError("ROM size must be a multiple of 2".to_string()));
        }
        let mut data = Box::new([0; 32768]);
        for i in 0..bytes.len() / 2 {
            data[i] = u16::from_be_bytes([bytes[i * 2], bytes[i * 2 + 1]]);
        }
        Ok(Rom(data))
    }

    pub fn fetch(&self, address: usize) -> Result<u16> {
        if address >= Self::ROM_SIZE {
            return Err(Error::InvalidROMAddress(address));
        }
        Ok(self.0[address])
    }
}