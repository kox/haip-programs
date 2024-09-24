use anchor_lang::prelude::*;

#[account]
pub struct Vote {
    pub counter: u64,
    pub bump: u8,
}

impl Space for Vote {
    const INIT_SPACE: usize = 8 + 8 + 1;
}