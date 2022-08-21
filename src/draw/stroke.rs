use super::{Brush, Point};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Stroke {
    pub points: Vec<Point>,
    pub brush: Brush,
}

impl Stroke {
    pub fn new(points: Vec<Point>, brush: Brush) -> Self {
        Stroke { points, brush }
    }

    pub fn set_brush(&mut self, brush: Brush) {
        self.brush = brush;
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }
}
