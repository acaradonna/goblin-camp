use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// World Representation and Core Spatial Components
///
/// This module defines the basic spatial structure of the game world,
/// including the tile-based map system and fundamental positioning components.
/// Enumeration of different tile types that can exist in the game world
/// Each tile type has different properties for pathfinding and interaction
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TileKind {
    /// Walkable floor tiles that entities can move through
    /// Created by mining or as part of initial world generation
    Floor,
    /// Solid wall tiles that block movement and can be mined
    /// Primary target for mining operations to create floors and items
    Wall,
    /// Water tiles (future feature for fluids simulation)
    /// Currently unused but reserved for water mechanics
    Water,
    /// Lava tiles (future feature for fluids and temperature)
    /// Currently unused but reserved for lava mechanics and danger
    Lava,
}

/// Configuration structure for map generation
/// Contains parameters needed to generate new game maps
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapGenConfig {
    /// Width of the map in tiles
    pub width: u32,
    /// Height of the map in tiles
    pub height: u32,
}

/// Component representing the 2D position of an entity in the world
/// Uses integer coordinates aligned with the tile grid
/// Position (0, 0) is typically the top-left corner
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position(pub i32, pub i32);

/// Component representing the velocity/movement direction of an entity
/// Currently used minimally but reserved for movement systems
/// Values represent delta movement per simulation step
#[derive(Component, Debug, Default)]
pub struct Velocity(pub i32, pub i32);

/// Component providing a human-readable name for entities
/// Used for debugging, logging, and future UI display
#[derive(Component, Debug)]
pub struct Name(pub String);

/// Resource representing the game world as a 2D tile-based map
/// This is the primary spatial representation of the game world,
/// storing all terrain and structural information
#[derive(Resource, Debug, Clone)]
pub struct GameMap {
    /// Width of the map in tiles
    pub width: u32,
    /// Height of the map in tiles
    pub height: u32,
    /// Flat vector storing all tiles in row-major order
    /// Index calculation: y * width + x
    pub tiles: Vec<TileKind>,
}

impl GameMap {
    /// Create a new map filled with floor tiles
    /// This is the basic constructor for an empty, walkable map
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: vec![TileKind::Floor; (width * height) as usize],
        }
    }

    /// Convert 2D coordinates to a 1D index into the tiles vector
    /// Returns None if coordinates are out of bounds
    /// Uses row-major ordering: index = y * width + x
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

    /// Check if the given coordinates are within the map bounds
    /// Returns true if (x, y) is a valid position on this map
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as u32) < self.width && (y as u32) < self.height
    }

    /// Get the tile type at the specified coordinates
    /// Returns None if coordinates are out of bounds
    pub fn get_tile(&self, x: i32, y: i32) -> Option<TileKind> {
        self.idx(x, y).map(|i| self.tiles[i])
    }

    /// Set the tile type at the specified coordinates
    /// Returns true if the tile was successfully set, false if out of bounds
    pub fn set_tile(&mut self, x: i32, y: i32, kind: TileKind) -> bool {
        if let Some(i) = self.idx(x, y) {
            self.tiles[i] = kind;
            true
        } else {
            false
        }
    }

    /// Check if a tile can be walked through by entities
    /// Currently only Floor tiles are walkable
    /// Returns false for out-of-bounds coordinates
    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        self.get_tile(x, y)
            .map(|t| matches!(t, TileKind::Floor))
            .unwrap_or(false)
    }
}
