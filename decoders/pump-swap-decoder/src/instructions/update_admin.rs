use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa1b028d53cb8b3e4")]
pub struct UpdateAdmin {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateAdminInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub new_admin: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateAdmin {
    type ArrangedAccounts = UpdateAdminInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let admin = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let new_admin = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(UpdateAdminInstructionAccounts {
            admin,
            global_config,
            new_admin,
            event_authority,
            program,
        })
    }
}
