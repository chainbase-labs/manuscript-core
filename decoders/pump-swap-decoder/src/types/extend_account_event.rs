use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ExtendAccountEvent {
    pub timestamp: i64,
    pub account: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub current_size: u64,
    pub new_size: u64,
}
