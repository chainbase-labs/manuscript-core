use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x95089ccaa0fcb0d9")]
pub struct GlobalConfig {
    pub admin: solana_pubkey::Pubkey,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub disable_flags: u8,
    pub protocol_fee_recipients: [solana_pubkey::Pubkey; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: solana_pubkey::Pubkey,
}
