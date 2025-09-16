use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapArgs {
    pub amount_in: u64,
    pub expect_amount_out: u64,
    pub min_return: u64,
    pub amounts: Vec<u64>,
    pub routes: Vec<Vec<Route>>,
}
