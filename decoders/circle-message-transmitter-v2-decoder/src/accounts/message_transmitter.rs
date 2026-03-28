use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4728b48e13cb23fc")]
pub struct MessageTransmitter {
    pub owner: solana_pubkey::Pubkey,
    pub pending_owner: solana_pubkey::Pubkey,
    pub attester_manager: solana_pubkey::Pubkey,
    pub pauser: solana_pubkey::Pubkey,
    pub paused: bool,
    pub local_domain: u32,
    pub version: u32,
    pub signature_threshold: u32,
    pub enabled_attesters: Vec<solana_pubkey::Pubkey>,
    pub max_message_body_size: u64,
}
