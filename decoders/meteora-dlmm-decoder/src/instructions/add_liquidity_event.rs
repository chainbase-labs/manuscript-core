use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d1f5e7d5ae3343dba")]
pub struct AddLiquidityEvent {
    pub lb_pair: solana_pubkey::Pubkey,
    pub from: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub amounts: [u64; 2],
    pub active_bin_id: i32,
}
