// Draw a collection of voronoi cells, animated while their sites randomly
// wander around.

// There's some mildly annoying mixing of float sizes back and forth here,
// because the voronoice lib uses f64s while nannou and rust's math constants
// use f32s.

use nannou::prelude::*;
use voronoice::*;

pub mod pos;
use pos::Pos;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Clone)]
struct Agent {
    pos: Pos,
    angle: f64,
    step_size: f64,
}

impl Agent {
    fn update1(&mut self, win: Rect) {
        // velocity random walk
        // add a little noise to the angle
        self.angle += 0.2 * random_range(-1.0, 1.0);
        // step the position
        self.pos.x += self.angle.cos() * self.step_size;
        self.pos.y += self.angle.sin() * self.step_size;
        // turn around if approaching the edges
        if self.pos.x + self.angle.cos() * self.step_size * 20.0 < win.left().into() {
            self.angle = PI as f64 - self.angle;
        }
        if self.pos.x + self.angle.cos() * self.step_size * 20.0 > win.right().into() {
            self.angle = PI as f64 - self.angle;
        }
        if self.pos.y + self.angle.sin() * self.step_size * 20.0 < win.bottom().into() {
            self.angle = -self.angle;
        }
        if self.pos.y + self.angle.sin() * self.step_size * 20.0 > win.top().into() {
            self.angle = -self.angle;
        }
    }

    fn update2(&mut self, win: Rect, agent_pos_vec: Vec<Point>) {}
}

enum UpdateMode {
    One,
    Two,
}

struct Model {
    agent_count: i32,
    agents: Vec<Agent>,
    voronoi: Voronoi,
    win: Rect,
    update_mode: UpdateMode,
}

impl Model {
    fn new(win: Rect) -> Self {
        let agent_count = 50;
        let agents: Vec<Agent> = Model::build_agents(agent_count, win);
        let voronoi =
            Model::build_voronoi(agents.clone().into_iter().map(|a| a.pos).collect(), win);
        let update_mode = UpdateMode::One;

        Model {
            agent_count,
            agents,
            voronoi,
            win,
            update_mode,
        }
    }

    fn build_agents(agent_count: i32, win: Rect) -> Vec<Agent> {
        (0..agent_count)
            .map(|_| Agent {
                pos: Pos::new(
                    random_range(win.left().into(), win.right().into()),
                    random_range(win.bottom().into(), win.top().into()),
                ),
                angle: random_range(-PI as f64, PI as f64),
                step_size: 0.3,
            })
            .collect()
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
    app.new_window()
        .size(1000, 1000)
        .view(view)
        .key_released(key_released)
        .build()
        .unwrap();
    let win = app.window_rect();
    Model::new(win)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // update agents
    model
        .agents
        .iter_mut()
        .for_each(|agent| match model.update_mode {
            UpdateMode::One => agent.update1(model.win),
            UpdateMode::Two => agent.update2(model.win),
        });
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

fn key_released(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::T => {
            model.update_mode = match model.update_mode {
                UpdateMode::One => UpdateMode::Two,
                UpdateMode::Two => UpdateMode::One,
            }
        }
        Key::R => model.agents = Model::build_agents(model.agent_count, model.win),
        _other_key => {}
    }
}
