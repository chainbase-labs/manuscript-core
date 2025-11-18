use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x03")]
pub struct Deposit {
    pub max_coin_amount: u64,
    pub max_pc_amount: u64,
    pub base_side: u64,
}

pub struct DepositInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub amm: solana_pubkey::Pubkey,
    pub amm_authority: solana_pubkey::Pubkey,
    pub amm_open_orders: solana_pubkey::Pubkey,
    pub amm_target_orders: solana_pubkey::Pubkey,
    pub lp_mint_address: solana_pubkey::Pubkey,
    pub pool_coin_token_account: solana_pubkey::Pubkey,
    pub pool_pc_token_account: solana_pubkey::Pubkey,
    pub serum_market: solana_pubkey::Pubkey,
    pub user_coin_token_account: solana_pubkey::Pubkey,
    pub user_pc_token_account: solana_pubkey::Pubkey,
    pub user_lp_token_account: solana_pubkey::Pubkey,
    pub user_owner: solana_pubkey::Pubkey,
    pub serum_event_queue: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, amm, amm_authority, amm_open_orders, amm_target_orders, lp_mint_address, pool_coin_token_account, pool_pc_token_account, serum_market, user_coin_token_account, user_pc_token_account, user_lp_token_account, user_owner, serum_event_queue, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositInstructionAccounts {
            token_program: token_program.pubkey,
            amm: amm.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            lp_mint_address: lp_mint_address.pubkey,
            pool_coin_token_account: pool_coin_token_account.pubkey,
            pool_pc_token_account: pool_pc_token_account.pubkey,
            serum_market: serum_market.pubkey,
            user_coin_token_account: user_coin_token_account.pubkey,
            user_pc_token_account: user_pc_token_account.pubkey,
            user_lp_token_account: user_lp_token_account.pubkey,
            user_owner: user_owner.pubkey,
            serum_event_queue: serum_event_queue.pubkey,
        })
    }
}
