/* use anchor_lang::prelude::*;
use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1}, fetch_plugin, instructions::{AddPluginV1CpiBuilder, RemovePluginV1CpiBuilder, UpdatePluginV1CpiBuilder}, types::{Attribute, Attributes, FreezeDelegate, Plugin, PluginAuthority, PluginType}, ID as MPL_CORE_ID
};

use crate::error::CustomErrorCode;

#[derive(Accounts)]
pub struct Propose<'info> {
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

    #[account(address = MPL_CORE_ID)]
    /// CHECK: this is a safe because we have the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}


impl<'info> Propose<'info> {
    pub fn propose(&mut self) -> Result<()> {
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

                        is_initialized = true;
                    }   
                }
                        /* attribute_list.push(Attribute {
                            key: String::from("state"),
                            value: "1".to_string()
                            // value: Clock::get()?.unix_timestamp.to_string(),
                        });
                        is_initialized = true; */
                     /* else {
                        attribute_list.push(attribute);
                    } */
                

                if !is_initialized {
                    attribute_list.push(Attribute {
                        key: "state".to_string(),
                        value: "0".to_string()
                        // value: Clock::get()?.unix_timestamp.to_string(),
                    });
                    attribute_list.push(Attribute {
                        key: "proposal_endtime".to_string(),
                        value: (Clock::get()?.slot + 100u64).to_string(),  
                        //  "0".to_string(),
                    });
                    /*a ttribute_list.push(Attribute {
                        key: "staked_time".to_string(),
                        value: "0".to_string(),
                    }); */
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
                    value: "0".to_string(),  // Clock::get()?.unix_timestamp.to_string(),
                });
                attribute_list.push(Attribute {
                    key: "proposal_endtime".to_string(),
                    value: (Clock::get()?.slot + 100u64).to_string(), //"0".to_string(),
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

    /* pub fn  unstake(&mut self) -> Result<()> {
        match fetch_plugin::<BaseAssetV1, Attributes>(
            &self.asset.to_account_info(), 
            PluginType::Attributes
        ) {
            Ok((_, fetched_attribute_list, _)) => {
                let mut attribute_list: Vec<Attribute> = vec![];
                let mut is_initialized = false;
                let mut staked_time: i64 = 0;

                for attribute in fetched_attribute_list.attribute_list {
                    if attribute.key == "staked" {
                        require!(attribute.value != "0", CustomErrorCode::NonStaked);
                    
                        is_initialized = true;

                        attribute_list.push(Attribute {
                            key: String::from("staked"),
                            value: String::from("0"),
                        });

                        staked_time = staked_time
                            .checked_add(Clock::get()?.unix_timestamp)
                            .ok_or(CustomErrorCode::Overflow)?
                            .checked_sub(
                                attribute.value.parse::<i64>()
                                .map_err(|_| CustomErrorCode::InvalidTimestamp)?
                            ) 
                            .ok_or(CustomErrorCode::Underflow)?;
                    } else if attribute.key == "staked_time" {
                        staked_time = staked_time
                            .checked_add(
                                attribute.value.parse::<i64>()
                                .map_err(|_| CustomErrorCode::InvalidTimestamp)?
                            )
                            .ok_or(CustomErrorCode::Overflow)?;
                        
                        attribute_list.push(Attribute {
                            key: String::from("staked_time"),
                            value: String::from("staked_time")
                        });
                    } else {
                        attribute_list.push(attribute);
                    }

                }
                
                require!(is_initialized, CustomErrorCode::NonStaked);
            
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
                return Err(CustomErrorCode::NonStaked.into());
            }
        }

        UpdatePluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .payer(&self.payer.to_account_info())
            .authority(Some(&self.update_authority.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .plugin(Plugin::FreezeDelegate(FreezeDelegate { frozen: false }))
            .invoke()?;
        
        RemovePluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
        .asset(&self.asset.to_account_info())
        .collection(Some(&self.collection.to_account_info()))
        .payer(&self.payer.to_account_info())
        .authority(Some(&self.update_authority.to_account_info()))
        .system_program(&self.system_program.to_account_info())
        .plugin_type(PluginType::FreezeDelegate)
        .invoke()?;

        Ok(())
    } */
}
 */