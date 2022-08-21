use serde::{Deserialize, Serialize};

use super::Point;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Motion {
    pub start: Point,
    pub end: Point,
    pub selection: Option<(Point, Point)>,
}

impl Motion {
    pub fn with_selection(start: Point, end: Point, up_left: Point, down_right: Point) -> Self {
        Motion {
            start,
            end,
            selection: Some((up_left, down_right)),
        }
    }

    pub fn new(start: Point, end: Point) -> Self {
        Motion {
            start,
            end,
            selection: None,
        }
    }
}
