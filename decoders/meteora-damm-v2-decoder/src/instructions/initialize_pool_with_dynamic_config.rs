use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x955248c5fdfc440f")]
pub struct InitializePoolWithDynamicConfig {
    pub params: InitializeCustomizablePoolParameters,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializePoolWithDynamicConfigInstructionAccounts {
    pub creator: solana_pubkey::Pubkey,
    pub position_nft_mint: solana_pubkey::Pubkey,
    pub position_nft_account: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub pool_creator_authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub token_a_vault: solana_pubkey::Pubkey,
    pub token_b_vault: solana_pubkey::Pubkey,
    pub payer_token_a: solana_pubkey::Pubkey,
    pub payer_token_b: solana_pubkey::Pubkey,
    pub token_a_program: solana_pubkey::Pubkey,
    pub token_b_program: solana_pubkey::Pubkey,
    pub token_2022_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePoolWithDynamicConfig {
    type ArrangedAccounts = InitializePoolWithDynamicConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [creator, position_nft_mint, position_nft_account, payer, pool_creator_authority, config, pool_authority, pool, position, token_a_mint, token_b_mint, token_a_vault, token_b_vault, payer_token_a, payer_token_b, token_a_program, token_b_program, token_2022_program, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializePoolWithDynamicConfigInstructionAccounts {
            creator: creator.pubkey,
            position_nft_mint: position_nft_mint.pubkey,
            position_nft_account: position_nft_account.pubkey,
            payer: payer.pubkey,
            pool_creator_authority: pool_creator_authority.pubkey,
            config: config.pubkey,
            pool_authority: pool_authority.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            payer_token_a: payer_token_a.pubkey,
            payer_token_b: payer_token_b.pubkey,
            token_a_program: token_a_program.pubkey,
            token_b_program: token_b_program.pubkey,
            token_2022_program: token_2022_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
