use nannou::prelude::*;

use crate::art_models::sky::Sky;
use crate::art_models::{tree::Tree, floor::Floor};


pub struct Model {
    pub window: WindowId,
    pub floor: Floor,
    pub trees: Vec<Tree>,
    pub sky: Sky,
}

const NUM_FLOOR_LINES: usize = 8;
const NUM_TREES: usize = 30;
const PADDING: f32 = 20.0;

impl Model {
    pub fn new(w: WindowId, rect: Rect) -> Model {
        let floor = Floor::new(NUM_FLOOR_LINES as u8, &rect);
        
        let mut trees = vec![];
        
        for _ in 0..NUM_TREES {
            let rid = random_range(0, NUM_FLOOR_LINES - 1);
            let floor_end = floor.lines[rid].end.y;
            let start = random_range(rect.left() + PADDING, rect.right() - PADDING);
            let height = random_range(rect.h() - 800.0, rect.h() - 700.0);
            let tree = Tree::new(start, floor_end, height, 15.0);
            trees.push(tree);
        }

        let floor_end = floor.lines[NUM_FLOOR_LINES - 1].end.y;

        // let sky = Sky::new(rect.bottom() + PADDING, &rect);
        let sky = Sky::new(floor_end + 30.0, 5, &rect);
        
        Model { window: w, floor, trees, sky }
    }
}