mod chip8;
mod chip8_bridge;
mod menu;
mod rom;

use crate::chip8_bridge::Chip8BridgePlugin;
use crate::menu::MenuPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MenuPlugin, Chip8BridgePlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(300., 200., 0.)));
}
