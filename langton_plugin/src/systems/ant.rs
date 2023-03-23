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

    //Toggle color of Sprite
    match tile {
        Tile::White => {
            let entity = commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::splat(2.)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(coordinates.x  as f32 - 500., coordinates.y as f32 - 500., 2.),
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
                y: (ant.coordinates.y + 2 + 1000) % 1000,
            }
        }
        Direction::South => {
            ant.coordinates = Coordinates {
                x: ant.coordinates.x,
                y: (ant.coordinates.y + 1000 - 2) % 1000,
            }
        }
        Direction::East => {
            ant.coordinates = Coordinates {
                x: (ant.coordinates.x + 2 + 1000) % 1000,
                y: ant.coordinates.y,
            }
        }
        Direction::West => {
            ant.coordinates = Coordinates {
                x: (ant.coordinates.x + 1000 - 2) % 1000,
                y: ant.coordinates.y,
            }
        }
    }
}
