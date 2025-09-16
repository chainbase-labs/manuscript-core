use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf8c69e91e17587c8")]
pub struct Swap {
    pub data: SwapArgs,
}

pub struct SwapInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub source_token_account: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub source_mint: solana_pubkey::Pubkey,
    pub destination_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Swap {
    type ArrangedAccounts = SwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, source_token_account, destination_token_account, source_mint, destination_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapInstructionAccounts {
            payer: payer.pubkey,
            source_token_account: source_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            source_mint: source_mint.pubkey,
            destination_mint: destination_mint.pubkey,
        })
    }
}
