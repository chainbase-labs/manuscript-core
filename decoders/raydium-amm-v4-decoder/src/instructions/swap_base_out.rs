use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0b")]
pub struct SwapBaseOut {
    pub max_amount_in: u64,
    pub amount_out: u64,
}

#[derive(Debug)]
pub struct SwapBaseOutInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub amm: solana_pubkey::Pubkey,
    pub amm_authority: solana_pubkey::Pubkey,
    pub amm_open_orders: solana_pubkey::Pubkey,
    pub amm_target_orders: Option<solana_pubkey::Pubkey>,
    pub pool_coin_token_account: solana_pubkey::Pubkey,
    pub pool_pc_token_account: solana_pubkey::Pubkey,
    pub serum_program: solana_pubkey::Pubkey,
    pub serum_market: solana_pubkey::Pubkey,
    pub serum_bids: solana_pubkey::Pubkey,
    pub serum_asks: solana_pubkey::Pubkey,
    pub serum_event_queue: solana_pubkey::Pubkey,
    pub serum_coin_vault_account: solana_pubkey::Pubkey,
    pub serum_pc_vault_account: solana_pubkey::Pubkey,
    pub serum_vault_signer: solana_pubkey::Pubkey,
    pub user_source_token_account: solana_pubkey::Pubkey,
    pub user_destination_token_account: solana_pubkey::Pubkey,
    pub user_source_owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapBaseOut {
    type ArrangedAccounts = SwapBaseOutInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        match accounts.len() {
            17 => {
                let [token_program, amm, amm_authority, amm_open_orders, pool_coin_token_account, pool_pc_token_account, serum_program, serum_market, serum_bids, serum_asks, serum_event_queue, serum_coin_vault_account, serum_pc_vault_account, serum_vault_signer, user_source_token_account, user_destination_token_account, user_source_owner, _remaining @ ..] =
                    accounts
                else {
                    return None;
                };

                Some(SwapBaseOutInstructionAccounts {
                    token_program: token_program.pubkey,
                    amm: amm.pubkey,
                    amm_authority: amm_authority.pubkey,
                    amm_open_orders: amm_open_orders.pubkey,
                    amm_target_orders: None,
                    pool_coin_token_account: pool_coin_token_account.pubkey,
                    pool_pc_token_account: pool_pc_token_account.pubkey,
                    serum_program: serum_program.pubkey,
                    serum_market: serum_market.pubkey,
                    serum_bids: serum_bids.pubkey,
                    serum_asks: serum_asks.pubkey,
                    serum_event_queue: serum_event_queue.pubkey,
                    serum_coin_vault_account: serum_coin_vault_account.pubkey,
                    serum_pc_vault_account: serum_pc_vault_account.pubkey,
                    serum_vault_signer: serum_vault_signer.pubkey,
                    user_source_token_account: user_source_token_account.pubkey,
                    user_destination_token_account: user_destination_token_account.pubkey,
                    user_source_owner: user_source_owner.pubkey,
                })
            }
            18 => {
                let [token_program, amm, amm_authority, amm_open_orders, amm_target_orders, pool_coin_token_account, pool_pc_token_account, serum_program, serum_market, serum_bids, serum_asks, serum_event_queue, serum_coin_vault_account, serum_pc_vault_account, serum_vault_signer, user_source_token_account, user_destination_token_account, user_source_owner, _remaining @ ..] =
                    accounts
                else {
                    return None;
                };

                Some(SwapBaseOutInstructionAccounts {
                    token_program: token_program.pubkey,
                    amm: amm.pubkey,
                    amm_authority: amm_authority.pubkey,
                    amm_open_orders: amm_open_orders.pubkey,
                    amm_target_orders: Some(amm_target_orders.pubkey),
                    pool_coin_token_account: pool_coin_token_account.pubkey,
                    pool_pc_token_account: pool_pc_token_account.pubkey,
                    serum_program: serum_program.pubkey,
                    serum_market: serum_market.pubkey,
                    serum_bids: serum_bids.pubkey,
                    serum_asks: serum_asks.pubkey,
                    serum_event_queue: serum_event_queue.pubkey,
                    serum_coin_vault_account: serum_coin_vault_account.pubkey,
                    serum_pc_vault_account: serum_pc_vault_account.pubkey,
                    serum_vault_signer: serum_vault_signer.pubkey,
                    user_source_token_account: user_source_token_account.pubkey,
                    user_destination_token_account: user_destination_token_account.pubkey,
                    user_source_owner: user_source_owner.pubkey,
                })
            }
            _ => None,
        }
    }
}
