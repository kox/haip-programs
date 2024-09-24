use anchor_lang::prelude::*;
use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1}, 
    ID as MPL_CORE_ID
};

use crate::Vote;

#[derive(Accounts)]
pub struct VoteProposal<'info> {
    pub owner: Signer<'info>,

    pub update_authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        has_one = owner, 
    )]
    pub asset: Account<'info, BaseAssetV1>,

    #[account(
        mut,
        has_one = update_authority
    )]
    pub collection: Account<'info, BaseCollectionV1>, 

    #[account(
        mut,
        seeds = [b"vote_proposal", asset.key().as_ref()],
        bump = vote.bump,
        
    )]
    pub vote: Account<'info, Vote>,

    #[account(address = MPL_CORE_ID)]
    /// CHECK: this is a safe because we have the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}


impl<'info> VoteProposal<'info> {
    pub fn vote(&mut self) -> Result<()> {
        self.vote.counter = self.vote.counter.checked_add(1).unwrap();

        Ok(())
    }
}