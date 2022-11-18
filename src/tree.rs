use nannou::rand::random_range;

use crate::lines::{multi::MultiLine, common::Line};

#[derive(Debug)]
pub struct Tree {
    pub trunk: MultiLine,
}

impl Tree {
    pub fn new(x: f32, start_y: f32, max_height: f32) -> Self {
        let y2 = start_y + max_height;        
        let value = random_range(0.6, 1.0);
        let weight = random_range(6.0, 15.0);

        let trunk = MultiLine::line(x, start_y, x, y2, weight, value);
        Self { trunk }
    }
}
