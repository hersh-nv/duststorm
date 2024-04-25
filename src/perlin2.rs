// Agents that move following a target, while being pushed around by Perlin noise.

use nannou::noise::{Perlin, Seedable};
use nannou::prelude::*;

pub mod agent;
pub mod pos;
use agent::Agent;
use pos::Pos;

fn main() {
    nannou::app(model).update(update).run();
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
        let agent_count = 1000;
        let noise_scale = 800.0;
        let noise_seed = random::<u32>();
        let perlin = Perlin::new().set_seed(noise_seed);
        let agents = (0..agent_count)
            .map(|_| Agent::new(win, false, 8f32))
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
            .map(|_| Agent::new(self.win, false, 8f32))
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
    model.agents.iter_mut().for_each(|a| {
        a.update2(
            model.perlin,
            model.noise_scale,
            Pos::new(0.0, 0.0),
            model.win,
            app.duration,
        )
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.rect()
        .wh(app.window_rect().wh())
        .hsva(0.0, 0.0, 0.0, 0.02);
    // draw the newest agent set
    model.agents.iter().for_each(|agent| {
        draw.line()
            .start(Vec2::new(agent.prev_pos.x, agent.prev_pos.y))
            .end(Vec2::new(agent.pos.x, agent.pos.y))
            .weight(1.5)
            .hsv(0.5 + agent.ttl / 20.0, 1.0, 1.0);
    });
    draw.to_frame(&app, &frame).unwrap();
}

fn key_released(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.noise_seed = (random_f32() * 10000.0).floor() as u32;
            model.perlin = Perlin::new().set_seed(model.noise_seed);
        }
        Key::R => {
            model.win = app.window_rect();
            model.reset_agents();
        }
        _other_key => {}
    }
}
