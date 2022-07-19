use anchor_lang::prelude::*;

use crate::{constants::HIDDEN_SECTION, errors::CandyError};

/// Candy machine configuration data.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct CandyMachineData {
    /// Price of an asset
    pub price: u64,
    /// Number of assets available
    pub items_available: u64,
    /// Symbol for the asset
    pub symbol: String,
    /// Secondary sales royalty basis points (0-10000)
    pub seller_fee_basis_points: u16,
    /// Max supply of each individual asset (default 0)
    pub max_supply: u64,
    /// Indicates if the asset is mutable or not (default yes)
    pub is_mutable: bool,
    /// Indicates whether to retain the update authority or not
    pub retain_authority: bool,
    /// List of creators
    pub creators: Vec<Creator>,
    /// Config line settings
    pub config_line_settings: ConfigLineSettings,
    /// Hidden setttings
    pub hidden_settings: Option<HiddenSettings>,
}

// Creator information.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Creator {
    /// Pubkey address
    pub address: Pubkey,
    /// Whether the creator is verified or not
    pub verified: bool,
    // Share of secondary sales royalty
    pub percentage_share: u8,
}

/// Hidden settings for large mints used with off-chain data.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct HiddenSettings {
    /// Asset prefix name
    pub name: String,
    /// Shared URI
    pub uri: String,
    /// Hash of the hidden settings file
    pub hash: [u8; 32],
}

/// Config line settings to allocate space for individual name + URI.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct ConfigLineSettings {
    /// Common name prefix
    pub prefix_name: String,
    /// Length of the remaining part of the name
    pub name_length: u32,
    /// Common URI prefix
    pub prefix_uri: String,
    /// Length of the remaining part of the URI
    pub uri_length: u32,
}

impl CandyMachineData {
    pub fn get_space_for_candy(&self) -> Result<usize> {
        Ok(if self.hidden_settings.is_some() {
            HIDDEN_SECTION
        } else {
            HIDDEN_SECTION
                + 4
                + (self.items_available as usize) * self.get_config_line_size()
                + 4
                + ((self
                    .items_available
                    .checked_div(8)
                    .ok_or(CandyError::NumericalOverflowError)?
                    + 1) as usize)
                + 4
                + (self.items_available as usize) * 4
        })
    }

    pub fn get_config_line_size(&self) -> usize {
        (self.config_line_settings.name_length + self.config_line_settings.uri_length) as usize
    }
}
