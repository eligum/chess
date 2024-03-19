use crate::graphics::*;
use bevy::{
    math::{vec2, vec3},
    prelude::*,
    window::{PresentMode, PrimaryWindow},
};
use engine::{
    board::{self, Move},
    parser, piece,
};

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
                        resolution: (900.0, 900.0).into(),
                        resizable: false,
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            asset_loading_plugin,
        ))
        .init_resource::<CursorWorldCoords>()
        .init_resource::<GrabToolState>()
        .add_event::<PieceGrabbedEvent>()
        .add_event::<PieceDroppedEvent>()
        .add_systems(
            Startup,
            (spawn_camera, spawn_board, spawn_pieces.after(spawn_board)),
        )
        .add_systems(
            Update,
            (
                cursor_position_system,
                board_action_detection_system,
                grab_event_listener,
                drop_event_listener,
                follow_cursor,
                color_occupied_squares,
            ),
        )
        .run();
}

fn board_action_detection_system(
    mouse: Res<ButtonInput<MouseButton>>,
    cursor_position: Res<CursorWorldCoords>,
    mut evw_piece_grab: EventWriter<PieceGrabbedEvent>,
    mut evw_piece_dropped: EventWriter<PieceDroppedEvent>,
    qy_board: Query<&Board>,
) {
    let board = qy_board.single();

    if mouse.just_pressed(MouseButton::Left) {
        //info!("Left mouse just pressed at position {}", cursor_position.0,);
        if let Some(index) = board.index_at(cursor_position.0) {
            info!("Clicked square with index {}", index);
            if let Some(piece) = board.bitboard.at(index) {
                if piece.color() == board.bitboard.color_to_move() {
                    evw_piece_grab.send(PieceGrabbedEvent { board_index: index });
                } else {
                    // TODO: Maybe a capture if a piece was selected
                }
            }
        }
    }

    if mouse.just_released(MouseButton::Left) {
        //info!("Left mouse just released at position {}", cursor_position.0);
        evw_piece_dropped.send(PieceDroppedEvent {
            board_index: board.index_at(cursor_position.0),
        });
    }
}

fn drop_event_listener(
    mut commands: Commands,
    mut grab_tool: ResMut<GrabToolState>,
    mut evr_piece_drop: EventReader<PieceDroppedEvent>,
    mut qy_piece: Query<(Entity, &mut Piece, &mut Transform)>,
    mut qy_window: Query<&mut Window, With<PrimaryWindow>>,
    mut qy_board: Query<&mut Board>,
) {
    for ev in evr_piece_drop.read() {
        info!("{:?}", ev);
        // Check if a piece is being grabbed.
        if let Some(piece_id_o) = grab_tool.dragged_piece_id {
            if let Some(index_t) = ev.board_index {
                // Get id of the possibly already existing piece at the target square.
                let target_square = qy_piece
                    .iter()
                    .find(|(_, p, _)| p.index == index_t)
                    .map(|(id, _, _)| id);
                // Get components of the piece at the origin square.
                if let Ok((_, mut piece, mut transform)) = qy_piece.get_mut(piece_id_o) {
                    let mut board = qy_board.single_mut();
                    let index_o = piece.index;
                    // Whether the move can be successfully applied or not, as long as the
                    // origin and target squares are different, we clear the selected piece.
                    if index_o != index_t {
                        grab_tool.selected_piece_id = None;
                    }
                    // TODO: Check if move is legal.
                    if board
                        .bitboard
                        .make_move(Move::from_indices(index_o, index_t))
                    {
                        let coords = board.position_at(index_t);
                        transform.scale = Vec3::splat(1.0);
                        transform.translation = Vec3::new(coords.x, coords.y, 0.1);
                        piece.index = index_t;
                        // Since the move was applied successfully, if the target square
                        // had a piece, that means the move was a capture so the captured
                        // piece must be despawned.
                        if let Some(piece_id_t) = target_square {
                            commands.entity(piece_id_t).despawn();
                        }
                    } else {
                        // Go back to the original square because the move did not update
                        // the state of the board.
                        *transform = grab_tool.dragged_piece_orig_transform;
                    }
                } else {
                    warn!("No entity with 'Piece' component and id {:?}", piece_id_o);
                }
            } else if let Ok((_, _, mut transform)) = qy_piece.get_mut(piece_id_o) {
                // Piece was dropped out of bounds of the board so we go back to
                // the original square and clear the currently selected piece.
                grab_tool.selected_piece_id = None;
                *transform = grab_tool.dragged_piece_orig_transform;
            } else {
                warn!("No entity with 'Piece' component and id {:?}", piece_id_o);
            }
            grab_tool.dragged_piece_id = None;
            qy_window.single_mut().cursor.icon = CursorIcon::Default;
        }
    }
}

fn grab_event_listener(
    mut grab_tool: ResMut<GrabToolState>,
    mut evr_piece_grab: EventReader<PieceGrabbedEvent>,
    mut qy_piece: Query<(Entity, &mut Transform, &Piece)>,
    mut qy_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    for ev in evr_piece_grab.read() {
        info!("{:?}", ev);
        for (e, mut t, p) in qy_piece.iter_mut() {
            if p.index == ev.board_index {
                // Color valid target squares for the grabbed piece.
                grab_tool.selected_piece_id = Some(e);
                grab_tool.dragged_piece_id = Some(e);
                grab_tool.dragged_piece_orig_transform = *t;
                t.scale = Vec3::splat(1.2);
                let mut window = qy_window.single_mut();
                window.cursor.icon = CursorIcon::Grabbing;
            }
        }
    }
}

fn follow_cursor(
    grab_tool: Res<GrabToolState>,
    cursor_world_coords: Res<CursorWorldCoords>,
    qy_board: Query<&GlobalTransform, With<Board>>,
    mut qy_piece: Query<&mut Transform, With<Piece>>,
) {
    if let Some(piece_entity) = grab_tool.dragged_piece_id {
        if let Ok(mut transform) = qy_piece.get_mut(piece_entity) {
            let board_transform = qy_board.single();
            let cursor = vec3(cursor_world_coords.0.x, cursor_world_coords.0.y, 0.2);
            transform.translation = cursor - board_transform.translation();
        }
    }
}

#[derive(Event, Debug)]
pub struct PieceGrabbedEvent {
    pub board_index: usize,
}

#[derive(Event, Default, Debug)]
pub struct PieceDroppedEvent {
    pub board_index: Option<usize>,
}

#[derive(Resource)]
pub struct GrabToolState {
    pub dragged_piece_id: Option<Entity>,
    pub dragged_piece_orig_transform: Transform,
    pub selected_piece_id: Option<Entity>,
}

impl Default for GrabToolState {
    fn default() -> Self {
        Self {
            dragged_piece_id: None,
            dragged_piece_orig_transform: default(),
            selected_piece_id: None,
        }
    }
}

#[derive(Component)]
pub struct Board {
    // graphics
    pub center: Vec2,
    pub size: Vec2,
    // internal representation
    pub bitboard: board::Board,
}

impl Board {
    /// Returns the index of the square located at `position` wrapped in `Some()`,
    /// or `None` if the position (in world coordinates) is outside the board.
    pub fn index_at(&self, position: Vec2) -> Option<usize> {
        let p_min = self.center - self.size / 2.0;
        let p_norm = position - p_min;
        if p_norm.x >= 0.0 && p_norm.x < self.size.x && p_norm.y > 0.0 && p_norm.y < self.size.y {
            let file = (p_norm.x / self.size.x * 8.0) as usize;
            let rank = (p_norm.y / self.size.y * 8.0) as usize;
            Some(rank * 8 + file)
        } else {
            None
        }
    }

    /// Returns the coordinates relative to the board's center of the square with
    /// index `index`.
    pub fn position_at(&self, index: usize) -> Vec2 {
        let file = index % 8;
        let rank = index / 8;
        let square_size = self.size / 8.0;
        let first_square = Vec2::ZERO - (self.size - square_size) / 2.0;
        first_square + vec2(square_size.x * file as f32, square_size.y * rank as f32)
    }
}

#[derive(Component)]
struct Square {
    index: usize,
}

#[derive(Component)]
struct Piece {
    index: usize,
}

#[derive(Resource, Default)]
struct CursorWorldCoords(Vec2);

fn color_occupied_squares(
    graphics: Res<Graphics>,
    qy_board: Query<&Board>,
    mut qy_squares: Query<(&Square, &mut Sprite)>,
) {
    let (light_color, dark_color) = graphics.board_theme;
    let tint = vec3(0.3, 0.3, 2.0);
    let board = qy_board.single();
    for (square, mut sprite) in qy_squares.iter_mut() {
        let file = square.index % 8;
        let rank = square.index / 8;
        if let Some(_) = board.bitboard.at(square.index) {
            if (file + rank) % 2 == 0 {
                sprite.color = dark_color * tint;
            } else {
                sprite.color = light_color * tint;
            }
        } else {
            if (file + rank) % 2 == 0 {
                sprite.color = dark_color;
            } else {
                sprite.color = light_color;
            }
        }
    }
}

fn spawn_camera(mut commands: Commands, qy_window: Query<&Window, With<PrimaryWindow>>) {
    let window = qy_window.single();
    let _ar = window.height() / window.width();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1.0), // Camera z = 1.0 to see sprites at z = 0.0
        ..default()
    });
}

fn cursor_position_system(
    mut cursor_world_coords: ResMut<CursorWorldCoords>,
    qy_window: Query<&Window, With<PrimaryWindow>>,
    qy_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = qy_camera.single();
    let window = qy_window.single();

    if let Some(world_coords) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        cursor_world_coords.0 = world_coords;
    }
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
                            Piece {
                                index: rank * 8 + file,
                            },
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
    let board_center = vec2(50.0, 0.0);

    let board_id = commands
        .spawn((
            Board {
                center: board_center,
                size: board_size,
                bitboard: board::Board::new(),
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
