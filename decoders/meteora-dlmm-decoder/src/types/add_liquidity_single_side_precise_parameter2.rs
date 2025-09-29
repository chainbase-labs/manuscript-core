use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AddLiquiditySingleSidePreciseParameter2 {
    pub bins: Vec<CompressedBinDepositAmount>,
    pub decompress_multiplier: u64,
    pub max_amount: u64,
}
