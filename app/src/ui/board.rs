use crate::graphics::Graphics;
use bevy::prelude::*;

type Bitboard = engine::board::Board;

pub const BOARD_SIZE: f32 = 640.0;

#[derive(Component)]
pub struct Board {
    // graphics
    pub center: Vec2,
    pub size: Vec2,
    // internal representation
    pub backend: Bitboard,
}

#[derive(Component, Debug)]
pub struct Square {
    pub index: usize,
}

#[derive(Component, Debug)]
pub struct Indicator {
    pub index: usize,
}

impl Board {
    /// Returns the index of the square located at `position` wrapped in `Some()`,
    /// or `None` if the position (in world coordinates) is outside the board.
    pub fn index_at(&self, position: Vec2) -> Option<usize> {
        let p_min = self.center - self.size / 2.0;
        let p_norm = position - p_min;
        if p_norm.x >= 0.0 && p_norm.x < self.size.x && p_norm.y >= 0.0 && p_norm.y < self.size.y {
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
        first_square + Vec2::new(square_size.x * file as f32, square_size.y * rank as f32)
    }
}

pub fn spawn_board(
    mut commands: Commands,
    graphics: Res<Graphics>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let square_size = Vec2::splat(BOARD_SIZE / 8.0);
    let board_size = Vec2::splat(BOARD_SIZE);
    let board_center = Vec2::new(200.0, 0.0);

    let board_id = commands
        .spawn((
            Board {
                center: board_center,
                size: board_size,
                backend: Bitboard::new(),
            },
            Transform::from_xyz(board_center.x, board_center.y, 0.0),
            // SpatialBundle {
            //     transform: Transform::from_xyz(board_center.x, board_center.y, 0.0),
            //     ..default()
            // },
        ))
        .id();

    // NOTE: Child transforms are relative to their parent's transform. Since in this
    // hierarchy board squares are children of a board entity, their transform remains
    // the same no matter the board position.
    let first_square = Vec2::ZERO - (board_size - square_size) / 2.0;
    let mut square_ids = [Entity::from_raw_u32(0).unwrap(); 64];
    let mut indicator_ids = [Entity::from_raw_u32(0).unwrap(); 64];

    let (light_squares_color, dark_squares_color) = graphics.board_theme;
    let (circle_mesh, _square_mesh, indicator_color) = &graphics.indicator_theme;

    for rank in 0..8 {
        for file in 0..8 {
            let index = rank * 8 + file;
            // Spawn board square
            square_ids[index] = commands
                .spawn((
                    Square { index },
                    Sprite {
                        color: if (rank + file) % 2 == 0 {
                            dark_squares_color
                        } else {
                            light_squares_color
                        },
                        custom_size: Some(square_size),
                        ..default()
                    },
                    Transform {
                        translation: Vec3::new(
                            first_square.x + square_size.x * file as f32,
                            first_square.y + square_size.y * rank as f32,
                            0.0,
                        ),
                        ..default()
                    },
                ))
                .id();
            // Spawn move indicator
            indicator_ids[index] = commands
                .spawn((
                    Indicator { index },
                    Visibility::Visible,
                    Transform {
                        translation: Vec3::new(
                            first_square.x + square_size.x * file as f32,
                            first_square.y + square_size.y * rank as f32,
                            0.2,
                        ),
                        ..default()
                    },
                    Mesh2d(circle_mesh.clone()),
                    MeshMaterial2d(indicator_color.clone()),
                ))
                .id();
        }
    }

    // Construct parent-child hierarchy
    commands
        .entity(board_id)
        .add_children(&square_ids)
        .add_children(&indicator_ids);
}
