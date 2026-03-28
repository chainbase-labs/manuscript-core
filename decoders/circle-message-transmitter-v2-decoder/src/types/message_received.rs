use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MessageReceived {
    pub caller: solana_pubkey::Pubkey,
    pub source_domain: u32,
    pub nonce: [u8; 32],
    pub sender: solana_pubkey::Pubkey,
    pub finality_threshold_executed: u32,
    pub message_body: Vec<u8>,
}
