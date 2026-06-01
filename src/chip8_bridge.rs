use crate::chip8::{COLUMNS, Chip8, ROWS};
use crate::menu::GameState;
use crate::rom::LoadedRom;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::*;

pub struct Chip8BridgePlugin;
impl Plugin for Chip8BridgePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Chip8>();
        app.add_systems(OnEnter(GameState::Game), setup_pixels);
        app.add_systems(OnEnter(GameState::Game), load_rom);
        app.add_systems(FixedUpdate, tick_cpu.run_if(in_state(GameState::Game)));
        app.add_systems(
            Update,
            translate_display_to_bevy.run_if(in_state(GameState::Game)),
        );
    }
}

#[derive(Component)]
pub struct Pixel {
    row: usize,
    column: usize,
}

pub fn setup_pixels(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            let mesh = commands
                .spawn((
                    Mesh2d(meshes.add(Rectangle::new(10., 10.))),
                    MeshMaterial2d(materials.add(Color::hsv((column + row) as f32, 0.2, 0.47))),
                    Transform::from_xyz(0., 32., 0.),
                ))
                .id();

            commands
                .spawn((
                    Pixel { row, column },
                    Visibility::Visible,
                    Transform::from_xyz(column as f32 * 10., row as f32 * 10., 0.),
                ))
                .add_child(mesh);
        }
    }
}

fn load_rom(rom: Res<LoadedRom>, mut chip8: ResMut<Chip8>) {
    chip8.load_rom(rom.bytes.as_slice())
}

fn tick_cpu(mut chip8: ResMut<Chip8>) {
    for _ in 0..10 {
        chip8.tick();
    }
}

fn translate_display_to_bevy(chip8: Res<Chip8>, mut pixels: Query<(&Pixel, &mut Visibility)>) {
    for (pixel, mut visibility) in pixels.iter_mut() {
        if chip8.display[ROWS - 1 - pixel.row][pixel.column] {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
