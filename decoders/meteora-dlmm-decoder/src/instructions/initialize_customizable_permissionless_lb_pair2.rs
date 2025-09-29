use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf349817e3313f16b")]
pub struct InitializeCustomizablePermissionlessLbPair2 {
    pub params: CustomizableParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeCustomizablePermissionlessLbPair2InstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_pubkey::Pubkey,
    pub token_mint_x: solana_pubkey::Pubkey,
    pub token_mint_y: solana_pubkey::Pubkey,
    pub reserve_x: solana_pubkey::Pubkey,
    pub reserve_y: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub user_token_x: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub token_badge_x: solana_pubkey::Pubkey,
    pub token_badge_y: solana_pubkey::Pubkey,
    pub token_program_x: solana_pubkey::Pubkey,
    pub token_program_y: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub user_token_y: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeCustomizablePermissionlessLbPair2 {
    type ArrangedAccounts = InitializeCustomizablePermissionlessLbPair2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, bin_array_bitmap_extension, token_mint_x, token_mint_y, reserve_x, reserve_y, oracle, user_token_x, funder, token_badge_x, token_badge_y, token_program_x, token_program_y, system_program, user_token_y, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(
            InitializeCustomizablePermissionlessLbPair2InstructionAccounts {
                lb_pair: lb_pair.pubkey,
                bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
                token_mint_x: token_mint_x.pubkey,
                token_mint_y: token_mint_y.pubkey,
                reserve_x: reserve_x.pubkey,
                reserve_y: reserve_y.pubkey,
                oracle: oracle.pubkey,
                user_token_x: user_token_x.pubkey,
                funder: funder.pubkey,
                token_badge_x: token_badge_x.pubkey,
                token_badge_y: token_badge_y.pubkey,
                token_program_x: token_program_x.pubkey,
                token_program_y: token_program_y.pubkey,
                system_program: system_program.pubkey,
                user_token_y: user_token_y.pubkey,
                event_authority: event_authority.pubkey,
                program: program.pubkey,
            },
        )
    }
}
