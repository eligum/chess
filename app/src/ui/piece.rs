use crate::graphics::Graphics;
use crate::ui::Board;
use bevy::{
    prelude::*,
    math::{vec2, vec3},
};
use engine::piece;

#[derive(Component)]
pub struct Piece {
    pub index: usize,
}

pub fn spawn_pieces(
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
