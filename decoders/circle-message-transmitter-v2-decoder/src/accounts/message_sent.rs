use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x83648538a6e1973c")]
pub struct MessageSent {
    pub rent_payer: solana_pubkey::Pubkey,
    pub created_at: i64,
    pub message: Vec<u8>,
}
