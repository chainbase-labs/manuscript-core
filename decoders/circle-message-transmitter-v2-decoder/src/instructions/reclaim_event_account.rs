use {
    super::super::types::*,
    carbon_core::{account_utils::next_account, borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5ec6b49f83ec0fae")]
pub struct ReclaimEventAccount {
    pub params: ReclaimEventAccountParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReclaimEventAccountInstructionAccounts {
    pub payee: solana_pubkey::Pubkey,
    pub message_transmitter: solana_pubkey::Pubkey,
    pub message_sent_event_data: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ReclaimEventAccount {
    type ArrangedAccounts = ReclaimEventAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payee = next_account(&mut iter)?;
        let message_transmitter = next_account(&mut iter)?;
        let message_sent_event_data = next_account(&mut iter)?;

        Some(ReclaimEventAccountInstructionAccounts {
            payee,
            message_transmitter,
            message_sent_event_data,
        })
    }
}
