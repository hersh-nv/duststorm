use crate::pos;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use pos::Pos;

#[derive(Clone)]
pub struct Agent {
    pub pos: Pos,   // (x,y) position
    step_size: f32, // in pixels
    pub ttl: f32,   // how many seconds to survive before regenerating in random location
    pub z_offset: f32,
    z: f32,
}

impl Agent {
    pub fn new(win: Rect, start_origin: bool, step_size: f32) -> Agent {
        Agent {
            pos: match start_origin {
                true => Pos::new(0.0, 0.0),
                false => Pos::new(
                    random_range(win.left() * 1.1, win.right() * 1.1),
                    random_range(win.bottom() * 1.1, win.top() * 1.1),
                ),
            },
            step_size: step_size,
            ttl: random_range(2.0, 10.0),
            z_offset: random_range(0f32, 1.0f32),
            z: 0.0,
        }
    }

    pub fn update1(&mut self, noise: Perlin, target: Pos, noise_scale: f64) {
        // take a fixed step in the noise direction
        let angle = noise.get([
            self.pos.x as f64 / noise_scale,
            self.pos.y as f64 / noise_scale,
            (self.z + self.z_offset * 4.0) as f64,
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

    pub fn update2(
        &mut self,
        noise: Perlin,
        noise_scale: f64,
        target: Pos,
        win: Rect,
        time: nannou::state::Time,
    ) {
        if self.ttl < 0.0 {
            *self = Agent::new(win, false, 2f32);
        } else {
            // take a fixed step in the noise direction
            let angle = noise.get([
                self.pos.x as f64 / noise_scale,
                self.pos.y as f64 / noise_scale,
                time.since_start.as_secs_f64() / 25.0,
            ]) as f32;
            let angle = angle * 2.0 * PI;
            self.pos.x += angle.cos() * self.step_size;
            self.pos.y += angle.sin() * self.step_size;
            // then take a proportional step in the target direction
            let dxy = target - self.pos;
            let dxy = dxy * 0.002; // acceleration factor, tweak for best results
            self.pos.x += dxy.x;
            self.pos.y += dxy.y;
            // update ttl
            self.ttl -= time.since_prev_update.as_secs_f32();
        }
    }
}
