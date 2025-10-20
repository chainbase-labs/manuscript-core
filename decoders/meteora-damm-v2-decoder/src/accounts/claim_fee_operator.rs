use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa630865622c8bc96")]
pub struct ClaimFeeOperator {
    pub operator: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 128],
}
