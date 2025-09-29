use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1de0b2ae4afca555b4")]
pub struct UpdateRewardFunderEvent {
    pub lb_pair: solana_pubkey::Pubkey,
    pub reward_index: u64,
    pub old_funder: solana_pubkey::Pubkey,
    pub new_funder: solana_pubkey::Pubkey,
}
