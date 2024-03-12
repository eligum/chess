use crate::graphics::*;
use bevy::{
    math::{vec2, vec3},
    prelude::*,
    window::PrimaryWindow,
};
use engine::{bitboard, parser, piece};

mod graphics;

const BOARD_SIZE: f32 = 720.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins((
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
            asset_loading_plugin,
        ))
        .add_systems(
            Startup,
            (spawn_camera, spawn_board, spawn_pieces.after(spawn_board)),
        )
        .add_systems(Update, (click_detection_system,))
        .run();
}

fn click_detection_system(
    mouse: Res<ButtonInput<MouseButton>>,
    qy_window: Query<&Window, With<PrimaryWindow>>,
    qy_board: Query<&Board>,
) {
    let window = qy_window.single();
    let board = qy_board.single();

    // if mouse.pressed(MouseButton::Left) {
    //     info!("left mouse currently pressed");
    // }

    if mouse.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            let world_cursor_position =
                Vec2::new(position.x - window.width() / 2.0, -position.y + window.height() / 2.0);
            info!(
                "left mouse just pressed at position {}",
                world_cursor_position,
            );
        }
    }

    if mouse.just_released(MouseButton::Left) {
        info!("left mouse just released");
    }
}

struct ActionState<A> {
    a: A,
}

#[derive(Clone, Copy, Debug)]
enum Action {
    DragPiece,
    DropPiece,
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

fn spawn_camera(mut commands: Commands, qy_window: Query<&Window, With<PrimaryWindow>>) {
    let window = qy_window.single();
    let _ar = window.height() / window.width();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1.0), // Camera z = 1.0 to see sprites at z = 0.0
        ..default()
    });
}

fn spawn_pieces(
    mut commands: Commands,
    graphics: Res<Graphics>,
    qy_board: Query<(Entity, &Board)>,
) {
    let (ref texture, ref layout) = graphics.piece_theme;
    let (board_id, board) = qy_board.single();

    let square_size = board.size / 8.0;
    let first_square = Vec2::ZERO - (board.size - square_size) / 2.0;
    let mut piece_ids: Vec<Entity> = Vec::with_capacity(32);

    for rank in 0..8 {
        for file in 0..8 {
            if let Some(piece_type) = board.bitboard.at(rank * 8 + file) {
                info!("At index {} found {:?}", rank * 8 + file, piece_type);
                piece_ids.push(
                    commands
                        .spawn((
                            PieceComp,
                            SpriteSheetBundle {
                                sprite: Sprite {
                                    custom_size: Some(board.size / 8.0),
                                    ..default()
                                },
                                transform: Transform {
                                    translation: vec3(first_square.x, first_square.y, 0.0)
                                        + vec3(
                                            square_size.x * file as f32,
                                            square_size.y * rank as f32,
                                            0.0,
                                        ),
                                    ..default()
                                },
                                texture: texture.clone(),
                                atlas: TextureAtlas {
                                    layout: layout.clone(),
                                    index: match piece_type {
                                        piece::Piece::Pawn(color) => match color {
                                            piece::Color::White => 5,
                                            piece::Color::Black => 11,
                                        },
                                        piece::Piece::Knight(color) => match color {
                                            piece::Color::White => 3,
                                            piece::Color::Black => 9,
                                        },
                                        piece::Piece::Bishop(color) => match color {
                                            piece::Color::White => 2,
                                            piece::Color::Black => 8,
                                        },
                                        piece::Piece::Rook(color) => match color {
                                            piece::Color::White => 4,
                                            piece::Color::Black => 10,
                                        },
                                        piece::Piece::Queen(color) => match color {
                                            piece::Color::White => 1,
                                            piece::Color::Black => 7,
                                        },
                                        piece::Piece::King(color) => match color {
                                            piece::Color::White => 0,
                                            piece::Color::Black => 6,
                                        },
                                    },
                                },
                                ..default()
                            },
                        ))
                        .id(),
                );
            }
        }
    }

    commands.entity(board_id).push_children(&piece_ids[..]);
}

fn spawn_board(mut commands: Commands, graphics: Res<Graphics>) {
    let (light_squares_color, dark_squares_color) = graphics.board_theme;

    let square_size = Vec2::splat(BOARD_SIZE / 8.0);
    let board_size = Vec2::splat(BOARD_SIZE);
    let board_center = vec2(200.0, 0.0);

    let board_id = commands
        .spawn((
            Board {
                center: board_center,
                size: board_size,
                light_color: light_squares_color,
                dark_color: dark_squares_color,
                bitboard: bitboard::Board::new(),
            },
            SpatialBundle {
                transform: Transform::from_xyz(board_center.x, board_center.y, 0.0),
                ..default()
            },
        ))
        .id();

    // NOTE: Child transforms are relative to their parent's transform. Since in this
    // hierarchy board squares are children of a board entity, their transform remains
    // the same no matter the board position.
    let first_square = Vec2::ZERO - (board_size - square_size) / 2.0;
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
    commands.entity(board_id).push_children(&square_ids[..]);
}
