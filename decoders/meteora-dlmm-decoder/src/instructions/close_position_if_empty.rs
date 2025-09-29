use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3b7cd4765b986e9d")]
pub struct ClosePositionIfEmpty {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClosePositionIfEmptyInstructionAccounts {
    pub position: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePositionIfEmpty {
    type ArrangedAccounts = ClosePositionIfEmptyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position, sender, rent_receiver, event_authority, program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(ClosePositionIfEmptyInstructionAccounts {
            position: position.pubkey,
            sender: sender.pubkey,
            rent_receiver: rent_receiver.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
