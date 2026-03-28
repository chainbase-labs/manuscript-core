use {
    super::super::types::*,
    carbon_core::{account_utils::next_account, borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xac172b0deed55596")]
pub struct AcceptOwnership {
    pub params: AcceptOwnershipParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AcceptOwnershipInstructionAccounts {
    pub pending_owner: solana_pubkey::Pubkey,
    pub message_transmitter: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AcceptOwnership {
    type ArrangedAccounts = AcceptOwnershipInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let pending_owner = next_account(&mut iter)?;
        let message_transmitter = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(AcceptOwnershipInstructionAccounts {
            pending_owner,
            message_transmitter,
            event_authority,
            program,
        })
    }
}
