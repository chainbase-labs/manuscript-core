use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1db94afc7d1bd7bc6f")]
pub struct LbPairCreateEvent {
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_step: u16,
    pub token_x: solana_pubkey::Pubkey,
    pub token_y: solana_pubkey::Pubkey,
}
