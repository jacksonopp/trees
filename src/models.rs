use nannou::prelude::*;

use crate::lines::{common::Line, solid::SolidLine};

pub struct Model {
    pub window: WindowId,
    pub floor: Floor,
}

#[derive(Debug, Clone)]
pub struct Floor {
    pub lines: Vec<SolidLine>,
}

impl Floor {
    pub fn new(num: u8, rect: &Rect) -> Self {
        let mut lines = vec![];

        let padding = 20.0;

        for i in 0..num {
            let y = rect.bottom() + padding + ((i as f32 + random::<f32>()) * (i as f32 / 1.15));
            let left = rect.left() + padding;
            let right = rect.right() - padding;
            let value = map_range(i as f32 + random::<f32>(), 0 as f32, num as f32, 0.5, 1.0);

            let line = SolidLine::line(left, y, right, y, 2.0, value);
            lines.push(line);
        }

        Self { lines }
    }
}

impl Model {
    pub fn new(w: WindowId, rect: Rect) -> Model {
        let floor = Floor::new(8, &rect);
        Model { window: w, floor }
    }
}
