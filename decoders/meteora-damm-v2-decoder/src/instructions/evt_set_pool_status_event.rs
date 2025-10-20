use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d64d54a035f5be492")]
pub struct EvtSetPoolStatusEvent {
    pub pool: solana_pubkey::Pubkey,
    pub status: u8,
}
