use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d2ddc5d181961ac68")]
pub struct AdminSetCoinCreatorEvent {
    pub timestamp: i64,
    pub admin_set_coin_creator_authority: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub old_coin_creator: solana_pubkey::Pubkey,
    pub new_coin_creator: solana_pubkey::Pubkey,
}
