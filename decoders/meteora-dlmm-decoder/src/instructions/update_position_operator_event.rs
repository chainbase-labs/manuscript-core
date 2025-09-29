use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d277330ccf62f4239")]
pub struct UpdatePositionOperatorEvent {
    pub position: solana_pubkey::Pubkey,
    pub old_operator: solana_pubkey::Pubkey,
    pub new_operator: solana_pubkey::Pubkey,
}
