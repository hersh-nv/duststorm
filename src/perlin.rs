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
        let dxy = dxy * 0.025; // acceleration factor, tweak for best results
        self.pos.x += dxy.x;
        self.pos.y += dxy.y;

        // lastly - push z offset a bit so we're constantly sliding up the x axis of the noise space
        self.z_offset += 0.01;
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
    pub draw_target: bool,
    pub draw_buffer: VecDeque<Draw>,
}

impl Model {
    pub fn new(win: Rect) -> Self {
        let agent_count = 10;
        let noise_scale = 300.0;
        let noise_seed = random::<u32>();
        let perlin = Perlin::new().set_seed(noise_seed);
        let agents = (0..agent_count)
            .map(|_| Agent {
                pos: Pos::new(0f32, 0f32),
                step_size: 6f32,
                z_offset: random_range(0f32, 1f32),
            })
            .collect();
        let target = Pos::new(0f32, 0f32);
        let draw_mode = DrawMode::Circle;
        let draw_target = false;
        let draw_buffer = VecDeque::new();

        Model {
            perlin,
            noise_seed,
            noise_scale,
            agents,
            win,
            target,
            draw_mode,
            draw_target,
            draw_buffer,
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
            let theta = app.time * PI * -1.0;
            let r = 300f32;
            model.target = Pos::new(r * theta.cos(), r * theta.sin());
        }
        DrawMode::Mouse => model.target = Pos::new(app.mouse.x, app.mouse.y),
    }
    model
        .agents
        .iter_mut()
        .for_each(|a| a.update(model.perlin, model.target, model.noise_scale));

    draw_to_buffer(app, model);
}

fn draw_to_buffer(_app: &App, model: &mut Model) {
    let draw = Draw::new();
    // draw agents
    model.agents_pos().iter().for_each(|a_pos| {
        draw.ellipse()
            .x_y(a_pos.x, a_pos.y)
            .radius(0.8)
            .color(WHITE);
    });
    // draw target
    if model.draw_target {
        draw.ellipse()
            .x_y(model.target.x, model.target.y)
            .radius(1.0)
            .color(RED);
    }

    // keep a trail of last x draws in buffer
    print!("Adding to buffer...");
    if model.draw_buffer.len() >= 60 {
        print!("full!");
        let _ = model.draw_buffer.pop_back();
    }
    model.draw_buffer.push_front(draw);
    print!("\n");
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.to_frame(&app, &frame).unwrap();
    print!("Drawing buffers...");
    model.draw_buffer.iter().for_each(|d: &Draw| {
        print!("#");
        d.to_frame(&app, &frame).unwrap();
    });
    print!("\n");
}

fn key_released(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::D => {
            model.draw_mode = match model.draw_mode {
                DrawMode::Circle => DrawMode::Mouse,
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
            model.reset_agents();
        }
        _other_key => {}
    }
}
