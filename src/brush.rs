use serde::{Deserialize, Serialize};

use crate::{Point, Color};

/// The data structure that holds information about how to draw a stroke.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Brush {
    /// The type of the brush.
    pub brush_type: BrushType,
    /// The color of the brush.
    pub color: Color,
    /// The width of the brush (diameter).
    pub width: f32,
}

impl Default for Brush {
    fn default() -> Self {
        Brush {
            brush_type: BrushType::Solid,
            color: Color::default(),
            width: 1.0,
        }
    }
}

impl Brush {
    /// Creates a new brush initiated with the properties passed to the function.
    pub fn new(brush_type: BrushType, color: Color, width: f32) -> Self {
        Brush {
            brush_type,
            color,
            width,
        }
    }

    /// Updates the color of the brush.
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Updates the width of the brush.
    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    /// Updates the type of the brush.
    pub fn set_brush_type(&mut self, brush_type: BrushType) {
        self.brush_type = brush_type;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BrushType {
    /// A solid circle.
    Solid,
    /// A circle eraser.
    Eraser,
    /// A circle brush (the usize is the amount of diffusion around the brush).
    Brush(usize),
    /// A custom brush.
    Custom(CustomBrush),
}

/// A custom brush type.
/// TODO: make a more extensible custom brush.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomBrush {
    /// A collection of collections of points
    /// which represent shapes. The last point
    /// is linked to the first.
    points: Vec<Vec<Point>>,
}
