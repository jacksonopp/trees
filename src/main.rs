use art_models::common::DrawModel;
use nannou::{color::named, prelude::*};

mod models;
use models::Model;

mod art_models;
mod events;
mod lines;

fn main() {
    nannou::app(model).event(event).run()
}

fn model(app: &App) -> Model {
    let window = app.new_window().size(500, 900).view(view).build().unwrap();

    let rect = app.window_rect();

    Model::new(window, rect)
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple } => {
            if let Some(event) = simple {
                match event {
                    KeyReleased(Key::S) => events::screenshot(app),
                    _ => (),
                }
            }
        }

        Event::Update(update) => events::update(app, model, update),
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(named::CORNSILK);

    model.floor.draw(&draw);

    model.trees.iter().for_each(|tree| tree.draw(&draw));

    model.sky.draw(&draw);

    draw.ellipse()
        .x_y(0.0, 0.0)
        .color(named::GREEN)
        .radius(20.0);

    draw.to_frame(app, &frame).unwrap();
}
