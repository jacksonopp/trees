/// The common trait for drawing a line
pub trait Line {
    fn line(x1: f32, y1: f32, x2: f32, y2: f32, weight: f32, value: f32) -> Self;
}