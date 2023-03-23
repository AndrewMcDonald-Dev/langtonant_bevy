use bevy::{
    prelude::{Entity, Resource},
    utils::HashMap,
};

use crate::components::Coordinates;
use std::ops::{Deref, DerefMut};
#[derive(Debug, Clone)]
pub struct TileMap {
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
    pub covered_tiles: HashMap<Coordinates, Entity>,
    sim_size: u16
}

#[derive(Debug, Clone)]
pub enum Tile {
    White,
    Black,
}

impl TileMap {
    pub fn empty(width: u16, height: u16, sim_size: u16) -> Self {
        let map = (0..height)
            .map(|_| (0..width).map(|_| Tile::White).collect())
            .collect();

        Self {
            height,
            width,
            map,
            covered_tiles: HashMap::new(),
            sim_size
        }
    }

    pub fn toggle_tile(&mut self, coordinates: Coordinates) {
        let temp = self.map[coordinates.y as usize][coordinates.x as usize].clone();

        self.map[coordinates.y as usize][coordinates.x as usize] = match temp {
            Tile::White => Tile::Black,
            Tile::Black => Tile::White,
        }
    }

    pub fn at(&self, coordinates: Coordinates) -> Tile {
        self.map[coordinates.y as usize][coordinates.x as usize].clone()
    }

    pub fn width(&self) -> u16 {
        self.width
    }
    pub fn height(&self) -> u16 {
        self.height
    }
    pub fn sim_size(&self) -> u16 {
        self.sim_size
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl Resource for TileMap {}
