use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dffc4106b1cca3580")]
pub struct PositionCloseEvent {
    pub position: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}
