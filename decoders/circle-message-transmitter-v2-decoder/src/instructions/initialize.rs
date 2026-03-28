use {
    super::super::types::*,
    carbon_core::{account_utils::next_account, borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize {
    pub params: InitializeParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub upgrade_authority: solana_pubkey::Pubkey,
    pub message_transmitter: solana_pubkey::Pubkey,
    pub message_transmitter_program_data: solana_pubkey::Pubkey,
    pub message_transmitter_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let upgrade_authority = next_account(&mut iter)?;
        let message_transmitter = next_account(&mut iter)?;
        let message_transmitter_program_data = next_account(&mut iter)?;
        let message_transmitter_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(InitializeInstructionAccounts {
            payer,
            upgrade_authority,
            message_transmitter,
            message_transmitter_program_data,
            message_transmitter_program,
            system_program,
            event_authority,
            program,
        })
    }
}
