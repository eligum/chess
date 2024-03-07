use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

const BOARD_SIZE: Vec2 = Vec2::new(800.0, 800.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_linear())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Chess engine".into(),
                        resolution: (900.0, 900.0).into(),
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
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Board
    create_board(&mut commands);

    // Pieces
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
    ));
}

fn create_board(
    commands: &mut Commands,
) {
    let ligth_squares_color = Color::hex("#FFD180").unwrap();
    let dark_squares_color = Color::hex("#795548").unwrap();

    let board_size = 600.0;
    let square_size = board_size / 8.0;
    let inital_pos = vec3(square_size, square_size, 0.0) / 2.0 - vec3(board_size / 2.0, board_size / 2.0, 0.0);

    for file in 0..8 {
        for rank in 0..8 {
            commands.spawn(
                (SpriteBundle {
                    transform: Transform {
                        translation: inital_pos + vec3(square_size * rank as f32, square_size * file as f32, 0.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: if (rank + file) % 2 == 0 {
                            dark_squares_color
                        } else {
                            ligth_squares_color
                        },
                        custom_size: Some(vec2(square_size, square_size)),
                        ..default()
                    },
                    ..default()
                }),
            );
        }
    }
}
