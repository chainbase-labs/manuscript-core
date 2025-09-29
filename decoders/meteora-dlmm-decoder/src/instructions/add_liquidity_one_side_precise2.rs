use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2133a3c975627de7")]
pub struct AddLiquidityOneSidePrecise2 {
    pub liquidity_parameter: AddLiquiditySingleSidePreciseParameter2,
    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityOneSidePrecise2InstructionAccounts {
    pub position: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_pubkey::Pubkey,
    pub user_token: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddLiquidityOneSidePrecise2 {
    type ArrangedAccounts = AddLiquidityOneSidePrecise2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position, lb_pair, bin_array_bitmap_extension, user_token, reserve, token_mint, sender, token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AddLiquidityOneSidePrecise2InstructionAccounts {
            position: position.pubkey,
            lb_pair: lb_pair.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            user_token: user_token.pubkey,
            reserve: reserve.pubkey,
            token_mint: token_mint.pubkey,
            sender: sender.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
