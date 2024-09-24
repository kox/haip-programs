pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2PxZ18HnizcT2oj944dhLdTUrzYXHZ8SmRFSJLYH8KWH");

#[program]
pub mod haip {
    use super::*;

    pub fn create_proposal(ctx: Context<CreateProposal>) -> Result<()> {
        ctx.accounts.create_proposal(&ctx.bumps)
    }

    pub fn vote_proposal(ctx: Context<VoteProposal>) -> Result<()> {
        ctx.accounts.vote()
    }

    pub fn solve_proposal(ctx: Context<SolveProposal>) -> Result<()> {
        ctx.accounts.solve_proposal()
    }
}

