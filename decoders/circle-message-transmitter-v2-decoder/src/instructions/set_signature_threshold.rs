use {
    super::super::types::*,
    carbon_core::{account_utils::next_account, borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa3139aa852d1d6db")]
pub struct SetSignatureThreshold {
    pub params: SetSignatureThresholdParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetSignatureThresholdInstructionAccounts {
    pub attester_manager: solana_pubkey::Pubkey,
    pub message_transmitter: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetSignatureThreshold {
    type ArrangedAccounts = SetSignatureThresholdInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let attester_manager = next_account(&mut iter)?;
        let message_transmitter = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(SetSignatureThresholdInstructionAccounts {
            attester_manager,
            message_transmitter,
            event_authority,
            program,
        })
    }
}
