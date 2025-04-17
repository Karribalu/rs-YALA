use std::process::id;
use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
pub struct Map {
    pub tiles: Vec<TileType>,
}

/**
The tiles are stored in a flattened manner
to find a tile for x,y co-ordinates is (y * SCREEN_WIDTH) + x;
The vice versa to find x,y co-ordinates for an index is
let x = index % SCREEN_WIDTH;
let y = index / SCREEN_WIDTH;
*/
impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Wall => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('#'));
                    }
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                }
            }
        }
    }
}