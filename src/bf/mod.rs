pub mod interp;
pub mod asm;
pub mod internals;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BFToken {
    PtrIncr,
    PtrDecr,
    CellIncr,
    CellDecr,
    CellOut,
    CellIn,
    JumpFwd,
    JumpBwd,
    Break
}

impl BFToken {
    pub fn parse(chr: char) -> Option<BFToken> {
        match chr {
            '>' => Some(BFToken::PtrIncr),
            '<' => Some(BFToken::PtrDecr),
            '+' => Some(BFToken::CellIncr),
            '-' => Some(BFToken::CellDecr),
            '.' => Some(BFToken::CellOut),
            ',' => Some(BFToken::CellIn),
            '[' => Some(BFToken::JumpFwd),
            ']' => Some(BFToken::JumpBwd),
            '°' => Some(BFToken::Break),
            _ => None
        }
    }
}
