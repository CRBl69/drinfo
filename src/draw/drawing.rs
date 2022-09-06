use std::collections::hash_map::HashMap;

use serde::{Deserialize, Serialize};

use super::{Layer, InstructionBox};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drawing {
    layers: HashMap<String, Layer>,
    layer_order: Vec<String>,
    width: u32,
    height: u32,
}

impl Default for Drawing {
    fn default() -> Drawing {
        Drawing::new(500, 500)
    }
}

#[derive(Debug)]
pub struct Error(pub String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

impl Drawing {
    pub fn new(height: u32, width: u32) -> Self {
        let layers = HashMap::new();
        Drawing {
            layers,
            width,
            height,
            layer_order: vec![],
        }
    }

    pub fn add_layer(&mut self, name: &str) -> Result<(), Error> {
        if !self.layers.contains_key(name) {
            self.layers
                .insert(name.to_string(), Layer::new());
            self.layer_order.push(name.to_string());
            Ok(())
        } else {
            Err(Error("Layer already exists".to_string()))
        }
    }

    pub fn layer_up(&mut self, name: &str) -> Result<(), Error> {
        if let Some(index) = self.layer_order.iter().position(|e| e == name) {
            if index < self.layer_order.len() && index > 0 {
                let n2 = self.layer_order[index-1].to_string();
                self.layer_order[index-1] = name.to_string();
                self.layer_order[index] = n2;
                Ok(())
            } else {
                Err(Error("Layer cannot be moved up, already at the top.".to_string()))
            }
        } else {
            Err(Error("Layer not found.".to_string()))
        }
    }

    pub fn layer_up_by(&mut self, name: &str, by: usize) -> Result<(), Error> {
        if let Some(index) = self.layer_order.iter().position(|e| e == name) {
            if index + by < self.layer_order.len() {
                let layer = self.layer_order.remove(index);
                self.layer_order.insert(index + by, layer);
                Ok(())
            } else {
                Err(Error("Layer cannot be moved up this much.".to_string()))
            }
        } else {
            Err(Error("Layer not found.".to_string()))
        }
    }

    pub fn layer_down_by(&mut self, name: &str, by: usize) -> Result<(), Error> {
        if let Some(index) = self.layer_order.iter().position(|e| e == name) {
            if index >= by {
                let layer = self.layer_order.remove(index);
                self.layer_order.insert(index - by, layer);
                Ok(())
            } else {
                Err(Error("Layer cannot be moved down this much.".to_string()))
            }
        } else {
            Err(Error("Layer not found.".to_string()))
        }
    }

    pub fn instruct(&mut self, instruction: InstructionBox, layer_name: &str) -> Result<(), Error> {
        let layer = self.layers.get_mut(layer_name);
        if let Some(l) = layer {
            l.instruct(instruction)
        } else {
            Err(Error("Layer not found.".to_string()))
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_layer_mut(&mut self, layer_name: &str) -> Option<&mut Layer> {
        self.layers.get_mut(layer_name)
    }
}
