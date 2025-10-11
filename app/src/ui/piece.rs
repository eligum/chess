use crate::graphics::Graphics;
use crate::ui::Board;
use bevy::prelude::*;
use engine::piece;

#[derive(Component)]
pub struct Piece {
    /// Determines the position of the piece on the board.
    pub index: usize,
    pub backend: piece::Piece,
}

pub fn spawn_pieces(
    mut commands: Commands,
    graphics: Res<Graphics>,
    qy_board: Query<(Entity, &Board)>,
) {
    let (ref texture, ref layout) = graphics.piece_theme;
    let Ok((board_id, board)) = qy_board.single() else {
        error!("Expected exactly one board, but found no board or more than one!");
        return;
    };

    let square_size = board.size / 8.0;
    let first_square = Vec2::ZERO - (board.size - square_size) / 2.0;
    let mut piece_ids: Vec<Entity> = Vec::with_capacity(32);

    for rank in 0..8 {
        for file in 0..8 {
            let index = rank * 8 + file;
            if let Some(piece_type) = board.bitboard.at(index) {
                info!("At index {} found {:?}", index, piece_type);
                piece_ids.push(
                    commands
                        .spawn((
                            Piece {
                                index,
                                backend: piece_type,
                            },
                            Visibility::Visible,
                            Transform {
                                translation: Vec3::new(first_square.x, first_square.y, 0.0)
                                    + Vec3::new(
                                        square_size.x * file as f32,
                                        square_size.y * rank as f32,
                                        0.1,
                                    ),
                                ..default()
                            },
                            Sprite {
                                custom_size: Some(square_size),
                                image: texture.clone(),
                                texture_atlas: Some(TextureAtlas {
                                    layout: layout.clone(),
                                    index: match piece_type {
                                        piece::Piece::King(color) => match color {
                                            piece::Color::White => 0,
                                            piece::Color::Black => 6,
                                        },
                                        piece::Piece::Queen(color) => match color {
                                            piece::Color::White => 1,
                                            piece::Color::Black => 7,
                                        },
                                        piece::Piece::Bishop(color) => match color {
                                            piece::Color::White => 2,
                                            piece::Color::Black => 8,
                                        },
                                        piece::Piece::Knight(color) => match color {
                                            piece::Color::White => 3,
                                            piece::Color::Black => 9,
                                        },
                                        piece::Piece::Rook(color) => match color {
                                            piece::Color::White => 4,
                                            piece::Color::Black => 10,
                                        },
                                        piece::Piece::Pawn(color) => match color {
                                            piece::Color::White => 5,
                                            piece::Color::Black => 11,
                                        },
                                    },
                                }),
                                ..default()
                            },
                        ))
                        .id(),
                );
            }
        }
    }

    commands.entity(board_id).add_children(&piece_ids);
}
