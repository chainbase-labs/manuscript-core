use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MessageSent {
    pub rent_payer: solana_pubkey::Pubkey,
    pub created_at: i64,
    pub message: Vec<u8>,
}
