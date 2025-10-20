use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Config {
    pub vault_config_key: solana_pubkey::Pubkey,
    pub pool_creator_authority: solana_pubkey::Pubkey,
    pub pool_fees: PoolFeesConfig,
    pub activation_type: u8,
    pub collect_fee_mode: u8,
    pub config_type: u8,
    pub padding_0: [u8; 5],
    pub index: u64,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub padding_1: [u64; 10],
}
