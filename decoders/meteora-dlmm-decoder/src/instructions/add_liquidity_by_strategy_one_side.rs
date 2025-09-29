use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2905eeaf64e106cd")]
pub struct AddLiquidityByStrategyOneSide {
    pub liquidity_parameter: LiquidityParameterByStrategyOneSide,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityByStrategyOneSideInstructionAccounts {
    pub position: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_pubkey::Pubkey,
    pub user_token: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub bin_array_lower: solana_pubkey::Pubkey,
    pub bin_array_upper: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl carbon_core::deserialize::ArrangeAccounts for AddLiquidityByStrategyOneSide {
    type ArrangedAccounts = AddLiquidityByStrategyOneSideInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position, lb_pair, bin_array_bitmap_extension, user_token, reserve, token_mint, bin_array_lower, bin_array_upper, sender, token_program, event_authority, program, remaining_accounts @ ..] =
            accounts
        else {
            return None;
        };

        Some(AddLiquidityByStrategyOneSideInstructionAccounts {
            position: position.pubkey,
            lb_pair: lb_pair.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            user_token: user_token.pubkey,
            reserve: reserve.pubkey,
            token_mint: token_mint.pubkey,
            bin_array_lower: bin_array_lower.pubkey,
            bin_array_upper: bin_array_upper.pubkey,
            sender: sender.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
