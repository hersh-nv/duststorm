// Agents that move following a target, while being pushed around by Perlin noise.

use std::collections::VecDeque;

use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Copy, Clone)]
struct Pos {
    x: f32,
    y: f32,
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Pos { x, y }
    }

    pub fn radius(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl std::ops::Sub<Pos> for Pos {
    type Output = Pos;
    fn sub(self, rhs: Pos) -> Self::Output {
        Pos::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Mul<f32> for Pos {
    type Output = Pos;
    fn mul(self, rhs: f32) -> Self::Output {
        Pos::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Div<f32> for Pos {
    type Output = Pos;
    fn div(self, rhs: f32) -> Self::Output {
        Pos::new(self.x / rhs, self.y / rhs)
    }
}

#[derive(Clone)]
struct Agent {
    pos: Pos,       // (x,y) position
    step_size: f32, // ??
    z_offset: f32,
}

impl Agent {
    pub fn update(&mut self, noise: Perlin, target: Pos, noise_scale: f64) {
        // take a fixed step in the noise direction
        let angle = noise.get([
            self.pos.x as f64 / noise_scale,
            self.pos.y as f64 / noise_scale,
            self.z_offset as f64,
        ]) as f32;
        let angle = angle * 2.0 * PI;
        self.pos.x += angle.cos() * self.step_size;
        self.pos.y += angle.sin() * self.step_size;
        // then take a proportional step in the target direction
        let dxy = target - self.pos;
        let dxy = dxy * 0.03; // acceleration factor, tweak for best results
        self.pos.x += dxy.x;
        self.pos.y += dxy.y;

        // lastly - push z offset a bit so we're constantly sliding up the x axis of the noise space
        self.z_offset += 0.02;
    }
}

enum DrawMode {
    Circle,
    FigureEight,
    Average,
    Mouse,
}

struct Model {
    perlin: Perlin,
    noise_seed: u32,
    noise_scale: f64,
    pub agents: Vec<Agent>,
    win: Rect,
    pub target: Pos,
    pub draw_mode: DrawMode,
    pub draw_target: bool,
    pub agents_history: VecDeque<Vec<Agent>>,
}

impl Model {
    pub fn new(win: Rect) -> Self {
        let agent_count = 40;
        let noise_scale = 400.0;
        let noise_seed = random::<u32>();
        let perlin = Perlin::new().set_seed(noise_seed);
        let agents = (0..agent_count)
            .map(|_| Agent {
                pos: Pos::new(0f32, 0f32),
                step_size: 6f32,
                z_offset: random_range(0f32, 4.0f32),
            })
            .collect();
        let target = Pos::new(0f32, 0f32);
        let draw_mode = DrawMode::Circle;
        let draw_target = false;
        let agents_history = VecDeque::new();

        Model {
            perlin,
            noise_seed,
            noise_scale,
            agents,
            win,
            target,
            draw_mode,
            draw_target,
            agents_history,
        }
    }

    pub fn reset_agents(&mut self) {
        self.agents = (0..self.agents.len())
            .map(|_| {
                // random r and theta around centre
                Agent {
                    pos: Pos::new(0f32, 0f32),
                    step_size: 6f32,
                    z_offset: random_range(0f32, 1f32),
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
        .view(view)
        .key_released(key_released)
        .build()
        .unwrap();
    Model::new(app.window_rect())
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // agents target a point on the canvas that updates according to the
    // selected draw mode:
    model.target = match model.draw_mode {
        DrawMode::Circle => {
            // tracks a circle moving clockwise around the canvas center
            let theta = app.time * PI * -1.0;
            let r = 300f32;
            Pos::new(r * theta.cos(), r * theta.sin())
        }
        DrawMode::FigureEight => {
            // tracks a vertical figure eight, twice as tall as wide
            let theta = app.time * PI * -1.0;
            let r = 300f32;
            Pos::new(r / 2.0 * -(theta * 2.0).sin(), r * theta.sin())
        }
        DrawMode::Average => {
            // tracks the average of all the current agent points (including the
            // agent trails), with an attraction factor to canvas center
            let target = model
                .agents_history
                .iter()
                .map(|agents| {
                    agents
                        .iter()
                        .map(|a| a.pos)
                        .reduce(|acc, pos| acc + pos)
                        .unwrap_or(Pos::new(0.0, 0.0))
                        / agents.len() as f32
                })
                .reduce(|acc, pos| acc + pos)
                .unwrap_or(Pos::new(0.0, 0.0))
                / model.agents_history.len() as f32;
            target * 0.8
        }
        DrawMode::Mouse => {
            // the current mouse position
            Pos::new(app.mouse.x, app.mouse.y)
        }
    };
    model
        .agents
        .iter_mut()
        .for_each(|a| a.update(model.perlin, model.target, model.noise_scale));

    if model.agents_history.len() >= 30 {
        let _ = model.agents_history.pop_back();
    }
    model.agents_history.push_front(model.agents.clone());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    // draw all buffered sets of agents
    model.agents_history.iter().for_each(|agents| {
        agents.iter().for_each(|agent| {
            draw.ellipse()
                .x_y(agent.pos.x, agent.pos.y)
                .radius(0.8)
                .color(WHITE);
        })
    });
    if model.draw_target {
        draw.ellipse()
            .x_y(model.target.x, model.target.y)
            .radius(1.0)
            .color(RED);
    }
    draw.to_frame(&app, &frame).unwrap();
}

fn key_released(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::D => {
            model.draw_mode = match model.draw_mode {
                DrawMode::Circle => DrawMode::FigureEight,
                DrawMode::FigureEight => DrawMode::Average,
                DrawMode::Average => DrawMode::Mouse,
                DrawMode::Mouse => DrawMode::Circle,
            };
        }
        Key::T => {
            model.draw_target = match model.draw_target {
                false => true,
                true => false,
            };
        }
        Key::Space => {
            model.noise_seed = (random_f32() * 10000.0).floor() as u32;
            model.perlin = Perlin::new().set_seed(model.noise_seed);
        }
        Key::R => {
            model.agents_history = VecDeque::new();
            model.reset_agents();
        }
        _other_key => {}
    }
}
