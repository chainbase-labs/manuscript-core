use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xad834e2696a57b0f")]
pub struct CommissionSplSwap2 {
    pub data: CommissionSwapArgs,
    pub order_id: u64,
}

pub struct CommissionSplSwap2InstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub source_token_account: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub source_mint: solana_pubkey::Pubkey,
    pub destination_mint: solana_pubkey::Pubkey,
    pub commission_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CommissionSplSwap2 {
    type ArrangedAccounts = CommissionSplSwap2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, source_token_account, destination_token_account, source_mint, destination_mint, commission_token_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CommissionSplSwap2InstructionAccounts {
            payer: payer.pubkey,
            source_token_account: source_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            source_mint: source_mint.pubkey,
            destination_mint: destination_mint.pubkey,
            commission_token_account: commission_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
