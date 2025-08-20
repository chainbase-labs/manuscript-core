use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d966bc77b7ccf66e4")]
pub struct SetMetaplexCoinCreatorEvent {
    pub timestamp: i64,
    pub base_mint: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub coin_creator: solana_pubkey::Pubkey,
}
