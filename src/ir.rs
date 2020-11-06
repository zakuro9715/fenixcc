#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Instruction {
    PushI(i64),
    PopI,
    AddI,
    SubI,
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct IR {
    pub instructions: Vec<Instruction>
}

impl IR {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, v :Instruction) {
        self.instructions.push(v)
    }
    pub fn pop(&mut self) -> Option<Instruction> {
        self.instructions.pop()
    }
    pub fn clear(&mut self) {
        self.instructions.clear()
    }
}

impl From<Vec<Instruction>> for IR {
    fn from(instructions: Vec<Instruction>) -> IR {
        Self { instructions }
    }
}
