use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BinLiquidityDistributionByWeight {
    pub bin_id: i32,
    pub weight: u16,
}
