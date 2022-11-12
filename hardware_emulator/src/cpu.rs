use crate::{Result, Memory, Error};

pub struct Cpu {
    pub reg_d: u16, // Data register
    pub reg_a: u16, // Address register
    pub pc: usize,
}

pub struct CpuOutput {
    pub write_to_ram: bool,
    pub alu_out: u16,
    pub memory_address: usize,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            reg_d: 0,
            reg_a: 0,
            pc: 0,
        }
    }

    pub fn increment_pc(&mut self) {
        self.pc += 1;
    }

    pub fn reset_pc(&mut self) {
        self.pc = 0;
    }

    pub fn execute(&mut self, instruction: u16, memory: &Memory) -> Result<CpuOutput> {
        let first_bit = instruction >> 15;
        
        if first_bit == 0 {
            self.execute_a_instruction(instruction)
        } else {
            self.execute_c_instruction(instruction, memory)
        }
    }

    fn execute_a_instruction(&mut self, instruction: u16) -> Result<CpuOutput> {
        let address = instruction & 0x7FFF;
        self.reg_a = address;
        self.increment_pc();
        Ok(CpuOutput {
            write_to_ram: false,
            alu_out: 0,
            memory_address: 0,
        })
    }

    fn execute_c_instruction(&mut self, instruction: u16, memory: &Memory) -> Result<CpuOutput> {
        if instruction >> 13 != 0b111 {
            return Err(Error::InvalidCInstructionPadding(self.pc));
        }

        let a = (instruction >> 12) & 0b1;
        let comp = (instruction >> 6) & 0b111111;
        let dest = (instruction >> 3) & 0b111;
        let jump = instruction & 0b111;

        let a_value = if a == 0 {
            self.reg_a
        } else {
            memory.fetch(self.reg_a as usize)?
        };

        let alu_out = self.alu_compute(comp, a_value)?;
        let memory_address = self.reg_a as usize;

        let write_to_ram = dest & 0b001 != 0;
        let write_to_d = dest & 0b010 != 0;
        let write_to_a = dest & 0b100 != 0;


        if write_to_d {
            self.reg_d = alu_out;
        }
        if write_to_a {
            self.reg_a = alu_out;
        }
        
        let ng = (alu_out >> 15) & 0b1 != 0;
        let zr = alu_out == 0;

        if (jump & 0b001 != 0 && !ng && !zr) || (jump & 0b010 != 0 && zr) || (jump & 0b100 != 0 && ng) {
            self.pc = self.reg_a as usize;
        } else {
            self.increment_pc();
        }

        Ok(CpuOutput {
            write_to_ram,
            alu_out,
            memory_address,
        })
    }

    fn alu_compute(&self, comp: u16, a_value: u16) -> Result<u16> {
        Ok(match comp {
            0b0101010 => 0,
            0b0111111 => 1,
            0b0111010 => 0xFFFF,
            0b0001100 => self.reg_d,
            0b0110000 => a_value,
            0b0001101 => !self.reg_d,
            0b0110001 => !a_value,
            0b0001111 => !(self.reg_d.wrapping_add(0xFFFF)),        // -REG_D
            0b0110011 => !(a_value.wrapping_add(0xFFFF)),           // -REG_A/M
            0b0011111 => self.reg_d.wrapping_add(1),                // REG_D + 1
            0b0110111 => a_value.wrapping_add(1),                   // REG_A/M + 1
            0b0001110 => self.reg_d.wrapping_add(0xFFFF),           // REG_D - 1
            0b0110010 => a_value.wrapping_add(0xFFFF),              // REG_A/M - 1
            0b0000010 => self.reg_d.wrapping_add(a_value),          // REG_D + REG_A/M
            0b0010011 => !((!self.reg_d).wrapping_add(a_value)),    // REG_D - REG_A/M
            0b0000111 => !(self.reg_d.wrapping_add(!a_value)),      // REG_A/M - REG_D
            0b0000000 => self.reg_d & a_value,
            0b0010101 => self.reg_d | a_value,
            _ => return Err(Error::InvalidCInstructionComp(self.pc-1)),
        })
    }

    
}