use serde::{Deserialize, Serialize};

use crate::Instruction;

use super::InstructionBox;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    history: Vec<InstructionBox>,
    history_index: usize,
    visible: bool,
}

impl Layer {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            history_index: 0,
            visible: true,
        }
    }

    pub fn undo(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
        }
    }

    pub fn redo(&mut self) {
        if self.history_index < self.history.len() {
            self.history_index += 1;
        }
    }

    pub fn clear(&mut self) {
        self.history.clear();
        self.history_index = 0;
    }

    pub fn get_rendered_strokes(&self) -> Vec<&InstructionBox> {
        let mut instructions = Vec::new();
        for i in 0..self.history_index {
            instructions.push(self.history.get(i).unwrap());
        }
        instructions
    }

    pub fn instruct(&mut self, instruction: InstructionBox) -> Result<(), crate::Error> {
        if let Instruction::Stroke(s) = &instruction.instruction {
            if s.points.len() < 2 {
                return Err(crate::Error("Stroke must have at least 2 points.".to_string()));
            }
        }
        self.history.truncate(self.history_index);
        self.history.push(instruction);
        self.history_index += 1;
        Ok(())
    }

    pub fn toggle_visibility(&mut self) -> &Self {
        self.visible = !self.visible;
        self
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn history(&self) -> &Vec<InstructionBox> {
        &self.history
    }

    pub fn history_index(&self) -> usize {
        self.history_index
    }
}
