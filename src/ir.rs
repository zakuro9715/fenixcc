#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Instruction {
    PushI(i64),
    PopI,
    AddI,
    SubI,
}

pub type IR = Vec<Instruction>;
