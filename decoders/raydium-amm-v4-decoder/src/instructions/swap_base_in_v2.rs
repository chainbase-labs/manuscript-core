use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x10")]
pub struct SwapBaseInV2 {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

#[derive(Debug)]
pub struct SwapBaseInV2InstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub amm: solana_pubkey::Pubkey,
    pub amm_authority: solana_pubkey::Pubkey,
    pub amm_coin_vault: solana_pubkey::Pubkey,
    pub amm_pc_vault: solana_pubkey::Pubkey,
    pub user_source_token_account: solana_pubkey::Pubkey,
    pub user_destination_token_account: solana_pubkey::Pubkey,
    pub user_source_owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapBaseInV2 {
    type ArrangedAccounts = SwapBaseInV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, amm, amm_authority, amm_coin_token_account, amm_pc_token_account, user_source_token_account, user_destination_token_account, user_source_owner, _remaining @ ..] =
            accounts
        else {
            return None;
        };
        Some(SwapBaseInV2InstructionAccounts {
            token_program: token_program.pubkey,
            amm: amm.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_coin_vault: amm_coin_token_account.pubkey,
            amm_pc_vault: amm_pc_token_account.pubkey,
            user_source_token_account: user_source_token_account.pubkey,
            user_destination_token_account: user_destination_token_account.pubkey,
            user_source_owner: user_source_owner.pubkey,
        })
    }
}
