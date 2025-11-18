use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x07")]
pub struct WithdrawPnl {}

pub struct WithdrawPnlInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub amm: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub amm_authority: solana_pubkey::Pubkey,
    pub amm_open_orders: solana_pubkey::Pubkey,
    pub pool_coin_token_account: solana_pubkey::Pubkey,
    pub pool_pc_token_account: solana_pubkey::Pubkey,
    pub coin_pnl_token_account: solana_pubkey::Pubkey,
    pub pc_pnl_token_account: solana_pubkey::Pubkey,
    pub pnl_owner_account: solana_pubkey::Pubkey,
    pub amm_target_orders: solana_pubkey::Pubkey,
    pub serum_program: solana_pubkey::Pubkey,
    pub serum_market: solana_pubkey::Pubkey,
    pub serum_event_queue: solana_pubkey::Pubkey,
    pub serum_coin_vault_account: solana_pubkey::Pubkey,
    pub serum_pc_vault_account: solana_pubkey::Pubkey,
    pub serum_vault_signer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawPnl {
    type ArrangedAccounts = WithdrawPnlInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, amm, amm_config, amm_authority, amm_open_orders, pool_coin_token_account, pool_pc_token_account, coin_pnl_token_account, pc_pnl_token_account, pnl_owner_account, amm_target_orders, serum_program, serum_market, serum_event_queue, serum_coin_vault_account, serum_pc_vault_account, serum_vault_signer, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawPnlInstructionAccounts {
            token_program: token_program.pubkey,
            amm: amm.pubkey,
            amm_config: amm_config.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            pool_coin_token_account: pool_coin_token_account.pubkey,
            pool_pc_token_account: pool_pc_token_account.pubkey,
            coin_pnl_token_account: coin_pnl_token_account.pubkey,
            pc_pnl_token_account: pc_pnl_token_account.pubkey,
            pnl_owner_account: pnl_owner_account.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            serum_program: serum_program.pubkey,
            serum_market: serum_market.pubkey,
            serum_event_queue: serum_event_queue.pubkey,
            serum_coin_vault_account: serum_coin_vault_account.pubkey,
            serum_pc_vault_account: serum_pc_vault_account.pubkey,
            serum_vault_signer: serum_vault_signer.pubkey,
        })
    }
}
