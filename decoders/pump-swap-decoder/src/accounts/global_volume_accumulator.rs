use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xca2af62b8ebe1eff")]
pub struct GlobalVolumeAccumulator {
    pub start_time: i64,
    pub end_time: i64,
    pub seconds_in_a_day: i64,
    pub mint: solana_pubkey::Pubkey,
    pub total_token_supply: [u64; 30],
    pub sol_volumes: [u64; 30],
}
