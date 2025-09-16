use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x813b450a844c2314")]
pub struct CommissionSolFromSwap {
    pub args: SwapArgs,
    pub commission_rate: u16,
    pub bridge_to_args: BridgeToArgs,
    pub offset: u8,
    pub len: u8,
}

pub struct CommissionSolFromSwapInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub source_token_account: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub source_mint: solana_pubkey::Pubkey,
    pub destination_mint: solana_pubkey::Pubkey,
    pub bridge_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_2022_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub commission_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CommissionSolFromSwap {
    type ArrangedAccounts = CommissionSolFromSwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, source_token_account, destination_token_account, source_mint, destination_mint, bridge_program, associated_token_program, token_program, token_2022_program, system_program, commission_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CommissionSolFromSwapInstructionAccounts {
            payer: payer.pubkey,
            source_token_account: source_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            source_mint: source_mint.pubkey,
            destination_mint: destination_mint.pubkey,
            bridge_program: bridge_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            token_program: token_program.pubkey,
            token_2022_program: token_2022_program.pubkey,
            system_program: system_program.pubkey,
            commission_account: commission_account.pubkey,
        })
    }
}
