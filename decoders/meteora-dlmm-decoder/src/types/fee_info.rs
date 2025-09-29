use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FeeInfo {
    pub fee_x_per_token_complete: u128,
    pub fee_y_per_token_complete: u128,
    pub fee_x_pending: u64,
    pub fee_y_pending: u64,
}
