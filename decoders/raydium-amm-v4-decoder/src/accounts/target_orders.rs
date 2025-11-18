use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

use serde_big_array::BigArray;

pub const TARGET_ORDERS_SIZE: usize = core::mem::size_of::<TargetOrders>();

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TargetOrders {
    pub owner: [u64; 4],
    #[serde(with = "BigArray")]
    pub buy_orders: [TargetOrder; 50],
    pub padding1: [u64; 8],
    pub target_x: u128,
    pub target_y: u128,
    pub plan_x_buy: u128,
    pub plan_y_buy: u128,
    pub plan_x_sell: u128,
    pub plan_y_sell: u128,
    pub placed_x: u128,
    pub placed_y: u128,
    pub calc_pnl_x: u128,
    pub calc_pnl_y: u128,
    #[serde(with = "BigArray")]
    pub sell_orders: [TargetOrder; 50],
    pub padding2: [u64; 6],
    pub replace_buy_client_id: [u64; 10],
    pub replace_sell_client_id: [u64; 10],
    pub last_order_numerator: u64,
    pub last_order_denominator: u64,
    pub plan_orders_cur: u64,
    pub place_orders_cur: u64,
    pub valid_buy_order_num: u64,
    pub valid_sell_order_num: u64,
    pub padding3: [u64; 10],
    pub free_slot_bits: u128,
}
