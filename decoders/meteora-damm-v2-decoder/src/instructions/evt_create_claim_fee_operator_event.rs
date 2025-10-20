use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d1506997844741cb1")]
pub struct EvtCreateClaimFeeOperatorEvent {
    pub operator: solana_pubkey::Pubkey,
}
