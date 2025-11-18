use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02")]
pub struct MonitorStep {
    pub plan_order_limit: u16,
    pub place_order_limit: u16,
    pub cancel_order_limit: u16,
}

pub struct MonitorStepInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub clock: solana_pubkey::Pubkey,
    pub amm: solana_pubkey::Pubkey,
    pub amm_authority: solana_pubkey::Pubkey,
    pub amm_open_orders: solana_pubkey::Pubkey,
    pub amm_target_orders: solana_pubkey::Pubkey,
    pub pool_coin_token_account: solana_pubkey::Pubkey,
    pub pool_pc_token_account: solana_pubkey::Pubkey,
    pub pool_withdraw_queue: solana_pubkey::Pubkey,
    pub serum_program: solana_pubkey::Pubkey,
    pub serum_market: solana_pubkey::Pubkey,
    pub serum_coin_vault_account: solana_pubkey::Pubkey,
    pub serum_pc_vault_account: solana_pubkey::Pubkey,
    pub serum_vault_signer: solana_pubkey::Pubkey,
    pub serum_req_q: solana_pubkey::Pubkey,
    pub serum_event_q: solana_pubkey::Pubkey,
    pub serum_bids: solana_pubkey::Pubkey,
    pub serum_asks: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MonitorStep {
    type ArrangedAccounts = MonitorStepInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, rent, clock, amm, amm_authority, amm_open_orders, amm_target_orders, pool_coin_token_account, pool_pc_token_account, pool_withdraw_queue, serum_program, serum_market, serum_coin_vault_account, serum_pc_vault_account, serum_vault_signer, serum_req_q, serum_event_q, serum_bids, serum_asks, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MonitorStepInstructionAccounts {
            token_program: token_program.pubkey,
            rent: rent.pubkey,
            clock: clock.pubkey,
            amm: amm.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            pool_coin_token_account: pool_coin_token_account.pubkey,
            pool_pc_token_account: pool_pc_token_account.pubkey,
            pool_withdraw_queue: pool_withdraw_queue.pubkey,
            serum_program: serum_program.pubkey,
            serum_market: serum_market.pubkey,
            serum_coin_vault_account: serum_coin_vault_account.pubkey,
            serum_pc_vault_account: serum_pc_vault_account.pubkey,
            serum_vault_signer: serum_vault_signer.pubkey,
            serum_req_q: serum_req_q.pubkey,
            serum_event_q: serum_event_q.pubkey,
            serum_bids: serum_bids.pubkey,
            serum_asks: serum_asks.pubkey,
        })
    }
}
