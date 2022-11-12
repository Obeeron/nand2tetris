use pixels::Pixels;
use crate::{Result, Memory};

pub struct Screen {
    pub pixels: Pixels,
}

impl Screen {
    pub fn new(pixels: Pixels) -> Self {
        Self {
            pixels,
        }
    }

    pub fn render(&self) -> Result<()> {
        self.pixels.render()?;
        Ok(())
    }

    pub fn write(&mut self, address: usize, value: u16) -> Result<()> {
        let frame = self.pixels.get_frame_mut();

        let frame_lower_bound = (address as usize - Memory::SCREEN_MEMORY_MAP_OFFSET) * 16 *4;
        for i in 0..16 {
            let color = if value & (1 << i) != 0 { 0xFF } else { 0x00 };
            let pixel = &mut frame[frame_lower_bound + i*4..frame_lower_bound + i*4 + 4];
            pixel[0] = color;
            pixel[1] = color;
            pixel[2] = color;
            pixel[3] = 0xFF;
        }
        Ok(())
    }
}