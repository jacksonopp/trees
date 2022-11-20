// use nannou::{rand::random_range, prelude::{Vec2, vec2}};
use nannou::prelude::*;

use crate::lines::{multi::MultiLine, common::Line};

use super::common::DrawModel;

#[derive(Debug)]
pub struct Tree {
    pub trunk: MultiLine,
    pub branches: Vec<MultiLine>
}

impl Tree {
    pub fn new(x: f32, start_y: f32, max_height: f32, branch_len: f32) -> Self {
        let trunk = create_trunk(start_y, max_height, x);

        let branches = create_branches(&trunk, max_height, x, start_y, branch_len);

        Self { trunk, branches }
    }
}

impl DrawModel for Tree {
    fn draw(&self, draw: &Draw) {
        self.trunk.lines.iter().for_each(|line| {
            line.draw(draw);
        });

        self.branches.iter().for_each(|branch| {
            branch.draw(draw);
        })
    }
}

fn create_branches(trunk: &MultiLine, max_height: f32, x: f32, start_y: f32, branch_len: f32) -> Vec<MultiLine> {
    let mut branches = vec![];
    let mut half_tree_height = (trunk.end.pos.y - trunk.start.pos.y) / 2.0;
    half_tree_height += random_range(-10.0, 10.0);

    for i in ((half_tree_height as i32)..(max_height as i32)).step_by(7) {
        let mut branch_start_y = start_y + i as f32;
        let mut branch_end_y = branch_start_y - 3.0;

        branch_start_y += random_range(-1.0, 1.0);
        branch_end_y += random_range(-1.0, 1.0);

        let left_branch_len = branch_len + random_range(-2.0, 2.0);
        let right_branch_len = branch_len + random_range(-2.0, 2.0);

        let left_branch = MultiLine::line(x, branch_start_y, x - left_branch_len, branch_end_y, 1.0, 1.0);
        branches.push(left_branch);
        let right_branch = MultiLine::line(x, branch_start_y, x + right_branch_len, branch_end_y, 1.0, 1.0);
        branches.push(right_branch);
    }

    branches
}

fn create_trunk(start_y: f32, max_height: f32, x: f32) -> MultiLine {
    let y2 = start_y + max_height;
    let value = random_range(0.6, 1.0);
    let weight = random_range(2.0, 6.0);
    let trunk = MultiLine::line(x, start_y, x, y2, weight, value);
    trunk
}
