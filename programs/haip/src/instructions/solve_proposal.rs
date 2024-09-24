use anchor_lang::prelude::*;
use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1}, fetch_plugin, instructions::UpdatePluginV1CpiBuilder, types::{Attribute, Attributes, Plugin, PluginType}, ID as MPL_CORE_ID
};

use crate::{error::CustomErrorCode, Vote};

#[derive(Accounts)]
pub struct SolveProposal<'info> {
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
        close = owner,
        seeds = [b"vote_proposal", asset.key().as_ref()],
        bump = vote.bump,
    )]
    pub vote: Account<'info, Vote>,

    #[account(address = MPL_CORE_ID)]
    /// CHECK: this is a safe because we have the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}


impl<'info> SolveProposal<'info> {
    pub fn solve_proposal(&mut self) -> Result<()> {
        // TODO: Add minimun votes to be considered as accepted
        match fetch_plugin::<BaseAssetV1, Attributes>(
            &self.asset.to_account_info(), 
            PluginType::Attributes
        ) {
            Ok((_, fetched_attribute_list, _)) => {
                let mut attribute_list: Vec<Attribute> = vec![];

                for attribute in fetched_attribute_list.attribute_list {
                    if attribute.key == "state" {
                        require!(attribute.value == "1", CustomErrorCode::NotProposed);
                        
                        if self.vote.counter > 0 {
                            attribute_list.push(Attribute {
                                key: String::from("state"),
                                value: "2".to_string(), 
                            });
    
                            attribute_list.push(Attribute {
                                key: String::from("campaign_endtime"),
                                value: (Clock::get()?.slot + 100u64).to_string(), 
                            });
                        } else {
                            attribute_list.push(Attribute {
                                key: String::from("state"),
                                value: "3".to_string(),  // FAILED
                            });
    
                        }

                    } // TODO: make this validation check 
                    /* else if attribute.key == "proposal_endtime" {
                        let current_slot = Clock::get()?.slot;

                        require!(current_slot > attribute.value.parse::<u64>().unwrap(), CustomErrorCode::ProposedStillOpen);
                    } */
                }

                UpdatePluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
                    .asset(&self.asset.to_account_info())
                    .collection(Some(&self.collection.to_account_info()))
                    .payer(&self.payer.to_account_info())
                    .authority(Some(&self.update_authority.to_account_info()))
                    .system_program(&self.system_program.to_account_info())
                    .plugin(Plugin::Attributes( Attributes { attribute_list }))
                    .invoke()?;
            },
            Err(_) => {
                return Err(CustomErrorCode::CustomError.into());
            }
        }

        Ok(())
    }
}
