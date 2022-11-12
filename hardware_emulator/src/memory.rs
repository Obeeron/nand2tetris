use crate::{Result, Error};

pub struct Memory (pub Box<[u16; Memory::TOTAL_SIZE]>);  // Data memory and Screen memory map

impl Memory {
    pub const TOTAL_SIZE: usize = 0x6001;
    pub const SCREEN_MEMORY_MAP_OFFSET: usize = 0x4000;
    pub const SCREEN_MEMORY_MAP_SIZE: usize = 0x2000;
    pub const KEYBOARD_REG_OFFSET: usize = 0x6000;

    pub fn new() -> Self {
        Self(Box::new([0; Self::TOTAL_SIZE]))
    }

    pub fn fetch(&self, address: usize) -> Result<u16> {
        if address >= Self::TOTAL_SIZE {
            return Err(Error::InvalidMemoryAddress(address));
        }
        Ok(self.0[address])
    }

    pub fn store(&mut self, address: usize, value: u16) -> Result<()> {
        if address >= Self::TOTAL_SIZE {
            return Err(Error::InvalidMemoryAddress(address));
        }
        self.0[address] = value;

        Ok(())
    }
   
    pub fn set_keyboard_reg(&mut self, keycode: u16) {
        self.0[Self::KEYBOARD_REG_OFFSET] = keycode;
    }

    pub fn is_screen_address(address: usize) -> bool {
        address >= Self::SCREEN_MEMORY_MAP_OFFSET && address < (Self::SCREEN_MEMORY_MAP_OFFSET + Self::SCREEN_MEMORY_MAP_SIZE)
    }
}