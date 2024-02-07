#[derive(Clone, Copy)]
pub struct Euler {
    x: f32,
    y: f32,
    z: f32,
}

impl Euler {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}
