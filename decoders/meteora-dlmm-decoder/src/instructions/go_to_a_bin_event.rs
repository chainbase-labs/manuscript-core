use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d3b8a4c448a83b043")]
pub struct GoToABinEvent {
    pub lb_pair: solana_pubkey::Pubkey,
    pub from_bin_id: i32,
    pub to_bin_id: i32,
}
