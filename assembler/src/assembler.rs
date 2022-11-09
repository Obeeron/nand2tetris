use std::{io::{BufReader, Lines, self, BufRead, Write, BufWriter}, fs::File, path::Path};

use crate::error::{Error, Result};
use crate::{symbol_table::SymbolTable, OutputFormat};

pub struct Assembler {
    st: SymbolTable,
    output_format: OutputFormat,
    save_preprocessed_file: bool,
    line_num: u16,
}

impl Assembler {
    pub fn new(output_format: OutputFormat, save_preprocessed_file: bool) -> Self {
        Self {
            st: SymbolTable::new(),
            output_format,
            save_preprocessed_file,
            line_num: 1,
        }
    }
    
    pub fn run(&mut self, input_file : String, output_name : Option<String>) -> Result<()> {
        let file = File::open(&input_file)?;
        let lines = io::BufReader::new(file).lines();
        let file_name_no_ext = Path::new(&input_file).file_stem().unwrap().to_str().unwrap();

        let mut preprocessed_lines = self.preprocess(lines)?;
        if self.save_preprocessed_file {
            let pp_file_name = format!("{}.i", file_name_no_ext);
            self.write_preprocessed_file(pp_file_name, &preprocessed_lines)?;
        }
        
        let out_file_name = if let Some(output_path) = output_name {
            output_path
        } else {
            format!("{}.hack", file_name_no_ext)
        };

        let mut out_buf = BufWriter::<File>::new(File::create(out_file_name)?);

        self.assemble(&mut preprocessed_lines, &mut out_buf)
    }

    fn write_preprocessed_file(&self, file_name: String, lines: &Vec<String>) -> Result<()> {
        let mut file = File::create(file_name)?;
        for line in lines {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }

    fn preprocess(&mut self, lines: Lines<BufReader<File>>) -> Result<Vec<String>> {
        let mut preprocessed_lines = Vec::new();
        let mut line_number = 0 as u16;

        for line in lines {
            let pp_line = self.preprocess_line(&line?, &line_number)?;
            if let Some(asm_line) = pp_line {
                preprocessed_lines.push(asm_line);
                line_number += 1;
            }
        }

        Ok(preprocessed_lines)
    }

    fn preprocess_line(&mut self, line: &String, line_number: & u16) -> Result<Option<String>> {
        // Preprocessed line
        let mut pp_line = String::new();

        // Remove any white spaces including tabs and \r in the string
        for c in line.chars() {
            if !c.is_whitespace(){
                pp_line.push(c);
            } 
        }

        // Remove comment
        if let Some(comment_index) = pp_line.find("//") {
            pp_line = pp_line [..comment_index].to_string();
        }

        // Remove empty lines
        if pp_line.is_empty() {
            return Ok(None);
        }

        // Add labels to the symbol table
        if pp_line.starts_with("("){
            if !pp_line.ends_with(")") {
                return Err(Error::PreprocessError(format!("Missing label closing parenthesis at line {}", line_number)));
            }
            let label = pp_line[1..pp_line.len()-1].to_string();
            self.st.add_label(&label, *line_number);
            return Ok(None);
        }
        
        Ok(Some(pp_line))
    }

    pub fn assemble(&mut self, lines: &mut Vec<String>, out_buf: &mut BufWriter<File>) -> Result<()> {
        
        for line in lines {
            let instr = self.assemble_line(line)?;
            self.save_instr(instr, out_buf)?;
            self.line_num+=1;
        }
        Ok(())
    }

    fn save_instr(&self, instr: u16, out_buf: &mut BufWriter<File>) -> Result<()> {
        match self.output_format {
            OutputFormat::Text => {
                let instr_str = format!("{:0>16b}", instr);
                writeln!(out_buf, "{}", instr_str)?;
            },
            OutputFormat::Binary => {
                let instr_bytes = instr.to_be_bytes();
                out_buf.write(&instr_bytes)?;
            }
        }
        Ok(())
    }


    fn assemble_line(&mut self, line: &String) -> Result<u16> {
        if line.starts_with("@") {
            Ok(self.assemble_a_instr(line)?)
        } else {
            Ok(self.assemble_c_instr(line)?)
        }
    }

    fn assemble_a_instr(&mut self, line: &String) -> Result<u16> {
        let mut is_symbol = false;
        
        let mut ram_addr = 0 as u16;
        let mut symbol = String::new();

        if line.len() == 1 {
            return Err(Error::AssembleError(format!("Invalid empty symbol at line {}", self.line_num)));
        }

        // Get the value or symbol
        for c in line.chars().skip(1) {
            if !is_symbol && c.is_digit(10) {
                ram_addr = ram_addr * 10 + c.to_digit(10).unwrap() as u16;
            } else {
                is_symbol = true;
                symbol.push(c);
            }
        }

        // If the address is hardcoded
        if is_symbol {
            // If a symbol is used, get the symbol address from the symbol table
            ram_addr = match self.st.get_symbol_addr(&symbol) {
                Some(addr) => addr,
                None => {
                    // If the symbol is not in the symbol table, add it to the table as a variable
                    self.st.add_variable(&symbol)?
                }
            }
        }

        // Create and return the instruction from the RAM address
        Ok(ram_addr)
    }

    fn assemble_c_instr(&mut self, line: &str) -> Result<u16> {
        let mut cursor_idx = 0;
        
        let dest_bits = self.get_dest_bits(&mut cursor_idx, line)?;
        let comp_bits = self.get_comp_bits(&mut cursor_idx, &line)?;
        let jump_bits = self.get_jump_bits(cursor_idx, &line)?;

        Ok(0b111 << 13 | comp_bits << 6 | dest_bits << 3 | jump_bits)
    }

    fn get_dest_bits(&self, cursor_idx: &mut usize, line: &str) -> Result<u16> {
        let egal_pos = match line.find("=") {
            Some(pos) => pos,
            None => return Ok(0b000)
        };
        
        let dest = &line[*cursor_idx..egal_pos];
        *cursor_idx = egal_pos + 1;

        Ok(match dest {
            "M"   => 0b001,
            "D"   => 0b010,
            "MD"  => 0b011,
            "A"   => 0b100,
            "AM"  => 0b101,
            "AD"  => 0b110,
            "AMD" => 0b111,
            _ => return Err(Error::AssembleError(format!("Invalid destination at line {}", self.line_num)))
        })
    }

    fn get_comp_bits(&self, cursor_idx: &mut usize, line: &str) -> Result<u16> {
        let semi_pos = match line.find(";") {
            Some(pos) => pos,
            None => line.len()
        };

        let comp = &line[*cursor_idx..semi_pos];
        *cursor_idx = semi_pos + 1;

        Ok(match comp {
            "0"   => 0b0101010,
            "1"   => 0b0111111,
            "-1"  => 0b0111010,
            "D"   => 0b0001100,
            "A"   => 0b0110000,
            "!D"  => 0b0001101,
            "!A"  => 0b0110001,
            "-D"  => 0b0001111,
            "-A"  => 0b0110011,
            "D+1" => 0b0011111,
            "A+1" => 0b0110111,
            "D-1" => 0b0001110,
            "A-1" => 0b0110010,
            "D+A" => 0b0000010,
            "D-A" => 0b0010011,
            "A-D" => 0b0000111,
            "D&A" => 0b0000000,
            "D|A" => 0b0010101,
            "M"   => 0b1110000,
            "!M"  => 0b1110001,
            "-M"  => 0b1110011,
            "M+1" => 0b1110111,
            "M-1" => 0b1110010,
            "D+M" => 0b1000010,
            "D-M" => 0b1010011,
            "M-D" => 0b1000111,
            "D&M" => 0b1000000,
            "D|M" => 0b1010101,
            _ => return Err(Error::AssembleError(format!("Invalid computation at line {}", self.line_num)))
        })
    }

    fn get_jump_bits(&self, cursor_idx: usize, line: &str) -> Result<u16> {
        if cursor_idx >= line.len() {
            return Ok(0b000);
        }
        
        Ok(match &line[cursor_idx..] {
            "JGT" => 0b001,
            "JEQ" => 0b010,
            "JGE" => 0b011,
            "JLT" => 0b100,
            "JNE" => 0b101,
            "JLE" => 0b110,
            "JMP" => 0b111,
            _ => return Err(Error::AssembleError(format!("Invalid jump at line {}", self.line_num)))
        })
    }
}