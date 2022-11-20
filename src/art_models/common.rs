use nannou::prelude::*;

/// A common trait for drawing the object in a model
pub trait DrawModel {
    fn draw(&self, draw: &Draw);
}