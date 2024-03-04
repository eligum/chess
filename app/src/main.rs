use bevy::prelude::*;

const BOARD_SIZE: Vec2 = Vec2::new(800.0, 800.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Board;

fn setup(mut commands: Commands) {
    let LIGHT_SQUARES_COLOR = Color::hex("#FFD180").unwrap();
    let DARK_SQUARES_COLOR = Color::hex("#795548").unwrap();

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Board
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: LIGHT_SQUARES_COLOR,
                custom_size: Some(BOARD_SIZE),
                ..default()
            },
            ..default()
        },
        Board,
    ));
}

// fn create_board(
//     commands: Commands,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let mesh = quads.add(Plane2d::new(Vec2::Y));
// }
