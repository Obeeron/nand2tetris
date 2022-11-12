use std::collections::HashMap;

pub struct Keyboard {
    pub reg: u16,                   // Keyboard register
    pub key_map: HashMap<u32,u16>,  // Map of winit scancodes to Hack keyboard codes
}

impl Keyboard {
    pub fn keycode_from_winit(&self, code: u32) -> u16 {
        match self.key_map.get(&code) {
            Some(key) => *key,
            None => 0,
        }
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        let mut key_map = HashMap::new();
        
        key_map.insert(1, 140);
        key_map.insert(2, '1' as u16);      // 1
        key_map.insert(3, '2' as u16);      // 2
        key_map.insert(4, '3' as u16);      // 3
        key_map.insert(5, '4' as u16);      // 4 
        key_map.insert(6, '5' as u16);      // 5
        key_map.insert(7, '6' as u16);      // 6
        key_map.insert(8, '7' as u16);      // 7
        key_map.insert(9, '8' as u16);      // 8
        key_map.insert(10, '9' as u16);     // 9
        key_map.insert(11, '0' as u16);     // 0
        key_map.insert(12, '-' as u16);     // -
        key_map.insert(13, '=' as u16);     // =
        key_map.insert(14, 129);            // Backspace
        key_map.insert(16, 'q' as u16);     // q
        key_map.insert(17, 'w' as u16);     // w
        key_map.insert(18, 'e' as u16);     // e
        key_map.insert(19, 'r' as u16);     // r
        key_map.insert(20, 't' as u16);     // t
        key_map.insert(21, 'y' as u16);     // y
        key_map.insert(22, 'u' as u16);     // u
        key_map.insert(23, 'i' as u16);     // i
        key_map.insert(24, 'o' as u16);     // o
        key_map.insert(25, 'p' as u16);     // p
        key_map.insert(26, '[' as u16);     // [
        key_map.insert(27, ']' as u16);     // ]
        key_map.insert(28, 128);            // Enter
        key_map.insert(30, 'a' as u16);     // a
        key_map.insert(31, 's' as u16);     // s
        key_map.insert(32, 'd' as u16);     // d
        key_map.insert(33, 'f' as u16);     // f
        key_map.insert(34, 'g' as u16);     // g
        key_map.insert(35, 'h' as u16);     // h
        key_map.insert(36, 'j' as u16);     // j
        key_map.insert(37, 'k' as u16);     // k
        key_map.insert(38, 'l' as u16);     // l
        key_map.insert(39, ';' as u16);     // ;
        key_map.insert(40, '\'' as u16);    // '
        key_map.insert(41, '`' as u16);     // `
        key_map.insert(43, '\\' as u16);    // \
        key_map.insert(44, 'z' as u16);     // z
        key_map.insert(45, 'x' as u16);     // x
        key_map.insert(46, 'c' as u16);     // c
        key_map.insert(47, 'v' as u16);     // v
        key_map.insert(48, 'b' as u16);     // b
        key_map.insert(49, 'n' as u16);     // n
        key_map.insert(50, 'm' as u16);     // m
        key_map.insert(51, ',' as u16);     // ,
        key_map.insert(52, '.' as u16);     // .
        key_map.insert(53, '/' as u16);     // /
        key_map.insert(57416, 131);         // Up arrow
        key_map.insert(57419, 130);         // Left arrow
        key_map.insert(57421, 132);         // Right arrow
        key_map.insert(57424, 133);         // Down arrow
        
        Self { reg: 0, key_map }
    }
}