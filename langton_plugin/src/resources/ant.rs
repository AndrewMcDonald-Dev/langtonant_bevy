use bevy::prelude::Resource;

use crate::Coordinates;

use super::tile_map::Tile;
#[derive(Debug, Clone, Resource)]
pub struct Ant {
    pub coordinates: Coordinates,
    pub direction: Direction,
}

impl Ant {
    pub fn change_direction(&mut self, tile: Tile) {
        self.direction = match tile {
            Tile::White => match self.direction {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            },
            Tile::Black => match self.direction {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            },
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
