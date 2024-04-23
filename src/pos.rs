use nannou::math::num_traits::Pow;

#[derive(Copy, Clone)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Pos { x, y }
    }

    pub fn radius(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn pow(&self, rhs: f32) -> Pos {
        Pos::new(
            self.x.signum() * self.x.abs().pow(rhs),
            self.y.signum() * self.y.abs().pow(rhs),
        )
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
