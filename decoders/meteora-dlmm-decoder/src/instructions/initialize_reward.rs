use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5f87c0c4f281e644")]
pub struct InitializeReward {
    pub reward_index: u64,
    pub reward_duration: u64,
    pub funder: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeRewardInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub reward_vault: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub token_badge: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeReward {
    type ArrangedAccounts = InitializeRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, reward_vault, reward_mint, token_badge, admin, token_program, system_program, rent, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeRewardInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            reward_vault: reward_vault.pubkey,
            reward_mint: reward_mint.pubkey,
            token_badge: token_badge.pubkey,
            admin: admin.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
