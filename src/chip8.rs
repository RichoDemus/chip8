use crate::chip8::Operation::{AddRegister, ClearScreen, Draw, Jump, SetIndex, SetRegister};
use bevy::prelude::Resource;

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

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    ClearScreen,
    SetRegister {
        vx: usize,
        value: u8,
    },
    AddRegister {
        vx: usize,
        value: u8,
    },
    SetIndex {
        value: u16,
    },
    Draw {
        register_x: usize,
        register_y: usize,
        height: u8,
    },
    Jump {
        program_counter: usize,
    },
}

impl From<u16> for Operation {
    fn from(op_word: u16) -> Self {
        // common fields
        let o = ((op_word >> 12) & 0xF) as u8; // high nibble (opcode)
        let x = ((op_word >> 8) & 0xF) as u8; // X nibble
        let y = ((op_word >> 4) & 0xF) as u8; // Y nibble
        let n = (op_word & 0xF) as u8; // low nibble
        let nn = (op_word & 0x00FF) as u8; // lowest 8 bits
        let nnn = op_word & 0x0FFF; // lowest 12 bits

        match o {
            0x0 => match op_word {
                0x00E0 => ClearScreen,
                other => panic!("Unhandled 0x0 opcode: {other:#04x}"),
            },

            0x1 => Jump {
                program_counter: nnn as usize,
            },

            0x6 => SetRegister {
                vx: x as usize,
                value: nn,
            },

            0x7 => AddRegister {
                vx: x as usize,
                value: nn,
            },

            0xA => SetIndex { value: nnn },

            0xD => Draw {
                register_x: x as usize,
                register_y: y as usize,
                height: n,
            },

            other => panic!("Unhandled opcode: {other:#x} (full: {op_word:#06x})"),
        }
    }
}

pub(crate) const ROWS: usize = 32;
pub(crate) const COLUMNS: usize = 64;

#[derive(Resource)]
pub(crate) struct Chip8 {
    pub(crate) display: [[bool; COLUMNS]; ROWS],
    memory: [u8; 4096],
    _stack: Vec<u16>,
    program_counter: usize,
    register_i: u16,
    registers: [u8; 16],
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            display: [[false; COLUMNS]; ROWS],
            memory: [0; 4096],
            _stack: Vec::new(),
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

        let operation: Operation = operation.into();

        match operation {
            ClearScreen => self.display = [[false; COLUMNS]; ROWS],
            SetRegister { vx, value } => {
                // println!("Set Register {vx} to {value:#x}")
                self.registers[vx] = value;
            }
            AddRegister { vx, value } => self.registers[vx] += value,
            SetIndex { value } => {
                //println!("Set Index Register to {value:#x}")
                self.register_i = value;
            }
            Draw {
                register_x,
                register_y,
                height,
            } => {
                let orig_x = self.registers[register_x] as usize % COLUMNS;
                let orig_y = self.registers[register_y] as usize % ROWS;
                self.registers[0xF] = 0;

                for row in 0..height as usize {
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
            Jump { program_counter } => {
                // println!("Jump program counter to {program_counter:#x}")
                self.program_counter = program_counter;
            }
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
    fn test_decode_clear_screen() {
        let operation: Operation = 0x00E0.into();
        assert_eq!(operation, ClearScreen)
    }

    #[test]
    fn test_decode_set_register() {
        let operation: Operation = 0x6512.into();
        assert_eq!(
            operation,
            SetRegister {
                vx: 0x5,
                value: 0x12
            }
        );
    }

    #[test]
    fn test_decode_draw() {
        let operation: Operation = 0xDABC.into();
        assert_eq!(
            operation,
            Draw {
                register_x: 0xA,
                register_y: 0xB,
                height: 0xC
            }
        );
    }

    #[test]
    fn test_decode_jump() {
        let operation: Operation = 0x1ABC.into();
        assert_eq!(
            operation,
            Jump {
                program_counter: 0xABC
            }
        );
    }

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
