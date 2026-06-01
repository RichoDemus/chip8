pub mod chip8;
pub mod chip8_bridge;
pub mod menu;
pub mod rom;

use crate::chip8_bridge::Chip8BridgePlugin;
use crate::menu::MenuPlugin;
use crate::rom::Roms;
use bevy::prelude::*;

fn main() {
    App::new()
        .init_resource::<Roms>()
        .add_plugins((DefaultPlugins, MenuPlugin, Chip8BridgePlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(300., 200., 0.)));
}
