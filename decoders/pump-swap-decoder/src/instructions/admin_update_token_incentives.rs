use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd10b7357d5177ccc")]
pub struct AdminUpdateTokenIncentives {
    pub start_time: i64,
    pub end_time: i64,
    pub seconds_in_a_day: i64,
    pub day_number: u64,
    pub token_supply_per_day: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AdminUpdateTokenIncentivesInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub global_volume_accumulator: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub global_incentive_token_account: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminUpdateTokenIncentives {
    type ArrangedAccounts = AdminUpdateTokenIncentivesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let admin = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let global_volume_accumulator = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let global_incentive_token_account = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(AdminUpdateTokenIncentivesInstructionAccounts {
            admin,
            global_config,
            global_volume_accumulator,
            mint,
            global_incentive_token_account,
            associated_token_program,
            system_program,
            token_program,
            event_authority,
            program,
        })
    }
}
