use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BridgeToArgs {
    pub adaptor_id: AdaptorID,
    pub to: Vec<u8>,
    pub order_id: u64,
    pub to_chain_id: u64,
    pub amount: u64,
    pub swap_type: SwapType,
    pub data: Vec<u8>,
    pub ext_data: Vec<u8>,
}
