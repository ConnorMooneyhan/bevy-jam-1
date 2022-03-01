mod components;
mod systems;
mod map;

mod prelude {
    pub use bevy::prelude::*;
    pub use crate::components::*;
    pub use crate::systems::*;
    pub use crate::map::*;
    pub const NUM_TILES: f32 = 32.0;
    pub const SCREEN_SIDE: f32 = 896.0;
    pub const TILE_SIZE: f32 = SCREEN_SIDE / NUM_TILES;
    pub const TOP: f32 = SCREEN_SIDE / 2.0;
    pub const RIGHT: f32 = SCREEN_SIDE / 2.0;
    pub const BOTTOM: f32 = -SCREEN_SIDE / 2.0;
    pub const LEFT: f32 = -SCREEN_SIDE / 2.0;
}

use prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GREEN))
        .insert_resource(WindowDescriptor {
            width: SCREEN_SIDE,
            height: SCREEN_SIDE,
            title: "Maze".to_string(),
            ..Default::default()
        })
        .add_state(GameState::Playing)
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_enter(GameState::Playing)
            .with_system(setup_cameras)
            .with_system(spawn_map)
        )
        .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Playing
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    for (row_pos, row) in MAP.iter().enumerate() {
        for (col_pos, tile) in row.chars().enumerate() {
            match tile {
                '#' => {
                    commands.spawn_bundle(SpriteBundle {
                        texture: asset_server.load("black_square.png"),
                        transform: Transform {
                            translation: Vec3::new(LEFT + (col_pos as f32 * TILE_SIZE) + (TILE_SIZE / 2.0), TOP - (row_pos as f32 * TILE_SIZE) - (TILE_SIZE / 2.0), 1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                },
                _ => ()
            }
        }
    };
}