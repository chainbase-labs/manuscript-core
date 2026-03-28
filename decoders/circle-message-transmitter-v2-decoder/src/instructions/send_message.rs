use {
    super::super::types::*,
    carbon_core::{account_utils::next_account, borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x392822b2bd0a411a")]
pub struct SendMessage {
    pub params: SendMessageParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SendMessageInstructionAccounts {
    pub event_rent_payer: solana_pubkey::Pubkey,
    pub sender_authority_pda: solana_pubkey::Pubkey,
    pub message_transmitter: solana_pubkey::Pubkey,
    pub message_sent_event_data: solana_pubkey::Pubkey,
    pub sender_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SendMessage {
    type ArrangedAccounts = SendMessageInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let event_rent_payer = next_account(&mut iter)?;
        let sender_authority_pda = next_account(&mut iter)?;
        let message_transmitter = next_account(&mut iter)?;
        let message_sent_event_data = next_account(&mut iter)?;
        let sender_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(SendMessageInstructionAccounts {
            event_rent_payer,
            sender_authority_pda,
            message_transmitter,
            message_sent_event_data,
            sender_program,
            system_program,
        })
    }
}
