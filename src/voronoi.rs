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
    angle: f32,
    step_size: f32,
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
            self.angle = PI - self.angle;
        }
        if self.pos.x + self.angle.cos() * self.step_size * 20.0 > win.right().into() {
            self.angle = PI - self.angle;
        }
        if self.pos.y + self.angle.sin() * self.step_size * 20.0 < win.bottom().into() {
            self.angle = -self.angle;
        }
        if self.pos.y + self.angle.sin() * self.step_size * 20.0 > win.top().into() {
            self.angle = -self.angle;
        }
    }

    fn update2(&mut self, win: Rect, sites_vec: &Vec<Pos>) {
        let mut next_pos = self.pos;

        // repel from every site with inverse square power
        for site in sites_vec.iter() {
            let dxy = *site - self.pos;
            // sites vec includes current agent; skip if match
            if dxy.x.abs() < std::f32::EPSILON && dxy.y.abs() < std::f32::EPSILON {
                continue;
            }
            let force = dxy.magnitude().pow(-2.0);
            let scalar = 10.0;
            next_pos = next_pos - dxy * force * scalar;
        }

        // then repel from bounds
        let bounds: Vec<Pos> = vec![
            Pos::new(win.left(), self.pos.y),   // left
            Pos::new(win.right(), self.pos.y),  // right
            Pos::new(self.pos.x, win.top()),    // top
            Pos::new(self.pos.x, win.bottom()), // bottom
        ];
        for bound in bounds.iter() {
            let dxy = *bound - self.pos;
            let force = dxy.magnitude().pow(-2.0);
            let scalar = 50.0;
            next_pos = next_pos - dxy * force * scalar;
        }

        self.pos = next_pos;
    }
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
        let agent_count = 40;
        let agents: Vec<Agent> = Model::build_agents(agent_count, win);
        let voronoi = Model::build_voronoi(
            agents
                .clone()
                .into_iter()
                .map(|a| Point {
                    x: a.pos.x as f64,
                    y: a.pos.y as f64,
                })
                .collect(),
            win,
        );
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
        let pad = 50.0;
        (0..agent_count)
            .map(|_| Agent {
                pos: Pos::new(
                    random_range(win.left() + pad, win.right() - pad),
                    random_range(win.bottom() + pad, win.top() - pad),
                ),
                angle: random_range(-PI, PI),
                step_size: 0.3,
            })
            .collect()
    }

    fn get_sites(&self) -> Vec<Pos> {
        self.agents.iter().map(|a| a.pos).collect()
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
            self.agents
                .clone()
                .into_iter()
                .map(|a| Point {
                    x: a.pos.x as f64,
                    y: a.pos.y as f64,
                })
                .collect(),
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
    let sites = model.get_sites();
    model
        .agents
        .iter_mut()
        .for_each(|agent| match model.update_mode {
            UpdateMode::One => agent.update1(model.win),
            UpdateMode::Two => agent.update2(model.win, &sites),
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
        draw.polyline()
            .weight(1.0)
            .points_closed(cell2)
            .color(WHITE);
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
