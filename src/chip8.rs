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
    delay_timer: u8,
    sound_timer: u8,
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
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}

impl Chip8 {
    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, byte) in rom.iter().enumerate() {
            self.memory[512 + i] = *byte;
        }
    }

    pub(crate) fn tick(&mut self, keys: &[bool; 16]) {
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
                        let carry = if minuend >= subtrahend { 1 } else { 0 };
                        self.registers[vx as usize] =
                            self.registers[vx as usize].wrapping_sub(self.registers[vy as usize]);
                        self.registers[0xF] = carry;
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
                        let carry = if minuend >= subtrahend { 1 } else { 0 };
                        self.registers[vx as usize] =
                            self.registers[vy as usize].wrapping_sub(self.registers[vx as usize]);
                        self.registers[0xF] = carry;
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

            0xB => {
                // Jump with offset
                self.program_counter = nnn as usize + self.registers[0] as usize;
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
            0xE => {
                match nn {
                    0x9E => {
                        // println!("Checking if {} is pressed", self.registers[vx as usize]);
                        if keys[self.registers[vx as usize] as usize] {
                            self.program_counter += 2;
                        }
                    }
                    0xA1 => {
                        // println!("Checking if {} is NOT pressed", self.registers[vx as usize]);
                        if !keys[self.registers[vx as usize] as usize] {
                            self.program_counter += 2;
                        }
                    }
                    other => panic!("Unhandled E-type opcode: {other:#x} (full: {operation:#06x})"),
                }
            }
            0xF => {
                match nn {
                    0x07 => {
                        // Get delay timer
                        self.registers[vx as usize] = self.delay_timer;
                    }
                    0x0A => {
                        // Get key
                        let mut key_was_pressed = false;
                        for (i, pressed) in keys.iter().enumerate() {
                            if *pressed {
                                self.registers[vx as usize] = i as u8;
                                key_was_pressed = true;
                            }
                        }
                        if !key_was_pressed {
                            self.program_counter -= 2;
                        }
                    }
                    0x15 => {
                        // Set delay timer
                        self.delay_timer = self.registers[vx as usize];
                    }
                    0x18 => {
                        // Set sound timer
                        self.sound_timer = self.registers[vx as usize];
                    }
                    0x1e => {
                        // Add to index
                        let (new_i, carry) = self
                            .register_i
                            .overflowing_add(self.registers[vx as usize] as u16);
                        self.register_i = new_i;
                        self.registers[0xF] = carry as u8;
                    }
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
                        //self.register_i += vx as u16 + 1;
                    }
                    0x65 => {
                        // Load memory into registers
                        for i in 0..=vx {
                            self.registers[i as usize] =
                                self.memory[self.register_i as usize + i as usize];
                        }
                        //self.register_i += vx as u16 + 1;
                    }
                    other => panic!("Unhandled F-opcode: {other:#x} (full: {operation:#06x})"),
                }
            }
            other => panic!("Unhandled opcode: {other:#x} (full: {operation:#06x})"),
        }
    }

    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
    pub fn should_beep(&self) -> bool {
        self.sound_timer > 0
    }
}

fn _add_fonts(memory: &mut [u8; 4096], fonts: [u8; 80]) {
    for (i, byte) in fonts.iter().enumerate() {
        memory[i + 80] = *byte;
    }
}
