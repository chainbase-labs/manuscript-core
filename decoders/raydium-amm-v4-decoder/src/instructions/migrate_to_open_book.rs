use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x05")]
pub struct MigrateToOpenBook {}

pub struct MigrateToOpenBookInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub amm: solana_pubkey::Pubkey,
    pub amm_authority: solana_pubkey::Pubkey,
    pub amm_open_orders: solana_pubkey::Pubkey,
    pub amm_token_coin: solana_pubkey::Pubkey,
    pub amm_token_pc: solana_pubkey::Pubkey,
    pub amm_target_orders: solana_pubkey::Pubkey,
    pub serum_program: solana_pubkey::Pubkey,
    pub serum_market: solana_pubkey::Pubkey,
    pub serum_bids: solana_pubkey::Pubkey,
    pub serum_asks: solana_pubkey::Pubkey,
    pub serum_event_queue: solana_pubkey::Pubkey,
    pub serum_coin_vault: solana_pubkey::Pubkey,
    pub serum_pc_vault: solana_pubkey::Pubkey,
    pub serum_vault_signer: solana_pubkey::Pubkey,
    pub new_amm_open_orders: solana_pubkey::Pubkey,
    pub new_serum_program: solana_pubkey::Pubkey,
    pub new_serum_market: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrateToOpenBook {
    type ArrangedAccounts = MigrateToOpenBookInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, system_program, rent, amm, amm_authority, amm_open_orders, amm_token_coin, amm_token_pc, amm_target_orders, serum_program, serum_market, serum_bids, serum_asks, serum_event_queue, serum_coin_vault, serum_pc_vault, serum_vault_signer, new_amm_open_orders, new_serum_program, new_serum_market, admin, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MigrateToOpenBookInstructionAccounts {
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            amm: amm.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            amm_token_coin: amm_token_coin.pubkey,
            amm_token_pc: amm_token_pc.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            serum_program: serum_program.pubkey,
            serum_market: serum_market.pubkey,
            serum_bids: serum_bids.pubkey,
            serum_asks: serum_asks.pubkey,
            serum_event_queue: serum_event_queue.pubkey,
            serum_coin_vault: serum_coin_vault.pubkey,
            serum_pc_vault: serum_pc_vault.pubkey,
            serum_vault_signer: serum_vault_signer.pubkey,
            new_amm_open_orders: new_amm_open_orders.pubkey,
            new_serum_program: new_serum_program.pubkey,
            new_serum_market: new_serum_market.pubkey,
            admin: admin.pubkey,
        })
    }
}
