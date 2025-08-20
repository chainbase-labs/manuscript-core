use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1de8f5c2eeeada3a59")]
pub struct CollectCoinCreatorFeeEvent {
    pub timestamp: i64,
    pub coin_creator: solana_pubkey::Pubkey,
    pub coin_creator_fee: u64,
    pub coin_creator_vault_ata: solana_pubkey::Pubkey,
    pub coin_creator_token_account: solana_pubkey::Pubkey,
}
