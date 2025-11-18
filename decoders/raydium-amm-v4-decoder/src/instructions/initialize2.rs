use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x01")]
pub struct Initialize2 {
    pub nonce: u8,
    pub open_time: u64,
    pub init_pc_amount: u64,
    pub init_coin_amount: u64,
}

pub struct Initialize2InstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub spl_associated_token_account: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub amm: solana_pubkey::Pubkey,
    pub amm_authority: solana_pubkey::Pubkey,
    pub amm_open_orders: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub coin_mint: solana_pubkey::Pubkey,
    pub pc_mint: solana_pubkey::Pubkey,
    pub pool_coin_token_account: solana_pubkey::Pubkey,
    pub pool_pc_token_account: solana_pubkey::Pubkey,
    pub pool_withdraw_queue: solana_pubkey::Pubkey,
    pub amm_target_orders: solana_pubkey::Pubkey,
    pub pool_temp_lp: solana_pubkey::Pubkey,
    pub serum_program: solana_pubkey::Pubkey,
    pub serum_market: solana_pubkey::Pubkey,
    pub user_wallet: solana_pubkey::Pubkey,
    pub user_token_coin: solana_pubkey::Pubkey,
    pub user_token_pc: solana_pubkey::Pubkey,
    pub user_lp_token_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize2 {
    type ArrangedAccounts = Initialize2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, spl_associated_token_account, system_program, rent, amm, amm_authority, amm_open_orders, lp_mint, coin_mint, pc_mint, pool_coin_token_account, pool_pc_token_account, pool_withdraw_queue, amm_target_orders, pool_temp_lp, serum_program, serum_market, user_wallet, user_token_coin, user_token_pc, user_lp_token_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(Initialize2InstructionAccounts {
            token_program: token_program.pubkey,
            spl_associated_token_account: spl_associated_token_account.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            amm: amm.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            lp_mint: lp_mint.pubkey,
            coin_mint: coin_mint.pubkey,
            pc_mint: pc_mint.pubkey,
            pool_coin_token_account: pool_coin_token_account.pubkey,
            pool_pc_token_account: pool_pc_token_account.pubkey,
            pool_withdraw_queue: pool_withdraw_queue.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            pool_temp_lp: pool_temp_lp.pubkey,
            serum_program: serum_program.pubkey,
            serum_market: serum_market.pubkey,
            user_wallet: user_wallet.pubkey,
            user_token_coin: user_token_coin.pubkey,
            user_token_pc: user_token_pc.pubkey,
            user_lp_token_account: user_lp_token_account.pubkey,
        })
    }
}
