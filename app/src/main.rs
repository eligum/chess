use bevy::prelude::*;

const BOARD_SIZE: Vec2 = Vec2::new(800.0, 800.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.6, 0.4, 0.3)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Chess engine".into(),
                        resolution: (1000.0, 1000.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Board;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let LIGHT_SQUARES_COLOR = Color::hex("#FFD180").unwrap();
    let DARK_SQUARES_COLOR = Color::hex("#795548").unwrap();

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Board
    let texture = asset_server.load("chess_pieces.png");

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            texture,
            sprite: Sprite {
                // color: LIGHT_SQUARES_COLOR,
                // custom_size: Some(BOARD_SIZE),
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
