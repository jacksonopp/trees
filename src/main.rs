use nannou::{prelude::*, color::named};

mod models;
use models::Model;

mod events;
mod lines;

fn main() {
    nannou::app(model).event(event).run()
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(500, 900)
        .view(view)
        .build()
        .unwrap();
    
    let rect = app.window_rect();

    Model::new(window, rect)
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple } => {
            if let Some(event) = simple {
                match event {
                    KeyReleased(Key::S) => events::screenshot(app),
                    _ => ()
                }
            }
        },

        Event::Update(update) => events::update(app, model, update),
        _ => ()
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(named::CORNSILK);
    draw.ellipse()
        .x_y(0.0, 0.0)
        .color(named::GREEN)
        .radius(20.0);

    model.floor.lines.iter().for_each(|line| {
        draw.line()
            .start(line.start)
            .end(line.end)
            .color(Rgba::new(0.0, 0.0, 0.0, line.value));
    });

    draw.to_frame(app, &frame).unwrap();

}