use {
    super::super::types::*,
    carbon_core::{account_utils::next_account, borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x26907fe11fe1ee19")]
pub struct ReceiveMessage {
    pub params: ReceiveMessageParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReceiveMessageInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub caller: solana_pubkey::Pubkey,
    pub authority_pda: solana_pubkey::Pubkey,
    pub message_transmitter: solana_pubkey::Pubkey,
    pub used_nonce: solana_pubkey::Pubkey,
    pub receiver: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ReceiveMessage {
    type ArrangedAccounts = ReceiveMessageInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let caller = next_account(&mut iter)?;
        let authority_pda = next_account(&mut iter)?;
        let message_transmitter = next_account(&mut iter)?;
        let used_nonce = next_account(&mut iter)?;
        let receiver = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(ReceiveMessageInstructionAccounts {
            payer,
            caller,
            authority_pda,
            message_transmitter,
            used_nonce,
            receiver,
            system_program,
            event_authority,
            program,
        })
    }
}
