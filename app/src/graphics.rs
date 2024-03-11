use bevy::prelude::*;

pub fn asset_loading_plugin(app: &mut App) {
    app.add_systems(PreStartup, load_graphics);
}

#[derive(Resource)]
pub struct Graphics {
    pub piece_theme: (Handle<Image>, Handle<TextureAtlasLayout>),
    pub board_theme: (Color, Color),
}

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
