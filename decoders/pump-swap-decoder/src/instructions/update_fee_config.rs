use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x68b867f258976b14")]
pub struct UpdateFeeConfig {
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [solana_pubkey::Pubkey; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateFeeConfigInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateFeeConfig {
    type ArrangedAccounts = UpdateFeeConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let admin = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(UpdateFeeConfigInstructionAccounts {
            admin,
            global_config,
            event_authority,
            program,
        })
    }
}
