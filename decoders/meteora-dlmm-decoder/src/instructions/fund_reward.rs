use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbc32f9a55d97263f")]
pub struct FundReward {
    pub reward_index: u64,
    pub amount: u64,
    pub carry_forward: bool,
    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct FundRewardInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub reward_vault: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub funder_token_account: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub bin_array: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FundReward {
    type ArrangedAccounts = FundRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, reward_vault, reward_mint, funder_token_account, funder, bin_array, token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(FundRewardInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            reward_vault: reward_vault.pubkey,
            reward_mint: reward_mint.pubkey,
            funder_token_account: funder_token_account.pubkey,
            funder: funder.pubkey,
            bin_array: bin_array.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
