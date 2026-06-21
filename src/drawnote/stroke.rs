use iced::{Point};

pub struct Stroke {
    pub points: Vec<Point<f32>>,
}

impl Stroke {
    pub fn new() -> Self {
        Stroke { points: Vec::new() }
    }
}