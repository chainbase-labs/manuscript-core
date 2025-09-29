use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6c66d555fb033515")]
pub struct InitializePermissionLbPair {
    pub ix_data: InitPermissionPairIx,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializePermissionLbPairInstructionAccounts {
    pub base: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_pubkey::Pubkey,
    pub token_mint_x: solana_pubkey::Pubkey,
    pub token_mint_y: solana_pubkey::Pubkey,
    pub reserve_x: solana_pubkey::Pubkey,
    pub reserve_y: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub token_badge_x: solana_pubkey::Pubkey,
    pub token_badge_y: solana_pubkey::Pubkey,
    pub token_program_x: solana_pubkey::Pubkey,
    pub token_program_y: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePermissionLbPair {
    type ArrangedAccounts = InitializePermissionLbPairInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [base, lb_pair, bin_array_bitmap_extension, token_mint_x, token_mint_y, reserve_x, reserve_y, oracle, admin, token_badge_x, token_badge_y, token_program_x, token_program_y, system_program, rent, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializePermissionLbPairInstructionAccounts {
            base: base.pubkey,
            lb_pair: lb_pair.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            token_mint_x: token_mint_x.pubkey,
            token_mint_y: token_mint_y.pubkey,
            reserve_x: reserve_x.pubkey,
            reserve_y: reserve_y.pubkey,
            oracle: oracle.pubkey,
            admin: admin.pubkey,
            token_badge_x: token_badge_x.pubkey,
            token_badge_y: token_badge_y.pubkey,
            token_program_x: token_program_x.pubkey,
            token_program_y: token_program_y.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
