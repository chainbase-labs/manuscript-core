use {
    super::super::types::*,
    carbon_core::{account_utils::next_account, borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8cabd38439c910fe")]
pub struct UpdatePauser {
    pub params: UpdatePauserParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdatePauserInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub message_transmitter: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePauser {
    type ArrangedAccounts = UpdatePauserInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let message_transmitter = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(UpdatePauserInstructionAccounts {
            owner,
            message_transmitter,
            event_authority,
            program,
        })
    }
}
