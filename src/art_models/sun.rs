use nannou::prelude::*;

use super::common::DrawModel;

pub struct Sun {
    pos: Point2,
    color: Hsla,
    radius: f32,
}

impl Sun {
    pub fn new(radius: f32, rect: &Rect) -> Self {
        let min_x = rect.left() + 20.0;
        let max_x = rect.right() - 20.0;

        let x = random_range(min_x, max_x);
        let y = random_range(-300.0, 300.0);
        let pos = vec2(x, y);

        let mut hue = map_range(pos.y, -300.0, 300.0, 13.0, 50.0);
        hue = hue / 360.0;
        let color = hsla(hue, 1.0, 0.5, 1.0);
        Self { pos, color, radius }
    }
}

impl DrawModel for Sun {
    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.pos)
            .radius(self.radius)
            .color(self.color);
    }
}