use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1004471ccc01281b")]
pub struct ClaimTokenIncentives {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimTokenIncentivesInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub user_ata: solana_pubkey::Pubkey,
    pub global_volume_accumulator: solana_pubkey::Pubkey,
    pub global_incentive_token_account: solana_pubkey::Pubkey,
    pub user_volume_accumulator: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimTokenIncentives {
    type ArrangedAccounts = ClaimTokenIncentivesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let user = next_account(&mut iter)?;
        let user_ata = next_account(&mut iter)?;
        let global_volume_accumulator = next_account(&mut iter)?;
        let global_incentive_token_account = next_account(&mut iter)?;
        let user_volume_accumulator = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;

        Some(ClaimTokenIncentivesInstructionAccounts {
            user,
            user_ata,
            global_volume_accumulator,
            global_incentive_token_account,
            user_volume_accumulator,
            mint,
            token_program,
            system_program,
            associated_token_program,
            event_authority,
            program,
            payer,
        })
    }
}
