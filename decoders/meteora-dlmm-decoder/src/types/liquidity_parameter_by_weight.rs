use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LiquidityParameterByWeight {
    pub amount_x: u64,
    pub amount_y: u64,
    pub active_id: i32,
    pub max_active_bin_slippage: i32,
    pub bin_liquidity_dist: Vec<BinLiquidityDistributionByWeight>,
}
