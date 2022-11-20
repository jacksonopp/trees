use crate::lines::{solid::SolidLine, common::Line};
use nannou::prelude::*;

use super::common::DrawModel;


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

impl DrawModel for Floor {
    fn draw(&self, draw: &Draw) {
        self.lines.iter().for_each(|line| {
            line.draw(draw)
        });
    }
}