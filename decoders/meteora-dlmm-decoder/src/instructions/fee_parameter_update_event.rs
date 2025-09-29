use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d304cf17590d7f22c")]
pub struct FeeParameterUpdateEvent {
    pub lb_pair: solana_pubkey::Pubkey,
    pub protocol_share: u16,
    pub base_factor: u16,
}
