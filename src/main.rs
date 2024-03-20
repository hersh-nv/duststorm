use nannou::prelude::*;
use voronoice::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    sites: Vec<voronoice::Point>,
}

fn model(app: &App) -> Model {
    let site_count = 50;
    let sites = (0..site_count)
        .map(|_| voronoice::Point {
            x: random_range(
                app.window_rect().left().into(),
                app.window_rect().right().into(),
            ),
            y: random_range(
                app.window_rect().left().into(),
                app.window_rect().right().into(),
            ),
        })
        .collect();
    Model { sites }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.sites.iter().for_each(|site| {
        draw.ellipse()
            .x_y(site.x as f32, site.y as f32)
            .radius(1.0)
            .color(WHITE);
    });
    draw.to_frame(app, &frame).unwrap();
}
