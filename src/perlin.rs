// Agents that move following a target, while being pushed around by Perlin noise.

use std::collections::VecDeque;

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
    z_offset: f32,
    z: f32,
}

impl Agent {
    pub fn update(&mut self, noise: Perlin, target: Pos, noise_scale: f64) {
        // take a fixed step in the noise direction
        let angle = noise.get([
            self.pos.x as f64 / noise_scale,
            self.pos.y as f64 / noise_scale,
            (self.z + self.z_offset) as f64,
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
        self.z += 0.02;
    }
}

enum TargetMode {
    Circle,
    FigureEight,
    Average,
    Mouse,
}

enum ColorMode {
    White,
    RedBlue,
}

struct Model {
    perlin: Perlin,
    noise_seed: u32,
    noise_scale: f64,
    pub agents: Vec<Agent>,
    win: Rect,
    pub target: Pos,
    pub target_mode: TargetMode,
    pub color_mode: ColorMode,
    pub draw_target: bool,
    pub agents_history: VecDeque<Vec<Agent>>,
}

impl Model {
    pub fn new(win: Rect) -> Self {
        let agent_count = 100;
        let noise_scale = 400.0;
        let noise_seed = random::<u32>();
        let perlin = Perlin::new().set_seed(noise_seed);
        let agents = (0..agent_count)
            .map(|_| Agent {
                pos: Pos::new(0f32, 0f32),
                step_size: 6f32,
                z_offset: random_range(0f32, 4.0f32),
                z: 0.0,
            })
            .collect();
        let target = Pos::new(0f32, 0f32);
        let target_mode = TargetMode::Circle;
        let color_mode = ColorMode::White;
        let draw_target = false;
        let agents_history = VecDeque::new();

        Model {
            perlin,
            noise_seed,
            noise_scale,
            agents,
            win,
            target,
            target_mode,
            color_mode,
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
                    z_offset: random_range(0f32, 4f32),
                    z: 0.0,
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
    // agents target a point on the canvas that updates according to the
    // selected draw mode:
    model.target = match model.target_mode {
        TargetMode::Circle => {
            // tracks a circle moving clockwise around the canvas center
            let theta = app.time * PI * -0.2;
            let r = 300f32;
            Pos::new(r * theta.cos(), r * theta.sin())
        }
        TargetMode::FigureEight => {
            // tracks a vertical figure eight, twice as tall as wide
            let theta = app.time * PI * -0.2;
            let r = 300f32;
            Pos::new(r / 2.0 * -(theta * 2.0).sin(), r * theta.sin())
        }
        TargetMode::Average => {
            // tracks the average of the newest agent set, with an attraction
            // factor to canvas center
            let target = model
                .agents_history
                .iter()
                .last()
                .map(|agents| {
                    agents
                        .iter()
                        .map(|a| a.pos)
                        .reduce(|acc, pos| acc + pos)
                        .unwrap_or(Pos::new(0.0, 0.0))
                        / agents.len() as f32
                })
                .unwrap_or(Pos::new(0.0, 0.0));
            target.pow(0.95)
        }
        TargetMode::Mouse => {
            // the current mouse position
            Pos::new(app.mouse.x, app.mouse.y)
        }
    };
    model
        .agents
        .iter_mut()
        .for_each(|a| a.update(model.perlin, model.target, model.noise_scale));

    if model.agents_history.len() >= 300 {
        let _ = model.agents_history.pop_front();
    }
    model.agents_history.push_back(model.agents.clone());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.keys.down.contains(&Key::R) {
        draw.background().color(BLACK);
    }
    // 'erase' the oldest agent set by overwriting in black
    model.agents_history.iter().nth(0).map(|agents| {
        agents.iter().for_each(|agent| {
            draw.ellipse()
                .x_y(agent.pos.x, agent.pos.y)
                .radius(0.8)
                .color(BLACK);
        })
    });
    // draw the newest agent set
    model.agents_history.iter().last().map(|agents| {
        agents.iter().for_each(|agent| {
            let color = match model.color_mode {
                ColorMode::White => WHITE,
                ColorMode::RedBlue => rgb(
                    15 + (agent.z_offset * 60.0) as u8,
                    0,
                    255 - (agent.z_offset * 60.0) as u8,
                ),
            };
            draw.ellipse()
                .x_y(agent.pos.x, agent.pos.y)
                .radius(0.8)
                .color(color);
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
            model.target_mode = match model.target_mode {
                TargetMode::Circle => TargetMode::FigureEight,
                TargetMode::FigureEight => TargetMode::Average,
                TargetMode::Average => TargetMode::Mouse,
                TargetMode::Mouse => TargetMode::Circle,
            };
        }
        Key::C => {
            model.color_mode = match model.color_mode {
                ColorMode::White => ColorMode::RedBlue,
                ColorMode::RedBlue => ColorMode::White,
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
