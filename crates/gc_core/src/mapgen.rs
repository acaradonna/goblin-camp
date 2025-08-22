use crate::world::{GameMap, TileKind};
use noise::{Fbm, NoiseFn, Seedable};

pub struct MapGenerator {
    pub seed: u32,
}

impl MapGenerator {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn generate(&self, width: u32, height: u32) -> GameMap {
        let mut map = GameMap::new(width, height);
        let fbm = Fbm::<noise::SuperSimplex>::new(0).set_seed(self.seed);
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                let nx = x as f64 / width as f64 - 0.5;
                let ny = y as f64 / height as f64 - 0.5;
                let e = fbm.get([nx * 3.0, ny * 3.0]);
                let kind = if e < -0.2 {
                    TileKind::Water
                } else if e > 0.6 {
                    TileKind::Wall
                } else {
                    TileKind::Floor
                };
                if let Some(i) = map.idx(x, y) {
                    map.tiles[i] = kind;
                }
            }
        }
        map
    }
}
