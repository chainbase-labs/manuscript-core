use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdbc0ea47bebf6650")]
pub struct InitializePosition {
    pub lower_bin_id: i32,
    pub width: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializePositionInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePosition {
    type ArrangedAccounts = InitializePositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, position, lb_pair, owner, system_program, rent, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializePositionInstructionAccounts {
            payer: payer.pubkey,
            position: position.pubkey,
            lb_pair: lb_pair.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
