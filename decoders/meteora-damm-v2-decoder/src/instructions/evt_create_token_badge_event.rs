use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d8d788674221c72a0")]
pub struct EvtCreateTokenBadgeEvent {
    pub token_mint: solana_pubkey::Pubkey,
}
