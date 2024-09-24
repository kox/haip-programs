use anchor_lang::prelude::*;
use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1}, fetch_plugin, instructions::{AddPluginV1CpiBuilder, UpdatePluginV1CpiBuilder}, types::{Attribute, Attributes, FreezeDelegate, Plugin, PluginAuthority, PluginType}, ID as MPL_CORE_ID
};

use crate::{error::CustomErrorCode, Vote};

#[derive(Accounts)]
pub struct CreateProposal<'info> {
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
        init_if_needed,
        payer = payer,
        seeds = [b"vote_proposal", asset.key().as_ref()],
        bump,
        space = Vote::INIT_SPACE
        
    )]
    pub vote: Account<'info, Vote>,

    #[account(address = MPL_CORE_ID)]
    /// CHECK: this is a safe because we have the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}


impl<'info> CreateProposal<'info> {
    pub fn create_proposal(&mut self, bumps: &CreateProposalBumps) -> Result<()> {
        self.vote.set_inner(Vote {
            counter: 0,
            bump: bumps.vote,
        });

        match fetch_plugin::<BaseAssetV1, Attributes>(
            &self.asset.to_account_info(), 
            PluginType::Attributes
        ) {
            Ok((_, fetched_attribute_list, _)) => {
                let mut attribute_list: Vec<Attribute> = vec![];
                let mut is_initialized = false;

                for attribute in fetched_attribute_list.attribute_list {
                    if attribute.key == "state" {
                        require!(attribute.value == "0", CustomErrorCode::AlreadyProposed);

                        attribute_list.push(Attribute {
                            key: String::from("state"),
                            value: "1".to_string(), // Clock::get()?.unix_timestamp.to_string(),
                        });

                        is_initialized = true;
                    }   
                }

                if !is_initialized {
                    attribute_list.push(Attribute {
                        key: "state".to_string(),
                        value: "1".to_string()
                    });
                    attribute_list.push(Attribute {
                        key: "proposal_endtime".to_string(),
                        value: (Clock::get()?.slot + 5u64).to_string(),  
                    });
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
                let mut attribute_list: Vec<Attribute> = vec![];
            
                attribute_list.push(Attribute {
                    key: "state".to_string(),
                    value: "1".to_string(), 
                });
                attribute_list.push(Attribute {
                    key: "proposal_endtime".to_string(),
                    value: (Clock::get()?.slot + 564).to_string(), 
                });

                AddPluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
                    .asset(&self.asset.to_account_info())
                    .collection(Some(&self.collection.to_account_info()))
                    .payer(&self.payer.to_account_info())
                    .authority(Some(&self.update_authority.to_account_info()))
                    .system_program(&self.system_program.to_account_info())
                    .plugin(Plugin::Attributes( Attributes { attribute_list }))
                    .invoke()?;
            }
        }
        
        AddPluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .payer(&self.payer.to_account_info())
            .authority(Some(&self.update_authority.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .plugin(Plugin::FreezeDelegate( FreezeDelegate { frozen: true }))
            .init_authority(PluginAuthority::UpdateAuthority)
            .invoke()?;

        Ok(())
    
    }
}