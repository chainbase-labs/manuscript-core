use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe517cb977ae3ad2a")]
pub struct Route {
    pub route_plan: Vec<RoutePlanStep>,
    pub in_amount: u64,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}

pub struct RouteInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub user_transfer_authority: solana_pubkey::Pubkey,
    pub user_source_token_account: solana_pubkey::Pubkey,
    pub user_destination_token_account: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub destination_mint: solana_pubkey::Pubkey,
    pub platform_fee_account: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Route {
    type ArrangedAccounts = RouteInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, user_transfer_authority, user_source_token_account, user_destination_token_account, destination_token_account, destination_mint, platform_fee_account, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RouteInstructionAccounts {
            token_program: token_program.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            user_source_token_account: user_source_token_account.pubkey,
            user_destination_token_account: user_destination_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            destination_mint: destination_mint.pubkey,
            platform_fee_account: platform_fee_account.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
