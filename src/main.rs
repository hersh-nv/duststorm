use nannou::prelude::*;
use voronoice::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    sites: Vec<Point>,
    voronoi: Voronoi,
}

fn model(app: &App) -> Model {
    let winrect = app.window_rect();

    let site_count = 50;
    let sites: Vec<Point> = (0..site_count)
        .map(|_| Point {
            x: random_range(winrect.left().into(), winrect.right().into()),
            y: random_range(winrect.left().into(), winrect.right().into()),
        })
        .collect();

    let voronoi = VoronoiBuilder::default()
        .set_sites(sites.clone())
        .set_bounding_box(BoundingBox::new_centered(
            winrect.w().into(),
            winrect.h().into(),
        ))
        .build()
        .expect("Provided sites don't generate a valid voronoi graph");

    Model { sites, voronoi }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    // draw points
    model.sites.iter().for_each(|site| {
        draw.ellipse()
            .x_y(site.x as f32, site.y as f32)
            .radius(1.0)
            .color(WHITE);
    });
    // draw cell bounds
    model.voronoi.iter_cells().for_each(|cell| {
        // cell verts are in Points which can't Into a Vec2, stupidly
        // so copy the cell and manually convert it ..?
        let cell2: Vec<Vec2> = cell
            .clone()
            .iter_vertices()
            .map(|vert| Vec2::new(vert.x as f32, vert.y as f32))
            .collect();
        draw.polyline().weight(1.0).points(cell2).color(WHITE);
    });
    draw.to_frame(app, &frame).unwrap();
}
