use crate::chip8::{COLUMNS, Chip8, ROWS};
use crate::menu::GameState;
use crate::rom::LoadedRom;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::*;
use std::time::Duration;

pub struct Chip8BridgePlugin;
impl Plugin for Chip8BridgePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Chip8>();
        app.insert_resource(Time::<Fixed>::from_duration(Duration::from_micros(16666)));
        app.add_systems(OnEnter(GameState::Game), setup_pixels);
        app.add_systems(OnEnter(GameState::Game), setup_chip8);
        app.add_systems(OnEnter(GameState::Game), setup_audio);
        app.add_systems(FixedUpdate, tick_cpu.run_if(in_state(GameState::Game)));
        app.add_systems(
            Update,
            translate_display_to_bevy.run_if(in_state(GameState::Game)),
        );
        app.add_systems(Update, beep.run_if(in_state(GameState::Game)));
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

fn setup_chip8(rom: Res<LoadedRom>, mut chip8: ResMut<Chip8>) {
    chip8.load_rom(rom.bytes.as_slice());
    chip8.add_fonts();
}

#[derive(Component)]
struct Beep;

fn setup_audio(mut commands: Commands, mut pitch_assets: ResMut<Assets<Pitch>>) {
    commands.spawn((
        AudioPlayer(pitch_assets.add(Pitch::new(220., Duration::new(1, 0)))),
        PlaybackSettings::LOOP,
        Beep,
    ));
}

fn tick_cpu(mut chip8: ResMut<Chip8>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    chip8.decrement_timers();
    chip8.waiting_for_vertical_blank = false;
    for _ in 0..20 {
        chip8.tick(&[
            keyboard_input.pressed(KeyCode::KeyX),
            keyboard_input.pressed(KeyCode::Digit1),
            keyboard_input.pressed(KeyCode::Digit2),
            keyboard_input.pressed(KeyCode::Digit3),
            keyboard_input.pressed(KeyCode::KeyQ),
            keyboard_input.pressed(KeyCode::KeyW),
            keyboard_input.pressed(KeyCode::KeyE),
            keyboard_input.pressed(KeyCode::KeyA),
            keyboard_input.pressed(KeyCode::KeyS),
            keyboard_input.pressed(KeyCode::KeyD),
            keyboard_input.pressed(KeyCode::KeyZ),
            keyboard_input.pressed(KeyCode::KeyC),
            keyboard_input.pressed(KeyCode::Digit4),
            keyboard_input.pressed(KeyCode::KeyR),
            keyboard_input.pressed(KeyCode::KeyF),
            keyboard_input.pressed(KeyCode::KeyV),
        ])
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

fn beep(chip8: Res<Chip8>, audio: Single<&mut AudioSink, With<Beep>>) {
    if chip8.should_beep() {
        audio.play()
    } else {
        audio.pause()
    }
}
