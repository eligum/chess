use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_graphics);
    }
}

#[derive(Resource)]
pub struct Graphics {
    pub piece_theme: (Handle<Image>, Handle<TextureAtlasLayout>),
    pub board_theme: (Color, Color),
    pub indicator_theme: (Handle<Mesh>, Handle<Mesh>, Handle<ColorMaterial>),
}

fn load_graphics(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Piece assets
    let texture_handle = asset_server.load("app/image/pieces_320x107.png");
    let layout_handle = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(107),
        6,
        2,
        None,
        Some(UVec2::new(0, 0)),
    ));
    // Board assets
    let ligth_squares_color = Color::srgb_u8(240, 217, 181);
    let dark_squares_color = Color::srgb_u8(181, 136, 99);
    // Indicator assets
    let circle_mesh_handle = meshes.add(Circle { radius: 5.0 });
    let rectangle_mesh_handle = meshes.add(Rectangle {
        half_size: Vec2::splat(5.0),
    });
    let material_handle = materials.add(ColorMaterial {
        color: Color::srgb_u8(46, 139, 87),
        texture: None,
        ..default()
    });

    commands.insert_resource(Graphics {
        piece_theme: (texture_handle, layout_handle),
        board_theme: (ligth_squares_color, dark_squares_color),
        indicator_theme: (circle_mesh_handle, rectangle_mesh_handle, material_handle),
    });
}
