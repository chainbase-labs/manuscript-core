use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1df2e7eb664163bdd3")]
pub struct SetBondingCurveCoinCreatorEvent {
    pub timestamp: i64,
    pub base_mint: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub coin_creator: solana_pubkey::Pubkey,
}
