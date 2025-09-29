use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash, Copy,
)]
pub struct UserRewardInfo {
    pub reward_per_token_completes: [u128; 2],
    pub reward_pendings: [u64; 2],
}
