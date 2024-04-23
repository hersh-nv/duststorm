// Agents that move following a target, while being pushed around by Perlin noise.

use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::prelude::*;

pub mod pos;
use pos::Pos;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Clone)]
struct Agent {
    pos: Pos,       // (x,y) position
    step_size: f32, // ??
}

impl Agent {
    pub fn update(&mut self, noise: Perlin, noise_scale: f64, target: Pos) {
        // take a fixed step in the noise direction
        let angle = noise.get([
            self.pos.x as f64 / noise_scale,
            self.pos.y as f64 / noise_scale,
        ]) as f32;
        let angle = angle * 2.0 * PI;
        self.pos.x += angle.cos() * self.step_size;
        self.pos.y += angle.sin() * self.step_size;
        // then take a proportional step in the target direction
        let dxy = target - self.pos;
        let dxy = dxy * 0.002; // acceleration factor, tweak for best results
        self.pos.x += dxy.x;
        self.pos.y += dxy.y;
    }
}

struct Model {
    perlin: Perlin,
    noise_seed: u32,
    noise_scale: f64,
    pub agents: Vec<Agent>,
    win: Rect,
}

impl Model {
    pub fn new(win: Rect) -> Self {
        let agent_count = 400;
        let noise_scale = 200.0;
        let noise_seed = random::<u32>();
        let perlin = Perlin::new().set_seed(noise_seed);
        let agents = (0..agent_count)
            .map(|_| Agent {
                pos: Pos::new(
                    random_range(win.left(), win.right()),
                    random_range(win.bottom(), win.top()),
                ),
                step_size: 1f32,
            })
            .collect();

        Model {
            perlin,
            noise_seed,
            noise_scale,
            agents,
            win,
        }
    }

    pub fn reset_agents(&mut self) {
        self.agents = (0..self.agents.len())
            .map(|_| {
                // random r and theta around centre
                Agent {
                    pos: Pos::new(
                        random_range(self.win.left(), self.win.right()),
                        random_range(self.win.bottom(), self.win.top()),
                    ),
                    step_size: 1f32,
                }
            })
            .collect();
    }

    pub fn _agents_pos(&self) -> Vec<Pos> {
        self.agents.iter().map(|agent| agent.pos).collect()
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1000, 1000)
        .view(view)
        .key_released(key_released)
        .build()
        .unwrap();
    Model::new(app.window_rect())
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model
        .agents
        .iter_mut()
        .for_each(|a| a.update(model.perlin, model.noise_scale, Pos::new(0.0, 0.0)));
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    // draw the newest agent set
    model.agents.iter().for_each(|agent| {
        draw.ellipse()
            .x_y(agent.pos.x, agent.pos.y)
            .radius(0.5)
            .color(WHITE);
    });
    draw.to_frame(&app, &frame).unwrap();
}

fn key_released(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.noise_seed = (random_f32() * 10000.0).floor() as u32;
            model.perlin = Perlin::new().set_seed(model.noise_seed);
        }
        Key::R => {
            model.reset_agents();
        }
        _other_key => {}
    }
}
