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

const CIRCLE: Circle = Circle { radius: 5.0 };
const RECTANGLE: Rectangle = Rectangle {
    half_size: Vec2::splat(5.0),
};

fn load_graphics(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Piece assets
    let texture_handle = asset_server.load("app/image/pieces_320x107.png");
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
    // Indicators
    let circle_mesh_handle = asset_server.add(CIRCLE.mesh().build());
    let rectangle_mesh_handle = asset_server.add(RECTANGLE.mesh().build());
    let material_handle = asset_server.add(ColorMaterial {
        color: Color::SEA_GREEN,
        texture: None,
    });

    commands.insert_resource(Graphics {
        piece_theme: (texture_handle, layout_handle),
        board_theme: (ligth_squares_color, dark_squares_color),
        indicator_theme: (circle_mesh_handle, rectangle_mesh_handle, material_handle),
    })
}
