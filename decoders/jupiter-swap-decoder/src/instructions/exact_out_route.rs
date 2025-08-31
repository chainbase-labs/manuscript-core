use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd033ef977b2bed5c")]
pub struct ExactOutRoute {
    pub route_plan: Vec<RoutePlanStep>,
    pub out_amount: u64,
    pub quoted_in_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}

pub struct ExactOutRouteInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub user_transfer_authority: solana_pubkey::Pubkey,
    pub user_source_token_account: solana_pubkey::Pubkey,
    pub user_destination_token_account: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub source_mint: solana_pubkey::Pubkey,
    pub destination_mint: solana_pubkey::Pubkey,
    pub platform_fee_account: solana_pubkey::Pubkey,
    pub token_2022_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ExactOutRoute {
    type ArrangedAccounts = ExactOutRouteInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, user_transfer_authority, user_source_token_account, user_destination_token_account, destination_token_account, source_mint, destination_mint, platform_fee_account, token_2022_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ExactOutRouteInstructionAccounts {
            token_program: token_program.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            user_source_token_account: user_source_token_account.pubkey,
            user_destination_token_account: user_destination_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            source_mint: source_mint.pubkey,
            destination_mint: destination_mint.pubkey,
            platform_fee_account: platform_fee_account.pubkey,
            token_2022_program: token_2022_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
