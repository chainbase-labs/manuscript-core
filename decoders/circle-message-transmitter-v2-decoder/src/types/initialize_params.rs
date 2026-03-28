use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializeParams {
    pub local_domain: u32,
    pub attester: solana_pubkey::Pubkey,
    pub max_message_body_size: u64,
    pub version: u32,
}
