use {
    super::super::types::*,
    crate::accounts::fees::Fees,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x06")]
pub struct SetParams {
    pub param: u8,
    pub value: Option<u64>,
    pub new_pubkey: Option<solana_pubkey::Pubkey>,
    pub fees: Option<Fees>,
    pub last_order_distance: Option<LastOrderDistance>,
    pub need_take_amounts: Option<NeedTake>,
}

pub struct SetParamsInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub amm: solana_pubkey::Pubkey,
    pub amm_authority: solana_pubkey::Pubkey,
    pub amm_open_orders: solana_pubkey::Pubkey,
    pub amm_target_orders: solana_pubkey::Pubkey,
    pub amm_coin_vault: solana_pubkey::Pubkey,
    pub amm_pc_vault: solana_pubkey::Pubkey,
    pub serum_program: solana_pubkey::Pubkey,
    pub serum_market: solana_pubkey::Pubkey,
    pub serum_coin_vault: solana_pubkey::Pubkey,
    pub serum_pc_vault: solana_pubkey::Pubkey,
    pub serum_vault_signer: solana_pubkey::Pubkey,
    pub serum_event_queue: solana_pubkey::Pubkey,
    pub serum_bids: solana_pubkey::Pubkey,
    pub serum_asks: solana_pubkey::Pubkey,
    pub amm_admin_account: solana_pubkey::Pubkey,
    pub new_amm_open_orders_account: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for SetParams {
    type ArrangedAccounts = SetParamsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, amm, amm_authority, amm_open_orders, amm_target_orders, amm_coin_vault, amm_pc_vault, serum_program, serum_market, serum_coin_vault, serum_pc_vault, serum_vault_signer, serum_event_queue, serum_bids, serum_asks, amm_admin_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };
        let new_amm_open_orders_account = accounts.get(16);

        Some(SetParamsInstructionAccounts {
            token_program: token_program.pubkey,
            amm: amm.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            amm_coin_vault: amm_coin_vault.pubkey,
            amm_pc_vault: amm_pc_vault.pubkey,
            serum_program: serum_program.pubkey,
            serum_market: serum_market.pubkey,
            serum_coin_vault: serum_coin_vault.pubkey,
            serum_pc_vault: serum_pc_vault.pubkey,
            serum_vault_signer: serum_vault_signer.pubkey,
            serum_event_queue: serum_event_queue.pubkey,
            serum_bids: serum_bids.pubkey,
            serum_asks: serum_asks.pubkey,
            amm_admin_account: amm_admin_account.pubkey,
            new_amm_open_orders_account: new_amm_open_orders_account.map(|a| a.pubkey),
        })
    }
}
