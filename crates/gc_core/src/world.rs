use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TileKind {
    Floor,
    Wall,
    Water,
    Lava,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapGenConfig {
    pub width: u32,
    pub height: u32,
}

#[derive(Component, Debug)]
pub struct Position(pub i32, pub i32);

#[derive(Component, Debug, Default)]
pub struct Velocity(pub i32, pub i32);

#[derive(Component, Debug)]
pub struct Name(pub String);

#[derive(Resource, Debug, Clone)]
pub struct GameMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<TileKind>,
}

impl GameMap {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: vec![TileKind::Floor; (width * height) as usize],
        }
    }
    pub fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y) = (x as u32, y as u32);
        if x >= self.width || y >= self.height {
            return None;
        }
        Some((y * self.width + x) as usize)
    }
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as u32) < self.width && (y as u32) < self.height
    }
    pub fn get_tile(&self, x: i32, y: i32) -> Option<TileKind> {
        self.idx(x, y).map(|i| self.tiles[i])
    }
    pub fn set_tile(&mut self, x: i32, y: i32, kind: TileKind) -> bool {
        if let Some(i) = self.idx(x, y) {
            self.tiles[i] = kind;
            true
        } else {
            false
        }
    }
    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        self.get_tile(x, y)
            .map(|t| matches!(t, TileKind::Floor))
            .unwrap_or(false)
    }
}
