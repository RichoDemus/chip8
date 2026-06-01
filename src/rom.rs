use bevy::prelude::*;

#[derive(Resource)]
pub struct Rom {
    pub bytes: Vec<u8>,
}

impl Rom {
    pub fn load_logo() -> Rom {
        Rom {
            bytes: include_bytes!("../roms/test/1-chip8-logo.ch8").to_vec(),
        }
    }

    pub fn load_opcodes() -> Rom {
        Rom {
            bytes: include_bytes!("../roms/test/X-opcodes.ch8").to_vec(),
        }
    }
}
