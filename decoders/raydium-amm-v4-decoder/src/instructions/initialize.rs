use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00")]
pub struct Initialize {
    pub nonce: u8,
    pub open_time: u64,
}

pub struct InitializeInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub amm: solana_pubkey::Pubkey,
    pub amm_authority: solana_pubkey::Pubkey,
    pub amm_open_orders: solana_pubkey::Pubkey,
    pub lp_mint_address: solana_pubkey::Pubkey,
    pub coin_mint_address: solana_pubkey::Pubkey,
    pub pc_mint_address: solana_pubkey::Pubkey,
    pub pool_coin_token_account: solana_pubkey::Pubkey,
    pub pool_pc_token_account: solana_pubkey::Pubkey,
    pub pool_withdraw_queue: solana_pubkey::Pubkey,
    pub pool_target_orders_account: solana_pubkey::Pubkey,
    pub user_lp_token_account: solana_pubkey::Pubkey,
    pub pool_temp_lp_token_account: solana_pubkey::Pubkey,
    pub serum_program: solana_pubkey::Pubkey,
    pub serum_market: solana_pubkey::Pubkey,
    pub user_wallet: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, system_program, rent, amm, amm_authority, amm_open_orders, lp_mint_address, coin_mint_address, pc_mint_address, pool_coin_token_account, pool_pc_token_account, pool_withdraw_queue, pool_target_orders_account, user_lp_token_account, pool_temp_lp_token_account, serum_program, serum_market, user_wallet, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeInstructionAccounts {
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            amm: amm.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            lp_mint_address: lp_mint_address.pubkey,
            coin_mint_address: coin_mint_address.pubkey,
            pc_mint_address: pc_mint_address.pubkey,
            pool_coin_token_account: pool_coin_token_account.pubkey,
            pool_pc_token_account: pool_pc_token_account.pubkey,
            pool_withdraw_queue: pool_withdraw_queue.pubkey,
            pool_target_orders_account: pool_target_orders_account.pubkey,
            user_lp_token_account: user_lp_token_account.pubkey,
            pool_temp_lp_token_account: pool_temp_lp_token_account.pubkey,
            serum_program: serum_program.pubkey,
            serum_market: serum_market.pubkey,
            user_wallet: user_wallet.pubkey,
        })
    }
}
