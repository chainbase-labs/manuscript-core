use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1df6e43a8291aa4fcc")]
pub struct FundRewardEvent {
    pub lb_pair: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub reward_index: u64,
    pub amount: u64,
}
