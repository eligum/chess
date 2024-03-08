use bevy::{
    math::{vec2, vec3},
    prelude::*,
    window::PrimaryWindow,
};

const BOARD_SIZE: f32 = 720.0;

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
        .add_systems(Startup, load_graphics)
        .add_systems(Update, rotate_piece_texture)
        .run();
}

#[derive(Component)]
struct Board;

#[derive(Component)]
struct Piece {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn load_graphics(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // 515x165
    let texture = asset_server.load("pieces_240_6x2.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(240.0), 6, 2, None, None);
    let layout_handle = texture_atlases.add(layout);

    commands.spawn((
        SpriteSheetBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(BOARD_SIZE / 8.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture,
            atlas: TextureAtlas {
                layout: layout_handle,
                index: 0,
            },
            ..default()
        },
        Piece { first: 0, last: 12 },
        AnimationTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
    ));
}

fn rotate_piece_texture(
    time: Res<Time>,
    mut query: Query<(&mut Piece, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = (atlas.index + 1) % indices.last;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    // Camera
    spawn_camera(&mut commands, window);

    // Board
    create_board(&mut commands);

    // Pieces
    // let texture = asset_server.load("b_queen.png");

    // commands.spawn(SpriteBundle {
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 0.0, 0.0),
    //         ..default()
    //     },
    //     texture,
    //     sprite: Sprite {
    //         // color: LIGHT_SQUARES_COLOR,
    //         // custom_size: Some(BOARD_SIZE),
    //         ..default()
    //     },
    //     ..default()
    // });
}

fn spawn_camera(commands: &mut Commands, window: &Window) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() * 0.0, window.height() * 0.0, 0.0),
        ..default()
    });
}

fn create_board(commands: &mut Commands) {
    let ligth_squares_color = Color::hex("f0d9b5").unwrap();
    let dark_squares_color = Color::hex("b58863").unwrap();

    let board_size = BOARD_SIZE;
    let square_size = board_size / 8.0;
    let inital_pos =
        vec3(square_size, square_size, 0.0) / 2.0 - vec3(board_size / 2.0, board_size / 2.0, 0.0);

    for file in 0..8 {
        for rank in 0..8 {
            commands.spawn(
                (SpriteBundle {
                    transform: Transform {
                        translation: inital_pos
                            + vec3(square_size * rank as f32, square_size * file as f32, 0.0),
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

    for i in 0..8 {
        // commands.spawn_batch(bundles_iter)
    }
}
