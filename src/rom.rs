use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct LoadedRom {
    pub bytes: Vec<u8>,
}

#[derive(Clone)]
pub struct Rom {
    pub name: String,
    pub bytes: Vec<u8>,
}

#[derive(Resource, Clone)]
pub struct Roms {
    pub roms: Vec<Rom>,
}

impl Default for Roms {
    fn default() -> Self {
        Self {
            roms: vec![
                // Rom {
                //     name: "Logo".to_string(),
                //     bytes: include_bytes!("../roms/test/1-chip8-logo.ch8").to_vec(),
                // },
                // Rom {
                //     name: "IBM Logo".to_string(),
                //     bytes: include_bytes!("../roms/test/2-ibm-logo.ch8").to_vec(),
                // },
                // Rom {
                //     name: "Corax+".to_string(),
                //     bytes: include_bytes!("../roms/test/3-corax+.ch8").to_vec(),
                // },
                // Rom {
                //     name: "opcodes".to_string(),
                //     bytes: include_bytes!("../roms/test/X-opcodes.ch8").to_vec(),
                // },
                // Rom {
                //     name: "flags".to_string(),
                //     bytes: include_bytes!("../roms/test/4-flags.ch8").to_vec(),
                // },
                Rom {
                    name: "Space Invaders".to_string(),
                    bytes: include_bytes!("../roms/games/Space_Invaders_[David Winter].ch8")
                        .to_vec(),
                },
                Rom {
                    name: "Keypad".to_string(),
                    bytes: include_bytes!("../roms/test/6-keypad.ch8").to_vec(),
                },
                Rom {
                    name: "Beep".to_string(),
                    bytes: include_bytes!("../roms/test/7-beep.ch8").to_vec(),
                },
                Rom {
                    name: "Quirks".to_string(),
                    bytes: include_bytes!("../roms/test/5-quirks.ch8").to_vec(),
                },
            ],
        }
    }
}
