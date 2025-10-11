mod graphics;
mod ui;

use crate::graphics::*;
use crate::ui::*;
use bevy::{
    camera::{Camera, Camera2d},
    math::{vec2, vec3},
    prelude::*,
    window::{CursorIcon, PresentMode, PrimaryWindow, SystemCursorIcon},
};
use engine::{
    board::{self, Move},
    generator::{self, MoveGen},
    parser, piece,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_plugins((
            // Bevy plugins
            DefaultPlugins
                .set(ImagePlugin::default_linear())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Chess engine".into(),
                        resolution: (800, 800).into(),
                        resizable: false,
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            // Custom plugins
            GraphicsPlugin,
        ))
        .init_resource::<CursorWorldCoords>()
        .init_resource::<GrabToolState>()
        .insert_resource(MoveGenerator {
            generator: generator::Naive::new(),
        })
        .add_message::<PieceGrabbedEvent>()
        .add_message::<PieceDroppedEvent>()
        .add_systems(
            Startup,
            (
                setup_camera,
                ui::spawn_board,
                ui::spawn_pieces.after(ui::spawn_board),
                // ui::spawn_indicators.after(ui::spawn_pieces),
            ),
        )
        .add_systems(
            Update,
            (
                update_cursor_world_position,
                board_action_detection_system,
                grab_event_listener::<generator::Naive>,
                drop_event_listener,
                follow_cursor,
                // color_occupied_squares,
            ),
        )
        .run();
}

// #[derive(States)]
// pub enum GameState {
//     InGame,
//     GameOver,
// }

/// Resets the squares' color to the board theme.
fn clear_squares_color() {}

fn board_action_detection_system(
    mouse: Res<ButtonInput<MouseButton>>,
    cursor_position: Res<CursorWorldCoords>,
    grab_tool: Res<GrabToolState>,
    mut mw_piece_grab: MessageWriter<PieceGrabbedEvent>,
    mut mw_piece_dropped: MessageWriter<PieceDroppedEvent>,
    qy_board: Query<&Board>,
) {
    let Ok(board) = qy_board.single() else {
        error!("Expected exactly one board, but found no board or more than one!");
        return;
    };

    // Grab/select piece/square
    if mouse.just_pressed(MouseButton::Left) {
        //info!("Left mouse just pressed at position {}", cursor_position.0,);
        if let Some(index) = board.index_at(cursor_position.0) {
            info!("Clicked square with index {}", index);
            if let Some(piece) = board.bitboard.at(index) {
                mw_piece_grab.write(PieceGrabbedEvent { board_index: index });
            } else {
                // TODO: Square selected event.
            }
        }
    }

    // Drop grabbed piece
    if mouse.just_released(MouseButton::Left) {
        //info!("Left mouse just released at position {}", cursor_position.0);
        if let Some(_) = grab_tool.dragged_piece_id {
            mw_piece_dropped.write(PieceDroppedEvent {
                board_index: board.index_at(cursor_position.0),
            });
        }
    }

    // Cancel selection or grabbing action
    if mouse.just_pressed(MouseButton::Right) {
        mw_piece_dropped.write(PieceDroppedEvent { board_index: None });
    }
}

fn drop_event_listener(
    mut commands: Commands,
    mut grab_tool: ResMut<GrabToolState>,
    mut mr_piece_drop: MessageReader<PieceDroppedEvent>,
    mut qy_piece: Query<(Entity, &mut Piece, &mut Transform)>,
    mut qy_board: Query<&mut Board>,
    qy_window: Single<Entity, With<PrimaryWindow>>,
) {
    for ev in mr_piece_drop.read() {
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
                    let Ok(mut board) = qy_board.single_mut() else {
                        error!("No board found!");
                        return;
                    };
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
                    grab_tool.selected_piece_id = None;
                    warn!("No entity with 'Piece' component and id {:?}", piece_id_o);
                }
            } else if let Ok((_, _, mut transform)) = qy_piece.get_mut(piece_id_o) {
                // Piece was dropped out of bounds of the board or the action was cancelled,
                // so we go back to the original square and clear the currently selected piece.
                grab_tool.selected_piece_id = None;
                *transform = grab_tool.dragged_piece_orig_transform;
            } else {
                grab_tool.selected_piece_id = None;
                warn!("No entity with 'Piece' component and id {:?}", piece_id_o);
            }
            // Reset grab tool state.
            grab_tool.dragged_piece_id = None;
            commands
                .entity(*qy_window)
                .insert(CursorIcon::System(SystemCursorIcon::Default));
            // Clear board indicators if a piece is not selected.
            if grab_tool.selected_piece_id.is_none() {}
        }
    }
}

fn grab_event_listener<G>(
    mut commands: Commands,
    mut grab_tool: ResMut<GrabToolState>,
    mut evr_piece_grab: MessageReader<PieceGrabbedEvent>,
    mut qy_piece: Query<(Entity, &mut Transform, &Piece)>,
    mut qy_squares: Query<&mut Sprite, With<Square>>,
    qy_board: Query<(&Board, &Children)>,
    qy_window: Single<Entity, With<PrimaryWindow>>,
    move_gen: Res<MoveGenerator<G>>,
) where
    G: MoveGen + std::marker::Send + std::marker::Sync,
{
    for ev in evr_piece_grab.read() {
        debug!("{:?}", ev);
        for (entity, mut transform, piece) in qy_piece.iter_mut() {
            if piece.index == ev.board_index {
                let Ok((board, children)) = qy_board.single() else {
                    error!("Expected exactly one board, but found no board or more than one!");
                    return;
                };
                if piece.backend.color() == board.bitboard.color_to_move() {
                    grab_tool.selected_piece_id = Some(entity);
                    grab_tool.dragged_piece_id = Some(entity);
                    grab_tool.dragged_piece_orig_transform = *transform;
                    transform.scale = Vec3::splat(1.2);
                    commands
                        .entity(*qy_window)
                        .insert(CursorIcon::System(SystemCursorIcon::Grabbing));

                    // Color valid target squares for the selected/grabbed piece.
                    // NOTE: The first 64 children of Board entity are the squares and
                    // they preserve the order of insertion.
                    let mut squares_ids = children.iter().take(64).collect::<Vec<_>>();

                    let moves = move_gen.generator.generate_moves(&board.bitboard);
                    let target_indices = generator::extract_target_indices(&moves, piece.index);

                    for index_t in target_indices {
                        if let Ok(mut square_sprite) = qy_squares.get_mut(squares_ids[index_t]) {
                            square_sprite.color = Color::WHITE;
                        }
                    }
                } else {
                    grab_tool.selected_piece_id = Some(entity);
                    // TODO: Maybe a capture.
                }
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
            let Ok(board_transform) = qy_board.single() else {
                error!("Expected exactly one board, but found no board or more than one!");
                return;
            };
            let cursor = Vec3::new(cursor_world_coords.0.x, cursor_world_coords.0.y, 0.2);
            transform.translation = cursor - board_transform.translation();
        }
    }
}

#[derive(Message, Debug)]
pub struct PieceGrabbedEvent {
    pub board_index: usize,
}

#[derive(Message, Default, Debug)]
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

#[derive(Resource)]
pub struct MoveGenerator<T: MoveGen + std::marker::Send + std::marker::Sync + 'static> {
    pub generator: T,
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
    let Ok(board) = qy_board.single() else {
        return;
    };
    for (square, mut sprite) in qy_squares.iter_mut() {
        let file = square.index % 8;
        let rank = square.index / 8;
        if let Some(_) = board.bitboard.at(square.index) {
            if (file + rank) % 2 == 0 {
                // sprite.color = dark_color * tint;
            } else {
                // sprite.color = light_color * tint;
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

fn setup_camera(mut commands: Commands, qy_window: Query<&Window, With<PrimaryWindow>>) {
    let Ok(window) = qy_window.single() else {
        return;
    };
    let _ar = window.height() / window.width();

    commands.spawn((
        Camera2d,
        Camera::default(),
        Transform::from_xyz(0.0, 0.0, 1.0), // Camera z = 1.0 to see sprites at z = 0.0
    ));
}

fn update_cursor_world_position(
    mut cursor_world_coords: ResMut<CursorWorldCoords>,
    qy_window: Query<&Window, With<PrimaryWindow>>,
    qy_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok((camera, camera_transform)) = qy_camera.single() else {
        return;
    };
    let Ok(window) = qy_window.single() else {
        return;
    };

    if let Some(world_coords) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        cursor_world_coords.0 = world_coords;
    }
}
