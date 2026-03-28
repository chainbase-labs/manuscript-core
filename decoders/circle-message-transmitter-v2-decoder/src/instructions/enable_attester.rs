use {
    super::super::types::*,
    carbon_core::{account_utils::next_account, borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x020bc173059404c6")]
pub struct EnableAttester {
    pub params: EnableAttesterParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct EnableAttesterInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub attester_manager: solana_pubkey::Pubkey,
    pub message_transmitter: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EnableAttester {
    type ArrangedAccounts = EnableAttesterInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let attester_manager = next_account(&mut iter)?;
        let message_transmitter = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(EnableAttesterInstructionAccounts {
            payer,
            attester_manager,
            message_transmitter,
            system_program,
            event_authority,
            program,
        })
    }
}
