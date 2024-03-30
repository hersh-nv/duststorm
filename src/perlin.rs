// Agents that move following a target, while being pushed around by Perlin noise.

use std::process::Output;

use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::text::rt::Point;
use nannou::{frame, prelude::*};

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

struct Agent {
    pos: Pos,       // (x,y) position
    step_size: f32, // ??
}

impl Agent {
    pub fn update(&mut self, noise: Perlin, target: Pos, noise_scale: f64) {
        // take a fixed step in the noise direction
        let angle = noise.get([
            self.pos.x as f64 / noise_scale,
            self.pos.y as f64 / noise_scale,
        ]) as f32;
        let angle = angle * 10.0 * PI;
        self.pos.x += angle.cos() * self.step_size;
        self.pos.y += angle.sin() * self.step_size;
        // then take a proportional step in the target direction
        let dxy = target - self.pos;
        let dxy = dxy * 0.05; // acceleration factor, tweak for best results
        self.pos.x += dxy.x;
        self.pos.y += dxy.y;
    }
}

enum DrawMode {
    Circle,
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
}

impl Model {
    pub fn new(win: Rect) -> Self {
        let agent_count = 100;
        let noise_scale = 250.0;
        let noise_seed = random::<u32>();
        let perlin = Perlin::new().set_seed(noise_seed);
        let agents = (0..agent_count)
            .map(|_| {
                // random r and theta around centre
                let maxdim = f32::max(win.right(), win.top());
                let r = random_range(0f32, maxdim);
                let theta = random_range(0f32, 2.0 * PI);
                Agent {
                    pos: Pos::new(r * theta.cos(), r * theta.sin()),
                    step_size: 10f32,
                }
            })
            .collect();
        let target = Pos::new(0f32, 0f32);
        let draw_mode = DrawMode::Circle;

        Model {
            perlin,
            noise_seed,
            noise_scale,
            agents,
            win,
            target,
            draw_mode,
        }
    }

    pub fn agents_pos(&self) -> Vec<Pos> {
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
    match model.draw_mode {
        DrawMode::Circle => {
            let theta = app.time * PI * 1.3;
            let r = 100f32;
            model.target = Pos::new(r * theta.cos(), r * theta.sin());
        }
        DrawMode::Mouse => model.target = Pos::new(app.mouse.x, app.mouse.y),
    }

    model
        .agents
        .iter_mut()
        .for_each(|a| a.update(model.perlin, model.target, model.noise_scale))
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.agents_pos().iter().for_each(|a_pos| {
        draw.ellipse()
            .x_y(a_pos.x, a_pos.y)
            .radius(1.0)
            .color(WHITE);
    });
    //draw target
    draw.ellipse()
        .x_y(model.target.x, model.target.y)
        .radius(1.0)
        .color(RED);
    draw.to_frame(app, &frame).unwrap();
}

fn key_released(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Key1 => {
            model.draw_mode = DrawMode::Circle;
        }
        Key::Key2 => {
            model.draw_mode = DrawMode::Mouse;
        }
        Key::Space => {
            model.noise_seed = (random_f32() * 10000.0).floor() as u32;
            model.perlin = Perlin::new().set_seed(model.noise_seed);
        }
        Key::R => {
            *model = Model::new(app.window_rect());
        }
        _other_key => {}
    }
}
