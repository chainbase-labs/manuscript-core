use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0c")]
pub struct SimulateInfo {
    pub param: u8,
    pub swap_base_in_value: Option<SwapInstructionBaseIn>,
    pub swap_base_out_value: Option<SwapInstructionBaseOut>,
}

pub struct SimulateInfoInstructionAccounts {
    pub amm: solana_pubkey::Pubkey,
    pub amm_authority: solana_pubkey::Pubkey,
    pub amm_open_orders: solana_pubkey::Pubkey,
    pub pool_coin_token_account: solana_pubkey::Pubkey,
    pub pool_pc_token_account: solana_pubkey::Pubkey,
    pub lp_mint_address: solana_pubkey::Pubkey,
    pub serum_market: solana_pubkey::Pubkey,
    pub serum_event_queue: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SimulateInfo {
    type ArrangedAccounts = SimulateInfoInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [amm, amm_authority, amm_open_orders, pool_coin_token_account, pool_pc_token_account, lp_mint_address, serum_market, serum_event_queue, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SimulateInfoInstructionAccounts {
            amm: amm.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            pool_coin_token_account: pool_coin_token_account.pubkey,
            pool_pc_token_account: pool_pc_token_account.pubkey,
            lp_mint_address: lp_mint_address.pubkey,
            serum_market: serum_market.pubkey,
            serum_event_queue: serum_event_queue.pubkey,
        })
    }
}
