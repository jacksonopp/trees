use crate::lines::{common::Line, solid::SolidLine};
use nannou::prelude::*;

use super::common::DrawModel;

pub struct SkyLine {
    points: Vec<Point2>,
    value: f32,
}

impl SkyLine {
    pub fn new(y_start: f32, amplitude: f32, frequency: f32, rect: &Rect) -> Self {
        let mut points = vec![];

        let i_start = (rect.left() + 20.0) as isize;
        let i_end = (rect.right() - 20.0) as isize;

        for i in i_start..i_end {
            let x = (i as f32) / frequency;
            let mut y = x.sin() * amplitude;
            y += random_range(-1.0, 1.0);

            points.push(vec2(i as f32, y_start + y));
        }

        let mut value = map_range(y_start, rect.bottom(), rect.top(), 0.3, 2.0);
        value = value.max(1.0);

        Self { points, value }
    }
}

pub struct Sky {
    sky_lines: Vec<SkyLine>,
}

impl Sky {
    pub fn new(y_start: f32, density: usize, rect: &Rect) -> Self {
        let mut sky_lines = vec![];
        let top = rect.top();

        for i in ((y_start as i32)..((top as i32) - 20)).step_by(density) {
            let sl = SkyLine::new(i as f32, 2.0, 3.0, rect);
            sky_lines.push(sl);
        }

        Self { sky_lines }
    }
}

impl DrawModel for Sky {
    fn draw(&self, draw: &Draw) {
        self.sky_lines.iter().for_each(|l| {
            let points = l.points.iter().map(|p| *p);

            draw.polyline()
                .weight(1.0)
                .points(points)
                .color(hsla(193.0 / 360.0, 1.0, 0.5, l.value));
        })
    }
}
