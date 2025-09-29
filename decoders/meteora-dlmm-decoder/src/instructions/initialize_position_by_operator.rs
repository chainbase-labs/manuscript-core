use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfbbdbef475fe2394")]
pub struct InitializePositionByOperator {
    pub lower_bin_id: i32,
    pub width: i32,
    pub fee_owner: solana_pubkey::Pubkey,
    pub lock_release_point: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializePositionByOperatorInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub base: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub operator_token_x: solana_pubkey::Pubkey,
    pub owner_token_x: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePositionByOperator {
    type ArrangedAccounts = InitializePositionByOperatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, base, position, lb_pair, owner, operator, operator_token_x, owner_token_x, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializePositionByOperatorInstructionAccounts {
            payer: payer.pubkey,
            base: base.pubkey,
            position: position.pubkey,
            lb_pair: lb_pair.pubkey,
            owner: owner.pubkey,
            operator: operator.pubkey,
            operator_token_x: operator_token_x.pubkey,
            owner_token_x: owner_token_x.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
