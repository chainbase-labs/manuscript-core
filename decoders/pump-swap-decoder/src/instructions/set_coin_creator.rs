use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd295802dbc3a4eaf")]
pub struct SetCoinCreator {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetCoinCreatorInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCoinCreator {
    type ArrangedAccounts = SetCoinCreatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let pool = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(SetCoinCreatorInstructionAccounts {
            pool,
            metadata,
            bonding_curve,
            event_authority,
            program,
        })
    }
}
