use bevy::prelude::Resource;
use std::collections::VecDeque;

const _FONTS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub(crate) const ROWS: usize = 32;
pub(crate) const COLUMNS: usize = 64;

#[derive(Resource)]
pub(crate) struct Chip8 {
    pub(crate) display: [[bool; COLUMNS]; ROWS],
    memory: [u8; 4096],
    stack: VecDeque<usize>,
    program_counter: usize,
    register_i: u16,
    registers: [u8; 16],
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            display: [[false; COLUMNS]; ROWS],
            memory: [0; 4096],
            stack: VecDeque::new(),
            program_counter: 512,
            register_i: 0,
            registers: [0; 16],
        }
    }
}

impl Chip8 {
    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, byte) in rom.iter().enumerate() {
            self.memory[512 + i] = *byte;
        }
    }

    pub(crate) fn tick(&mut self) {
        let operation = ((self.memory[self.program_counter] as u16) << 8)
            | self.memory[self.program_counter + 1] as u16;
        self.program_counter += 2;

        let opcode = ((operation >> 12) & 0xF) as u8; // high nibble (opcode)
        let vx = ((operation >> 8) & 0xF) as u8; // X nibble
        let vy = ((operation >> 4) & 0xF) as u8; // Y nibble
        let n = (operation & 0xF) as u8; // low nibble
        let nn = (operation & 0x00FF) as u8; // lowest 8 bits
        let nnn = operation & 0x0FFF; // lowest 12 bits

        match opcode {
            0x0 => match operation {
                0x00E0 => {
                    // Clear Screen
                    self.display = [[false; COLUMNS]; ROWS]
                }
                0x00EE => {
                    // Return
                    self.program_counter = self.stack.pop_front().unwrap();
                }
                other => panic!("Unhandled 0x0 opcode: {other:#04x}"),
            },

            0x1 => {
                // Jump
                self.program_counter = nnn as usize;
            }
            0x2 => {
                // Call subroutinee
                self.stack.push_front(self.program_counter);
                self.program_counter = nnn as usize;
            }
            0x3 => {
                // Skip if equals value
                if self.registers[vx as usize] == nn {
                    self.program_counter += 2;
                }
            }
            0x4 => {
                // Skip if not equals value
                if self.registers[vx as usize] != nn {
                    self.program_counter += 2;
                }
            }
            0x5 => {
                // Skip if equals another register
                if self.registers[vx as usize] == self.registers[vy as usize] {
                    self.program_counter += 2;
                }
            }

            0x6 => {
                // Set register
                self.registers[vx as usize] = nn;
            }

            0x7 => {
                // Add register
                self.registers[vx as usize] = self.registers[vx as usize].wrapping_add(nn);
            }

            0x8 => {
                // Logical and arithmetic
                match n {
                    0x0 => {
                        // set vx to vy
                        self.registers[vx as usize] = self.registers[vy as usize];
                    }
                    0x1 => {
                        // set vx to vx or vy
                        self.registers[vx as usize] |= self.registers[vy as usize];
                    }
                    0x2 => {
                        // set vx to vx and vy
                        self.registers[vx as usize] &= self.registers[vy as usize];
                    }
                    0x3 => {
                        // set vx to vx xor vy
                        self.registers[vx as usize] ^= self.registers[vy as usize];
                    }
                    0x4 => {
                        // add vy to vx, set carry to 1 if overflow
                        let (new_vx, carry) = self.registers[vx as usize]
                            .overflowing_add(self.registers[vy as usize]);
                        self.registers[vx as usize] = new_vx;
                        self.registers[0xF] = carry as u8;
                    }
                    0x5 => {
                        // subtract vy from vx
                        // sets carry flag to 0 if overflow, 1 if no overflow
                        let minuend = self.registers[vx as usize];
                        let subtrahend = self.registers[vy as usize];
                        if minuend >= subtrahend {
                            self.registers[0xF] = 1;
                        } else {
                            self.registers[0xF] = 0
                        }
                        self.registers[vx as usize] =
                            self.registers[vx as usize].wrapping_sub(self.registers[vy as usize]);
                    }
                    0x6 => {
                        // Shift Right
                        // Ambigious if this should happen or not:
                        self.registers[vx as usize] = self.registers[vy as usize];

                        // normal behavior
                        let lsb = self.registers[vx as usize] & 0x01;
                        self.registers[vx as usize] >>= 1;
                        self.registers[0xF] = lsb;
                    }
                    0x7 => {
                        // subtract vx from vy
                        // sets carry flag to 0 if overflow, 1 if no overflow
                        let minuend = self.registers[vy as usize];
                        let subtrahend = self.registers[vx as usize];
                        if minuend >= subtrahend {
                            self.registers[0xF] = 1;
                        } else {
                            self.registers[0xF] = 0
                        }
                        self.registers[vx as usize] =
                            self.registers[vy as usize].wrapping_sub(self.registers[vx as usize]);
                    }
                    0xE => {
                        // Shift Left
                        // Ambigious if this should happen or not:
                        self.registers[vx as usize] = self.registers[vy as usize];

                        // normal behavior
                        let msb = (self.registers[vx as usize] & 0x80) >> 7;
                        self.registers[vx as usize] <<= 1;
                        self.registers[0xF] = msb;
                    }
                    other => panic!(
                        "Unhandled Logical and arithmetic opcode: {other:#x} (full: {operation:#06x})"
                    ),
                }
            }

            0x9 => {
                // Skip if not equals another register
                if self.registers[vx as usize] != self.registers[vy as usize] {
                    self.program_counter += 2;
                }
            }

            0xA => {
                self.register_i = nnn;
            }

            0xD => {
                // Draw
                let orig_x = self.registers[vx as usize] as usize % COLUMNS;
                let orig_y = self.registers[vy as usize] as usize % ROWS;
                self.registers[0xF] = 0;

                for row in 0..n as usize {
                    let sprite_byte = self.memory[self.register_i as usize + row];
                    let sprite_bits: [bool; 8] =
                        core::array::from_fn(|i| (sprite_byte & (1 << (7 - i))) != 0);

                    #[allow(clippy::needless_range_loop)]
                    for col in 0..8 {
                        let x = (orig_x + col) % COLUMNS;
                        let y = (orig_y + row) % ROWS;

                        if sprite_bits[col] {
                            if self.display[y][x] {
                                self.display[y][x] = false;
                                self.registers[0xF] = 1;
                            } else {
                                self.display[y][x] = true;
                            }
                        }
                    }
                }
            }
            0xF => {
                match nn {
                    0x33 => {
                        // Binary-coded decimal conversion
                        let value = self.registers[vx as usize];
                        self.memory[self.register_i as usize] = value / 100;
                        self.memory[self.register_i as usize + 1] = (value / 10) % 10;
                        self.memory[self.register_i as usize + 2] = value % 10;
                    }
                    0x55 => {
                        // Store registers into memory
                        for i in 0..=vx {
                            self.memory[self.register_i as usize + i as usize] =
                                self.registers[i as usize];
                        }
                    }
                    0x65 => {
                        // Load memory into registers
                        for i in 0..=vx {
                            self.registers[i as usize] =
                                self.memory[self.register_i as usize + i as usize];
                        }
                    }
                    other => panic!("Unhandled F-opcode: {other:#x} (full: {operation:#06x})"),
                }
            }
            other => panic!("Unhandled opcode: {other:#x} (full: {operation:#06x})"),
        }
    }
}

fn _add_fonts(memory: &mut [u8; 4096], fonts: [u8; 80]) {
    for (i, byte) in fonts.iter().enumerate() {
        memory[i + 80] = *byte;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut chip8 = Chip8::default();
        chip8.load_rom(include_bytes!("../roms/test/1-chip8-logo.ch8"));
        for _i in 0..40 {
            chip8.tick();
        }
        fn print_bool_grid(grid: &[[bool; COLUMNS]; ROWS], true_char: char, false_char: char) {
            for row in grid {
                for &cell in row {
                    print!("{}", if cell { true_char } else { false_char });
                }
                println!();
            }
        }
        print_bool_grid(&chip8.display, '■', ' ');
    }
}
