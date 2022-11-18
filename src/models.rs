use nannou::prelude::*;

use crate::{lines::{common::Line, solid::SolidLine}, tree::Tree};


#[derive(Debug, Clone)]
pub struct Floor {
    pub lines: Vec<SolidLine>,
}

impl Floor {
    pub fn new(num: u8, rect: &Rect) -> Self {
        let mut lines = vec![];

        let padding = 20.0;

        for i in 0..num {
            let y1 = rect.bottom() + padding + ((i as f32 + random::<f32>()) * (i as f32 / 1.15));
            let y2 = rect.bottom() + padding + ((i as f32 + random::<f32>()) * (i as f32 / 1.15));
            let left = rect.left() + padding;
            let right = rect.right() - padding;
            let value = map_range(i as f32 + random::<f32>(), 0 as f32, num as f32, 0.5, 1.0);

            let line = SolidLine::line(left, y1, right, y2, 2.0, value);
            lines.push(line);
        }

        Self { lines }
    }
}

pub struct Model {
    pub window: WindowId,
    pub floor: Floor,
    pub trees: Vec<Tree>,
}

const NUM_FLOOR_LINES: usize = 8;
const NUM_TREES: usize = 8;
const PADDING: f32 = 20.0;

impl Model {
    pub fn new(w: WindowId, rect: Rect) -> Model {
        let floor = Floor::new(NUM_FLOOR_LINES as u8, &rect);
        
        let mut trees = vec![];
        
        for _ in 0..NUM_TREES {
            let rid = random_range(0, NUM_FLOOR_LINES - 1);
            let floor_end = floor.lines[rid].end.y;
            let start = random_range(rect.left() + PADDING, rect.right() - PADDING);
            let height = random_range(rect.h() - 700.0, rect.h() - 300.0);
            let tree = Tree::new(start, floor_end, height);
            trees.push(tree);
        }
        
        Model { window: w, floor, trees }
    }
}
