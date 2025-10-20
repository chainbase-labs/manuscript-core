use carbon_core::{borsh, CarbonDeserialize};

use crate::types::PoolFeeParameters;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d83cfb4aeb449a536")]
pub struct EvtCreateConfigEvent {
    pub pool_fees: PoolFeeParameters,
    pub vault_config_key: solana_pubkey::Pubkey,
    pub pool_creator_authority: solana_pubkey::Pubkey,
    pub activation_type: u8,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub collect_fee_mode: u8,
    pub index: u64,
    pub config: solana_pubkey::Pubkey,
}
