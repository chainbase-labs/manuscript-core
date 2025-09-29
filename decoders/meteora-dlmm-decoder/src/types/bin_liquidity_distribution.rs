use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BinLiquidityDistribution {
    pub bin_id: i32,
    pub distribution_x: u16,
    pub distribution_y: u16,
}
