mod chip8;

use crate::chip8::{COLUMNS, Chip8, ROWS};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Chip8>()
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_pixels)
        .add_systems(FixedUpdate, tick_cpu)
        .add_systems(Update, translate_display_to_bevy)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(300., 200., 0.),
        // Projection::Orthographic({
        //     let mut projection = OrthographicProjection::default_2d();
        //     projection.scaling_mode = bevy::camera::ScalingMode::Fixed {
        //         width: 100.,
        //         height: 200.,
        //     };
        //     projection
        // }),
    ));
}

#[derive(Component)]
struct Pixel {
    row: usize,
    column: usize,
}

fn setup_pixels(
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
