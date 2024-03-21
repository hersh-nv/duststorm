use nannou::prelude::*;
use voronoice::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

#[derive(Clone)]
struct Agent {
    pos: Point,
    angle: f64,
}

impl Agent {
    fn update1(&mut self) {
        // velocity random walk
        // add a little noise to the angle
        self.angle += 0.2 * random_range(-1.0, 1.0);
        // step the position
        self.pos.x += self.angle.cos();
        self.pos.y += self.angle.sin();
        // wrap around the position // todo
    }

    fn update2(&mut self) {
        // position random walk
        // step the position
        self.pos.x += 2.5 * random_range(-1.0, 1.0);
        self.pos.y += 2.5 * random_range(-1.0, 1.0);
        // wrap around the position // todo
    }
}

struct Model {
    agents: Vec<Agent>,
    voronoi: Voronoi,
    win: Rect,
}

impl Model {
    fn new(win: Rect) -> Self {
        let agent_count = 200;
        let agents: Vec<Agent> = (0..agent_count)
            .map(|_| Agent {
                pos: Point {
                    x: random_range(win.left().into(), win.right().into()),
                    y: random_range(win.bottom().into(), win.top().into()),
                },
                angle: random_range(-PI as f64, PI as f64),
            })
            .collect();

        let voronoi =
            Model::build_voronoi(agents.clone().into_iter().map(|a| a.pos).collect(), win);

        Model {
            agents,
            voronoi,
            win,
        }
    }

    fn get_sites(&self) -> Vec<&Point> {
        self.agents.iter().map(|a| &a.pos).collect()
    }

    fn build_voronoi(sites: Vec<Point>, win: Rect) -> Voronoi {
        VoronoiBuilder::default()
            .set_sites(sites)
            .set_bounding_box(BoundingBox::new_centered(win.w().into(), win.h().into()))
            .build()
            .expect("Provided sites don't generate a valid voronoi graph")
    }

    fn rebuild_voronoi(&mut self) {
        self.voronoi = Model::build_voronoi(
            self.agents.clone().into_iter().map(|a| a.pos).collect(),
            self.win,
        );
    }
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    Model::new(win)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // update agents
    model.agents.iter_mut().for_each(|agent| agent.update1());
    // redraw voronoi cells
    model.rebuild_voronoi();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    // draw points
    model.get_sites().iter().for_each(|site| {
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
