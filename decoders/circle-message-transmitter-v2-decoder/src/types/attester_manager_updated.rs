use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AttesterManagerUpdated {
    pub previous_attester_manager: solana_pubkey::Pubkey,
    pub new_attester_manager: solana_pubkey::Pubkey,
}
