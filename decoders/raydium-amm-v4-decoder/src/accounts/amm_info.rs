use {
    super::{super::types::*, fees::Fees},
    carbon_core::{borsh, CarbonDeserialize},
};

pub const AMM_INFO_SIZE: usize = core::mem::size_of::<AmmInfo>();

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AmmInfo {
    pub status: u64,
    pub nonce: u64,
    pub order_num: u64,
    pub depth: u64,
    pub coin_decimals: u64,
    pub pc_decimals: u64,
    pub state: u64,
    pub reset_flag: u64,
    pub min_size: u64,
    pub vol_max_cut_ratio: u64,
    pub amount_wave: u64,
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
    pub min_price_multiplier: u64,
    pub max_price_multiplier: u64,
    pub sys_decimal_value: u64,
    pub fees: Fees,
    pub out_put: OutPutData,
    pub token_coin: solana_pubkey::Pubkey,
    pub token_pc: solana_pubkey::Pubkey,
    pub coin_mint: solana_pubkey::Pubkey,
    pub pc_mint: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub open_orders: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub serum_dex: solana_pubkey::Pubkey,
    pub target_orders: solana_pubkey::Pubkey,
    pub withdraw_queue: solana_pubkey::Pubkey,
    pub token_temp_lp: solana_pubkey::Pubkey,
    pub amm_owner: solana_pubkey::Pubkey,
    pub lp_amount: u64,
    pub client_order_id: u64,
    pub padding: [u64; 2],
}
