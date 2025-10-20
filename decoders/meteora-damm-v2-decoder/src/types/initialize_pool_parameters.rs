use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializePoolParameters {
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub activation_point: Option<u64>,
}
