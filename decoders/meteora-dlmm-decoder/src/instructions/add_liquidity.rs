use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb59d59438fb63448")]
pub struct AddLiquidity {
    pub liquidity_parameter: LiquidityParameter,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityInstructionAccounts {
    pub position: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_pubkey::Pubkey,
    pub user_token_x: solana_pubkey::Pubkey,
    pub user_token_y: solana_pubkey::Pubkey,
    pub reserve_x: solana_pubkey::Pubkey,
    pub reserve_y: solana_pubkey::Pubkey,
    pub token_x_mint: solana_pubkey::Pubkey,
    pub token_y_mint: solana_pubkey::Pubkey,
    pub bin_array_lower: solana_pubkey::Pubkey,
    pub bin_array_upper: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub token_x_program: solana_pubkey::Pubkey,
    pub token_y_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl carbon_core::deserialize::ArrangeAccounts for AddLiquidity {
    type ArrangedAccounts = AddLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position, lb_pair, bin_array_bitmap_extension, user_token_x, user_token_y, reserve_x, reserve_y, token_x_mint, token_y_mint, bin_array_lower, bin_array_upper, sender, token_x_program, token_y_program, event_authority, program, remaining_accounts @ ..] =
            accounts
        else {
            return None;
        };

        Some(AddLiquidityInstructionAccounts {
            position: position.pubkey,
            lb_pair: lb_pair.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            user_token_x: user_token_x.pubkey,
            user_token_y: user_token_y.pubkey,
            reserve_x: reserve_x.pubkey,
            reserve_y: reserve_y.pubkey,
            token_x_mint: token_x_mint.pubkey,
            token_y_mint: token_y_mint.pubkey,
            bin_array_lower: bin_array_lower.pubkey,
            bin_array_upper: bin_array_upper.pubkey,
            sender: sender.pubkey,
            token_x_program: token_x_program.pubkey,
            token_y_program: token_y_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
