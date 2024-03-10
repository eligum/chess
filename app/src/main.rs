use bevy::{
    math::{vec2, vec3},
    prelude::*,
    window::PrimaryWindow,
};
use engine::{bitboard, parser, piece};

const BOARD_SIZE: f32 = 720.0;
const ASPECT_RATIO: f32 = 9.0 / 16.0;
const VIEWPORT_WIDTH: f32 = 1000.0;
const VIEWPORT_HEIGHT: f32 = 1000.0 * ASPECT_RATIO;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_linear())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Chess engine".into(),
                        resolution: (1600.0, 900.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(
            Startup,
            (
                load_graphics,
                spawn_camera,
                spawn_board.after(load_graphics),
                spawn_pieces.after(spawn_board),
            ),
        )
        .run();
}

#[derive(Resource)]
struct Graphics {
    piece_theme: (Handle<Image>, Handle<TextureAtlasLayout>),
    board_theme: (Color, Color),
}

#[derive(Component)]
struct Board {
    // graphics
    center: Vec2,
    size: Vec2,
    light_color: Color,
    dark_color: Color,
    // internal representation
    bitboard: bitboard::Board,
}

#[derive(Component)]
struct Square {
    index: usize,
}

#[derive(Component)]
struct PieceComp;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn load_graphics(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Piece assets
    let texture_handle = asset_server.load("image/pieces_320x107.png");
    let layout_handle = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::splat(106.5),
        6,
        2,
        None,
        None,
    ));
    // Board assets
    let ligth_squares_color = Color::hex("f0d9b5").unwrap();
    let dark_squares_color = Color::hex("b58863").unwrap();

    commands.insert_resource(Graphics {
        piece_theme: (texture_handle, layout_handle),
        board_theme: (ligth_squares_color, dark_squares_color),
    })
}

fn spawn_pieces(mut commands: Commands, graphics: Res<Graphics>, qy_board: Query<&Board>) {
    let (ref texture, ref layout) = graphics.piece_theme;

    let board = qy_board.single();

    commands.spawn((SpriteSheetBundle {
        sprite: Sprite {
            custom_size: Some(board.size / 8.0),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.1),
        texture: texture.clone(),
        atlas: TextureAtlas {
            layout: layout.clone(),
            index: 0,
        },
        ..default()
    },));

    // spawn_piece(
    //     &mut commands,
    //     0,
    //     piece::Piece::King(piece::Color::White),
    //     texture.clone(),
    //     layout.clone(),
    // );
}

// fn spawn_piece(
//     &mut commands: Commands,
//     square: u32,
//     piece_type: piece::Piece,
//     texture: Handle<Image>,
//     layout: Handle<TextureAtlasLayout>,
// ) {
//     commands.spawn((
//         SpriteSheetBundle {
//             sprite: Sprite {
//                 custom_size: Some(board.size / 8.0),
//                 ..default()
//             },
//             transform: Transform::from_xyz(0.0, 0.0, 0.1),
//             texture,
//             atlas: TextureAtlas {
//                 layout,
//                 index: match piece_type {
//                     piece::Piece::Pawn(color) => match color {
//                         piece::Color::White => 0,
//                         piece::Color::Black => 6,
//                     },
//                     piece::Piece::Knight(color) => match color {
//                         piece::Color::White => 1,
//                         piece::Color::Black => 7,
//                     },
//                     piece::Piece::Bishop(color) => match color {
//                         piece::Color::White => 2,
//                         piece::Color::Black => 8,
//                     },
//                     piece::Piece::Rook(color) => match color {
//                         piece::Color::White => 3,
//                         piece::Color::Black => 9,
//                     },
//                     piece::Piece::Queen(color) => match color {
//                         piece::Color::White => 4,
//                         piece::Color::Black => 10,
//                     },
//                     piece::Piece::King(color) => match color {
//                         piece::Color::White => 5,
//                         piece::Color::Black => 11,
//                     },
//                 },
//             },
//             ..default()
//         },
//         PieceComp,
//     ));
// }

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    let ar = window.height() / window.width();
    let viewport = Vec2::new(VIEWPORT_WIDTH, VIEWPORT_WIDTH * ar);

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1.0), // Camera z = 1.0 to see sprites at z = 0.0
        projection: OrthographicProjection {
            area: Rect {
                min: viewport / -2.0,
                max: viewport / 2.0,
            },
            ..default()
        },
        ..default()
    });
}

fn spawn_board(mut commands: Commands, graphics: Res<Graphics>) {
    let square_size = Vec2::splat(BOARD_SIZE / 8.0);
    let board_size = Vec2::splat(BOARD_SIZE);
    let board_center = vec2(0.0, 0.0);

    let (light_squares_color, dark_squares_color) = graphics.board_theme;

    let board_id = commands
        .spawn(Board {
            center: board_center,
            size: board_size,
            light_color: light_squares_color,
            dark_color: dark_squares_color,
            bitboard: bitboard::Board::new(),
        })
        .id();

    let first_square = board_center - (board_size - square_size) / 2.0;
    let mut square_ids = [Entity::from_raw(0); 64];

    for rank in 0..8 {
        for file in 0..8 {
            let index = rank * 8 + file;
            square_ids[index] = commands
                .spawn((
                    Square { index },
                    SpriteBundle {
                        transform: Transform {
                            translation: vec3(first_square.x, first_square.y, 0.0)
                                + vec3(
                                    square_size.x * file as f32,
                                    square_size.y * rank as f32,
                                    0.0,
                                ),
                            ..default()
                        },
                        sprite: Sprite {
                            color: if (rank + file) % 2 == 0 {
                                dark_squares_color
                            } else {
                                light_squares_color
                            },
                            custom_size: Some(square_size),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .id();
        }
    }

    // Construct parent/child hierarchy
    // commands.entity(board_id).push_children(&square_ids[..]);
}
