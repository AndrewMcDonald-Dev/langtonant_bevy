use bevy::{log, prelude::*};

use crate::{
    components::Coordinates,
    resources::{ant::Ant, ant::Direction, tile_map::Tile, tile_map::TileMap},
};

pub fn ant_movement(mut tile_map: ResMut<TileMap>, mut ant: ResMut<Ant>, mut commands: Commands) {
    // Change direction of ant
    let coordinates = ant.coordinates;
    let tile = tile_map.at(coordinates);
    ant.change_direction(tile.clone());
    let tile_map_height = tile_map.height();
    let tile_map_width = tile_map.width();
    let tile_map_sim = tile_map.sim_size();

    //Toggle color of Sprite
    match tile {
        Tile::White => {
            let entity = commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::splat(tile_map_sim as f32)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(coordinates.x  as f32 - (tile_map.width() / 2) as f32, coordinates.y as f32 - (tile_map.height() / 2) as f32, 2.),
                    ..Default::default()
                })
                .insert(Name::new(format!(
                    "Tile ({}, {})",
                    coordinates.x, coordinates.y
                )))
                .id();
            tile_map.covered_tiles.insert(coordinates, entity);
        }
        Tile::Black => {
            let entity = tile_map.covered_tiles.get(&coordinates);
            match entity {
                Some(e) => commands.entity(*e).despawn(),
                None => log::error!("Bad"),
            }
        }
    }
    tile_map.toggle_tile(coordinates);

    //Move ant
    match ant.direction {
        Direction::North => {
            ant.coordinates = Coordinates {
                x: ant.coordinates.x,
                y: (ant.coordinates.y + tile_map_sim + tile_map_height) % tile_map_height,
            }
        }
        Direction::South => {
            ant.coordinates = Coordinates {
                x: ant.coordinates.x,
                y: (ant.coordinates.y + tile_map_height- tile_map_sim) % tile_map_height,
            }
        }
        Direction::East => {
            ant.coordinates = Coordinates {
                x: (ant.coordinates.x + tile_map_sim + tile_map_width) % tile_map_width,
                y: ant.coordinates.y,
            }
        }
        Direction::West => {
            ant.coordinates = Coordinates {
                x: (ant.coordinates.x + tile_map_width - tile_map_sim) % tile_map_width,
                y: ant.coordinates.y,
            }
        }
    }
}
