use std::collections::HashMap;

use crate::error::Error;

pub struct SymbolTable {
    symbols: HashMap<String, u16>,
    ram_addr: u16,
}

impl SymbolTable {
    // Adds a symbol table entry given a symbol and its address
    // If the address is None, add the symbol to the symbol table and increment the RAM address
    // If the address is Some, add the symbol to the symbol table with the given address
    pub fn new() -> Self {
        let mut symbols = HashMap::new();

        symbols.insert("R0".to_string(), 0);
        symbols.insert("R1".to_string(), 1);
        symbols.insert("R2".to_string(), 2);
        symbols.insert("R3".to_string(), 3);
        symbols.insert("R4".to_string(), 4);
        symbols.insert("R5".to_string(), 5);
        symbols.insert("R6".to_string(), 6);
        symbols.insert("R7".to_string(), 7);
        symbols.insert("R8".to_string(), 8);
        symbols.insert("R9".to_string(), 9);
        symbols.insert("R10".to_string(), 10);
        symbols.insert("R11".to_string(), 11);
        symbols.insert("R12".to_string(), 12);
        symbols.insert("R13".to_string(), 13);
        symbols.insert("R14".to_string(), 14);
        symbols.insert("R15".to_string(), 15);

        symbols.insert("SCREEN".to_string(), 16384);
        symbols.insert("KBD".to_string(), 24576);
        symbols.insert("SP".to_string(), 0);
        symbols.insert("LCL".to_string(), 1);
        symbols.insert("ARG".to_string(), 2);
        symbols.insert("THIS".to_string(), 3);
        symbols.insert("THAT".to_string(), 4);

        Self {
            symbols,
            ram_addr: 16,
        }
    }

    // Add a symbol to the symbol table
    pub fn add_label(&mut self, label: &str, line_num: u16) {
        self.symbols.insert(label.to_string(), line_num);
    }

    // Add a variable to the symbol table and increment the RAM address
    // Throw an exception if the variable is already in the symbol table
    // Returns the address of the variable
    pub fn add_variable(&mut self, variable: &str) -> Result<u16, Error> {
        if self.symbols.contains_key(variable) {
            return Err(Error::SymbolTableError(format!("Variable {} already exists in the symbol table", variable)));
        }
        self.symbols.insert(variable.to_string(), self.ram_addr);
        self.ram_addr += 1;
        Ok(self.ram_addr - 1)
    }

    // Get the address of a symbol
    // Return None if the symbol is not in the symbol table
    pub fn get_symbol_addr(&self, symbol: &str) -> Option<u16> {
        self.symbols.get(symbol).copied()
    }
}

// impl debug

// Display
impl std::fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (symbol, addr) in &self.symbols {
            writeln!(f, "{}: {}", symbol, addr)?;
        }
        Ok(())
    }
}