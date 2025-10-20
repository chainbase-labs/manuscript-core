use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbc32f9a55d97263f")]
pub struct FundReward {
    pub reward_index: u8,
    pub amount: u64,
    pub carry_forward: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct FundRewardInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub reward_vault: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub funder_token_account: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FundReward {
    type ArrangedAccounts = FundRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, reward_vault, reward_mint, funder_token_account, funder, token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(FundRewardInstructionAccounts {
            pool: pool.pubkey,
            reward_vault: reward_vault.pubkey,
            reward_mint: reward_mint.pubkey,
            funder_token_account: funder_token_account.pubkey,
            funder: funder.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
