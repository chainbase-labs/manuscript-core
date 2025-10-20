use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UserRewardInfo {
    pub reward_per_token_checkpoint: [u8; 32],
    pub reward_pendings: u64,
    pub total_claimed_rewards: u64,
}
