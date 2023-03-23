pub mod components;
pub mod resources;
mod systems;
use bevy::prelude::{App, Commands, Plugin};
use components::Coordinates;
use resources::ant::{Ant, Direction};
use resources::tile_map::TileMap;

pub struct LangtonPlugin;

impl Plugin for LangtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board)
            .add_system(systems::ant::ant_movement);
    }
}

impl LangtonPlugin {
    pub fn create_board(mut commands: Commands) {
        let tile_map = TileMap::empty(1000, 1000);

        commands.insert_resource(tile_map);
        commands.insert_resource(Ant {
            coordinates: Coordinates { x: 500, y: 500 },
            direction: Direction::North,
        });
    }
}
