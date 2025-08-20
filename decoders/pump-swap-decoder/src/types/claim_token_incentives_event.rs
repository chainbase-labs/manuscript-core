use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ClaimTokenIncentivesEvent {
    pub user: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub amount: u64,
}
