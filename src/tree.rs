// use nannou::{rand::random_range, prelude::{Vec2, vec2}};
use nannou::prelude::*;

use crate::lines::{multi::MultiLine, common::Line};

#[derive(Debug)]
pub struct Tree {
    pub trunk: MultiLine,
    pub branches: Vec<Vec2>
}

impl Tree {
    pub fn new(x: f32, start_y: f32, max_height: f32) -> Self {
        let trunk = create_branches(start_y, max_height, x);

        let mut branches = vec![];

        let mut half_tree_height = (trunk.end.pos.y - trunk.start.pos.y) / 2.0;
        half_tree_height += random_range(-10.0, 10.0);

        for i in ((half_tree_height as i32)..(max_height as i32)).step_by(7) {
            branches.push(vec2(x, start_y + i as f32));
        }

        Self { trunk, branches }
    }
}

fn create_branches(start_y: f32, max_height: f32, x: f32) -> MultiLine {
    let y2 = start_y + max_height;
    let value = random_range(0.6, 1.0);
    let weight = random_range(2.0, 6.0);
    let trunk = MultiLine::line(x, start_y, x, y2, weight, value);
    trunk
}
