use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SendMessageParams {
    pub destination_domain: u32,
    pub recipient: solana_pubkey::Pubkey,
    pub destination_caller: solana_pubkey::Pubkey,
    pub min_finality_threshold: u32,
    pub message_body: Vec<u8>,
}
