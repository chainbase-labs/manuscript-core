use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf228759149606968")]
pub struct AdminSetCoinCreator {
    pub coin_creator: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AdminSetCoinCreatorInstructionAccounts {
    pub admin_set_coin_creator_authority: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminSetCoinCreator {
    type ArrangedAccounts = AdminSetCoinCreatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let admin_set_coin_creator_authority = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(AdminSetCoinCreatorInstructionAccounts {
            admin_set_coin_creator_authority,
            global_config,
            pool,
            event_authority,
            program,
        })
    }
}
