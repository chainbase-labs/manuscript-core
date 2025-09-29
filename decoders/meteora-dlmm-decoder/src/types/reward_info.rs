use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RewardInfo {
    pub mint: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub reward_duration: u64,
    pub reward_duration_end: u64,
    pub reward_rate: u128,
    pub last_update_time: u64,
    pub cumulative_seconds_with_empty_liquidity_reward: u64,
}
