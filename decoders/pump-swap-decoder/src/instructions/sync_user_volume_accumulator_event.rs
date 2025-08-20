use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dc57aa77c74515bff")]
pub struct SyncUserVolumeAccumulatorEvent {
    pub user: solana_pubkey::Pubkey,
    pub total_claimed_tokens_before: u64,
    pub total_claimed_tokens_after: u64,
}
