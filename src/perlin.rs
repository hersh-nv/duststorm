use nannou::prelude::*;
use noise::Perlin;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

type Vec2 = (f32, f32);

struct Agent {
    pos: Vec2,
    vel: Vec2,
    step_size: f32,
}

struct Model {
    perlin: Perlin,
    agents: Vec<Agent>,
    win: Rect,
}

impl Model {
    pub fn new(win: Rect) -> Self {
        let agent_count = 100;
        let perlin = Perlin::new(random::<u32>());
        let agents = (0..agent_count)
            .map(|_| Agent {
                pos: (
                    random_range(win.left(), win.right()),
                    random_range(win.bottom(), win.top()),
                ),
                vel: (0f32, 0f32),
                step_size: 1f32,
            })
            .collect();
        Model {
            perlin,
            agents,
            win,
        }
    }

    pub fn agents_pos(&self) -> Vec<Vec2> {
        self.agents.iter().map(|agent| agent.pos).collect()
    }
}

fn model(app: &App) -> Model {
    Model::new(app.window_rect())
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.agents_pos().iter().for_each(|a_pos| {
        draw.ellipse()
            .x_y(a_pos.0, a_pos.1)
            .radius(1.0)
            .color(WHITE);
    });
    draw.to_frame(app, &frame).unwrap();
}
